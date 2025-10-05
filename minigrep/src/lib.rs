pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    let q_lower = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&q_lower) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let q = "ty";
        let c = "\
            qwertyuiop
            asdfghjkl
            zxcvbnm
        ";
        assert_eq!(vec!["qwertyuiop"], search(q, c));
    }

    #[test]
    fn case_insensitive() {
        let q = "TY";
        let c = "\
            qwertyuiop
            asdfghjkl
            zxcvbnm
        ";
        assert_eq!(vec!["qwertyuiop"], search_insensitive(q, c));
    }
}
