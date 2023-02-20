FROM rust AS packager
WORKDIR /app

# Build only dependencies to speed up subsequent builds
ADD Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release --locked

FROM packager AS builder
ARG BIN_NAME=main

WORKDIR /app
# Add all sources and rebuild
ADD . .

RUN cargo generate
RUN touch src/main.rs && \
    cargo build --release && \
    strip target/release/$BIN_NAME && \
    cp target/release/$BIN_NAME /app_bin


FROM scratch
COPY --from=builder /etc/ssl /etc/ssl
COPY --from=builder /app_bin /app_bin
CMD ["/app_bin"]
