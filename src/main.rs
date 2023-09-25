extern crate memmap;
use std::ptr::{read_volatile, write_volatile};

fn main() -> std::io::Result<()> {
    const CDRATIO              : u32 = 0x3; // This must match the CD ratio of your MSEL setting, 0x0:CDRATIO of 1, 0x1:CDRATIO of 2, 0x2:CDRATIO of 4, 0x3:CDRATIO of 8
    const RBF_PATH             : &str = "sdcard/fpga.rbf"; // This must match the location of your rbf file on the device
    const FPGA_MANAGER_REGS_ADR: u32 = 0xFF706000;
    const FPGA_MANAGER_DATA_ADR: u32 = 0xFFB90000;

    let devmem_file = std::fs::OpenOptions::new().read(true).write(true).open("/dev/mem")?;
    let mut mmap = unsafe {memmap::MmapOptions::new().offset(FPGA_MANAGER_REGS_ADR as u64).len(8).map_mut(&devmem_file)?};
    let fpga_regs = unsafe {std::slice::from_raw_parts_mut(mmap.as_mut_ptr() as *mut u32, mmap.len() / 4)}; // FPGA manager registers slice

    let mut data_mmap = unsafe {memmap::MmapOptions::new().offset(FPGA_MANAGER_DATA_ADR as u64).len(4).map_mut(&devmem_file)?};
    let fpga_data = data_mmap.as_mut_ptr() as *mut u32; // FPGA manager data

    let rbf_file = std::fs::OpenOptions::new().read(true).open(RBF_PATH)?;
    let rbf_mmap = unsafe {memmap::MmapOptions::new().map(&rbf_file)?};
    let rbf_data = unsafe {std::slice::from_raw_parts(rbf_mmap.as_ptr() as *mut u32, rbf_mmap.len() / 4)};

    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | 0x1)}; //set en (HPS takes control)
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | 0x4)}; //set nconfigpull (FPGA off)
    while (unsafe {read_volatile(&fpga_regs[0])} & 0x7) != 0x1 {}; //wait for status update
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0xC0)}; //reset cdratio
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | CDRATIO << 6)}; //set cdratio
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0x4)}; //reset nconfigpull (FPGA on)
    while (unsafe {read_volatile(&fpga_regs[0])} & 0x7) != 0x2 {}; //wait for status update
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | 0x100)}; //set axicfgen

    for rbf_u32_word in rbf_data.iter() {
        unsafe {*fpga_data = *rbf_u32_word}; // write rbf data to FPGA manager data
    }

    while (unsafe {read_volatile(&fpga_regs[0])} & 0x7) != 0x4 {}; //wait for status update
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0x100)};// reset axicfgen
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0x1)};// reset en (HPS releases control)

    Ok(())
}