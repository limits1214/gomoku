pub fn get_table_name() -> &'static str {
    super::config::get_dynamo_settings().table_name.as_str()
}
