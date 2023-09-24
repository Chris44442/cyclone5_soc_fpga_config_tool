# Cyclone V SoC FPGA Config Tool

On the Cyclone V SoC the HPS can access the FPGA manager to configure the FPGA. This tool automates the entire process and should run on any Distro. Do not try to use it on Arria, Stratix, Agilex or other devices, it will not work (although with a few tweaks it might). To work on your device, you will need to make sure `cdratio` and `RBF_FILE` are set correctly.

Shoutout to [Nicolás Hasbún](https://github.com/nhasbun/de10nano_fpga_linux_config) who insprired me to make this tool in Rust.

## Documentation

To understand what this tool does, refer to the `Cyclone V Hard Processor System Technical Reference Manual` and also the `Cyclone V HPS Register Address Map and Definitions` which can both be found online.

## Dependencies

To build the tool you will need:
- Rust and Cargo
- Cross Compiler (tested with `arm-unknown-linux-gnueabi` on Buildroot and `arm-unknown-linux-gnueabihf` on Angstrom)

## Build the FPGA Config Tool

Build the tool with:

```
cargo build --target=arm-unknown-linux-gnueabi --release
```

Obviously you can also natively compile on your device if you have Rust and Cargo installed on it.

## How to use the tool

Copy the tool to the device, e.g. via SSH. On the device you need to have access to the rbf file, e.g. by mounting your SD card. Run the tool.

This is my update script, although you will need to make changes for your flow, but it should give you a general idea.

```
#!/bin/bash

source ~/.fpga_config_de10
IP=$SOC_IP_DE10
RBF="../build/DE10.rbf"
RBF_HPS="~/sdcard/fpga.rbf"

ssh root@$IP 'mkdir -p sdcard && mount /dev/mmcblk0p1 ~/sdcard'
scp $RBF root@$IP:$RBF_HPS > /dev/null
ssh root@$IP './fpga_config_tool && umount /dev/mmcblk0p1'
```

Note: I have successfully tested the tool on the Terasic DE10-Nano and the Enclustra PE1/SA2, on Buildroot and Angstrom. Presumably it should run on any Cyclone V SoC device on any Linux distro.