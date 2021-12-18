// syscall/num.rs

pub const EXIT:		usize = 0x1;
pub const SPWN:		usize = 0x2;
pub const READ:		usize = 0x3;
pub const WRITE:	usize = 0x4;
pub const OPEN:		usize = 0x5;
pub const CLOSE:	usize = 0x6;
pub const STAT:		usize = 0x7;
pub const DUP:		usize = 0x8;
pub const SLEEP:	usize = 0x9;
pub const UPTIME:	usize = 0xA;
pub const REALTIME:	usize = 0xB;
