use super::BlockDevice;
use crate::mm::{
    frame_alloc, frame_dealloc, kernel_token, FrameTracker, PageTable,  PageTableImpl,
    PhysAddr, PhysPageNum, StepByOne, VirtAddr,
};
use alloc::{sync::Arc, vec::Vec};
use lazy_static::*;
use spin::Mutex;
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

const VIRT_IO_BLOCK_SZ: usize = 512;
const BLOCK_SIZE: usize = 1024; // ext4 块大小

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock(Mutex<VirtIOBlk<'static>>);

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<Arc<FrameTracker>>> = Mutex::new(Vec::new());
}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, mut block_id: usize, buf: &mut [u8]) {
        for buf in buf.chunks_mut(VIRT_IO_BLOCK_SZ) {
            self.0
                .lock()
                .read_block(block_id, buf)
                .expect("Error when reading VirtIOBlk");
            block_id += 1;
        }
    }

    fn write_block(&self, mut block_id: usize, buf: &[u8]) {
        for buf in buf.chunks(VIRT_IO_BLOCK_SZ) {
            self.0
                .lock()
                .write_block(block_id, buf)
                .expect("Error when writing VirtIOBlk");
            block_id += 1;
        }
    }
}

impl lwext4_rs::BlockDeviceInterface for VirtIOBlock {
    fn read_block(&mut self, buf: &mut [u8], mut block_id: u64, _block_count: u32) -> lwext4_rs::Result<usize> {
        block_id *= (BLOCK_SIZE / VIRT_IO_BLOCK_SZ) as u64;
        let mut count = 0;
        for chunk in buf.chunks_mut(VIRT_IO_BLOCK_SZ) {
            self.0.lock().read_block(block_id as usize, chunk)
                .expect("VirtIOBlk read failed");
            block_id += 1;
            count += 1;
        }
        Ok(count)
    }

    fn write_block(&mut self, buf: &[u8], mut block_id: u64, _block_count: u32) -> lwext4_rs::Result<usize> {
        block_id *= (BLOCK_SIZE / VIRT_IO_BLOCK_SZ) as u64;
        let mut count = 0;
        for chunk in buf.chunks(VIRT_IO_BLOCK_SZ) {
            self.0.lock().write_block(block_id as usize, chunk)
                .expect("VirtIOBlk write failed");
            block_id += 1;
            count += 1;
        }
        Ok(count)
    }

    fn close(&mut self) -> lwext4_rs::Result<()> {
        Ok(())
    }

    fn open(&mut self) -> lwext4_rs::Result<lwext4_rs::BlockDeviceConfig> {
        let dev = self.0.lock();
        let capacity_512_blocks = dev.get_capacity();
        let block_size = BLOCK_SIZE;
        let capacity_blocks = capacity_512_blocks / (block_size / VIRT_IO_BLOCK_SZ);

        Ok(lwext4_rs::BlockDeviceConfig {
            block_size: block_size as u32,
            block_count: capacity_blocks as u64,
            part_size: (capacity_blocks * block_size) as u64,
            part_offset: 0,
        })
    }

    fn lock(&mut self) -> lwext4_rs::Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> lwext4_rs::Result<()> {
        Ok(())
    }
}

impl VirtIOBlock {
    #[allow(unused)]
    pub fn new() -> Self {
        Self(Mutex::new(
            VirtIOBlk::new(unsafe { &mut *(VIRTIO0 as *mut VirtIOHeader) }).unwrap(),
        ))
    }
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let mut ppn_base = PhysPageNum(0);
    for i in 0..pages {
        let frame = frame_alloc().unwrap();
        if i == 0 {
            ppn_base = frame.ppn;
        }
        assert_eq!(frame.ppn.0, ppn_base.0 + i);
        QUEUE_FRAMES.lock().push(frame);
    }
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysAddr, pages: usize) -> i32 {
    let mut ppn_base: PhysPageNum = pa.into();
    for _ in 0..pages {
        frame_dealloc(ppn_base);
        ppn_base.step();
    }
    0
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr(paddr.0)
}

lazy_static! {
    static ref KERNEL_TOKEN: usize = kernel_token();
}

#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    PageTableImpl::from_token(*KERNEL_TOKEN)
        .translate_va(vaddr)
        .unwrap()
}
