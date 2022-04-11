FROM rust as builder
WORKDIR /opt/website

# Install Trunk
RUN cargo install trunk

# Copy website source files to working directory
COPY ./ ./

# Default command, serve the website.
CMD ["trunk", "serve"]