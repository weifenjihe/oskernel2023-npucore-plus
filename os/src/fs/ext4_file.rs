use embedded_io::Read;

use super::{file_trait::File, DiskInodeType, layout::StatMode};

impl File for lwext4_rs::File {
    fn deep_clone(&self) -> alloc::sync::Arc<dyn File> {
        todo!()
    }

    fn readable(&self) -> bool {
        (self.file.flags % 2) == 0
    }

    fn writable(&self) -> bool {
        (self.file.flags % 2) == 1
    }

    fn read(&self, offset: Option<&mut usize>, buf: &mut [u8]) -> usize {
        todo!()
    }

    fn write(&self, offset: Option<&mut usize>, buf: &[u8]) -> usize {
        todo!()
    }

    fn r_ready(&self) -> bool {
        todo!()
    }

    fn w_ready(&self) -> bool {
        todo!()
    }

    fn read_user(&self, offset: Option<usize>, buf: crate::mm::UserBuffer) -> usize {
        todo!()
    }

    fn write_user(&self, offset: Option<usize>, buf: crate::mm::UserBuffer) -> usize {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }

    fn get_stat(&self) -> super::Stat {
        todo!()
    }

    fn get_statx(&self) -> super::Statx {
        todo!()
    }

    fn get_file_type(&self) -> DiskInodeType {
        if self.is_file {
            DiskInodeType::from_char('-')
        }
        else{
            DiskInodeType::from_char('d')
        }
    }

    fn info_dirtree_node(
        &mut self,
        dirnode_ptr: alloc::sync::Weak<super::directory_tree::DirectoryTreeNode>,
    ) {
        self.file.inode = dirnode_ptr.as_ptr() as u32;
    }

    fn get_dirtree_node(
        &self,
    ) -> Option<alloc::sync::Arc<super::directory_tree::DirectoryTreeNode>> {
        // alloc::sync::Arc::new(self.dir.de.inode as *const u32)
        todo!()
    }

    fn open(&self, flags: super::OpenFlags, special_use: bool) -> alloc::sync::Arc<dyn File> {
        todo!()
    }

    fn open_subfile(
        &self,
    ) -> Result<alloc::vec::Vec<(alloc::string::String, alloc::sync::Arc<dyn File>)>, isize> {
        let path = &self.path.as_ptr();
        if(path.is_null()){
            return Err(-1);
        }
        let result = unsafe {
            let mut file = lwext4_rs::File::new();
            let mut files = alloc::vec::Vec::new();
            let mut dirent = lwext4_rs::Dirent::new();
            let mut i = 0;
            loop {
                let ret = lwext4_rs::ext4_readdir(&mut dirent, path, i);
                if ret == 0 {
                    break;
                }
                if dirent.name.is_null() {
                    break;
                }
                let name = unsafe { core::ffi::CStr::from_ptr(dirent.name).to_str().unwrap() };
                let mut file = lwext4_rs::File::new();
                unsafe {
                    lwext4_rs::ext4_fopen2(&mut file.file, dirent.name, 0);
                }
                files.push((alloc::string::String::from(name), file));
                i += 1;
            }
            files
        };
        Ok(result)
    }

    fn create(
        &self,
        name: &str,
        file_type: DiskInodeType,
    ) -> Result<alloc::sync::Arc<dyn File>, isize> {
        todo!()
    }

    fn link_child(&self, name: &str, child: &Self) -> Result<(), isize>
    where
        Self: Sized,
    {
        todo!()
    }

    fn unlink(&self, delete: bool) -> Result<(), isize> {
        todo!()
    }

    fn get_dirent(&self, count: usize) -> alloc::vec::Vec<super::Dirent> {
        todo!()
    }

    fn lseek(&self, offset: isize, whence: super::SeekWhence) -> Result<usize, isize> {
        todo!()
    }

    fn modify_size(&self, diff: isize) -> Result<(), isize> {
        todo!()
    }

    fn truncate_size(&self, new_size: usize) -> Result<(), isize> {
        todo!()
    }

    fn set_timestamp(&self, ctime: Option<usize>, atime: Option<usize>, mtime: Option<usize>) {
        todo!()
    }

    fn get_single_cache(
        &self,
        offset: usize,
    ) -> Result<alloc::sync::Arc<spin::Mutex<super::cache::PageCache>>, ()> {
        todo!()
    }

    fn get_all_caches(
        &self,
    ) -> Result<alloc::vec::Vec<alloc::sync::Arc<spin::Mutex<super::cache::PageCache>>>, ()> {
        todo!()
    }

    fn oom(&self) -> usize {
        todo!()
    }

    fn hang_up(&self) -> bool {
        todo!()
    }

    fn fcntl(&self, cmd: u32, arg: u32) -> isize {
        todo!()
    }
}
