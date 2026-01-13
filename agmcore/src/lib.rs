pub fn core_logic() {
    println!("This is the core logic.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        core_logic();
    }
}