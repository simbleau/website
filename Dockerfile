FROM nginx as webserver
COPY ./dist /usr/share/nginx/html

ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]
