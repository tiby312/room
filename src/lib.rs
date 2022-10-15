//! Build xml / html / svg programmatically by chaining structs together or by closures. Instead of using a templating engine, write data/markup that 'looks like' rust.
//!
//! You can find hypermelon on [github](https://github.com/tiby312/hypermelon) and [crates.io](https://crates.io/crates/hypermelon).
//! Documentation at [docs.rs](https://docs.rs/hypermelon)

pub mod tools;
use std::fmt;
pub mod attr;
pub mod build;
pub mod elem;
use attr::*;
pub mod render;

use elem::*;
use tools::WriteWrap;

pub mod prelude {
    //! The hypermelon prelude
    pub use super::attrs;
    pub use super::elem::Elem;
    pub use super::elems;
    pub use super::format_move;
}

///
/// Render elements to a writer
///
pub fn render<E: Elem + Locked, W: fmt::Write>(elem: E, writer: W) -> fmt::Result {
    render::Renderer::new().render(elem, writer)
}

///
/// Render elements to a writer that allows for escaping elements.
///
pub fn render_escapable<E: Elem, W: fmt::Write>(elem: E, writer: W) -> fmt::Result {
    render::Renderer::new().render_escapable(elem, writer)
}

///
/// An std out that implements fmt::Write
///
pub fn stdout_fmt() -> tools::Adaptor<std::io::Stdout> {
    tools::upgrade_write(std::io::stdout())
}

///
/// call `Elem::append()` without having to have Elem in scope.
///
pub fn append<R: Elem, K: Elem>(a: R, k: K) -> Append<R, K> {
    a.append(k)
}

///
/// Chain together a list of attrs
///
#[macro_export]
macro_rules! attrs {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::attr::Attr;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}

///
/// Chain together a list of elements
///
#[macro_export]
macro_rules! elems {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::elem::Elem;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}
