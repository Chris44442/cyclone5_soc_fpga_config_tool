# Cyclone V SoC FPGA Config Linux Tool

With this tool you can configure the FPGA fabric with your rbf-file by 
accessing the FPGA manager from the HPS of your Cyclone V SoC. Do not try to 
use it on other devices.

To better understand what this tool does, refer to the 
`Cyclone V Hard Processor System Technical Reference Manual` and also the 
`Cyclone V HPS Register Address Map and Definitions` which can both be found 
on the Intel website.

## Build the tool

In the source file make sure `RBF_PATH` and `CDRATIO` are set correctly, 
e.g. 0x3 on the Terasic DE10-Nano and 0x2 on the Enclustra PE1/SA2.

Use Docker to build the tool. To build it yourself instead, `cat Dockerfile` 
to find out how.

```bash
docker build -t cfg_tool -f Dockerfile .
docker create --name temp_container cfg_tool
docker cp temp_container:/home/target/arm-unknown-linux-gnueabi/release/fpga_config_tool ./fpga_config_tool
docker rm temp_container
```

## How to use the tool

Copy the tool from your host PC to the device, e.g. via SSH. On the device 
make sure to have access to the rbf file, e.g. by mounting your SD card. 
Run the tool.

The scripts in the `util` directory should give you a general idea, although 
you will most likely need to make changes if you want to use them. `SCP` the 
tool first, then run the configure script.

## Note

I have successfully tested the tool on the Terasic DE10-Nano and the 
Enclustra PE1/SA2, each on Buildroot. Presumably it should run on any 
Cyclone V SoC device on any Linux distro. When running old distro releases 
cross compile compatibilty issues with glibc library may happen, in which 
case you may want to consider upgrading to a new kernel and root file system 
or compile natively.
