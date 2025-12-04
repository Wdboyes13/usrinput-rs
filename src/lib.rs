/// Prompt user for input and read line
///
/// # Arguments
/// * `buf` - A mutable `String` input buffer
/// * `args` - Format string for printing
///
/// # Returns
/// `Result<usize, std::io::Error>` - Size of bytes read or error
///
/// # Examples
/// ```
/// let mut inp = String::new();
/// input!(inp, "Enter your name: ").expect("Failed to read input!");
/// ```
#[macro_export]
macro_rules! input {
    ($buf:ident, $($args:tt)*) => {{
        print!($($args)*);
        match std::io::Write::flush(&mut std::io::stdout()) {
            Ok(_) => std::io::stdin().read_line(&mut $buf),
            Err(e) => Err(e)
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn t0() -> Result<(), std::io::Error> {
        let mut s = String::new();
        match input!(s, "Enter something: ") {
            Ok(_) => {
                println!("Got input: {}", s);
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
        Ok(())
    }
}
