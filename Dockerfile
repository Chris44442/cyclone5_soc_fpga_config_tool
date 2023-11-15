FROM archlinux:latest

# Update the system and install necessary packages
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm base-devel
RUN pacman -Syu --noconfirm

# Clean up package cache
RUN pacman -Scc --noconfirm

COPY . /home
WORKDIR /home

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup update

