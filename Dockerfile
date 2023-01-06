FROM osomahe/rust-trunk:22.05 as builder

# Build
RUN mkdir /.stage
COPY index.html /.stage/
COPY robots.txt /.stage/
COPY sitemap.xml /.stage/
COPY Cargo.toml /.stage/
COPY scss /.stage/scss
COPY src /.stage/src
COPY static /.stage/static
WORKDIR /.stage
RUN trunk build --release

# Run
FROM nginx:alpine
COPY website.nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /.stage/dist/ /usr/share/nginx/html/