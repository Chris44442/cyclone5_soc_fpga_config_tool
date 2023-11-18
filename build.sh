#!/bin/bash

docker build -t cfg_tool -f Dockerfile .
docker create --name temp_container cfg_tool
docker cp temp_container:/home/target/arm-unknown-linux-gnueabi/release/fpga_config_tool ./fpga_config_tool
docker rm temp_container

