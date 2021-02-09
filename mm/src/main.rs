use std::marker::PhantomData;

use mm::{
    Arch, BuddyAllocator, BumpAllocator, EmulateArch, FrameAllocator, FrameCount, MemoryArea,
    PageFlush, PageFlushAll, PageMapper, PageTable, PhysicalAddress, VirtualAddress, GIGA_BYTE,
    KILO_BYTE, MEGA_BYTE, TERA_BYTE,
};

pub fn format_size(size: usize) -> String {
    if size >= 2 * TERA_BYTE {
        format!("{}TB", size / TERA_BYTE)
    } else if size >= 2 * GIGA_BYTE {
        format!("{}GB", size / GIGA_BYTE)
    } else if size >= 2 * MEGA_BYTE {
        format!("{}MB", size / MEGA_BYTE)
    } else if size >= 2 * KILO_BYTE {
        format!("{}KB", size / KILO_BYTE)
    } else {
        format!("{}B", size)
    }
}

unsafe fn dump_tables<A: Arch>(table: PageTable<A>) {
    let level = table.level();
    for i in 0..A::PAGE_ENTRIES {
        if level == 0 {
            if let Some(entry) = table.entry(i) {
                if entry.present() {
                    let base = table.entry_base(i).unwrap();
                    println!("0x{:X}: 0x{:X}", base.data(), entry.address().data())
                }
            }
        } else {
            if let Some(next) = table.next(i) {
                dump_tables(next);
            }
        }
    }
}

pub struct SlabNode<A> {
    next: PhysicalAddress,
    count: usize,
    phantom: PhantomData<A>,
}

impl<A: Arch> SlabNode<A> {
    pub fn new(next: PhysicalAddress, count: usize) -> Self {
        Self {
            next,
            count,
            phantom: PhantomData,
        }
    }

    pub fn empty() -> Self {
        Self::new(PhysicalAddress::new(0), 0)
    }

    pub unsafe fn insert(&mut self, phys: PhysicalAddress) {
        let virt = A::phys_to_virt(phys);
        A::write(virt, self.next);
        self.next = phys;
        self.count += 1;
    }

    pub unsafe fn remove(&mut self) -> Option<PhysicalAddress> {
        if self.count > 0 {
            let phys = self.next;
            let virt = A::phys_to_virt(phys);
            self.next = A::read(virt);
            self.count -= 1;
            Some(phys)
        } else {
            None
        }
    }
}

pub struct SlabAllocator<A> {
    nodes: [SlabNode<A>; 4],
    phantom: PhantomData<A>,
}

impl<A: Arch> SlabAllocator<A> {
    pub unsafe fn new(areas: &'static [MemoryArea], offset: usize) -> Self {
        let mut allocator = Self {
            nodes: [
                SlabNode::empty(),
                SlabNode::empty(),
                SlabNode::empty(),
                SlabNode::empty(),
            ],
            phantom: PhantomData,
        };
        let mut area_offset = offset;
        for area in areas.iter() {
            if area_offset < area.size {
                let area_base = area.base.add(area_offset);
                let area_size = area.size - area_offset;
                allocator.free(area_base, area_size);
                area_offset = 0;
            } else {
                area_offset -= area.size;
            }
        }
        allocator
    }

    pub unsafe fn allocator(&mut self, size: usize) -> Option<PhysicalAddress> {
        for level in 0..A::PAGE_LEVELS - 1 {
            let level_shift = level * A::PAGE_ENTRY_SHIFT + A::PAGE_SHIFT;
            let level_size = 1 << level_shift;
            if size <= level_size {
                if let Some(base) = self.nodes[level].remove() {
                    self.free(base.add(size), level_size - size);
                    return Some(base);
                }
            }
        }
    }

    pub unsafe fn free(&mut self, mut base: PhysicalAddress, mut size: usize) {
        for level in (0..A::PAGE_LEVELS - 1).rev() {
            let level_shift = level * A::PAGE_ENTRY_SHIFT + A::PAGE_SHIFT;
            let level_size = 1 << level_shift;
            while size >= level_size {
                println!("add {:X} {}", base.data(), format_size(level_size));
                self.nodes[level].insert(base);
                base = base.add(level_size);
                size -= level_size;
            }
        }
    }

