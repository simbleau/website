#  Personal Website
[![build](https://img.shields.io/github/workflow/status/simbleau/website/build?style=for-the-badge&logo=github)](https://github.com/simbleau/website/actions/workflows/build.yml)
[![sponsor me](https://img.shields.io/badge/sponsor-30363D?style=for-the-badge&logo=GitHub-Sponsors&logoColor=#white)](https://github.com/sponsors/simbleau)
[![buy me a coffee](https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/simbleau)\
This repo contains my personal website's source code.

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

#  Deployment
##  Option 1: [Docker](https://docker.com) (Recommended)
[![Docker AMD64 Image](https://badgen.net/docker/size/simbleau/website/latest/amd64?icon=docker&label=amd64)](https://hub.docker.com/r/simbleau/website/)
[![Docker ARM64 Image](https://badgen.net/docker/size/simbleau/website/latest/arm64?icon=docker&label=arm64v8)](https://hub.docker.com/r/simbleau/website/)\
Quick setup: `docker run -p 80:80 simbleau/website:latest`

##  Option 2: Building
###  Dependencies
-  [Rust](https://www.rust-lang.org/)
-  [trunk](https://trunkrs.dev/) (`cargo install trunk`)
###  Serving
-  `trunk build` (generates the `dist/` folder)
-  Serve the `dist/` folder with a web server such as [nginx](https://www.nginx.com/).

# License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
