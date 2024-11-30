use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Default)]
pub enum PackageManager {
    #[default]
    Dnf,
    Apt,
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let manager_name = match self {
            Self::Dnf => "dnf",
            Self::Apt => "apt",
        };

        f.write_str(manager_name)
    }
}
