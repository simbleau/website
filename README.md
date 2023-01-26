# <img src="static/logo.svg" width="20px" height="20px"/> Spencer C. Imbleau's Website
A repository containing the source code for my website.

**To view the website, [click here](https://spencer.imbleau.com).**\
**To see how I deploy with Kubernetes at home, [click here](https://github.com/simbleau/home-ops/tree/main/charts/my-website).**

---
[![last release](https://img.shields.io/github/release-date/simbleau/website?logo=github&label=Last%20Release)](https://github.com/simbleau/website/releases)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/simbleau/website/ci.yml?logo=github&label=CI)](https://github.com/simbleau/website/actions/workflows/ci.yml)

# 🤖 Deployment
- Code is tested using [GitHub Actions](https://github.com/simbleau/website/actions/workflows/ci.yml)
Deployment is automated by [GitHub Actions](https://github.com/simbleau/website/actions).
- Pushes to `main` trigger a new [`latest`](https://github.com/simbleau/website/releases/tag/latest) release and [DockerHub image](https://hub.docker.com/r/simbleau/website/tags).
- Manually pushed [releases](https://github.com/simbleau/website/releases) also upload a correspondingly tagged [DockerHub image](https://hub.docker.com/r/simbleau/website/tags).

# 🔧 Development
- **Dependencies**
  - [Rust](https://www.rust-lang.org/)
  - [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- **Serve**:
  ```bash
  trunk serve --open
  ```

# 🌐 Serving
## Option 1: [Docker](https://docker.com) (Recommended)
[![Docker AMD64 Image](https://badgen.net/docker/size/simbleau/website/latest/amd64?icon=docker&label=amd64)](https://hub.docker.com/r/simbleau/website/tags)
[![Docker ARM64 Image](https://badgen.net/docker/size/simbleau/website/latest/arm64?icon=docker&label=arm64v8)](https://hub.docker.com/r/simbleau/website/tags)\
[![Docker Image Version (tag latest semver)](https://img.shields.io/docker/v/simbleau/website/latest?label=version%20%28latest%29)](https://hub.docker.com/r/simbleau/website/tags)

- **Pull**:
  ```bash
  docker pull simbleau/website:latest
  ```
- **Run**:
  ```bash
  docker run -p 80:80 simbleau/website:latest
  ```

## Option 2: Manual
- **Build** *(generates a `dist/` folder)*:
  ```bash
  trunk build --release
  ```
- Serve the `dist/` folder with a web server such as [nginx](https://www.nginx.com/)
  - Note: [`website.nginx.conf`](website.nginx.conf) is the nginx configuration that the Docker image uses

## 🔏 License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
