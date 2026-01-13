use agmcore::core_logic;

pub fn run() {
    core_logic();
    println!("GUI not implemented yet.");
    // iced will need more setup here for a basic window.
    // For now, this is a placeholder.
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}