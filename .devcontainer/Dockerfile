FROM debian:bookworm-slim
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates git build-essential pkg-config \
    sqlite3 libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install sea-orm-cli sqlx-cli cargo-udeps cargo-deny --locked
RUN mkdir -p /workspaces
WORKDIR /workspaces
