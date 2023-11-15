FROM archlinux:latest

# Update the system and install necessary packages
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm base-devel
RUN pacman -Syu --noconfirm

# Clean up package cache
RUN pacman -Scc --noconfirm

# Import files from repository
COPY . /home
WORKDIR /home

# Install Rustup, Cargo and cross compiler
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add arm-unknown-linux-gnueabi

RUN cargo build --release

