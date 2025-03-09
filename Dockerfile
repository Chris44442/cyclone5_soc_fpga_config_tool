FROM ubuntu:24.04

RUN apt-get update

# Install necessary packages
RUN apt-get install -y \
    gcc \
    gcc-arm-linux-gnueabihf \
    curl

# Install Rustup, Cargo and cross compiler
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add arm-unknown-linux-gnueabihf

WORKDIR /home

