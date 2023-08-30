use std::{process::Command, collections::HashMap, str::FromStr};
use stellar_sdk::{CallBuilder, Server, types::Asset, utils::{Direction, Endpoint}};
use stellar_xdr::{TransactionEnvelope, Type, TypeVariant};

pub enum OperationType {
  UploadContract,
  CreateContract,
  InvokeMethod
}

pub struct OperationFee {
  fee: i32,
  operation_type: OperationType,
  method_name: String
}

pub struct Account {
  pub address: String,
  pub pwd: String
}

impl Account {
  fn default () -> Account {
    Account{address: "".to_string(), pwd: "".to_string()}
  }

  fn new(ad: String, p: String) -> Account {
    Account { address: ad, pwd: p }
  }
}

pub struct Test {
  pub accounts: HashMap<String, Account>,
  pub fees: HashMap<String, OperationType>,
  pub contract_id: String
}

impl Test {
  pub fn new() -> Test {
    Test { 
        accounts: HashMap::new(), 
        fees: HashMap::new(),
        contract_id: "".to_string(),
    }
  }
}

pub fn setup_account_names(test: &mut Test) {
  test.accounts.insert("admin".to_string(), Account::default());
  test.accounts.insert("player_1".to_string(), Account::default());
  test.accounts.insert("player_2".to_string(), Account::default());
  test.accounts.insert("player_3".to_string(), Account::default() );
}

pub fn create_accounts(test: &mut Test) {
  for (acc, val) in test.accounts.iter_mut() {
    let output = Command::new("soroban")
                .arg("config").arg("identity").arg("generate").arg(&acc)
                .output();

    if output.is_ok() {
        let address = Command::new("soroban")
        .arg("config").arg("identity").arg("address").arg(&acc)
        .output();

        let pwd = Command::new("soroban")
        .arg("config").arg("identity").arg("show").arg(&acc)
        .output();

        if address.is_ok() && pwd.is_ok() {
            let ad = String::from_utf8(address.unwrap().stdout).unwrap().trim().to_owned();
            fund_account(&ad).expect("Fund wrong");

            let p = String::from_utf8(pwd.unwrap().stdout).unwrap().trim().to_owned();
            *val = Account::new(ad, p);
        }
    }
  }
}

fn fund_account(acc: &String) -> Result<(), Box<dyn std::error::Error>> {
  let url = "https://friendbot-futurenet.stellar.org/?addr=".to_string() + &acc;
  let resp = reqwest::blocking::get(url)?.text()?;
  println!("{:#?}", resp);
  Ok(())
}

pub fn deploy_contract(test: &mut Test, wasm_path: &String) {
  // let pwd = Command::new("pwd").output().expect("msg");
  let output = Command::new("soroban")
                      .arg("contract").arg("deploy")
                      .arg("--source").arg("admin")
                      .arg("--wasm").arg(&wasm_path)
                      .arg("--network").arg("futurenet")
                      .output()
                      .expect("Error deploying contract");

  if !output.stdout.is_empty() {
    test.contract_id = String::from_utf8(output.stdout).unwrap().trim().to_owned();
    print!("{}", &test.contract_id);
  }
  if !output.stderr.is_empty() {
    print!("{}", String::from_utf8(output.stderr).unwrap());
  }
}
