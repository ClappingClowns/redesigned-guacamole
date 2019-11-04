use std::fmt::Display;

pub fn stringify<T: Display>(x: T) -> String {
    x.to_string()
}
