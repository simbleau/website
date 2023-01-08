# Warning: This docker file assumes the dist/ is built already with trunk build --release.
# Only for use in CI.

FROM nginx:alpine
# Copy dist/
COPY dist/ /usr/share/nginx/html/
# Apply server config
RUN rm /etc/nginx/conf.d/default.conf
COPY website.nginx.conf /etc/nginx/conf.d/website.conf