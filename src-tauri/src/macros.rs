use crate::config::DEBUG_ENABLED;
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        {
            if crate::config::DEBUG_ENABLED {
                println!($($arg)*);
            }
        }
    };
}
