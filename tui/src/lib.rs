use agmcore::core_logic;

pub fn run() {
    core_logic();
    println!("TUI not implemented yet.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}