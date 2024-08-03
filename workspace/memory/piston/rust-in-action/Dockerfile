FROM rust:1.65

WORKDIR /opt/rust-in-action-memory-piston-main
COPY . .
RUN [[ -f Cargo.lock ]] && rm Cargo.lock || true
RUN cargo build
CMD ["rust-in-action-memory-piston-main"]
