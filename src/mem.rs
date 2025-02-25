#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_features)]

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB};


pub unsafe fn init(physmem_offset: VirtAddr) -> OffsetPageTable<'static>
{
	let lvl4_tab = active_lvl4_tab(physmem_offset);
	OffsetPageTable::new(lvl4_tab, physmem_offset)
}


unsafe fn active_lvl4_tab(physmem_offset: VirtAddr) -> &'static mut PageTable
{
	use x86_64::registers::control::Cr3;
	let(lvl4_tab_frame, _) = Cr3::read();
	let phys = lvl4_tab_frame.start_address();
	let virt = physmem_offset + phys.as_u64();
	let pagetab_ptr: *mut PageTable = virt.as_mut_ptr();
	&mut *pagetab_ptr
}


// This creates an example mapping to frame 0xb8000.
pub fn new_example_mapping(page: Page, mapper: &mut OffsetPageTable, framealloc: &mut impl FrameAllocator<Size4KiB>)
{
	use x86_64::structures::paging::PageTableFlags as Flags;
	let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
	let flags = Flags::PRESENT | Flags::WRITABLE;
	let map_to_result = unsafe
	{
		mapper.map_to(page, frame, flags, framealloc)
	};
	map_to_result.expect("[ERR] MAP_TO FAILURE").flush();
}


// This is a FrameAllocator that will always return a value of "None".
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator
{
	fn allocate_frame(&mut self) -> Option<PhysFrame>
	{
		None
	}
}


// This is a FrameAllocator that will return any usable frames from the memory map of the bootloader.
pub struct BootInfoFrameAllocator
{
	memmap: &'static MemoryMap,
	next: usize,
}


impl BootInfoFrameAllocator
{
	// This creates a FrameAllocator from a memory map.
	pub unsafe fn init(memmap: &'static MemoryMap) -> Self
	{
		BootInfoFrameAllocator
		{
			memmap,
			next: 0,
		}
	}

	// This returns an iterator over usable frames, as specified by the memory map.
	fn usableframes(&self) -> impl Iterator<Item = PhysFrame>
	{
		// This figures out which regions, from the memory map, are usable.
		let regions = self.memmap.iter();
		let usableregions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

		// This maps each region to its respective address range.
		let addr_ranges = usableregions.map(|r| r.range.start_addr()..r.range.end_addr());

		// This transforms to an iterator of the frame start addresses.
		let frameaddr = addr_ranges.flat_map(|r| r.step_by(4096));

		// This creates a PhysFrame type from each start address.
		frameaddr.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
	}
}


unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator
{
	fn allocate_frame(&mut self) -> Option<PhysFrame>
	{
		let frame = self.usableframes().nth(self.next);
		self.next += 1;
		frame
	}
}
