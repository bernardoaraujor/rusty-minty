use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use url::Url;

mod ipfs;
mod mint;

#[derive(Debug, StructOpt)]
#[structopt(name = "rusty-minty", about = "Mint ERC721 NFTs.")]
struct Opt {
    /// Input File Path
    #[structopt(name = "input-file", short = "i", parse(from_os_str))]
    input_file: PathBuf,

    /// Web3 Provider
    #[structopt(name = "web3-provider", short = "w")]
    web3_provider: String,

    /// Minter Account Private Key
    #[structopt(name = "private-key", short = "p")]
    priv_key: String,

    /// Token Owner Address
    #[structopt(name = "token-owner", short = "o")]
    token_owner: String,

    /// Gas limit
    #[structopt(name = "gas-limit", short = "g", default_value = "10000000")]
    gas_limit: u64,

    /// ERC721 Contract Address
    #[structopt(name = "contract-address", short = "c")]
    contract_address: String,

    /// ERC721 Contract ABI JSON Path
    #[structopt(name = "contract-abi", short = "a", parse(from_os_str), default_value = "abi/contracts/Minty.sol/Minty.json")]
    contract_abi: PathBuf,
}

#[tokio::main]
async fn main() {
    // parse and check CLI args
    let input_file_path = Opt::from_args().input_file;
    let input_file = match File::open(input_file_path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let web3_provider = Opt::from_args().web3_provider;
    match Url::parse(&web3_provider) {
        Err(e) => panic!("{}", e),
        _ => {}
    }

    let priv_key = Opt::from_args().priv_key;
    let priv_key = match secp256k1::SecretKey::from_str(&priv_key) {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    let token_owner = Opt::from_args().token_owner;
    let token_owner = match web3::types::Address::from_str(&token_owner) {
        Ok(o) => o,
        Err(e) => panic!("{}", e),
    };

    let gas_limit = Opt::from_args().gas_limit;
    if gas_limit > 30_000_000 {
        panic!("Max gas limit: 30000000");
    }

    let contract_address = Opt::from_args().contract_address;
    let contract_address = match web3::types::Address::from_str(&contract_address) {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    };

    let contract_abi = Opt::from_args().contract_abi;
    let contract_abi = match File::open(contract_abi) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    // upload file to IPFS
    let cid = ipfs::add_file(input_file).await;

    // mint NFT
    if let Err(e) = mint::mint(
        &web3_provider,
        priv_key,
        token_owner,
        gas_limit,
        contract_address,
        contract_abi,
        cid,
    )
    .await
    {
        panic!("{}", e);
    }
}
