<div align="center">

# Spencer C. Imbleau's Website

**To view the website, [click here](https://spencer.imbleau.com).**

[![last release](https://img.shields.io/github/release-date/simbleau/website?logo=github&label=Last%20Release)](https://github.com/simbleau/website/releases)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/simbleau/website/ci.yml?logo=github&label=CI)](https://github.com/simbleau/website/actions/workflows/ci.yml)

</div>

## About

My website is written in Rust with a frontend framework called [Yew](https://yew.rs), compiled to WebAssembly. Code is tested using [GitHub Actions](https://github.com/simbleau/website/actions/workflows/ci.yml) and deployment is automated through CI/CD with Terraform. Pushing to `main` will deploy to prod (AWS) and cutting a new [release](https://github.com/simbleau/website/releases/tag/latest) will attach the optimized `dist/` and sh256 hashes.

## 🔧 Development

- **Dependencies**
  - [Rust](https://www.rust-lang.org/)
  - [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
  - `wasm32-unknown-unknown` (`rustup target add wasm32-unknown-unknown`)

- **Serve**:

  ```bash
  trunk serve --open
  ```

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
