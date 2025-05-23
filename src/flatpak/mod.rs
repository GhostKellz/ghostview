#[allow(dead_code)]
pub struct FlatpakPackage {
    pub name: String,
    pub app_id: String,
    pub description: String,
}

#[allow(dead_code)]
pub trait FlatpakManager {
    fn list_installed(&self) -> Vec<FlatpakPackage>;
    fn search(&self, query: &str) -> Vec<FlatpakPackage>;
}

#[allow(dead_code)]
pub struct Flatpak;

impl FlatpakManager for Flatpak {
    fn list_installed(&self) -> Vec<FlatpakPackage> {
        // TODO: Implement using `flatpak list`
        vec![]
    }
    fn search(&self, _query: &str) -> Vec<FlatpakPackage> {
        // TODO: Implement using `flatpak search`
        vec![]
    }
}