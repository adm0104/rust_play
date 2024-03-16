FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's bin directory to PATH
ENV PATH="/root/.cargo/bin:${PATH}"


SHELL ["/bin/bash", "-c"]