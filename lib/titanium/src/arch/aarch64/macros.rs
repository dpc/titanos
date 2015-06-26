//! Convenient macro to read a CPU register



#[macro_export]
macro_rules! def_reg_read {
    ( $reg:ident, $t:ty ) => {
            #[inline]
            pub fn read() -> $t {
                let val : $t;
                unsafe {
                    asm!(concat!("mrs $0, ", stringify!($reg))
                         : "=r"(val)
                         :
                         :
                         : "volatile"
                        );
                }
                val
            }
    }
}
#[macro_export]
macro_rules! def_reg_write {
    ( $reg:ident, $t:ty ) => {
        #[inline]
        pub fn write(val : $t) {
            unsafe {
                asm!(concat!("msr ", stringify!($reg), ", $0" )
                     :
                     : "r"(val)
                     :
                     : "volatile"
                    );
            }
        }
    }
}

#[macro_export]
macro_rules! def_reg {
    ($reg:ident, $t:ty, rw, $($field:ident($l:expr, $r:expr),)* ) => {
        #[allow(non_snake_case)]
        pub mod $reg {
            def_reg_read!($reg, $t);
            def_reg_write!($reg, $t);
            def_bitfields!($t, $($field($l, $r),)*);
        }
    };
    ($reg:ident, $t:ty, ro, $($field:ident($l:expr, $r:expr),)* ) => {
        #[allow(non_snake_case)]
        pub mod $reg {
            def_reg_read!($reg, $t);
            def_bitfields!($t, $($field($l, $r),)*);
        }
    };
    ($reg:ident, $t:ty, wo, $($field:ident($l:expr, $r:expr),)* ) => {
        #[allow(non_snake_case)]
        pub mod $reg {
            def_reg_write!($reg, $t);
            def_bitfields!($t, $($field($l, $r),)*);
        }
    };

}
