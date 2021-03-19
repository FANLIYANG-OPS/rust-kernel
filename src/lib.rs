use log::info;

pub mod syscall;
pub mod time;
static mut INIT_ENV: &[u8] = &[];

pub extern "C" fn userspace_init() {
    let path = b"initfs:/bin/init";
    let env = unsafe { INIT_ENV };
    if let Err(err) = syscall::chdir(b"initfs") {
        info!("Failed to enter initfs ({}).", err);
        info!("Perhaps the kernel was compiled with an incorrect INITFS_FOLDER environment variable value?");
        panic!("Unexpected error while trying to enter initfs: .");
    }
}





