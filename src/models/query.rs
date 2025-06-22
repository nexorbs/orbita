use serde_json::Value;

#[derive(Debug)]
pub enum QueryResult {
    Select(Vec<serde_json::Map<String, Value>>),
    Modify(u64),
}

impl QueryResult {
    pub fn rows_count(&self) -> usize {
        match self {
            QueryResult::Select(rows) => rows.len(),
            QueryResult::Modify(affected) => *affected as usize,
        }
    }

    pub fn is_select(&self) -> bool {
        matches!(self, QueryResult::Select(_))
    }
}
