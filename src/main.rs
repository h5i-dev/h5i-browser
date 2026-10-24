//! The `h5i-browser` binary: a thin entry point over the engine's own command
//! line. Everything lives in the `h5i-browser` library crate; this just hands
//! it the process arguments. See `h5i_browser::cli` for the real interface.

fn main() -> ! {
    h5i_browser::cli::main(std::env::args_os())
}
