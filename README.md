#  Personal Website
This repo contains my personal website's source code.

#  Deployment
##  Option 1: [Docker](https://docker.com) (Recommended)
[![Docker AMD64 Image](https://badgen.net/docker/size/simbleau/website/latest/amd64?icon=docker&label=amd64)](https://hub.docker.com/r/simbleau/website/)
[![Docker ARM64 Image](https://badgen.net/docker/size/simbleau/website/latest/arm64?icon=docker&label=arm64v8)](https://hub.docker.com/r/simbleau/website/)\
It's simple: `docker run -p 80:80 simbleau/website:latest`

##  Option 2: Building
###  Dependencies
-  [Rust](https://www.rust-lang.org/)
-  [trunk](https://trunkrs.dev/) (`cargo install trunk`)
###  Serving
-  `trunk build` (generates the `dist/` folder)
-  Serve the `dist/` folder with a web server such as [nginx](https://www.nginx.com/).

# Development
##  Dependencies
-  [Rust](https://www.rust-lang.org/)
-  [trunk](https://trunkrs.dev/) (`cargo install trunk`)
##  Serving
-  `trunk serve --port 8080` (✅ Hot-reloading)
-  Open your web browser to [http://localhost:8080/](http://localhost:8080/).
##  Updating Docker Images
-  `trunk build`
-  `export BUILD_TAG='simbleau/website:latest'`
-  `docker buildx build --platform linux/arm64/v8,linux/amd64 -t $BUILD_TAG --push .`

# License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
