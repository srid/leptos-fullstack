# leptos-fullstack

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

### Tech used

- Nix + crane
- https://trunkrs.dev/ (over `cargo-leptos`)

### Workspace model

This repo uses the workspace model (as championed by [start-axum-workspace](https://github.com/leptos-rs/start-axum-workspace) template) primarily because the backend-common-frontend split (prior to PR #5) is not suitable when using [server functions](https://docs.rs/leptos/latest/leptos/attr.server.html) in leptos. Consequently, expect a bunch of `cfg` attributes or macros that make backend-only condition upon the `ssr` flag (which is set when compiling the crate for backend, as opposed to wasm frontend, which uses `csr`)