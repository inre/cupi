use mmap;
use libc;
use std::fs::{OpenOptions, File};
use std::os::unix::io::AsRawFd;
use bcm270x::BLOCK_SIZE;
use {Register, RegisterDesc, Result};

pub struct SystemMemory(File);
pub struct MemoryMap(mmap::MemoryMap);

unsafe impl Send for MemoryMap {}

impl SystemMemory {
    pub fn new() -> Result<SystemMemory> {
        let f = try!(OpenOptions::new().read(true).write(true).open("/dev/mem"));
        // construct struct
        Ok(SystemMemory(f))
    }

    pub unsafe fn mmap(&self, base: usize) -> Result<MemoryMap> {
        let mem_map = try!(mmap::MemoryMap::new(BLOCK_SIZE, &[
            mmap::MapOption::MapReadable,
            mmap::MapOption::MapWritable,
            mmap::MapOption::MapFd(self.0.as_raw_fd()),
            mmap::MapOption::MapOffset(base),
            mmap::MapOption::MapNonStandardFlags(libc::MAP_SHARED)
        ]));
        Ok(MemoryMap(mem_map))
    }
}

impl MemoryMap {
    #[inline(always)]
    pub unsafe fn offset<S>(&self, offset: isize) -> *mut S {
        (self.0.data() as *const S).offset(offset) as *mut S
    }

    #[inline(always)]
    pub fn register<R: RegisterDesc>(&self, desc: R) -> Register<R> {
        unsafe { Register::<R> { ptr: self.offset(desc.offset() as isize), desc: desc } }
    }
}
