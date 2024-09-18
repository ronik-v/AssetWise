pub fn is_valid_date(date_str: &str) -> bool {
    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").is_ok()
}