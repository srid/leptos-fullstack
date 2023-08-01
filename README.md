# leptos-fullstack

> [!WARNING] 
> Work in progress

A template for full-stack Rust apps using Leptos. Tech used:

- [Leptos](https://leptos.dev/) full-stack framework 
    - [server functions](https://docs.rs/leptos/latest/leptos/attr.server.html)
    - [ssr + hydration](https://leptos-rs.github.io/leptos/ssr/index.html)
- [Axum](https://github.com/tokio-rs/axum) for backend server
- [Tailwind CSS](https://tailwindcss.com/)
- Build system
  - `cargo-leptos`
  - [Nix Flakes](https://zero-to-flakes.com/) + [crane](https://github.com/ipetkov/crane)

## Running locally

```
just watch
```

## Building

```
nix build
```

You can also `nix run` the app directly.

## Using as flake-parts module

Import `this flake + /nix/flake-module.nix` as a flake-parts module.

In future, we might create a standalone flake-parts module for Leptos.

## Notes

### Single-crate model

This repo uses the single-crate model  primarily because the backend-common-frontend split (prior to PR #5; similar to [start-axum-workspace](https://github.com/leptos-rs/start-axum-workspace) template) is not suitable when using [server functions](https://docs.rs/leptos/latest/leptos/attr.server.html) in leptos. Consequently, expect a bunch of `cfg` attributes or macros that make backend-only code conditional upon the `ssr` flag (which is set when compiling the crate for backend, as opposed to wasm frontend, which uses `csr`)

### cargo-leptos

We delegate the building of leptos app to `cargo-leptos`.