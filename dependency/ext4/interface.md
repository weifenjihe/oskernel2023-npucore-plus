# Interface

|      | lwext4-c                                                 | lwext4-rs                                                                                               |
| ---- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| ✔️ | `ext4_device_register`/`ext4_device_unregister`      | `RegisterHandle::register` / `drop`                                                                 |
| ✔️ | `ext4_mount`/`ext4_umount`                           | `MountHandle::mount` / `drop`                                                                       |
| ✔️ | `ext4_journal_start`/`ext4_journal_stop`             | `FileSystem::new` / `drop`                                                                          |
| ✔️ | `ext4_recover`                                         | `MountHandle::mount`                                                                                  |
| ✔️ | `ext4_mount_point_stats`                               | `MountHandle::stats`                                                                                  |
| ✔️ | `ext4_cache_write_back`                                | `FileSystem::new` / `drop`                                                                          |
| ✔️ | `ext4_cache_flush`                                     | `File::flush`                                                                                         |
| ✔️ | `ext4_fremove`                                         | `FileSystem::remove_file`                                                                             |
| ✔️ | `ext4_flink`                                           | `FileSystem::hard_link`                                                                               |
| ✔️ | `ext4_frename`                                         | `FileSystem::rename`                                                                                  |
| ✔️ | `ext4_fopen`/`ext4_fopen2`/`ext4_fclose`           | `OpenOptions::open` / `drop`                                                                        |
| ✔️ | `ext4_ftruncate`                                       | `File::set_len`                                                                                       |
| ✔️ | `ext4_fread`                                           | `File::read`                                                                                          |
| ✔️ | `ext4_fwrite`                                          | `File::write`                                                                                         |
| ✔️ | `ext4_fseek`                                           | `File::seek`                                                                                          |
| ✔️ | `ext4_raw_inode_fill`                                  | `File::metedata` / `FileSystem::metedata`                                                           |
| ✔️ | `ext4_inode_exist`                                     | `FileSystem::exists`                                                                                  |
| ✔️ | `ext4_mode_set`                                        | `File::set_permissions` / `FileSystem::set_permissions`                                             |
| ✔️ | `ext4_atime_set`/`ext4_mtime_set`/`ext4_ctime_set` | `File::set_times` / `File::set_modified` / `FileSystem::set_times` / `FileSystem::set_modified` |
| ✔️ | `ext4_fsymlink`                                        | `FileSystem::soft_link`                                                                               |
| ✔️ | `ext4_readlink`                                        | `FileSystem::read_link`                                                                               |
| ✔️ | `ext4_mknod`                                           | `FileSystem::mknod`                                                                                   |
| ✔️ | `ext4_setxattr`                                        | `FileSytem::set_xattr`                                                                                |
| ✔️ | `ext4_getxattr`                                        | `FileSystem::get_xattr`                                                                               |
| ✔️ | `ext4_listxattr`                                       | `FileSystem::list_xattr`                                                                              |
| ❕   | `ext4_removexattr`                                     | `FileSystem::remove_xattr`                                                                            |
| ✔️ | `ext4_dir_rm`                                          | `FileSystem::remove_dir` / `FileSystem::remove_dir_all`                                             |
| ✔️ | `ext4_dir_mv`                                          | `FileSystem::rename`                                                                                  |
| ✔️ | `ext4_dir_mk`                                          | `FileSystem::create_dir` / `FileSystem::create_dir_all`                                             |
| ✔️ | `ext4_dir_open`/`ext4_dir_close`                     | `FileSystem::readdir`                                                                                 |
| ✔️ | `ext4_dir_entry_next`                                  | `ReadDir::next`                                                                                       |
| ✔️ | `ext4_dir_entry_rewind`                                | `ReadDir::rewind`                                                                                     |
| ✔️ | `ext4_owner_set`                                       | `FileSystem::chown`                                                                                   |
| ✔️ | `ext4_ftell`                                           | `FileSystem::stream_position`                                                                         |
