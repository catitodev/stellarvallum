//! CSV output sanitization to prevent formula injection attacks.

/// Sanitize a cell value for safe CSV output.
/// Neutralizes formula injection by prefixing dangerous characters with a single quote,
/// escaping double-quotes, and wrapping in delimiters when needed.
pub fn sanitize_csv_cell(value: &str) -> String {
    let starts_with_formula_char = value.starts_with('=')
        || value.starts_with('+')
        || value.starts_with('-')
        || value.starts_with('@')
        || value.starts_with('\t')
        || value.starts_with('\r');

    let escaped = value.replace('"', "\"\"");

    let needs_quoting = escaped.contains(',')
        || escaped.contains('\n')
        || escaped.contains('"')
        || starts_with_formula_char;

    if starts_with_formula_char {
        format!("\"'{}\"", escaped)
    } else if needs_quoting {
        format!("\"{}\"", escaped)
    } else {
        escaped
    }
}

/// Write a full CSV row with sanitized cells.
pub fn csv_row(cells: &[&str]) -> String {
    cells
        .iter()
        .map(|c| sanitize_csv_cell(c))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_unchanged() {
        assert_eq!(sanitize_csv_cell("hello"), "hello");
    }

    #[test]
    fn formula_prefix_neutralized() {
        assert_eq!(sanitize_csv_cell("=CMD()"), "\"'=CMD()\"");
        assert_eq!(sanitize_csv_cell("+1+1"), "\"'+1+1\"");
        assert_eq!(sanitize_csv_cell("-1-1"), "\"'-1-1\"");
        assert_eq!(sanitize_csv_cell("@SUM(A1)"), "\"'@SUM(A1)\"");
    }

    #[test]
    fn quotes_doubled() {
        assert_eq!(sanitize_csv_cell("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn commas_quoted() {
        assert_eq!(sanitize_csv_cell("a,b"), "\"a,b\"");
    }

    #[test]
    fn newlines_quoted() {
        assert_eq!(sanitize_csv_cell("line1\nline2"), "\"line1\nline2\"");
    }
}
