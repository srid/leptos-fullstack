mod app;
mod thing;

#[cfg(not(feature = "ssr"))]
pub fn main() {
    app::main()
}
