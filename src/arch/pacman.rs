use super::ArchPackage;
use super::ArchPackageManager;

#[allow(dead_code)]
pub struct Pacman;

impl ArchPackageManager for Pacman {
    fn list_installed(&self) -> Vec<ArchPackage> {
        // TODO: Implement using `pacman -Q`
        vec![]
    }
    fn search(&self, _query: &str) -> Vec<ArchPackage> {
        // TODO: Implement using `pacman -Ss`
        vec![]
    }
}