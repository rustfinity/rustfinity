pub fn filter_starts_with(
    input: &[String],
    keyword: &str,
) -> impl Iterator<Item = String> {
    input.iter().filter(move |s| s.starts_with(keyword))
}
