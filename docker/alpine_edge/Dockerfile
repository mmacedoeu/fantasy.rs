FROM alpine:edge AS builder

# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  build-base \
  cargo \
  cmake \
  linux-headers \
  perl \
  rust

WORKDIR /fantasy
COPY . /fantasy

RUN cargo build --release --target x86_64-alpine-linux-musl --verbose
RUN strip target/x86_64-alpine-linux-musl/release/fantasy

FROM alpine:edge
# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  libstdc++ \
  libgcc

WORKDIR /usr/bin
COPY --from=builder /fantasy/target/x86_64-alpine-linux-musl/release/fantasy ./
ENTRYPOINT ["./fantasy"]