# Cyclone V SoC FPGA Config CLI Tool

With this CLI tool you can configure the FPGA fabric with your rbf-file by 
accessing the FPGA manager from the HPS Linux of your Cyclone V SoC. Do not try to 
use it on devices other than Cyclone V SoC.

To better understand what this tool does, refer to 
[Cyclone V Hard Processor System Technical Reference Manual](https://www.intel.com/content/www/us/en/docs/programmable/683126/21-2/hard-processor-system-technical-reference.html)
and 
[Cyclone V HPS Register Address Map and Definitions](https://www.intel.com/content/www/us/en/programmable/hps/cyclone-v/hps.html#sfo1410067808053.html#sfo1410067808053).

Note: I have successfully tested the tool on the Terasic DE10-Nano and the 
Enclustra PE1/SA2, each on Buildroot. Presumably it should run on any 
Cyclone V SoC device on any Linux distro. When running old distro releases 
cross compile compatibilty issues with glibc library may happen, in which 
case you may want to consider upgrading to a new kernel and root file system, 
compile natively, or try using an older docker image.

## Build the tool

If you don't want to use the release binary, you can build the tool from source.

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

Usage: fpga_config_tool [OPTIONS]

Options:
  -r <RBF_PATH>      Path of the rbf file, relative to this binary [default: sdcard/fpga.rbf]
  -c <CD_RATIO>      CD ratio of the MSEL setting of your board [default: 8] [possible values: 1, 2, 4, 8]
  -h, --help         Print help
  -V, --version      Print version
```

