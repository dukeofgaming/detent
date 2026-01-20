FROM rust:1.82-slim-bullseye

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000

# Install build dependencies and useful tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        git \
        openssh-client \
        curl \
        ca-certificates \
        pkg-config \
        libssl-dev \
        build-essential \
        && \
    groupadd --gid "${USER_GID}" "${USERNAME}" && \
    useradd -s /bin/bash --uid "${USER_UID}" --gid "${USER_GID}" -m "${USERNAME}" && \
    rm -rf /var/lib/apt/lists/*

ENV CARGO_TARGET_DIR=/workspace/target
WORKDIR /workspace

RUN chown -R "${USERNAME}":"${USERNAME}" /workspace

USER "${USERNAME}"

CMD ["bash"]
