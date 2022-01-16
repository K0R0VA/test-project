use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SmartContract {
    pub id: u32,
    pub address: String,
}

impl SmartContract {
    pub fn get_smart_contracts() -> anyhow::Result<Vec<SmartContract>> {
        let file = File::open("./assets/giversListArray.json")?;
        let reader = BufReader::new(file);
        let smart_contracts = serde_json::from_reader(reader)?;
        Ok(smart_contracts)
    }
}

#[derive(Serialize, Clone, Copy, Default)]
pub struct SmartContractData {
    pub seed: u32,
    pub complexity: u32,
    pub grams: u32,
    pub interval: u32,
}

impl PartialEq for SmartContractData {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed
    }
}

impl From<String> for SmartContractData {
    fn from(_: String) -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_smart_contracts() {
        let smart_contracts = SmartContract::get_smart_contracts();
        assert!(smart_contracts.is_ok());
        if let Ok(smart_contracts) = smart_contracts {
            assert_eq!(10, smart_contracts.len());
        }
    }
}
