import '@nomiclabs/hardhat-waffle';
import '@nomiclabs/hardhat-solpp';
import '@nomiclabs/hardhat-etherscan';
import 'hardhat-typechain';
import 'hardhat-contract-sizer';

const prodConfig = {
    // UPGRADE_NOTICE_PERIOD: 0,
    MAX_AMOUNT_OF_REGISTERED_TOKENS: 511,
    // PRIORITY_EXPIRATION: 101,
    DUMMY_VERIFIER: false
};
const testnetConfig = {
    UPGRADE_NOTICE_PERIOD: 0,
    MAX_AMOUNT_OF_REGISTERED_TOKENS: 511,
    // PRIORITY_EXPIRATION: 101,
    DUMMY_VERIFIER: false
};
const testConfig = {
    UPGRADE_NOTICE_PERIOD: 0,
    MAX_AMOUNT_OF_REGISTERED_TOKENS: 5,
    PRIORITY_EXPIRATION: 101,
    DUMMY_VERIFIER: true
};

const localConfig = Object.assign({}, prodConfig);
// @ts-ignore
localConfig.UPGRADE_NOTICE_PERIOD = 0;
localConfig.DUMMY_VERIFIER = process.env.CONTRACTS_TEST_DUMMY_VERIFIER === 'true';
// @ts-ignore
localConfig.EASY_EXODUS = process.env.CONTRACTS_TEST_EASY_EXODUS === 'true';

const contractDefs = {
    rinkeby: testnetConfig,
    ropsten: testnetConfig,
    mainnet: prodConfig,
    test: testConfig,
    localhost: localConfig
};

export default {
    solidity: {
        version: '0.7.6',
        settings: {
            optimizer: {
                enabled: true,
                runs: 200
            }
        }
    },
    contractSizer: {
        runOnCompile: false
    },
    paths: {
        sources: './contracts'
    },
    solpp: {
        defs: (() => {
            if (process.env.CONTRACT_TESTS) {
                return contractDefs.test;
            }
            return contractDefs[process.env.CHAIN_ETH_NETWORK];
        })()
    },
    networks: {
        env: {
            url: process.env.ETH_CLIENT_WEB3_URL
        },
        hardhat: {
            allowUnlimitedContractSize: true
        }
    },
    etherscan: {
        apiKey: process.env.MISC_ETHERSCAN_API_KEY
    }
};
