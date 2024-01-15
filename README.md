# Spencer C. Imbleau's Website

The source code for my website.

**To view the website, [click here](https://spencer.imbleau.com).**

---
[![last release](https://img.shields.io/github/release-date/simbleau/website?logo=github&label=Last%20Release)](https://github.com/simbleau/website/releases)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/simbleau/website/ci.yml?logo=github&label=CI)](https://github.com/simbleau/website/actions/workflows/ci.yml)

## 🤖 Deployment

- Code is tested using [GitHub Actions](https://github.com/simbleau/website/actions/workflows/ci.yml)
Deployment is automated by [GitHub Actions](https://github.com/simbleau/website/actions).
- Pushes to `main` trigger a new [`latest`](https://github.com/simbleau/website/releases/tag/latest) release and deploy to AWS S3 + Cloudfront.

## 🔧 Development

- **Dependencies**
  - [Rust](https://www.rust-lang.org/)
  - [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- **Serve**:

  ```bash
  trunk serve --open
  ```

## 🔏 License

This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
