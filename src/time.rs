// time.rs

use crate::cmos::CMOS;
use core::hint::spin_loop;
use core::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use x86_64::instructions::{interrupts, port::Port};

const PITDIV: usize = 1193;
pub const PITFREQ: f64 = 3_579_545.0 / 3.0;
const PITINTV: f64 = (PITDIV as f64) / PITFREQ;

static PIT_TICK: AtomicUsize = AtomicUsize::new(0);
static LAST_RTCUPDATE: AtomicUsize = AtomicUsize::new(0);
static CLOCK_PER_NS: AtomicU64 = AtomicU64::new(0);

pub fn tick() -> usize
{
	PIT_TICK.load(Ordering::Relaxed)
}

pub fn time_between_ticks() -> f64
{
	PITINTV
}

pub fn last_rtcupdate() -> usize
{
	LAST_RTCUPDATE.load(Ordering::Relaxed)
}

pub fn halt()
{
	let disabled = !interrupts::are_enabled();
	interrupts::enable_and_hlt();
	if disabled
	{
		interrupts::disable();
	}
}

fn rdtsc() -> u64
{
	unsafe
	{
		core::arch::x86_64::_mm_lfence();
		core::arch::x86_64::_rdtsc()
	}
}

pub fn sleep(nsec: u64)
{
	let start = rdtsc();
	let delta = nsec * CLOCK_PER_NS.load(Ordering::Relaxed);
	while rdtsc() - start < delta
	{
		spin_loop();
	}
}

pub fn nwait(nsec: u64)
{
	let start = rdtsc();
	let delta = nsec * CLOCK_PER_NS.load(Ordering::Relaxed);
	while rdtsc() - start < delta
	{
		spin_loop();
	}
}

pub fn set_pitfreq_div(divider: u16, channel: u8)
{
	interrupts::without_interrupts(||
	{
		let bytes = divider.to_le_bytes();
		let mut cmd: Port<u8> = Port::new(0x43);
		let mut data: Port<u8> = Port::new(0x40 + channel as u16);
		let opmode = 6;
		let accmode = 3;
		unsafe
		{
			cmd.write((channel << 6) | (accmode << 4) | opmode);
			data.write(bytes[0]);
			data.write(bytes[1]);
		}
	});
}

pub fn pit_intrhandler()
{
	PIT_TICK.fetch_add(1, Ordering::Relaxed);
}

pub fn rtc_intrhandler()
{
	LAST_RTCUPDATE.store(tick(), Ordering::Relaxed);
	CMOS::new().notify_intrend();
}
