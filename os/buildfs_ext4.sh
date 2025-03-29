#!/bin/bash
# buildfs_ext4.sh
# 用于生成 ext4 文件系统镜像并写入文件系统内容

# 定义文件系统镜像的目录和镜像文件
U_EXT4_DIR="../easy-fs-ext4"
U_EXT4=$1

# 确保镜像文件存在（touch 会新建空文件）
touch ${U_EXT4}

# 生成200MB大小的空镜像文件
sudo dd if=/dev/zero of=${U_EXT4} bs=1M count=200

# 格式化为 ext4 文件系统
sudo mkfs.ext4 -F ${U_EXT4}

# 显示镜像文件的分区信息（可选）
sudo fdisk -l ${U_EXT4}

# 清除旧的挂载目录（如果存在），然后新建挂载目录
if [ -d ${U_EXT4_DIR}/fs_ext4 ]; then 
    sudo rm -rf ${U_EXT4_DIR}/fs_ext4
fi
sudo mkdir -p ${U_EXT4_DIR}/fs_ext4

# 挂载镜像文件（使用 loop 设备）
sudo mount -o loop ${U_EXT4} ${U_EXT4_DIR}/fs_ext4

# 创建基本根文件系统目录结构
sudo mkdir -p ${U_EXT4_DIR}/fs_ext4/etc
sudo mkdir -p ${U_EXT4_DIR}/fs_ext4/bin
sudo mkdir -p ${U_EXT4_DIR}/fs_ext4/root

# 创建 /etc/passwd 文件，写入 root 用户信息
sudo sh -c "echo -e 'root:x:0:0:root:/root:/bash\n' > ${U_EXT4_DIR}/fs_ext4/etc/passwd"
# 创建 .bash_history 文件
sudo touch ${U_EXT4_DIR}/fs_ext4/root/.bash_history

# 复制用户程序到根文件系统中
for programname in $(ls ../user/src/bin)
do
    sudo cp -r ../user/target/riscv64gc-unknown-none-elf/release/${programname%.rs} ${U_EXT4_DIR}/fs_ext4/${programname%.rs}
done

# for programname in $(ls ../user/riscv64)
# do
#     sudo cp -r ../user/riscv64/$programname ${U_EXT4_DIR}/fs_ext4/
# done

# for programname in $(ls -A ../user/busybox_lua_testsuites)
# do
#     sudo cp -r ../user/busybox_lua_testsuites/$programname ${U_EXT4_DIR}/fs_ext4/
# done

# 卸载镜像，完成文件系统构建
sudo umount ${U_EXT4_DIR}/fs_ext4
