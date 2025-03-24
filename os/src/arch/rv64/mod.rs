#[cfg(feature = "board_k210")]
#[path = "board/k210.rs"]
pub mod board;
#[cfg(feature = "board_fu740")]
#[path = "board/fu740.rs"]
pub mod board;
#[cfg(all(not(feature = "board_k210"), not(feature = "board_fu740")))]
#[path = "board/qemu.rs"]
pub mod board;
pub mod config;
mod sbi;
#[cfg(feature = "board_k210")]
mod sdcard;
pub mod sv39;
pub mod switch;
pub mod syscall_id;
pub mod time;
pub mod trap;
pub type PageTableImpl = sv39::Sv39PageTable;
pub use sbi::{console_flush, console_getchar, console_putchar, set_timer, shutdown};
pub use sv39::tlb_invalidate;
pub use switch::__switch;
pub fn machine_init() {
    #[cfg(feature = "board_fu740")]
    board::clock_init();
    trap::init();
    trap::enable_timer_interrupt();
    time::set_next_trigger();
}
pub fn bootstrap_init() {}
pub const BLOCK_SZ: usize = 512;
