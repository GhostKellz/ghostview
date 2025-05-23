pub mod chaotic;

#[allow(dead_code)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[allow(dead_code)]
pub trait AurPackageManager {
    fn search(&self, query: &str) -> Vec<AurPackage>;
}