    pub unsafe fn remaining(&mut self) -> usize {
        let mut remaining = 0;
        for level in (0..A::PAGE_LEVELS - 1).rev() {
            let level_shift = level * A::PAGE_ENTRY_SHIFT + A::PAGE_SHIFT;
            let level_size = 1 << level_shift;
            remaining += self.nodes[level].count * level_size;
        }
        remaining
    }
}

unsafe fn new_tables<A: Arch>(areas: &'static [MemoryArea]) {
    let mut size = 0;
    for area in areas.iter() {
        size += area.size;
    }
    println!("Memory:{}", format_size(size));
    let mut bump_allocator = BumpAllocator::<A>::new(areas, 0);
    {
        let mut mapper =
            PageMapper::<A, _>::create(&mut bump_allocator).expect("failed to create Mapper");
        for area in areas.iter() {
            for i in 0..area.size / A::PAGE_SIZE {
                let phys = area.base.add(i * A::PAGE_SIZE);
                let virt = A::phys_to_virt(phys);
                let flush = mapper
                    .map_phys(virt, phys, A::ENTRY_FLAG_WRITABLE)
                    .expect("failed to map page to frame");
                flush.ignore();
            }
        }
        mapper.make_current();
    }
    let offset = bump_allocator.offset();
    println!("Permanently used:{}", format_size(offset));
    let mut allocator = BuddyAllocator::<A>::new(bump_allocator).unwrap();
    for i in 0..16 {
        {
            let phys_opt = allocator.allocate_one();
            println!("page {}:{:X?}", i, phys_opt);
            if i % 3 == 0 {
                if let Some(phys) = phys_opt {
                    println!("free {}:{:X?}", i, phys_opt);
                    allocator.free_one(phys);
                }
            }
        }
        {
            let phys_opt = allocator.allocate(FrameCount::new(16));
            println!("page 16 {}:{:X?}", i, phys_opt);
            if i % 2 == 0 {
                if let Some(phys) = phys_opt {
                    println!("free*16 {}:{:X?}", i, phys_opt);
                    allocator.free(phys, FrameCount::new(16));
                }
            }
        }
    }

    let mut mapper = PageMapper::<A, _>::current(&mut allocator);
    let flush_all = PageFlushAll::new();
    for i in 0..16 {
        let virt = VirtualAddress::new(MEGA_BYTE + i * A::PAGE_SIZE);
        let flush = mapper
            .map(virt, A::ENTRY_FLAG_USER | A::ENTRY_FLAG_WRITABLE)
            .expect("failed tp map page");
        flush_all.consume(flush);
    }
    flush_all.flush();
    for i in 0..16 {
        let virt = VirtualAddress::new(MEGA_BYTE + i * A::PAGE_SIZE);
        let flush = mapper.unmap_phys(virt).expect("failed to unmap page");
        flush_all.consume(flush);
    }
    flush_all.flush();
    let usage = allocator.usage();
    println!("Allocator usgae:");
    println!("Used:{}", format_size(usage.used().data() * A::PAGE_SIZE));
    println!("Free:{}", format_size(usage.free().data() * A::PAGE_SIZE));
    println!("Total:{}", format_size(usage.tatal().data() * A::PAGE_SIZE));
}

unsafe fn inner<A: Arch>() {
    let areas = A::init();
    new_tables::<A>(areas);
    for i in &[1, 2, 4, 8, 16, 32] {
        let phys = PhysicalAddress::new(i * MEGA_BYTE);
        let virt = A::phys_to_virt(phys);
        // test read
        println!(
            "0x{:X} (0x{:X}) = 0x{:X}",
            virt.data(),
            phys.data(),
            A::read::<u8>(virt)
        );
        // test write
        A::write::<u8>(virt, 0x5A);
        // test read
        println!(
            "0x{:X} (0x{:X}) = 0x{:X}",
            virt.data(),
            phys.data(),
            A::read::<u8>(virt)
        );
    }
}

fn main() {
    unsafe {
        inner::<EmulateArch>();
    }
}
