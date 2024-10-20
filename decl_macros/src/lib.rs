#![feature(macro_metavar_expr)]
pub mod test;

pub use num_traits::Pow;
#[allow(unused)]
pub use paste::paste;

///derive_units!(_per_s (suffix), [[Kibit, Mibit, Gibit, Tibit]: 1024f64,
///                                [kbit, Mbit, Gbit, Tbit]: 1000f64,
///                                [kiB, MiB, GiB, TiB]: 1024f64*8f64,
///                                [kB, MB, GB, TB]: 1000f64*8f64,])
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
