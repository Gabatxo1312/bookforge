#[derive(Clone)]
pub struct TemplateCtx {
    pub base_path: String,
}

impl TemplateCtx {
    pub fn asset(&self, path: &str) -> String {
        if self.base_path.is_empty() || self.base_path == "/" {
            format!("/{}", path)
        } else {
            format!("{}/{}", self.base_path.trim_end_matches('/'), path)
        }
    }
}
