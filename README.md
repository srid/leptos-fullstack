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

### server fns are decoupled

The `common` contains server fns but without a function body (uses `unimplemeted!` macro). The actual implementation lives in the `backend` crates, where we assume the exact same function signature and server macro spec is used. 

This decoupling is essential to avoid cyclic dependency or to avoid making the `common` a gigantic monolith with backend logic.