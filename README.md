# vuj.io

[![github.com vujio](https://img.shields.io/badge/github-vujio-informational?style=flat-square&logo=github)](https://crates.io/crates/vujio)
[![crates.io vujio](https://img.shields.io/crates/v/vujio.svg?style=flat-square&logo=rust)](https://crates.io/crates/vujio)

_/vu-hē-oʊ/_

## Description

An _experimental_ fast and pertinent web platform for modern devices.  
Rust backend and TypeScript frontend.  

## Roadmap

- [x] Framework macros
- [x] Client bundling
- [ ] Realtime communication
- [ ] Shared state bindings
- [ ] Component system
- [ ] 3D rendering
- [ ] TBA

## Values

- Fast development, delivery, and runtime.
- Smaller, simpler, easier.

_See also [Rust by Example](https://doc.rust-lang.org/rust-by-example/) and [TypeScript Design Goals](https://github.com/Microsoft/TypeScript/wiki/TypeScript-Design-Goals)_

## Setup

Rust nightly is required.  
_See [Rust Documentation: Unstable Features](https://doc.rust-lang.org/cargo/reference/unstable.html#unstable-features)._

1. [Install VS Code](https://code.visualstudio.com/download)
2. Open `vujio.code-workspace`
3. Install recommended extensions: (VS Code will prompt)  
   [./.vscode/extensions.json](./.vscode/extensions.json)
4. [Install Rust](https://www.rust-lang.org/tools/install)
5. Set to Rust Nightly, run:  
```rustup default nightly && rustup update```

### Workspace Hotkeys

- Run w/ Debugger: `F5`
- Run and Watch Changes: `CRTL`+`SHIFT`+`B`
- Toggle Breakpoint: `F9`

## Entrypoints

- Server: [src/main.rs](src/main.rs)  
- Client: [src/main.ts](src/main.ts)

## Crates

- [Cargo.toml](Cargo.toml)
- [vujio/Cargo.toml](vujio/Cargo.toml)
- - [vujio_client/Cargo.toml](vujio_client/Cargo.toml)
- - [vujio_server/Cargo.toml](vujio_server/Cargo.toml)

## License

Licenses are available at your option:  
 - [MIT License](LICENSE-MIT.md)
 - [Apache License Version 2.0](LICENSE-APACHE.md)

All contributions are subject to dual license as defined in [Apache License Version 2.0](LICENSE-APACHE.md).