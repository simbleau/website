FROM ubuntu as builder

# Install dependencies
RUN apt-get update
RUN apt-get install -y curl build-essential openssl pkg-config binaryen

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Trunk
# This works... but is VERY SLOW!
# RUN cargo install trunk
# Faster: we can download the latest binary directly.
# TODO: Make more cross-platform friendly. Only works on linux.
RUN apt-get install -y jq wget
RUN export TRUNK_VERSION=$(curl https://api.github.com/repos/thedodd/trunk/releases/latest | jq -r '.name') ; \
    echo "Latest version: ${TRUNK_VERSION}" ; \
    wget "https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz"
RUN tar -xf trunk-x86_64-unknown-linux-gnu.tar.gz
RUN chmod +x trunk
RUN mkdir /root/bin
RUN mv trunk /root/bin/trunk
ENV PATH="/root/bin:${PATH}"

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