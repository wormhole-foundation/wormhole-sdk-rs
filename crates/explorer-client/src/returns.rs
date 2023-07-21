#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Return<P> {
    data: P,
}
