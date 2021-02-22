use core::slice;

use crate::data::*;
use crate::error::*;
use crate::number::*;

pub trait Scheme {
    fn handle(&self, packet: &mut Packet) {
        let res = match packet.a {
            SYS_OPEN => self.open(
                unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) },
                packet.d,
                packet.uid,
                packet.gid,
            ),
            SYS_CHMOD => self.chmod(
                unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) },
                packet.d as u16,
                packet.uid,
                packet.gid,
            ),
            _ => Err(Error::new(ENOSYS)),
        };
        packet.a = Error::mux(res);
    }

    #[allow(unused_variables)]
    fn open(&self, path: &[u8], flags: usize, uid: u32, gid: u32) -> Result<usize> {
        Err(Error::new(ENOENT))
    }

    #[allow(unused_variables)]
    fn chmod(&self, path: &[u8], mode: u16, uid: u32, gid: u32) -> Result<usize> {
        Err(Error::new(ENOENT))
    }
}
