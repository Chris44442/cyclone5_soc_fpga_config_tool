# Cyclone V SoC FPGA Config Linux Tool

On Cyclone V SoC devices the HPS can access the FPGA manager to configure the FPGA. This tool automates the entire process and should run on any Linux distro. Do not try to use it on Arria, Stratix, Agilex or other devices, it will not work (although with a few tweaks it might). To work on your device, you will need to make sure `cdratio` and `RBF_FILE` are set correctly. The current cdratio is set for the Terasic DE10-Nano.

Shoutout to [Nicolás Hasbún](https://github.com/nhasbun/de10nano_fpga_linux_config) whose work inspired me to make this tool in Rust.

## Documentation

To better understand what this tool does, refer to the `Cyclone V Hard Processor System Technical Reference Manual` and also the `Cyclone V HPS Register Address Map and Definitions` which can both be found online.

## Dependencies

To build the tool you need:
- Rust and Cargo installed
- With Rustup add the cross compiler (usually `arm-unknown-linux-gnueabi`, otherwise try `arm-unknown-linux-gnueabihf`)
- For cross compilation don't forget to state the linker in the `cargo.toml`, e.g.:
```
[target.arm-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```

## Build the tool

Build the tool with:

```
cargo build --target=arm-unknown-linux-gnueabi --release
```

Of course you can also natively compile on your SoC device if you have Rust and Cargo installed on it.

## How to use the tool

Copy the tool from your host PC to the device, e.g. via SSH. On the device you need to have access to the rbf file, e.g. by mounting your SD card. Run the tool.

Showing you my copy and update script should give you a general idea, although you will most likely need to make changes if you want to use it. Copy the tool once:


```
#!/bin/bash

source ~/.fpga_config_de10
IP=$SOC_IP_DE10
CFGTOOL="target/arm-unknown-linux-gnueabi/release/fpga_config_tool"
CFGTOOL_HPS="~/fpga_config_tool"

scp $CFGTOOL root@$IP:$CFGTOOL_HPS
```

Now you can update your FPGA everytime you have a new rbf. Blazingly fast.

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

Note: I have successfully tested the tool on the Terasic DE10-Nano and the Enclustra PE1/SA2, each on Buildroot. Presumably it should run on any Cyclone V SoC device on any Linux distro. When running old Distro releases cross compile compatibilty issues with glibc library may happen, in which case you may want to consider upgrading to new kernel and root file system or compile natively.