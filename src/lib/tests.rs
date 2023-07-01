#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str  = "Is this working\nfor all the rows\nor not?";

    #[test]
    fn test_search() {
        let query = "all";
        let result = search(query, & CONTENTS);
        assert_eq!(result, vec!["for all the rows"])
    }

    #[test]
    fn test_case_sensitive_search() {
        let query = "AlL";
        let result = case_sensitive_search(query, & CONTENTS);
        assert_eq!(result, vec!["for all the rows"])
    }
}
