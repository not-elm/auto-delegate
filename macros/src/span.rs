use proc_macro::Span;

pub fn with_in_tests(span: &Span) -> bool {
    span.source_file()
        .path()
        .iter()
        .any(|p| p == "tests")
}


