mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;
mod vmm;

use address::VPNRange;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use memory_set::{memory_alloc, memory_free, remap_test};
pub use memory_set::{kernel_token, MapPermission, MemorySet, KERNEL_SPACE, MapType};
use page_table::PTEFlags;
pub use page_table::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, PageTable,
    PageTableEntry, UserBuffer, UserBufferIterator,
};
pub use vmm::{FRAME_QUE, P2V_MAP, do_pgfault};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
