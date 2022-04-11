FROM rust as builder
WORKDIR /opt/website

# Install Trunk
RUN cargo install trunk

# Copy website source files to working directory
COPY ./ ./

# Build the website
RUN trunk build

# Translate to an nginx web server after build
FROM nginx as webserver
COPY --from=builder /opt/website/dist /usr/share/nginx/html

ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]