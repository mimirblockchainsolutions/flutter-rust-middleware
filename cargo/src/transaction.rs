use mimir_common::types::{Bytes, U256};
use reqwest;
use serde_json::Value;
use transact::RawTxBuilder;
use mimir_crypto::secp256k1::{Signer, Address};
use error::Error;

macro_rules! opt_slice {
    ($opt:ident) => {
        $opt.as_ref().map(|val| val.as_ref())
    }
}
// try some stuff with the optional fields like get them, guess or infer
pub fn build_transaction(
    signer: Signer,
    nonce: U256,
    gas_price: U256,
    gas_limit: U256,
    to: Address,
    value: Option<U256>,
    data: Option<Bytes>,
) -> Bytes {
    let calldata = RawTxBuilder {
        signer: signer,
        nonce: Some(&nonce),
        gas_price: Some(&gas_price),
        gas_limit: Some(&gas_limit),
        to: Some(&to),
        value: opt_slice!(value),
        data: opt_slice!(data),
    }.finish();
    calldata.into()
}

pub fn send_transaction(signed: Bytes) -> Result<Value, Error> {
    let rpc = json!({
        "method": "eth_sendRawTransaction",
        "params": [signed],
        "id": 0,
        "jsonrpc": "2.0"
    });
    let client = reqwest::Client::new();
    let mut res = client.post("http://127.0.0.1:8545").json(&rpc).send()?;
    let rsp_json = res.json()?;
    Ok(rsp_json)
}

pub fn get_nonce(addr: Address) -> Result<U256, Error> {
    let rpc = json!({
        "method": "eth_getTransactionCount",
        "params": [addr, "latest"],
        "id": 0,
        "jsonrpc": "2.0"
    });
    let client = reqwest::Client::new();
    let mut res = client.post("http://127.0.0.1:8545").json(&rpc).send()?;
    let rslt: JsonResult<U256> = res.json()?;
    Ok(rslt.result)
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonResult<T> {
    result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateTx {
    pub to: Address,
    pub from: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}

pub fn estimate_gas_cost(tx: EstimateTx) -> Result<U256, Error> {
    let rpc = json!({
        "method": "eth_estimateGas",
        "params": [tx],
        "id": 0,
        "jsonrpc": "2.0"
    });
    let client = reqwest::Client::new();
    let mut res = client.post("http://127.0.0.1:8545").json(&rpc).send()?;
    let rsp_json: Value = res.json()?;
    if let Some(Value::String(price_string)) = rsp_json.get("result") {
        let parsed = price_string.parse()?;
        Ok(parsed)
    } else {
        let msg = "expected jsonrpc result of type `String` for gas price";
        Err(Error::message(msg))
    }
}

pub fn get_gas_price() -> Result<U256, Error> {
    let rpc = json!({
        "method": "eth_gasPrice",
        "id": 0,
        "jsonrpc": "2.0"
    });
    let client = reqwest::Client::new();
    let mut res = client.post("http://127.0.0.1:8545").json(&rpc).send()?;
    let rsp_json: Value = res.json()?;
    if let Some(Value::String(price_string)) = rsp_json.get("result") {
        let parsed = price_string.parse()?;
        Ok(parsed)
    } else {
        let msg = "expected jsonrpc result of type `String` for gas price";
        Err(Error::message(msg))
    }
}
