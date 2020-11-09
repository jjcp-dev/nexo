pub struct ClassNames {
    pub flex_col: String,
    pub flex_row: String,
    pub rectangle: String,
    pub text: String,
    pub prefix: String,
}

impl ClassNames {
    pub fn new(prefix: String) -> ClassNames {
        ClassNames {
            flex_col: format!("{}-flex-col", prefix).to_string(),
            flex_row: format!("{}-flex-row", prefix).to_string(),
            rectangle: format!("{}-rectangle", prefix).to_string(),
            text: format!("{}-text", prefix).to_string(),
            prefix: prefix,
        }
    }
}
