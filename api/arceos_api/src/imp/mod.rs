mod mem;
mod task;

cfg_fs! {
    mod fs;
    pub use fs::*;
}

cfg_net! {
    mod net;
    pub use net::*;
}

mod sys {
    pub use axhal::misc::terminate as ax_terminate;
}

mod time {
    pub use axhal::time::{current_time as ax_current_time, TimeValue as AxTimeValue};
}

mod stdio {
    use core::fmt;

    pub fn ax_console_read_byte() -> Option<u8> {
        axhal::console::getchar().map(|c| if c == b'\r' { b'\n' } else { c })
    }

    pub fn ax_console_write_bytes(buf: &[u8]) -> crate::AxResult<usize> {
        axhal::console::write_bytes(buf);
        Ok(buf.len())
    }

    pub fn ax_console_write_fmt(args: fmt::Arguments) -> fmt::Result {
        axlog::print_fmt(args)
    }
}

pub use mem::*;
pub use stdio::*;
pub use sys::*;
pub use task::*;
pub use time::*;
