pub mod arg_parser;
pub mod opt_struct;

pub use arg_parser::*;
pub use opt_struct::*;

#[macro_export]
macro_rules! log {
    ($opts:expr, $string:expr, $level:expr) => {
        if $opts.verbose >= $level && !$opts.quiet {
            println!("{}", $string);
        }
    };
}
