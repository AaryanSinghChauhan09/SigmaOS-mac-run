#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppleSiliconChip { M1, M2, M3, M4, Unknown }

#[derive(Debug)]
pub struct MacBootConfig {
    pub chip: AppleSiliconChip,
    pub pcie_tunnel_enabled: bool,
}

impl MacBootConfig {
    pub fn from_chip_id(id: &str) -> Self {
        let chip = match id {
            "t8103" => AppleSiliconChip::M1,
            "t8112" => AppleSiliconChip::M2,
            "t8122" => AppleSiliconChip::M3,
            "t8132" => AppleSiliconChip::M4,
            _ => AppleSiliconChip::Unknown,
        };
        Self { chip, pcie_tunnel_enabled: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mac_boot_config_aaa() {
        // Arrange & Act
        let cfg1 = MacBootConfig::from_chip_id("t8103");
        let cfg2 = MacBootConfig::from_chip_id("t8132");
        // Assert
        assert_eq!(cfg1.chip, AppleSiliconChip::M1);
        assert_eq!(cfg2.chip, AppleSiliconChip::M4);
        assert!(cfg1.pcie_tunnel_enabled);
    }
}
