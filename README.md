<div align="center">

<img src="static/logo.svg" width="144px" height="144px"/>

# Spencer C. Imbleau's Website
[![build](https://img.shields.io/github/workflow/status/simbleau/website/build?style=for-the-badge&logo=github)](https://github.com/simbleau/website/actions/workflows/build.yml)
[![sponsor me](https://img.shields.io/badge/sponsor-30363D?style=for-the-badge&logo=GitHub-Sponsors&logoColor=#white)](https://github.com/sponsors/simbleau)
[![buy me a coffee](https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/simbleau)

<h3>To view the website, click <a href="https://spencer.imbleau.com">here</a>.</h3>

</div>

---

# 🤖 GitOps
- Builds are tested using [GitHub Actions](https://github.com/simbleau/website/actions/workflows/build.yml)
- Builds are released using [GitHub Actions](https://github.com/simbleau/website/actions/workflows/build.yml) to [DockerHub](https://hub.docker.com/r/simbleau/website).
- [FluxCD](https://fluxcd.io/) monitors DockerHub's [`simbleau/website:latest`](https://hub.docker.com/r/simbleau/website/tags?name=latest) and automatically upgrades on [my home infrastructure](https://github.com/simbleau/home-ops).


# 🔧 Development
## Dependencies
- [Rust](https://www.rust-lang.org/)
- [trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [wasm32-unkown-unknown](https://yew.rs/docs/getting-started/introduction#install-webassembly-target) (`rustup target add wasm32-unknown-unknown`)
## Serving
- Serve: `trunk serve --port 8080`
- Preview: [`http://localhost:8080/`](http://localhost:8080/) (✅ Hot-reloading)

# 🌐 Serving
## Option 1: [Docker](https://docker.com) (Recommended)
[![Docker AMD64 Image](https://badgen.net/docker/size/simbleau/website/latest/amd64?icon=docker&label=amd64)](https://hub.docker.com/r/simbleau/website/tags)
[![Docker ARM64 Image](https://badgen.net/docker/size/simbleau/website/latest/arm64?icon=docker&label=arm64v8)](https://hub.docker.com/r/simbleau/website/tags)\
[![Docker Image Version (tag latest semver)](https://img.shields.io/docker/v/simbleau/website/latest?label=version%20%28latest%29)](https://hub.docker.com/r/simbleau/website/tags)\
Quick setup: `docker run -p 80:80 simbleau/website:latest`
## Option 2: Trunk + nginx
### Dependencies
- [Rust](https://www.rust-lang.org/)
- [trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [wasm32-unkown-unknown](https://yew.rs/docs/getting-started/introduction#install-webassembly-target) (`rustup target add wasm32-unknown-unknown`)
### Serving
- `trunk build --release` generates the `dist/` folder
- Serve the `dist/` folder with a web server such as [nginx](https://www.nginx.com/)
  - Note: [`website.nginx.conf`](website.nginx.conf) is the nginx configuration that the Docker image uses

## 🔏 License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.