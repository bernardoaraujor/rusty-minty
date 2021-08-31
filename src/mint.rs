use std::fs::File;
use web3::contract::tokens::Tokenize;

pub async fn mint(
    web3_provider: &str,
    priv_key: secp256k1::SecretKey,
    token_owner: web3::types::Address,
    gas_limit: u64,
    contract_address: web3::types::H160,
    contract_abi: File,
    cid: String,
) -> web3::contract::Result<()> {
    let transport = web3::transports::Http::new(web3_provider)?;
    let web3 = web3::Web3::new(transport);

    let eth = web3.eth();
    let contract_abi = ethabi::Contract::load(contract_abi).unwrap();
    let contract = web3::contract::Contract::new(eth, contract_address, contract_abi);

    let tx_data = contract
        .abi()
        .function("mintToken")?
        .encode_input(&(token_owner, cid).into_tokens())?;
    let tx_obj = web3::types::TransactionParameters {
        to: Some(contract_address),
        data: web3::types::Bytes(tx_data),
        gas: web3::types::U256([gas_limit, 0, 0, 0]),
        ..Default::default()
    };

    let tx_signed = web3.accounts().sign_transaction(tx_obj, &priv_key).await?;

    let result = web3
        .eth()
        .send_raw_transaction(tx_signed.raw_transaction)
        .await?;

    println!("Mint Tx: {:?}", result);

    Ok(())
}
