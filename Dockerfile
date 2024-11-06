# Use Arch Linux as the base image
FROM archlinux:latest

# Update the package list and install basic packages
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm base base-devel

# Clean the package cache to reduce image size
RUN pacman -Scc --noconfirm

RUN pacman -S --noconfirm git lzo rustup
RUN rustup default stable
RUN git clone https://github.com/narad-muni/broadcast_gate

WORKDIR /broadcast_gate

RUN cargo build --release --bin broadcast_gate

# Set a default command for the container
CMD ["/bin/bash"]