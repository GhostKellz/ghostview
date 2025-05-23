use super::AurPackage;
use super::AurPackageManager;

#[allow(dead_code)]
pub struct ChaoticAur;

impl AurPackageManager for ChaoticAur {
    fn search(&self, _query: &str) -> Vec<AurPackage> {
        // TODO: Implement Chaotic-AUR search
        vec![]
    }
}