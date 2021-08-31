require("@nomiclabs/hardhat-waffle");
require('hardhat-abi-exporter');

const INFURA_API_KEY = "KEY";
const RINKEBY_PRIVATE_KEY = "YOUR RINKEBY PRIVATE KEY"

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
    solidity: "0.7.3",

    defaultNetwork: 'localhost',
    networks: {
        hardhat: {},
        localhost: {},
        rinkeby: {
            url: `https://rinkeby.infura.io/v3/${INFURA_API_KEY}`,
            // accounts: [`0x${RINKEBY_PRIVATE_KEY}`]
        },

    }
};

