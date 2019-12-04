use lazy_static::*;
use rcore_fs::vfs::*;
use rcore_fs_sfs::SimpleFileSystem;
use alloc::{ sync::Arc, vec::Vec };

mod device;

lazy_static! {
    pub static ref ROOT_INODE: Arc<INode> = {
        let device = {
            extern "C" {
                fn _user_img_start();
                fn _user_img_end();
            };
            let start = _user_img_start as usize;
            let end = _user_img_end as usize;
            Arc::new(unsafe { device::MemBuf::new(start, end) })        
        };

        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        sfs.root_inode()
        
    };
}

pub trait INodeExt {
    fn read_as_vec(&self) -> Result<Vec<u8>>;
}

impl INodeExt for INode {
    fn read_as_vec(&self) -> Result<Vec<u8>> {
        let size = self.metadata()?.size;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        self.read_at(0, buf.as_mut_slice())?;
        Ok(buf)
    }
}

pub fn init() {
    let mut id = 0;
    while let Ok(name) = ROOT_INODE.get_entry(id) {
        id += 1;
        println!("{}", name);
    }
}