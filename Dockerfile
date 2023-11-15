FROM ubuntu:22.04
# ENV DEBIAN_FRONTEND=noninteractive

# Update the system and install necessary packages
RUN apt-get update
RUN apt-get install -y \
    gcc \
    gcc-arm-linux-gnueabi \
    curl
RUN apt-get update
RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*

# Import files from repository
COPY . /home
WORKDIR /home

# Install Rustup, Cargo and cross compiler
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add arm-unknown-linux-gnueabi

# Build the project
RUN cargo build --release

