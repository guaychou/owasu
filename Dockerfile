FROM rust:1.61.0-alpine3.15 as build-env
WORKDIR /app
ADD . /app
RUN apk add --no-cache musl-dev            \
    && rustup component add rustfmt clippy \
    && cargo fmt --all -- --check          \
    && cargo clippy                        \
    && RUSTFLAGS="-C link-arg=-s --cfg unsound_local_offset" cargo build --release

FROM gcr.io/distroless/static-debian11:nonroot-amd64
ENV TZ="Asia/Jakarta"
WORKDIR /app
COPY --from=build-env /app/target/release/owasu /app
CMD ["./owasu"]