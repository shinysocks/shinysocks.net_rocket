FROM rust
WORKDIR /shinysocks.net
COPY . .
RUN cargo build --release
CMD ["target/release/shinysocksdotnet"]
EXPOSE 8888
