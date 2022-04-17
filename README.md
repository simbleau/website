#  Personal Website
This repo contains my personal website's source code.

# Local Development

#  Dependencies
-  [Rust](https://www.rust-lang.org/)
-  [trunk](https://trunkrs.dev/) (`cargo install trunk`)

#  Serving
-  For development purposes: `trunk serve --port 8080` (✅ Hot-reloading)
-  For production purposes: `trunk build` to generate the `dist/` folder, and deploy a production web server such as nginx.

#  Docker
[![Docker AMD64 Image](https://badgen.net/docker/size/simbleau/website/latest/amd64?icon=docker&label=amd64)](https://hub.docker.com/r/simbleau/website/)
[![Docker ARM64 Image](https://badgen.net/docker/size/simbleau/website/latest/arm64?icon=docker&label=arm64v8)](https://hub.docker.com/r/simbleau/website/)\
Quick Setup: `docker run -p 80:80 simbleau/website:latest`

##  Updating Docker Images
###  Dependencies
-  [Docker](https://docker.com)
-  [Buildx CLI Plugin](https://docs.docker.com/buildx/working-with-buildx/)
###  Instructions
-  `trunk build`
-  `export BUILD_TAG='simbleau/website:latest'`
-  `docker buildx build --platform linux/arm64/v8,linux/amd64 -t $BUILD_TAG --push .`

# License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
