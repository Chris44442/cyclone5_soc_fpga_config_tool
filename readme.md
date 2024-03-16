# Cyclone V SoC FPGA Config Linux Tool

With this tool you can configure the FPGA fabric with your rbf-file by 
accessing the FPGA manager from the HPS Linux of your Cyclone V SoC. Do not try to 
use it on other devices.

To better understand what this tool does, refer to the 
`Cyclone V Hard Processor System Technical Reference Manual` and also the 
`Cyclone V HPS Register Address Map and Definitions` which can both be found 
on the Intel website.

Note: I have successfully tested the tool on the Terasic DE10-Nano and the 
Enclustra PE1/SA2, each on Buildroot. Presumably it should run on any 
Cyclone V SoC device on any Linux distro. When running old distro releases 
cross compile compatibilty issues with glibc library may happen, in which 
case you may want to consider upgrading to a new kernel and root file system, 
compile natively, or try using an older docker image.

## Build the tool

- Build with Docker:

```bash
./build.sh
```

- Build with Cargo:

```bash
cargo build --release
```

## How to use the tool

Copy the tool from your host PC to the device, e.g. via SSH (use 
`util/scp_tool.sh` as example). On the device make sure to have access to the 
rbf-file, e.g. by mounting your SD card. Then run the tool (use 
`util/config_fpga.sh ` as example).

Make sure `RBF_PATH` and `CDRATIO` are set correctly, 
e.g. 0x3 for the Terasic DE10-Nano and 0x2 for the Enclustra PE1/SA2. Refer to 
the documentation of your board if you are unsure.

```txt
Embedded Linux tool to configure your Cyclone V FPGA fabric from the HPS

Usage: cooli [OPTIONS]

Options:
  -r <RBF_PATH>      Path of the rbf file, relative to this binary [default: sdcard/fpga.rbf]
  -c <CD_RATIO>      CD ratio of the MSEL setting of your board [default: 8] [possible values: 1, 2, 4, 8]
  -h, --help         Print help
  -V, --version      Print version
```

