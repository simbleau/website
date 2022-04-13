#  Personal Website
![Docker Version](https://img.shields.io/docker/v/simbleau/website)
[![Artifact Hub](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/website)](https://artifacthub.io/packages/search?repo=website)

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