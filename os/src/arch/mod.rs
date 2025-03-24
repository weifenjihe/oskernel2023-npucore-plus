#[cfg(feature = "rv64")]
mod rv64;
#[cfg(feature = "rv64")]
pub use rv64::{
    board::BlockDeviceImpl,
    board::MMIO,
    bootstrap_init, config, console_flush, console_getchar, console_putchar, machine_init,
    set_timer, shutdown,
    time::{get_clock_freq, get_time, TICKS_PER_SEC},
    PageTableImpl, __switch, syscall_id, tlb_invalidate,
    trap::{
        self, get_bad_instruction, get_exception_cause, ExceptionImpl, InterruptImpl, TrapImpl,
    },
    BLOCK_SZ,
};
