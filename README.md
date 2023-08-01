# leptos-fullstack

Tech used,

- Nix + crane
- `cargo-leptos`
- TailwindCSS

WIP

- [x] use nightly
- [x] use axum
- [x] do vanilla wasm + trunk
    - [x] `nix build` must work
- [x] leptos integrate
- [x] tailwind
- final
    - [x] [server functions](https://docs.rs/leptos/latest/leptos/attr.server.html)
    - [ ] [ssr](https://leptos-rs.github.io/leptos/ssr/index.html)

## Running locally

```
just watch
```

## Building

```
nix build
```

You can also `nix run` the app directly.

## Notes

### Single-crate model

This repo uses the single-crate model  primarily because the backend-common-frontend split (prior to PR #5; similar to [start-axum-workspace](https://github.com/leptos-rs/start-axum-workspace) template) is not suitable when using [server functions](https://docs.rs/leptos/latest/leptos/attr.server.html) in leptos. Consequently, expect a bunch of `cfg` attributes or macros that make backend-only code conditional upon the `ssr` flag (which is set when compiling the crate for backend, as opposed to wasm frontend, which uses `csr`)

### cargo-leptos

We delegate the building of leptos app to `cargo-leptos`.