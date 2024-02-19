FROM rustlang/rust:nightly

RUN useradd -ms /bin/bash website
USER website
WORKDIR /home/website/resource
COPY . .

RUN rustup target add wasm32-unknown-unknown &&\
  cargo install cargo-leptos
RUN cargo leptos build --release -vv
EXPOSE 3000
CMD ["cargo", "leptos", "serve"]

