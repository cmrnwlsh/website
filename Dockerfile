FROM rust:1.79

RUN useradd -ms /bin/bash website
USER website
WORKDIR /home/website

COPY . .
RUN cargo install --locked cargo-leptos &&\
  cargo leptos build --release

CMD ["./target/release/website"]
