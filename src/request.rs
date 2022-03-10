pub struct Request {
    query: Vec<String>,
}

impl Request {
    pub fn new(query: Vec<String>) -> Self {
        Self { query }
    }

    pub fn command(&self) -> &str {
        self.query[0].as_str()
    }

    pub fn arity(&self) -> i64 {
        self.query.len().try_into().unwrap()
    }

    pub fn arguments(&self) -> &[String] {
        &self.query[1..]
    }
}