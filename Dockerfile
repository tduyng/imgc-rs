FROM rust:1-alpine AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN apk add --no-cache nasm musl-dev
RUN cargo install --profile release --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/imgc /usr/local/bin/imgc
CMD ["imgc"]
