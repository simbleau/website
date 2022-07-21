FROM nginx as webserver
COPY ./dist /usr/share/nginx/html
RUN rm /etc/nginx/conf.d/default.conf
COPY ./website.nginx.conf /etc/nginx/conf.d/website.conf

ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]
