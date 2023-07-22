/// Pagination information
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Pagination {
    /// URL of next page (if any). This is sometimes the empty string.
    /// To prevent misuse, we add an accessor.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    next: Option<String>,
}
impl Pagination {
    pub fn next(&self) -> Option<&str> {
        match self.next {
            Some(ref s) if s.is_empty() => None,
            ref s => s.as_deref(),
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Return<P> {
    /// The returned data.
    pub data: P,
    /// Pagination information (if any)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pagination: Option<Pagination>,
}
