use std::{
    fmt::format,
    fs::File,
    io::{Read, Write},
    process::{Child, Stdio},
    thread,
    time::Duration,
};

use crate::model::{SmartContract, SmartContractData};

pub struct LiteClient {
    child: Child,
    smart_contract: SmartContract,
    data: Option<SmartContractData>,
}

impl LiteClient {
    fn new(smart_contract: SmartContract) -> anyhow::Result<Self> {
        let child = std::process::Command::new("./lite-client")
            .current_dir("./assets")
            .stdin(Stdio::piped())
            // .stdout(Stdio::piped())
            .spawn()?;
        Ok(Self {
            child,
            smart_contract,
            data: None,
        })
    }
    fn last(&mut self) -> anyhow::Result<()> {
        if let Some(writer) = self.child.stdin.as_mut() {
            writer.write_all(b"last\n")?;
            writer.flush()?;
        }
        Ok(())
    }
    fn get_pow_params(&mut self) -> anyhow::Result<String> {
        if let Some(writer) = self.child.stdin.as_mut() {
            let address = &self.smart_contract.address;
            let command = format!("runmethod  {address}  get_pow_params\n");
            writer.write_all(command.as_bytes())?;
            writer.flush()?;
        }
        if let Some(reader) = self.child.stdout.as_mut() {
            let mut buffer = String::new();
            reader.read_to_string(&mut buffer)?;
            return Ok(buffer);
        }
        Ok("".to_string())
    }
    pub fn set_data(&mut self, new_data: SmartContractData) -> anyhow::Result<()> {
        if let Some(data) = &self.data {
            if data.seed != new_data.seed {
                let json = serde_json::to_string(&new_data)?;
                self.data = Some(new_data);
                let mut file = File::open("./log.txt")?;
                file.write_all(json.as_bytes())?;
                drop(file);
            }
        } else {
            self.data = Some(new_data);
        }
        Ok(())
    }
    pub fn spawn(smart_contract: SmartContract) -> anyhow::Result<()> {
        let this = Self::new(smart_contract)?;
        thread::sleep(Duration::new(1, 0));
        let _ = thread::spawn(move || {
            let mut this = this;
            loop {
                thread::sleep(Duration::new(1, 0));
                if let Err(e) = this.last() {
                    println!("{e}");
                    break;
                }
                if let Ok(params) = this.get_pow_params().map(SmartContractData::from) {
                    if let Err(e) = this.set_data(params) {
                        println!("{e}");
                        break;
                    }
                }
            }
        }).join();
        Ok(())
    }
}
