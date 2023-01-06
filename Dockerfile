FROM ubuntu:latest as builder

# Install dependencies
RUN apt-get update
RUN apt-get install -y curl build-essential libssl-dev cmake pkg-config openssl

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN rustup install nightly

# Install Trunk
RUN cargo install wasm-bindgen-cli
RUN cargo install trunk

# Stage
RUN mkdir /.stage
# Build matter
COPY Cargo.toml /.stage/
COPY rust-toolchain.toml /.stage/
# Web matter
COPY index.html /.stage/
COPY robots.txt /.stage/
COPY sitemap.xml /.stage/
COPY scss /.stage/scss
COPY src /.stage/src
COPY static /.stage/static

# Build
WORKDIR /.stage
RUN trunk build --release

# Run
FROM nginx:alpine
COPY website.nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /.stage/dist/ /usr/share/nginx/html/