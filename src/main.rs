extern crate memmap;
use std::ptr::{read_volatile, write_volatile};

fn main() {
    const CDRATIO              : u32 = 0x3; // This must match the CD ratio of your MSEL setting, 0x0:CDRATIO of 1, 0x1:CDRATIO of 2, 0x2:CDRATIO of 4, 0x3:CDRATIO of 8
    const RBF_FILE             : &str = "sdcard/fpga.rbf"; // This must match the location of your rbf file on the device
    const FPGA_MANAGER_REGS_ADR: u32 = 0xFF706000;
    const FPGA_MANAGER_DATA_ADR: u32 = 0xFFB90000;
    const CTRL_OFFSET          : u32 = 0x4;
    const MMAPPING_LEN         : usize = 8;
    let ctrl_offset_index      : usize = (CTRL_OFFSET/4) as usize;

    let f = std::fs::OpenOptions::new().read(true).write(true).open("/dev/mem").unwrap();
    let mut mmap = unsafe {memmap::MmapOptions::new().offset(FPGA_MANAGER_REGS_ADR as u64).len(MMAPPING_LEN).map_mut(&f).unwrap()};
    let u32_slice = unsafe {std::slice::from_raw_parts_mut(mmap.as_mut_ptr() as *mut u32, mmap.len() / 4)};

    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) | 0x1)}; //set en (HPS takes control)
    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) | 0x4)}; //set nconfigpull (FPGA off)
    while (unsafe {read_volatile(&u32_slice[0])} & 0x7) != 0x1 {}; //wait for status update
    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) & !0xC0)}; //reset cdratio
    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) | CDRATIO << 6)}; //set cdratio
    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) & !0x4)}; //reset nconfigpull (FPGA on)
    while (unsafe {read_volatile(&u32_slice[0])} & 0x7) != 0x2 {}; //wait for status update
    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) | 0x100)}; //set axicfgen

    let mut data_mmap = unsafe { memmap::MmapOptions::new().offset(FPGA_MANAGER_DATA_ADR as u64).len(4).map_mut(&f).unwrap() };
    let data_u32_slice = unsafe {std::slice::from_raw_parts_mut(data_mmap.as_mut_ptr() as *mut u32, data_mmap.len() / 4)};

    let rbf_file = std::fs::OpenOptions::new().read(true).open(RBF_FILE).unwrap();
    let rbf_mmap = unsafe { memmap::MmapOptions::new().map(&rbf_file).unwrap() };
    let rbf_u32_slice = unsafe {std::slice::from_raw_parts(rbf_mmap.as_ptr() as *mut u32, rbf_mmap.len() / 4)};

    for u32word in rbf_u32_slice.iter() {
        data_u32_slice[0] = unsafe {read_volatile(u32word)};
    }

    while (unsafe {read_volatile(&u32_slice[0])} & 0x7) != 0x4 {}; //wait for status update
    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) & !0x100)};// reset axicfgen
    unsafe {write_volatile(&mut u32_slice[ctrl_offset_index] as *mut u32, read_volatile(&u32_slice[ctrl_offset_index]) & !0x1)};// reset en (HPS releases control)
}