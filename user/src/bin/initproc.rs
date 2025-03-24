#![no_std]
#![no_main]
use user_lib::{exec, exit, fork, wait, waitpid, yield_, shutdown};

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main());
}

#[no_mangle]
fn main() -> i32 {
    let bash_path = "/bin/bash\0";
    let environ = [
        "SHELL=/bash\0".as_ptr(),
        "PWD=/\0".as_ptr(),
        "LOGNAME=root\0".as_ptr(),
        "MOTD_SHOWN=pam\0".as_ptr(),
        "HOME=/root\0".as_ptr(),
        "LANG=C.UTF-8\0".as_ptr(),
        "TERM=vt220\0".as_ptr(),
        "USER=root\0".as_ptr(),
        "SHLVL=0\0".as_ptr(),
        "OLDPWD=/root\0".as_ptr(),
        "PS1=\x1b[1m\x1b[32mNPUCore\x1b[0m:\x1b[1m\x1b[34m\\w\x1b[0m\\$ \0".as_ptr(),
        "_=/bin/bash\0".as_ptr(),
        "PATH=/:/bin\0".as_ptr(),
        "LD_LIBRARY_PATH=/\0".as_ptr(),
        core::ptr::null(),
    ];
//     let schedule_text: &str= "
// echo aaa > lat_sig\0
// ./time-test\0
// ./busybox_testcode.sh\0
// ./lua_testcode.sh\0
// ./libctest_testcode.sh\0
// ./libc-bench\0
// ";
// ./dhry2reg 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench DHRY2 test(lps): \"$0}'\0
// ./whetstone-double 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+.[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+.[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench WHETSTONE test(MFLOPS): \"$0}'\0
// ./syscall 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SYSCALL test(lps): \"$0}'\0
// ./pipe 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench PIPE test(lps): \"$0}'\0
// ./spawn 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SPAWN test(lps): \"$0}'\0
// UB_BINDIR=./ ./execl 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench EXECL test(lps): \"$0}'\0
// ./arithoh 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench ARITHOH test(lps): \"$0}'\0
// ./short 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHORT test(lps): \"$0}'\0
// ./int 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench INT test(lps): \"$0}'\0
// ./long 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench LONG test(lps): \"$0}'\0
// ./float 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FLOAT test(lps): \"$0}'\0
// ./double 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench DOUBLE test(lps): \"$0}'\0
// ./hanoi 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench HANOI test(lps): \"$0}'\0
// ./syscall 10 exec | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench EXEC test(lps): \"$0}'\0
// ./cyclictest_testcode.sh\0
// echo latency measurements\0
// lmbench_all lat_syscall -P 1 null\0
// lmbench_all lat_syscall -P 1 read\0
// lmbench_all lat_syscall -P 1 write\0
// busybox mkdir -p /var/tmp\0
// busybox touch /var/tmp/lmbench\0
// lmbench_all lat_syscall -P 1 stat /var/tmp/lmbench\0
// lmbench_all lat_syscall -P 1 fstat /var/tmp/lmbench\0
// lmbench_all lat_syscall -P 1 open /var/tmp/lmbench\0
// lmbench_all lat_select -n 100 -P 1 filev\0
// lmbench_all lat_sig -P 1 install\0
// lmbench_all lat_sig -P 1 catch\0
// lmbench_all lat_sig -P 1 prot lat_sig\0
// lmbench_all lat_pipe -P 1\0
// lmbench_all lat_proc -P 1 fork\0
// lmbench_all lat_proc -P 1 exec\0
// busybox cp hello /tmp\0
// lmbench_all lat_proc -P 1 shell\0
// lmbench_all lmdd label=\"File /var/tmp/XXX write bandwidth:\" of=/var/tmp/XXX move=1m fsync=1 print=3\0
// lmbench_all lat_pagefault -P 1 /var/tmp/XXX\0
// lmbench_all lat_mmap -P 1 512k /var/tmp/XXX\0
// busybox echo file system latency\0
// lmbench_all lat_fs /var/tmp\0
// busybox echo Bandwidth measurements\0
// lmbench_all bw_pipe -P 1\0
// lmbench_all bw_file_rd -P 1 512k io_only /var/tmp/XXX\0
// lmbench_all bw_file_rd -P 1 512k open2close /var/tmp/XXX\0
// lmbench_all bw_mmap_rd -P 1 512k mmap_only /var/tmp/XXX\0
// lmbench_all bw_mmap_rd -P 1 512k open2close /var/tmp/XXX\0
// busybox echo context switch overhead\0
// lmbench_all lat_ctx -P 1 -s 32 2 4 8 16 24 32 64 96\0

// ./fstime -w -t 20 -b 256 -m 500 | ./busybox grep -o \"WRITE COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_WRITE_SMALL test(KBps): \"$0}'\0
// ./fstime -r -t 20 -b 256 -m 500 | ./busybox grep -o \"READ COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_READ_SMALL test(KBps): \"$0}'\0
// ./fstime -c -t 20 -b 256 -m 500 | ./busybox grep -o \"COPY COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_COPY_SMALL test(KBps): \"$0}'\0
// ./fstime -w -t 20 -b 1024 -m 2000 | ./busybox grep -o \"WRITE COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_WRITE_MIDDLE test(KBps): \"$0}'\0
// ./fstime -r -t 20 -b 1024 -m 2000 | ./busybox grep -o \"READ COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_READ_MIDDLE test(KBps): \"$0}'\0
// ./fstime -c -t 20 -b 1024 -m 2000 | ./busybox grep -o \"COPY COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_COPY_MIDDLE test(KBps): \"$0}'\0
// ./fstime -w -t 20 -b 4096 -m 8000 | ./busybox grep -o \"WRITE COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_WRITE_BIG test(KBps): \"$0}'\0
// ./fstime -r -t 20 -b 4096 -m 8000 | ./busybox grep -o \"READ COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_READ_BIG test(KBps): \"$0}'\0
// ./fstime -c -t 20 -b 4096 -m 8000 | ./busybox grep -o \"COPY COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_COPY_BIG test(KBps): \"$0}'\0
// ./looper 20 ./multi.sh 1 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHELL1 test(lpm): \"$0}'\0
// ./looper 20 ./multi.sh 8 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHELL8 test(lpm): \"$0}'\0
// ./looper 20 ./multi.sh 16 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHELL16 test(lpm): \"$0}'\0
// ";
// ./iozone_testcode.sh\0
// ./test_all.sh\0
// ./libctest_testcode.sh\0
// ./iozone_testcode.sh\0
// ./netperf_testcode.sh\0
// ./unixbench_testcode.sh\0
// ./cyclictest_testcode.sh\0
// ./iperf_testcode.sh\0
// ./lmbench_testcode.sh\0

/*./unixbench_testcode.sh\0 全文 其中context1卡住 spawn之后的卡住
./dhry2reg 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench DHRY2 test(lps): \"$0}'\0
./whetstone-double 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+.[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+.[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench WHETSTONE test(MFLOPS): \"$0}'\0
./syscall 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SYSCALL test(lps): \"$0}'\0
./context1 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox tail -n1 | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench CONTEXT test(lps): \"$0}'\0
./pipe 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench PIPE test(lps): \"$0}'\0
./spawn 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SPAWN test(lps): \"$0}'\0
UB_BINDIR=./ ./execl 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench EXECL test(lps): \"$0}'\0
./fstime -w -t 20 -b 256 -m 500 | ./busybox grep -o \"WRITE COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_WRITE_SMALL test(KBps): \"$0}'\0
./fstime -r -t 20 -b 256 -m 500 | ./busybox grep -o \"READ COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_READ_SMALL test(KBps): \"$0}'\0
./fstime -c -t 20 -b 256 -m 500 | ./busybox grep -o \"COPY COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_COPY_SMALL test(KBps): \"$0}'\0
./fstime -w -t 20 -b 1024 -m 2000 | ./busybox grep -o \"WRITE COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_WRITE_MIDDLE test(KBps): \"$0}'\0
./fstime -r -t 20 -b 1024 -m 2000 | ./busybox grep -o \"READ COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_READ_MIDDLE test(KBps): \"$0}'\0
./fstime -c -t 20 -b 1024 -m 2000 | ./busybox grep -o \"COPY COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_COPY_MIDDLE test(KBps): \"$0}'\0
./fstime -w -t 20 -b 4096 -m 8000 | ./busybox grep -o \"WRITE COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_WRITE_BIG test(KBps): \"$0}'\0
./fstime -r -t 20 -b 4096 -m 8000 | ./busybox grep -o \"READ COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_READ_BIG test(KBps): \"$0}'\0
./fstime -c -t 20 -b 4096 -m 8000 | ./busybox grep -o \"COPY COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FS_COPY_BIG test(KBps): \"$0}'\0
./looper 20 ./multi.sh 1 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHELL1 test(lpm): \"$0}'\0
./looper 20 ./multi.sh 8 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHELL8 test(lpm): \"$0}'\0
./looper 20 ./multi.sh 16 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHELL16 test(lpm): \"$0}'\0
./arithoh 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench ARITHOH test(lps): \"$0}'\0
./short 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench SHORT test(lps): \"$0}'\0
./int 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench INT test(lps): \"$0}'\0
./long 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench LONG test(lps): \"$0}'\0
./float 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench FLOAT test(lps): \"$0}'\0
./double 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench DOUBLE test(lps): \"$0}'\0
./hanoi 10 | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench HANOI test(lps): \"$0}'\0
./syscall 10 exec | ./busybox grep -o \"COUNT|[[:digit:]]\\+|\" | ./busybox grep -o \"[[:digit:]]\\+\" | ./busybox awk '{print \"Unixbench EXEC test(lps): \"$0}'\0
*/

    // let mut exit_code: i32 = 0;
    // for line in schedule_text.lines(){
    //     let argv = [
    //         bash_path.as_ptr(),
    //         "-c\0".as_ptr(),
    //         line.as_ptr(),
    //         core::ptr::null(),
    //     ];
    //     let pid = fork();
    //     if pid == 0 {
    //         exec(bash_path, &argv, &environ);
    //     } else {
    //         waitpid(pid as usize, &mut exit_code);
    //     }
    // }
    // shutdown();
    if fork() == 0 {
        exec(bash_path, &[bash_path.as_ptr() as *const u8, core::ptr::null()], &environ);
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            // ECHLD is -10
            if pid == -10 {
                yield_();
                continue;
            }
            // user_lib::println!(
            //     "[initproc] Released a zombie process, pid={}, exit_code={}",
            //     pid,
            //     exit_code,
            // );
        }
    }
    
    0
}
