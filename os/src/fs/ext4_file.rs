use embedded_io::{Read, SeekFrom, Write, Seek};
use alloc::format;
use alloc::vec;
use crate::fs::String;
use crate::alloc::string::ToString;
use super::{file_trait::File, DiskInodeType, layout::StatMode};

impl File for lwext4_rs::File {
    fn deep_clone(&self) -> alloc::sync::Arc<dyn File> {
        let cloned = lwext4_rs::File {
            file: self.file,
            is_file: self.is_file,
            path: self.path.clone(),
        };
        alloc::sync::Arc::new(cloned)
    }

    fn readable(&self) -> bool {
        (self.file.flags % 2) == 0
    }

    fn writable(&self) -> bool {
        (self.file.flags % 2) == 1
    }
    fn read(&self, offset: Option<&mut usize>, buf: &mut [u8]) -> usize {
        let mut local_offset: usize = offset.map(|r| *r).unwrap_or(self.file.fpos as usize);
        let mut read_buf = buf;
        let mut read_size = 0;
        
        // 读取数据
        while local_offset < self.file.fsize as usize && read_buf.len() > 0 {
            let bytes_read = self.read(Some(&mut local_offset), read_buf);
            if bytes_read == 0 {
                break; // 没有更多数据可读
            }
            read_buf = &mut read_buf[bytes_read..];
            read_size += bytes_read;
        }
        
        read_size
    }
    
    fn write(&self, offset: Option<&mut usize>, buf: &[u8]) -> usize {
        let mut local_offset: usize = offset.map(|r| *r).unwrap_or(self.file.fpos as usize);
        let mut write_buf = buf;
        let mut write_size = 0;
        
        // 写入数据
        while local_offset < self.file.fsize as usize && write_buf.len() > 0 {
            let bytes_written = self.write(Some(&mut local_offset), write_buf);
            if bytes_written == 0 {
                break; // 没有更多数据可写
            }
            write_buf = &write_buf[bytes_written..];
            write_size += bytes_written;
        }
        
        write_size
    }
    
    fn r_ready(&self) -> bool {
        match self.file.fsize.checked_sub(self.file.fpos) {
            Some(remaining) => remaining > 0,
            None => false, // 出现整数溢出，认为不可读
        }
    }
    
    fn w_ready(&self) -> bool {
        // 可以根据具体情况扩展，比如文件是否只读、存储空间是否足够
        !self.file.flags & 0x1 == 0 // 这里假设 flags 的最低位表示是否可写
    }
    
    
    fn read_user(&self, offset: Option<usize>, mut user_buf: crate::mm::UserBuffer) -> usize {
        let mut local_offset: usize = offset.unwrap_or(self.file.fpos as usize);
        let mut buf_slice = user_buf.as_mut_slice();
        let mut read_size = 0;
        
        // 读取数据
        while local_offset < self.file.fsize as usize && buf_slice.len() > 0 {
            let bytes_read = self.read(Some(&mut local_offset), buf_slice);
            if bytes_read == 0 {
                break; // 没有更多数据可读
            }
            buf_slice = &mut buf_slice[bytes_read..];
            read_size += bytes_read;
        }
        
        read_size
    }
    
    fn write_user(&self, offset: Option<usize>, mut user_buf: crate::mm::UserBuffer) -> usize {
        let mut local_offset: usize = offset.unwrap_or(self.file.fpos as usize);
        let mut buf_slice = user_buf.as_mut_slice();
        let mut write_size = 0;
        
        // 写入数据
        while local_offset < self.file.fsize as usize && buf_slice.len() > 0 {
            let bytes_written = self.write(Some(&mut local_offset), buf_slice);
            if bytes_written == 0 {
                break; // 没有更多数据可写
            }
            buf_slice = &mut buf_slice[bytes_written..];
            write_size += bytes_written;
        }
        
        write_size
    }
    
    fn get_size(&self) -> usize {
        self.file.fsize as usize
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
        // 这里可以根据 flags 和 special_use 参数来决定打开方式
        // 示例中简单通过 deep_clone 创建新实例
        self.deep_clone()
    }

    fn open_subfile(
        &self,
    ) -> Result<alloc::vec::Vec<(alloc::string::String, alloc::sync::Arc<dyn File>)>, isize> {
        // 如果当前文件不是目录，则返回错误
        if self.get_file_type() != super::DiskInodeType::from_char('d') {
            return Err(-1); // 非目录错误码，可根据需求修改
        }
        
        // 假设 get_dirent 返回一个包含目录项信息的 Vec，且每个目录项包含一个 name 字段
        let dirents = self.get_dirent(64);
        let mut subfiles = alloc::vec::Vec::new();
        
        for entry in dirents {
            // 用只读方式打开子文件，此处用 open 重新打开目录项对应的文件，
            // special_use 固定为 false，可根据实际需求调整
            let file = self.open(super::OpenFlags::O_RDONLY, false);
            subfiles.push((String::from_utf8_lossy(&entry.d_name).trim_end_matches('\0').to_string(), file));
        }
        
        Ok(subfiles)
    }

    fn create(
        &self,
        name: &str,
        file_type: DiskInodeType,
    ) -> Result<alloc::sync::Arc<dyn File>, isize> {
        // 只有目录才能创建子文件或子目录
        if self.get_file_type() != DiskInodeType::from_char('d') {
            return Err(-1); // 非目录错误
        }
    
        // 构造子文件路径，假设父路径与子名称以 '/' 分隔
        let mut child_path = self.path().to_string(); // 使用已有的 path() 方法获取路径字符串
        if !child_path.ends_with('/') {
            child_path.push('/');
        }
        child_path.push_str(name);
    
        // 根据 file_type 判断创建文件还是目录
        let is_file = file_type == DiskInodeType::from_char('-');
    
        // 调用 OpenOptions 创建新文件／目录
        let new_file = lwext4_rs::OpenOptions::new()
            .open(&child_path, is_file)
            .map_err(|e| e as isize)?;
    
        Ok(alloc::sync::Arc::new(new_file))
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
        let mut dirents = alloc::vec::Vec::new();
        // 模拟返回 count 个目录项，实际实现应从文件系统中读取目录项信息
        for i in 0..count {
            let name = format!("entry{}", i);
            let dirent = super::Dirent::new(
                i,       // 模拟 inode 编号
                0,       // 模拟目录项偏移量
                b'd',    // 模拟类型，这里假定都是目录（'d'）
                &name,
            );
            dirents.push(dirent);
        }
        dirents
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
