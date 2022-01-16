use std::fs::File;

use lite_client::LiteClient;
use model::SmartContract;

mod lite_client;
mod model;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smart_contracts = SmartContract::get_smart_contracts()?;
    let file = File::create("./log.txt")?;
    drop(file);
    smart_contracts
        .into_iter()
        .try_for_each(|smart_contract| LiteClient::spawn(smart_contract))?;
    Ok(())
}
