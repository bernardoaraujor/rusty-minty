use ipfs_api::IpfsClient;
use std::fs::File;

pub async fn add_file(input_file: File) -> String {
    let client = IpfsClient::default();

    let cid = match client.add(input_file).await {
        Ok(res) => res.hash,
        Err(e) => panic!("error adding file to IPFS: {}", e),
    };
    println!("IPFS CID: {}", cid);
    cid
}
