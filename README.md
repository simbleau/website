#  Personal Website
This repo contains my personal website's source code.

##  Dependencies
-  [Rust](https://www.rust-lang.org/)
-  [trunk](https://trunkrs.dev/) (`cargo install trunk`)

##  Local Development
-  `trunk serve` ✅ Hot-reloading

##  Deployment
###  Dependencies
 -  [Docker](https://docker.com)
 -  [Buildx CLI Plugin](https://docs.docker.com/buildx/working-with-buildx/)

###  Pushing
-  `trunk build`
-  `export BUILD_TAG='simbleau/website:construction'`
-  `docker buildx build --platform linux/arm64/v8,linux/amd64 -t $BUILD_TAG --push .`