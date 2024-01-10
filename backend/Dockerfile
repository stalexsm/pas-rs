FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /pas-rs

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /pas-rs/target/x86_64-unknown-linux-musl/release/pas-rs /pas-rs
ENTRYPOINT ["/pas-rs"]

EXPOSE 3000
