# .devcontainer/Dockerfile
ARG TAG=1-bullseye
FROM mcr.microsoft.com/devcontainers/rust:${TAG}

# Packages
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get \
    -y install --no-install-recommends \
        vim \
        curl \
        openssh-client \
        rust-src \
        git && \
    rm -rf /var/lib/apt/lists/*

# Add the docker commands to ensure ~/.ssh and ~/.gitconfig are available
# Note: These files are now bind-mounted from the host in devcontainer.json
# Therefore, these commands are not strictly necessary, but are kept to ensure 
# the correct permissions are set in case the files do not exist on the host.
ARG USERNAME=vscode
RUN mkdir -p /home/${USERNAME}/.ssh && \
    chown -R ${USERNAME} /home/${USERNAME}/.ssh && \
    chmod 700 /home/${USERNAME}/.ssh
RUN touch /home/${USERNAME}/.gitconfig && \
    chown ${USERNAME} /home/${USERNAME}/.gitconfig && \
    chmod 600 /home/${USERNAME}/.gitconfig


# Upgrade the Rust to toolchain to the latest stable version
# Download the latest stable version
RUN rustup update stable
# Set the default toolchain to be the active toolchain
RUN rustup default stable
# Add the rust-src component for use with rust-analyzer
RUN rustup component add rust-src
