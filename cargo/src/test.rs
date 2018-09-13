// -------------------------------
fn testimate() -> Result<U256, Error> {
    let test_tx = EstimateTx {
        to: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
            .parse()
            .unwrap(),
        from: Some(
            "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
                .parse()
                .unwrap(),
        ),
        value: Some(0xdeadbeefu32.into()),
        data: Some("0xdeadbeef".parse().unwrap()),
    };
    estimate_gas_cost(test_tx)
}

fn test_tx() {
    let nonce = 1u32.into();
    let value = 0xdeadbeefu32.into();
    let gas_price = 0x01u32.into();
    let gas_limit = 0x5555u32.into();
    let signer = dev_signer();
    let to = "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
        .parse()
        .unwrap();
    let calldata = build_transaction(
        signer,
        Some(nonce),
        Some(gas_price),
        Some(gas_limit),
        Some(to),
        Some(value),
        None,
    );
    send_transaction(calldata);
    // println!("calldata = {:?}", calldata)
}
