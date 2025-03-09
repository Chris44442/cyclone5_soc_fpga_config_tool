use std::fs::OpenOptions;
use memmap2::{MmapOptions, Mmap, MmapMut};
use std::slice::{from_raw_parts_mut, from_raw_parts};
use std::ptr::{write_volatile,read_volatile};

use clap::Parser;
/// Embedded Linux tool to configure your Cyclone V FPGA fabric from the HPS
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the rbf file, relative to this binary
    #[arg(short, long, default_value_t = String::from("sdcard/fpga.rbf"),)]
    rbf_path: String,
    /// CD ratio of the MSEL setting of your board
    #[arg(short, long, default_value_t = String::from("4"), value_parser = ["1", "2", "4", "8"])]
    cd_ratio: String,
}

fn main() {
    let binding = Args::parse();
    let rbf_path: &str = binding.rbf_path.as_str();
    let cd_ratio: u32 = match Args::parse().cd_ratio.as_str() {
        "1" => 0,
        "2" => 1,
        "4" => 2,
         _  => 3,
    };

    let (reg, _mmap0) = mut_slice_from_file_with_adr(FPGA_MANAGER_REGS_ADR, 8, "/dev/mem");
    let (dat, _mmap1) = mut_slice_from_file_with_adr(FPGA_MANAGER_DATA_ADR, 4, "/dev/mem");
    let (rbf, _mmap2) = slice_from_file(rbf_path);

    EN.write(reg, 1); // hand off control to HPS
    NCONFIGPULL.write(reg, 1); // put FPGA into reset phase
    while STATUS_MODE.read(reg) != 1 {}; // wait for FPGA to be in reset phase
    CDRATIO.write(reg, cd_ratio); // set cd_ratio
    NCONFIGPULL.write(reg, 0); // get FPGA out of reset phase
    while STATUS_MODE.read(reg) != 2 {}; // wait for FPGA to be in configuration phase
    AXICFGEN.write(reg, 1); // enable AXI data transfer

    for rbf_word in rbf.iter() {
        FPGA_DATA.write(dat, *rbf_word); // write rbf data
    }

    while STATUS_MODE.read(reg) != 4 {}; // wait for FPGA to be in user mode phase
    AXICFGEN.write(reg, 0); // disable AXI data transfer
    EN.write(reg, 0); // HPS releases controls
}


struct Reg {
    offset: usize,
}

impl Reg {
    fn write(&self, slice: &mut [u32], value: u32) {
        unsafe {write_volatile(&mut slice[self.offset], value)};
    }
    fn _read(&self, slice: &mut [u32]) -> u32 {
        unsafe {read_volatile(&mut slice[self.offset]) }
    }
}

struct RegField {
    offset: usize,
    mask: u32,
    lsb: u8,
}

impl RegField {
    fn write(&self, slice: &mut [u32], value: u32) {
        let new_value_at_bit_position = (value << self.lsb) & self.mask;
        let old_remaining_bit_values = unsafe { read_volatile(&mut slice[self.offset])} & !self.mask ;
        unsafe {write_volatile(&mut slice[self.offset], old_remaining_bit_values | new_value_at_bit_position)};
    }
    fn read(&self, slice: &mut [u32]) -> u32 {
        (unsafe { read_volatile(&mut slice[self.offset]) } & self.mask) >> self.lsb
    }
}

fn mut_slice_from_file_with_adr<'a>(adr: usize, len: usize, path: &str) -> (&'a mut [u32], MmapMut ) {
    let f = OpenOptions::new().read(true).write(true).create(true).open(path).expect("Error opening file path");
    let mut mmap = unsafe { MmapOptions::new().offset(adr as u64).len(len).map_mut(&f).expect("Error creating mutable mmap") };
    let slice = unsafe { from_raw_parts_mut(mmap.as_mut_ptr() as *mut u32, mmap.len() / 4) };
    (slice, mmap) // to use slice, mmap must not be out of scope
}

fn slice_from_file<'a>(path: &str) -> (&'a [u32], Mmap ) {
    let f = OpenOptions::new().read(true).write(false).create(false).open(path).expect("Error opening file path");
    let mmap = unsafe { MmapOptions::new().map(&f).expect("Error creating mmap") };
    let slice = unsafe { from_raw_parts(mmap.as_ptr() as *mut u32, mmap.len() / 4) };
    (slice, mmap) // to use slice, mmap must not be out of scope
}

const FPGA_MANAGER_REGS_ADR: usize = 0xFF706000;
const FPGA_MANAGER_DATA_ADR: usize = 0xFFB90000;

const FPGA_DATA: Reg = Reg {
    offset: 0x0,
};

const STATUS_MODE: RegField = RegField {
    offset: 0x0,
    mask: 0x7,
    lsb: 0,
};

const EN: RegField = RegField {
    offset: 0x1,
    mask: 0x1,
    lsb: 0,
};

const NCONFIGPULL: RegField = RegField {
    offset: 0x1,
    mask: 0x4,
    lsb: 2,
};

const CDRATIO: RegField = RegField {
    offset: 0x1,
    mask: 0xc0,
    lsb: 6,
};

const AXICFGEN: RegField = RegField {
    offset: 0x1,
    mask: 0x100,
    lsb: 8,
};

