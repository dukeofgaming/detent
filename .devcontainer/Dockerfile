FROM ubuntu:22.04

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000

# Install build dependencies, useful tools, and Docker CLI
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        git \
        openssh-client \
        curl \
        ca-certificates \
        pkg-config \
        libssl-dev \
        build-essential \
        sudo \
        gnupg \
        && \
    # Add Docker's official GPG key and repository
    install -m 0755 -d /etc/apt/keyrings && \
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg && \
    chmod a+r /etc/apt/keyrings/docker.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu jammy stable" > /etc/apt/sources.list.d/docker.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends docker-ce-cli docker-buildx-plugin docker-compose-plugin && \
    # Create user with sudo access
    groupadd --gid "${USER_GID}" "${USERNAME}" && \
    useradd -s /bin/bash --uid "${USER_UID}" --gid "${USER_GID}" -m "${USERNAME}" && \
    echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME} && \
    rm -rf /var/lib/apt/lists/*

# Install Rust for the vscode user
USER "${USERNAME}"
ENV RUSTUP_HOME=/home/${USERNAME}/.rustup \
    CARGO_HOME=/home/${USERNAME}/.cargo \
    PATH=/home/${USERNAME}/bin:/home/${USERNAME}/.cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && \
    rustup component add clippy rustfmt && \
    mkdir -p /home/${USERNAME}/bin && \
    printf '#!/bin/bash\nexec sudo /usr/bin/docker "$@"\n' > /home/${USERNAME}/bin/docker && \
    chmod +x /home/${USERNAME}/bin/docker


# Install OpenCode
RUN curl -fsSL https://opencode.ai/install | bash

# TODO: Make this dynamic
ENV CARGO_TARGET_DIR=/workspaces/detent/target
WORKDIR /workspaces/detent

CMD ["bash"]
