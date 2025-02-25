// clock.rs

use crate::cmos::CMOS;
use crate::time;

const D_BEFORE_MON: [u64; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

pub fn realtime() -> f64
{
	let rtc = CMOS::new().rtc();
	let timestamp = 86400 * d_before_yr(rtc.year as u64)
			+ 86400 * d_before_mon(rtc.year as u64, rtc.month as u64)
			+ 86400 * (rtc.day - 1) as u64
			+ 3600 * rtc.hour as u64
			+ 60 * rtc.minute as u64
			+ rtc.second as u64;
	let fract = time::time_between_ticks() * (time::tick() - time::last_rtcupdate()) as f64;
	(timestamp as f64) + fract
}

pub fn uptime() -> f64
{
	time::time_between_ticks() * time::tick() as f64
}


fn d_before_yr(year: u64) -> u64
{
	(1970..year).fold(0, |days, y|
	{
		days + if leapyr(y)
		{
			366
		}
		else
		{
			365
		}
	})
}

fn d_before_mon(year: u64, month: u64) -> u64
{
	let leapd = leapyr(year) && month > 2;
	D_BEFORE_MON[(month as usize) - 1] + if leapd
	{
		1
	}
	else
	{
		0
	}
}

fn leapyr(year: u64) -> bool
{
	if year % 4 != 0
	{
		false
	}
	else if year % 100 != 0
	{
		true
	}
	else if year % 400 != 0
	{
		false
	}
	else
	{
		true
	}
}
