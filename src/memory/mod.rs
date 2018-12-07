pub use self::area_frame_allocator::AreaFrameAllocator;

mod area_frame_allocator;
mod paging;

use self::paging::PhysicalAddress;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Frame {
    pub number: usize,
    pub address: usize,
    pub length: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE, address: address, length: PAGE_SIZE}
    }

    fn containing_number(number: usize) -> Frame {
        Frame{ number: number, address: number*PAGE_SIZE, length: PAGE_SIZE}
    }

    fn start_address(&self) -> PhysicalAddress {
        self.address
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
