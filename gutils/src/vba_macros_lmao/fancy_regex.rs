#[macro_export]
macro_rules! f_regex {
    ($re:expr) => {{
        use fancy_regex::Regex as FancyRegex;
        // type Lazy = std::sync::LazyLock<FancyRegex>;
        // static RE: Lazy = Lazy::new(|| FancyRegex::new($re).expect("Failed to parse static regex"));
        let re: FancyRegex = FancyRegex::new($re).expect("Failed to parse static regex");
        re
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn match_numbers() {
        let number = f_regex!("[0-9]+");
        assert!(number.is_match("1824").unwrap());
        assert!(!number.is_match("this wont match").unwrap());
    }

    #[test]
    #[should_panic]
    fn invalid_syntax() {
        let _ = f_regex!("?[");
    }
}
