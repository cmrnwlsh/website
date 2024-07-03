FROM rust:1.79 as builder

WORKDIR /usr/src/website
COPY . .
RUN cargo install --locked cargo-leptos &&\
  cargo leptos build --release && mkdir data &&\
  mv -t data target style
WORKDIR /usr/src/website/data/target
RUN rm -rf ./release/deps/*.d &&\
  rm -rf ./release/build &&\
  rm -rf ./release/examples &&\
  rm -rf ./release/incremental &&\
  rm -rf ./release/website.d &&\
  rm -rf ./front/wasm32-unknown-unknown/wasm-release/deps/*.d &&\
  rm -rf ./front/wasm32-unknown-unknown/wasm-release/build &&\
  rm -rf ./front/wasm32-unknown-unknown/wasm-release/examples &&\
  rm -rf ./front/wasm32-unknown-unknown/wasm-release/incremental &&\
  rm -rf ./front/wasm32-unknown-unknown/wasm-release/website.d &&\
  rm -rf ./front/wasm32-unknown-unknown/wasm-release/libwebsite.d &&\
  rm -rf ./front/wasm-release/deps/*.d &&\
  rm -rf ./front/wasm-release/build &&\
  rm -rf ./front/wasm-release/examples &&\
  rm -rf ./front/wasm-release/incremental

FROM debian:bookworm-slim
RUN rm -rf /var/lib/apt/lists/* && useradd -ms /bin/bash website
USER website
WORKDIR /home/website
COPY --from=builder /usr/src/website/data .
CMD ["./target/release/website"]
