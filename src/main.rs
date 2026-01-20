
pub fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("{}", hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
