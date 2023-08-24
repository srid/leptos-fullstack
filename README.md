# leptos-fullstack

A Nix template for [full-stack web apps](https://srid.ca/nojs) in Rust using Leptos. Tech used:

- [Leptos](https://leptos.dev/) full-stack framework 
    - [server functions](https://docs.rs/leptos/latest/leptos/attr.server.html)
    - [ssr + hydration](https://leptos-rs.github.io/leptos/ssr/index.html)
- [Axum](https://github.com/tokio-rs/axum) as backend framework
- [Tailwind CSS](https://tailwindcss.com/) for styling
- Build system
  - [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)
  - [Nix Flakes](https://zero-to-flakes.com/) + [crane](https://github.com/ipetkov/crane)
- Autoformatting using `rustfmt` and `leptosfmt`

## Running locally

Enter the Nix shell, and run:

```
# To spin up the dev server
just watch

# To run tests
just test
```

## Building

```
nix build

# To build the rust doccs
nix build .#leptos-fullstack-docs
```

You can also `nix run` the app directly.

## Using as flake-parts module

Import `./nix/flake-module.nix` from this repos as a flake-parts module. For an example project, see [nix-browser](https://github.com/juspay/nix-browser).

In the future, we might create a standalone flake-parts module for Leptos.

## Notes

### Single-crate model

This repo uses the single-crate model primarily because the backend-common-frontend split (prior to PR #5; similar to [start-axum-workspace](https://github.com/leptos-rs/start-axum-workspace) template) is not suitable when using [server functions](https://docs.rs/leptos/latest/leptos/attr.server.html) in leptos. Consequently, expect a bunch of `cfg` attributes or macros that make backend-only code conditional upon the `ssr` flag (which is set when compiling the crate for the backend, as opposed to the wasm frontend, which uses `csr`)

## Credits

The Rust code is largely based on [start-axum](https://github.com/leptos-rs/start-axum). Also, credit goes [to @benwis](https://github.com/benwis/benwis_leptos/blob/928ea7e7c20a86be91ad27e75cf297a2fbef681d/flake.nix#L148-L153) for pointers to how `crane` can be made to work with `cargo-leptos`.
