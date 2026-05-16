//! CSV output sanitization against formula injection.

/// Sanitize a cell value to prevent CSV injection attacks.
/// Prefixes formula characters with a single quote and properly escapes/quotes values.
pub fn sanitize_cell(value: &str) -> String {
    let starts_with_formula = value.starts_with('=')
        || value.starts_with('+')
        || value.starts_with('-')
        || value.starts_with('@')
        || value.starts_with('\t')
        || value.starts_with('\r');

    let escaped = value.replace('"', "\"\"");

    let needs_quoting = escaped.contains(',')
        || escaped.contains('\n')
        || escaped.contains('"')
        || starts_with_formula;

    if starts_with_formula {
        format!("\"'{}\"", escaped)
    } else if needs_quoting {
        format!("\"{}\"", escaped)
    } else {
        escaped
    }
}

/// Build a full CSV row from sanitized cells.
pub fn build_row(cells: &[&str]) -> String {
    cells.iter()
        .map(|c| sanitize_cell(c))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_unchanged() {
        assert_eq!(sanitize_cell("hello"), "hello");
    }

    #[test]
    fn formula_prefix_neutralized() {
        assert_eq!(sanitize_cell("=CMD()"), "\"'=CMD()\"");
        assert_eq!(sanitize_cell("+1+1"), "\"'+1+1\"");
        assert_eq!(sanitize_cell("-1-1"), "\"'-1-1\"");
        assert_eq!(sanitize_cell("@SUM(A1)"), "\"'@SUM(A1)\"");
    }

    #[test]
    fn quotes_doubled() {
        assert_eq!(sanitize_cell("say \"hello\""), "\"say \"\"hello\"\"\"");
    }

    #[test]
    fn commas_quoted() {
        assert_eq!(sanitize_cell("a,b"), "\"a,b\"");
    }

    #[test]
    fn newlines_quoted() {
        assert_eq!(sanitize_cell("line1\nline2"), "\"line1\nline2\"");
    }
}
