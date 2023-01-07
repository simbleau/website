FROM ubuntu as builder

# Install dependencies
RUN apt-get update
RUN apt-get install -y curl build-essential openssl pkg-config binaryen

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Trunk
RUN cargo install trunk

# Stage
RUN mkdir /.stage
# Cargo matter
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
# Copy dist/
COPY --from=builder /.stage/dist/ /usr/share/nginx/html/
# Apply server config
RUN rm /etc/nginx/conf.d/default.conf
COPY website.nginx.conf /etc/nginx/conf.d/website.conf