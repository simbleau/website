#  Personal Website
[![Docker AMD64 Image](https://badgen.net/docker/size/simbleau/website/latest/amd64?icon=docker&label=amd64)](https://hub.docker.com/r/simbleau/website/)
[![Docker ARM64 Image](https://badgen.net/docker/size/simbleau/website/latest/arm64?icon=docker&label=arm64v8)](https://hub.docker.com/r/simbleau/website/) \
This repo contains my personal website's source code.

#  Dependencies
-  [Rust](https://www.rust-lang.org/)
-  [trunk](https://trunkrs.dev/) (`cargo install trunk`)

#  Local Development
-  `trunk serve` ✅ Hot-reloading

#  Docker Deployment
##  Dependencies
-  [Docker](https://docker.com)
-  [Buildx CLI Plugin](https://docs.docker.com/buildx/working-with-buildx/)

##  Pushing
-  `trunk build`
-  `export BUILD_TAG='simbleau/website:latest'`
-  `docker buildx build --platform linux/arm64/v8,linux/amd64 -t $BUILD_TAG --push .`

# License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
