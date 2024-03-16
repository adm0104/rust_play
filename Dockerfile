FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    git \
    curl \
    vim \
    && rm -rf /var/lib/apt/lists/*

SHELL ["/bin/bash", "-c"]