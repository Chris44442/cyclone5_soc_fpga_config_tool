#!/bin/bash

IP=192.168.123.456
CFGTOOL="fpga_config_tool"
CFGTOOL_HPS="~/fpga_config_tool"

scp $CFGTOOL root@$IP:$CFGTOOL_HPS

