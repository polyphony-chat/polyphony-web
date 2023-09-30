<div align="center">

[![Discord]][Discord-invite]
[![Build][build-shield]][build-url]
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Issues][issues-shield]][issues-url]
<img src="https://img.shields.io/static/v1?label=Status&message=Early%20Development&color=blue">

</br>
<div align="center">
  <a href="https://github.com/polyphony-chat/polyphony-web">
    <img src="https://raw.githubusercontent.com/polyphony-chat/design/main/branding/polyphony-2-4.png" alt="Logo" width="128" height="128">
  </a>

<h3 align="center">Polyphony Web</h3>

  <p align="center">
    A multi-instance, cross-platform Discord/Spacebar API-compatible chat client, written in Rust using Yew and optionally Tauri.
    <br />
    <a href="https://github.com/polyphony-chat/polyphony-web"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/polyphony-chat/polyphony-web/issues">Report Bug</a>
    ·
    <a href="https://github.com/polyphony-chat/polyphony-web/issues">Request Feature</a>
    ·
    <a href="https://discord.gg/8tKSC8wzDq">Join Discord</a>
  </p>
</div>

</div>

## Setting up a dev environment

Add wasm as a compilation target

```
rustup target add wasm32-unknown-unknown
```

Install `tauri-cli` and `trunk`
```sh
cargo install tauri-cli trunk --locked
```

To start the web-based application, change into the `crates/polyphony-wasm` directory and run `trunk serve`.

To start the Tauri powered Desktop-App instead, change into the `crates/polyphony-tauri` directory and run `cargo tauri dev`. 

> Note that the Tauri Dev Server will try to run and listen for the `trunk serve` Dev server. If it fails to do so (perhaps because Port `8080` is already occupied), it will not be able to start the Tauri App.

[Rust]: https://img.shields.io/badge/Rust-orange?style=plastic&logo=rust
[Rust-url]: https://www.rust-lang.org/
[build-shield]: https://img.shields.io/github/actions/workflow/status/polyphony-chat/polyphony-web/rust.yml?style=flat
[build-url]: https://github.com/polyphony-chat/polyphony-web/blob/main/.github/workflows/rust.yml
[contributors-shield]: https://img.shields.io/github/contributors/polyphony-chat/polyphony-web.svg?style=flat
[contributors-url]: https://github.com/polyphony-chat/polyphony-web/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/polyphony-chat/polyphony-web.svg?style=flat
[forks-url]: https://github.com/polyphony-chat/polyphony-web/network/members
[stars-shield]: https://img.shields.io/github/stars/polyphony-chat/polyphony-web.svg?style=flat
[stars-url]: https://github.com/polyphony-chat/polyphony-web/stargazers
[issues-shield]: https://img.shields.io/github/issues/polyphony-chat/polyphony-web.svg?style=flat
[issues-url]: https://github.com/polyphony-chat/polyphony-web/issues
[license-shield]: https://img.shields.io/github/license/polyphony-chat/polyphony-web.svg?style=f;at
[license-url]: https://github.com/polyphony-chat/polyphony-web/blob/master/LICENSE
[Discord]: https://dcbadge.vercel.app/api/server/m3FpcapGDD?style=flat
[Discord-invite]: https://discord.com/invite/m3FpcapGDD
