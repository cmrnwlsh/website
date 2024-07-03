FROM rust:1.79 as builder

WORKDIR /usr/src/website
COPY . .
RUN cargo install --locked cargo-leptos &&\
  cargo leptos build --release && mkdir data &&\
  mv -t data target style &&\
  rm -rf ./target/release/deps/*.d &&\
  rm -rf ./target/release/build &&\
  rm -rf ./target/release/examples &&\
  rm -rf ./target/release/incremental &&\
  rm -rf ./target/release/website.d &&\
  rm -rf ./target/front/wasm32-unknown-unknown/wasm-release/deps/*.d &&\
  rm -rf ./target/front/wasm32-unknown-unknown/wasm-release/build &&\
  rm -rf ./target/front/wasm32-unknown-unknown/wasm-release/examples &&\
  rm -rf ./target/front/wasm32-unknown-unknown/wasm-release/incremental &&\
  rm -rf ./target/front/wasm32-unknown-unknown/wasm-release/website.d &&\
  rm -rf ./target/front/wasm32-unknown-unknown/wasm-release/libwebsite.d &&\
  rm -rf ./target/front/wasm-release/deps/*.d &&\
  rm -rf ./target/front/wasm-release/build &&\
  rm -rf ./target/front/wasm-release/examples &&\
  rm -rf ./target/front/wasm-release/incremental

FROM debian:bookworm-slim
RUN rm -rf /var/lib/apt/lists/* && useradd -ms /bin/bash website
USER website
WORKDIR /home/website
COPY --from=builder /usr/src/website/data .
CMD ["./target/release/website"]
