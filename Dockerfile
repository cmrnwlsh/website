FROM rust:1.79 as builder

WORKDIR /usr/src/website
COPY . .
RUN cargo install --locked cargo-leptos &&\
  cargo leptos build --release && mkdir data && mv -t data target style

FROM debian:bookworm-slim
RUN rm -rf /var/lib/apt/lists/* && useradd -ms /bin/bash website
USER website
WORKDIR /home/website
COPY --from=builder /usr/src/website/data .
CMD ["./target/release/website"]
