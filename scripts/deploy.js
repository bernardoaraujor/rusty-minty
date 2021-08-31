const hre = require("hardhat");

async function main() {
    const Minty = await hre.ethers.getContractFactory("Minty");
    const minty = await Minty.deploy("Minty", "MNT");

    await minty.deployed();

    console.log("Minty deployed to:", minty.address);
}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });
