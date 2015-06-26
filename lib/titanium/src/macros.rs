#[macro_export]
macro_rules! def_bitfields {
    ( $t:ty, $($field:ident($l:expr, $r:expr),)* ) => {
            $(
                #[allow(non_snake_case)]
                pub mod $field {
                    pub const SHIFT : $t = $r as $t;
                    pub const BITS : $t = (1 << ($l as $t + 1 - $r as $t)) - 1;
                    pub const MASK : $t = ((1 << ($l as $t + 1 - $r as $t)) - 1) << $r;

                    pub fn from(val : $t) -> $t {
                        (val >> SHIFT) & BITS
                    }
                    pub fn to(val : $t) -> $t {
                        (val & BITS) << SHIFT
                    }
                }
            )*
    }
}


