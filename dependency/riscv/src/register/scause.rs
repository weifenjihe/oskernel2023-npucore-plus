//! scause register

use bit_field::BitField;
use core::mem::size_of;

/// scause register
#[derive(Clone, Copy)]
pub struct Scause {
    bits: usize,
}

/// Trap Cause
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}
impl Trap {
    /// Is trap cause a timer.
    #[inline]
    pub fn is_timer(&self) -> bool {
        if let Trap::Interrupt(Interrupt::SupervisorTimer) = self {
            true
        } else {
            false
        }
    }

    /// Is trap cause a syscall.
    #[inline]
    pub fn is_syscall(&self) -> bool {
        if let Trap::Exception(Exception::UserEnvCall) = self {
            true
        } else {
            false
        }
    }
}
/// Interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    UserSoft,
    VirtualSupervisorSoft,
    SupervisorSoft,
    UserTimer,
    VirtualSupervisorTimer,
    SupervisorTimer,
    UserExternal,
    VirtualSupervisorExternal,
    SupervisorExternal,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    VirtualSupervisorEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    InstructionGuestPageFault,
    LoadGuestPageFault,
    VirtualInstruction,
    StoreGuestPageFault,
    Unknown,
}

impl Interrupt {
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Interrupt::UserSoft,
            1 => Interrupt::SupervisorSoft,
            2 => Interrupt::VirtualSupervisorSoft,
            4 => Interrupt::UserTimer,
            5 => Interrupt::SupervisorTimer,
            6 => Interrupt::VirtualSupervisorTimer,
            8 => Interrupt::UserExternal,
            9 => Interrupt::SupervisorExternal,
            10 => Interrupt::VirtualSupervisorExternal,
            _ => Interrupt::Unknown,
        }
    }
}

impl Exception {
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Exception::InstructionMisaligned,
            1 => Exception::InstructionFault,
            2 => Exception::IllegalInstruction,
            3 => Exception::Breakpoint,
            5 => Exception::LoadFault,
            6 => Exception::StoreMisaligned,
            7 => Exception::StoreFault,
            8 => Exception::UserEnvCall,
            10 => Exception::VirtualSupervisorEnvCall,
            12 => Exception::InstructionPageFault,
            13 => Exception::LoadPageFault,
            15 => Exception::StorePageFault,
            20 => Exception::InstructionGuestPageFault,
            21 => Exception::LoadGuestPageFault,
            22 => Exception::VirtualInstruction,
            23 => Exception::StoreGuestPageFault,
            _ => Exception::Unknown,
        }
    }
}

impl Scause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the code field
    pub fn code(&self) -> usize {
        let bit = 1 << (size_of::<usize>() * 8 - 1);
        self.bits & !bit
    }

    /// Trap Cause
    #[inline]
    pub fn cause(&self) -> Trap {
        if self.is_interrupt() {
            Trap::Interrupt(Interrupt::from(self.code()))
        } else {
            Trap::Exception(Exception::from(self.code()))
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        self.bits.get_bit(size_of::<usize>() * 8 - 1)
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }
}

read_csr_as!(Scause, 0x142, __read_scause);
write_csr!(0x142, __write_scause);

/// Writes the CSR
#[inline]
pub unsafe fn write(bits: usize) {
    _write(bits)
}

/// Set supervisor cause register to corresponding cause.
#[inline]
pub unsafe fn set(cause: Trap) {
    let bits = match cause {
        Trap::Interrupt(i) => {
            (match i {
                Interrupt::UserSoft => 0,
                Interrupt::SupervisorSoft => 1,
                Interrupt::VirtualSupervisorSoft => 2,
                Interrupt::UserTimer => 4,
                Interrupt::SupervisorTimer => 5,
                Interrupt::VirtualSupervisorTimer => 6,
                Interrupt::UserExternal => 8,
                Interrupt::SupervisorExternal => 9,
                Interrupt::VirtualSupervisorExternal => 10,
                Interrupt::Unknown => panic!("unknown interrupt"),
            } | (1 << (size_of::<usize>() * 8 - 1)))
        } // interrupt bit is 1
        Trap::Exception(e) => match e {
            Exception::InstructionMisaligned => 0,
            Exception::InstructionFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::Breakpoint => 3,
            Exception::LoadFault => 5,
            Exception::StoreMisaligned => 6,
            Exception::StoreFault => 7,
            Exception::UserEnvCall => 8,
            Exception::VirtualSupervisorEnvCall => 10,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StorePageFault => 15,
            Exception::InstructionGuestPageFault => 20,
            Exception::LoadGuestPageFault => 21,
            Exception::VirtualInstruction => 22,
            Exception::StoreGuestPageFault => 23,
            Exception::Unknown => panic!("unknown exception"),
        }, // interrupt bit is 0
    };
    _write(bits);
}
