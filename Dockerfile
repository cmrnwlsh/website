FROM rust:1.79 as builder

WORKDIR /usr/src/website
COPY . .
RUN cargo install --locked cargo-leptos &&\
  cargo leptos build --release && mkdir data &&\
  mv -t data target style
WORKDIR /usr/src/website/data/target
RUN rm -rf ./release/deps &&\
  rm -rf ./release/build &&\
  rm -rf ./release/examples &&\
  rm -rf ./release/incremental &&\
  rm -rf ./release/website.d &&\
  rm -rf ./front

FROM debian:bookworm-slim
RUN rm -rf /var/lib/apt/lists/* && useradd -ms /bin/bash website
USER website
WORKDIR /home/website
COPY --from=builder /usr/src/website/data .
CMD ["./target/release/website"]
