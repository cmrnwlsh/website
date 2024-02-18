FROM rustlang/rust:nightly

RUN useradd -ms /bin/bash website
USER website
WORKDIR /home/website/resource
COPY . .

RUN rustup target add wasm32-unknown-unknown && cargo install cargo-leptos
CMD ["cargo", "leptos", "serve"]

