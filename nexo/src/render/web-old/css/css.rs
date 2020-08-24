use std::fmt::Write;

use super::ClassNames;

pub struct Css {
    css: String,
}

macro_rules! class {
    ($out:expr, $n:expr => $v:expr) => {
        write!(&mut $out, "{}:{{{};}}", $n, $v)
    };
}

impl Css {
    pub fn new() -> Css {
        Css {
            css: String::with_capacity(200 * 1024),
        }
    }

    pub fn generate(&mut self) {}

    // pub fn generate(&mut self) {
    //     write!(
    //         self.css,
    //         "
    //         {prefix}-reset {{
    //         }}

    //         {prefix}-flex-row {{
    //             display: flex;
    //             flex-direction: row;
    //         }}

    //         {prefix}-flex-col {{
    //             display: flex;
    //             flex-direction: column;
    //         }}
    //         ",
    //         prefix = self.prefix
    //     );
    // }
}
