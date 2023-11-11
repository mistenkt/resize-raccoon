macro_rules! define_debug_levels {
    ($($name:ident = $value:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum DebugLevel {
            $($name = $value),*
        }

        impl DebugLevel {
            pub fn from_u32(value: u32) -> Option<Self> {
                match value {
                    $($value => Some(DebugLevel::$name),)*
                    _ => None,
                }
            }
        }
    };
}


#[macro_export]
macro_rules! debug_log_level {
    ($level:expr, $($arg:tt)*) => {
        if $crate::debug_utils::current_debug_level() as u8 >= $level as u8 {
            println!($($arg)*);
        }
    };
}


#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if $crate::debug_utils::current_debug_level()as u8 >= $crate::debug_utils::DebugLevel::Common as u8 {
            println!($($arg)*);
        }
    };
}

pub fn current_debug_level() -> DebugLevel {
    let level = std::env::var("APP_DEBUG")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<u32>()
        .unwrap_or(0);

    DebugLevel::from_u32(level).unwrap_or(DebugLevel::None)
}

define_debug_levels! {
    None = 0,
    Common = 1,
    Verbose = 2,
}
