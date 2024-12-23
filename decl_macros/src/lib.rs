#![cfg_attr(nightly, feature(macro_metavar_expr))]

//pub mod iter;
pub mod test;

use std::process::Command;

use color_eyre::{eyre::eyre, Report, Result, Section as _, SectionExt as _};
pub use num_traits::Pow;
#[allow(unused)]
pub use paste::paste;

///derive_units!(_per_s (suffix), [[Kibit, Mibit, Gibit, Tibit]: 1024f64,
///                                [kbit, Mbit, Gbit, Tbit]: 1000f64,
///                                [kiB, MiB, GiB, TiB]: 1024f64*8f64,
///                                [kB, MB, GB, TB]: 1000f64*8f64,])

#[cfg(feature = "nightly")]
#[macro_export]
macro_rules! derive_units {
    ($suffix:ident, $base:ident: $type:ty, $idx:ident, ($(($($unit:ident),+): $factor:expr),+)) => {

        $(
            $(
                $crate::paste! {
                    pub fn [<$unit $suffix>](&self) -> $type {
                        let $idx = ${index()};
                        self.$base / ($factor) // first gets first power et cetera
                    }
                }
            )+
        )+
    };
}

#[macro_export]
macro_rules! tracing_dbg {
    ($level:ident, $val:expr) => {{
        let val = $val;
        tracing::event!(tracing::Level::$level, "`{}` = {}", stringify!($val), val);
        val
    }};
}

pub trait Output {
    fn output2(&mut self) -> Result<String, Report>;
}

impl Output for Command {
    fn output2(&mut self) -> Result<String, Report> {
        let output = self.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(eyre!("cmd exited with non-zero status code"))
                .with_section(move || stdout.trim().to_string().header("Stdout:"))
                .with_section(move || stderr.trim().to_string().header("Stderr:"))
        } else {
            Ok(stdout.into())
        }
    }
}
