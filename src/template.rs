pub struct Template {
    pub data: String,
}

impl Template {
    pub fn new<T>(inp: T) -> Self
    where
        T: std::fmt::Display,
    {
        Template {
            data: inp.to_string(),
        }
    }

    pub fn template<T, M>(self, key: T, value: M) -> Self
    where
        T: std::fmt::Display,
        M: std::fmt::Display,
    {
        Self {
            data: self
                .data
                .replace(&format!("{{{{{}}}}}", key), value.to_string().as_str()),
        }
    }

    pub fn build(self) -> String {
        self.data
    }
}
