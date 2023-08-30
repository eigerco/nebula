#![cfg(test)]
use std::{process::Command, error::Error, str::FromStr};
use stellar_sdk::{CallBuilder, Server, utils::{Endpoint}};
use stellar_xdr::{Type, TypeVariant, TransactionResultMeta, TransactionMeta, TransactionResult, TransactionEnvelope};

mod common;

fn invoke_init(test: &common::Test) {
  let output = Command::new("soroban")
                  .arg("contract").arg("invoke")
                  .arg("--id").arg(&test.contract_id)
                  .arg("--source").arg("admin")
                  .arg("--network").arg("futurenet")
                  .arg("--")
                  .arg("init")
                      .arg("--admin").arg(&test.accounts["admin"].address)
                      .arg("--voting_period_secs").arg("3600")
                      .arg("--target_approval_rate_bps").arg("50000")
                      .arg("--total_voters").arg("3")
                  .output()
                  .expect("Could not invoke 'init'");


    if !output.stdout.is_empty() {
      print!("{}", String::from_utf8(output.stdout).unwrap());
    }
    if !output.stderr.is_empty() {
      print!("{}", String::from_utf8(output.stderr).unwrap());
    }

  // if !output.stdout.is_empty() {
  //   print!("{}", String::from_utf8(output.stdout).unwrap());
  //   return Ok(());
  // } else {
  //   let errorMsg = "Error";
  //   if !output.stderr.is_empty() {
  //     errorMsg += format!(": {}", String::from_utf8(output.stderr).unwrap());
  //     print!("{}", &errorMsg);
  //   }
  //   return Err("fdafda");
  // }
}

fn invoke_create_proposal(test: &common::Test) {
  let output = Command::new("soroban")
    .arg("contract").arg("invoke")
    .arg("--id").arg(&test.contract_id)
    .arg("--source").arg("admin") 
    .arg("--network").arg("futurenet")
    .arg("--")
    .arg("create_proposal")
    .arg("--id").arg("1")
    .output()
    .expect("Could not invoke 'create_proposal'");

  if !output.stdout.is_empty() {
    print!("{}", String::from_utf8(output.stdout).unwrap());
  }
  if !output.stderr.is_empty() {
    print!("{}", String::from_utf8(output.stderr).unwrap());
  }
}

fn invoke_vote(test: &common::Test) {
  for (acc, val) in test.accounts.iter() {
    if acc != "admin" {
      let output = Command::new("soroban")
      .arg("contract").arg("invoke")
      .arg("--id").arg(&test.contract_id)
      .arg("--source").arg(&acc)
      .arg("--network").arg("futurenet")
      .arg("--")
      .arg("vote")
        .arg("--voter").arg(&val.address)
        .arg("--id").arg("1")
      .output()
      .expect(&format!("Could not invoke 'vote' for player {}", acc));

      if !output.stdout.is_empty() {
        print!("{}", String::from_utf8(output.stdout).unwrap());
      }
      if !output.stderr.is_empty() {
        print!("{}", String::from_utf8(output.stderr).unwrap());
      }
    }
  }
}

fn check_results(test: &common::Test) {
  let s = String::from("https://horizon-futurenet.stellar.org/");
  let s = Server::new(s);

  // Load transactions of an account
  let my_account_id = String::from(&test.accounts["player_2"].address);
  let my_txs = s
      .transactions()
      .include_failed(false)
      .for_endpoint(Endpoint::Accounts(my_account_id))
      .call()
      .unwrap();

  // code below works, but we need result_meta_xdr not result_xdr
  // let result_meta = my_txs._embedded.records[0].result_xdr.to_string();
  // println!("{}", result_meta);
  // let typ = Type::from_xdr_base64(TypeVariant::from_str("TransactionResult").unwrap(), result_meta).expect("Error parsing XDR");
  // let decoded_result: &TransactionResult = typ.value().downcast_ref().unwrap();

  // The idea is to get last transaction and parse result_meta_xdr out of it. Unfortunately this does not work (panics at running Type::from_xdr_base64())
  let result_meta = my_txs._embedded.records.last().unwrap().result_meta_xdr.to_string();
  println!("{}", result_meta);
  let typ = Type::from_xdr_base64(TypeVariant::from_str("TransactionResultMeta").unwrap(), result_meta).expect("Error parsing XDR");
  let decoded_result: &TransactionResultMeta = typ.value().downcast_ref().unwrap();
  
  // let mut vote_result = 0;
  // match &result_meta.tx_apply_processing {
  //   soroban_sdk::xdr::TransactionMeta::V3(meta) => 
  //   { 
  //     let soroban_meta = meta.soroban_meta.clone().unwrap();
  //     let body = &soroban_meta.events[0].body;
  //     match &body {
  //       soroban_sdk::xdr::ContractEventBody::V0(body) => match body.data {
  //           soroban_sdk::xdr::ScVal::U32(r) => vote_result = r,
  //           // soroban_sdk::xdr::ScVal::I32(r) => vote_result = r,
  //           // soroban_sdk::xdr::ScVal::U64(r) => vote_result = r,
  //           // soroban_sdk::xdr::ScVal::I64(r) => vote_result = r,
  //           _ => todo!()
  //       }
  //   } }
  //   _ => todo!()
  // }
  // let result = result_meta.tx_apply_processing
  // println!("{}\n", vote_result);
}

#[test]
fn voting_tests() {
  let mut test = common::Test::new();
  common::setup_account_names(&mut test);
  common::create_accounts(&mut test);
  common::deploy_contract(&mut test, &"../../target/wasm32-unknown-unknown/release/voting.wasm".to_string());
  invoke_init(&test);
  invoke_create_proposal(&test);
  invoke_vote(&test);
  check_results(&test);
}
