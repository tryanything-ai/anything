pub fn trim_newline(s: &String) -> String {
    let mut s = s.clone();
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trims_newlines_from_string() {
        let s = String::from("hello\n");
        assert_eq!(trim_newline(&s), "hello");

        let s = String::from("hello\r\n");
        assert_eq!(trim_newline(&s), "hello");

        let s = String::from("hello");
        assert_eq!(trim_newline(&s), "hello");
    }
}
