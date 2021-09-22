#[allow(dead_code)]
#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    F64(f64),
    Str(String),
    Arr(Vec<Json>),
    Map(std::collections::HashMap<String, Json>)
}
