# Start with a rust alpine image
FROM rust:1-alpine3.16
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev git pkgconfig openssl-dev
# set the workdir
WORKDIR /app
# copy only the source files
COPY Cargo.lock Cargo.toml /app/
COPY src /app/src
# do a release build
RUN cargo build --config net.git-fetch-with-cli=true  --release
RUN strip /app/target/release/rusty-ip

# # use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.16
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=0 /app/target/release/rusty-ip .
# set the binary as entrypoint
ENTRYPOINT ["/rusty-ip"]