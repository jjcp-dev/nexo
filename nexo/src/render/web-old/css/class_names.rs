pub struct ClassNames {
    flex_col: String,
    flex_row: String,
    rectangle: String,
    text: String,
    prefix: String,
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

    #[inline]
    pub fn flex_col(&self) -> &str {
        &self.flex_col
    }

    #[inline]
    pub fn flex_row(&self) -> &str {
        &self.flex_row
    }

    #[inline]
    pub fn rectangle(&self) -> &str {
        &self.rectangle
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }
}
