extern crate memmap;
use std::ptr::{read_volatile, write_volatile};

use clap::Parser;
/// Embedded Linux tool to configure your Cyclone V FPGA fabric from the HPS
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the rbf file, relative to this binary
    #[arg(short, long, default_value_t = String::from("sdcard/fpga.rbf"),)]
    rbf_path: String,
    /// CD ratio of the MSEL setting of your board
    #[arg(short, long, default_value_t = String::from("8"), value_parser = ["1", "2", "4", "8"])]
    cd_ratio: String,
}

fn main() -> std::io::Result<()> {
    let binding = Args::parse();
    let rbf_path: &str = binding.rbf_path.as_str();
    let cd_ratio: u32 = match Args::parse().cd_ratio.as_str() {
        "1" => 0,
        "2" => 1,
        "4" => 2,
         _  => 3,
    };
    // const cd_ratio : u32 = 0x3; // This must match the CD ratio of your MSEL setting, 0x0:cd_ratio of 1, 0x1:cd_ratio of 2, 0x2:cd_ratio of 4, 0x3:cd_ratio of 8
    const FPGA_MANAGER_REGS_ADR: u32 = 0xFF706000;
    const FPGA_MANAGER_DATA_ADR: u32 = 0xFFB90000;

    let devmem_file = std::fs::OpenOptions::new().read(true).write(true).open("/dev/mem")?;
    let mut fpga_regs_mmap = unsafe {memmap::MmapOptions::new().offset(FPGA_MANAGER_REGS_ADR as u64).len(8).map_mut(&devmem_file)?};
    let fpga_regs = unsafe {std::slice::from_raw_parts_mut(fpga_regs_mmap.as_mut_ptr() as *mut u32, fpga_regs_mmap.len() / 4)}; // FPGA manager registers slice

    let mut fpga_data_mmap = unsafe {memmap::MmapOptions::new().offset(FPGA_MANAGER_DATA_ADR as u64).len(4).map_mut(&devmem_file)?};
    let fpga_data = fpga_data_mmap.as_mut_ptr() as *mut u32; // FPGA manager data

    let rbf_file = std::fs::OpenOptions::new().read(true).open(rbf_path)?;
    let rbf_mmap = unsafe {memmap::MmapOptions::new().map(&rbf_file)?};
    let rbf_data = unsafe {std::slice::from_raw_parts(rbf_mmap.as_ptr() as *mut u32, rbf_mmap.len() / 4)};

    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | 0x1)}; //set en (HPS takes control)
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | 0x4)}; //set nconfigpull (FPGA off)
    while (unsafe {read_volatile(&fpga_regs[0])} & 0x7) != 0x1 {}; //wait for status update
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0xC0)}; //reset cdratio
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | cd_ratio << 6)}; //set cdratio
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0x4)}; //reset nconfigpull (FPGA on)
    while (unsafe {read_volatile(&fpga_regs[0])} & 0x7) != 0x2 {}; //wait for status update
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) | 0x100)}; //set axicfgen

    for rbf_u32_word in rbf_data.iter() {
        unsafe {*fpga_data = *rbf_u32_word};
    }

    while (unsafe {read_volatile(&fpga_regs[0])} & 0x7) != 0x4 {}; //wait for status update
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0x100)};// reset axicfgen
    unsafe {write_volatile(&mut fpga_regs[1] as *mut u32, read_volatile(&fpga_regs[1]) & !0x1)};// reset en (HPS releases control)
    Ok(())
}
