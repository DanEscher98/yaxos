FROM rust:latest

# Set environment variables
ENV USER=root \
  CARGO_HOME=/usr/local/cargo \
  RUSTUP_HOME=/usr/local/rustup

# Install needed packages
RUN apt-get update && apt-get install -y build-essential vim

# Install required Rust stuff
RUN rustup toolchain install nightly && \
  rustup default nightly && \
  rustup component add rust-src && \
  rustup component add llvm-tools-preview && \
  cargo install bootimage

WORKDIR /yaxos
COPY ./src /yaxos/src
COPY Cargo.toml /yaxos
COPY .cargo/config.toml /yaxos/.cargo/
COPY x86_64-yaxos.json /yaxos
COPY rust-toolchain /yaxos

CMD ["bash"]
