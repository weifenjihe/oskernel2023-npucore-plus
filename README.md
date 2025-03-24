# Welcome to NPUcore+

# 运行调试内核

## 解压 sdcard.img.gz

gunzip –c sdcard.img.gz > sdcard.img

## 运行命令

make run

## 调试命令

make all MODE=debug

qemu-system-riscv64 -machine virt -kernel kernel-qemu -m 128M -nographic -smp 2 -bios sbi-qemu -drive file=sdcard.img,if=none,format=raw,id=x0  -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -device virtio-net-device,netdev=net -netdev user,id=net -S -s

> **环境依赖：**
>
> - vscode安装c/c++插件
> - 安装RiscV对应的GDB调试器:
> ```bash
> mkdir tmp
> cd /tmp
> wget https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14.tar.gz
> tar -zxf riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14.tar.gz
> cd riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14/bin
> sudo cp ./* /usr/local/bin/
> cd /usr/local/bin/
> sudo chmod 777 ./*
> ```



# 开发人员

队伍：NPUcore+，西北工业大学

队员：黄培源（进程管理），李宇洋（文件系统），郭睆（内存管理）

# 文档

1. [系统调用](docs/FileSystem.md)
2. [内存管理](docs/MemoryManage.md)
3. [进程管理](docs/ProcessManage.md)
4. [文件系统](docs/FileSystem.md)
5. [适配评测](docs/ForOJ.md)