
pub enum Json {
    Null,
    F64(f64),
    Str(String),
    Arr(Vec<Json>),
    Map(std::collections::HashMap<String, Json>)
}
