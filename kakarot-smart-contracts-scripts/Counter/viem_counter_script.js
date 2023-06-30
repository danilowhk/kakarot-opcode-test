import { createPublicClient, createWalletClient, http } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { mainnet } from 'viem/chains';
import fs from 'fs';
import fetch from 'node-fetch';
import nock from 'nock';


globalThis.fetch = logAndFetch;

const kakarot_chain = {
    id: 1263227476,
    network: 'homestead',
    name: 'Ethereum',
    nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
    blockExplorers: {
      etherscan: { name: 'Etherscan', url: 'https://etherscan.io' },
      default: { name: 'Etherscan', url: 'https://etherscan.io' }
    },
    contracts: {
      ensRegistry: { address: '0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e' },
      ensUniversalResolver: {
        address: '0xc0497E381f536Be9ce14B0dD3817cBcAe57d2F62',
        blockCreated: 16966585
      },
      multicall3: {
        address: '0xca11bde05977b3631167028862be2a173976ca11',
        blockCreated: 14353601
      }
    },
    formatters: undefined,
    serializers: undefined
  }

// import { createPublicClient, createWalletClient, http } from 'viem'
// import { mainnet } from 'viem/chains'
// import { wagmiContractConfig } from './abi'


// Your private key and RPC URL
const privateKey = '';
const rpcUrl = 'http://0.0.0.0:3030';


// // Connect to the network
// let provider = new ethers.providers.JsonRpcProvider(rpcUrl);

// const client = viem.createPublicClient({
//   chain: fantom,
//   transport: http(rpcUrl)
// })

// console.log(client)

// ----------------

// We connect to the Contract using a Provider, so we will only
// have read-only access to the Contract
// let wallet = new ethers.Wallet(privateKey, provider);

const account = privateKeyToAccount(privateKey)

console.log(account)

const walletClient = createWalletClient({
    account,
    chain: kakarot_chain,
    transport: http(rpcUrl)
  })

// console.log(walletClient)



// ---------------------

// Contract ABI and bytecode


async function main() {
    //const [address] = await walletClient.getAddresses()

    const abi = [
      {
        "inputs": [],
        "name": "retrieve",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "num",
            "type": "uint256"
          }
        ],
        "name": "store",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      }
    ]
    const bytecode = "608060405234801561000f575f80fd5b506101438061001d5f395ff3fe608060405234801561000f575f80fd5b5060043610610034575f3560e01c80632e64cec1146100385780636057361d14610056575b5f80fd5b610040610072565b60405161004d919061009b565b60405180910390f35b610070600480360381019061006b91906100e2565b61007a565b005b5f8054905090565b805f8190555050565b5f819050919050565b61009581610083565b82525050565b5f6020820190506100ae5f83018461008c565b92915050565b5f80fd5b6100c181610083565b81146100cb575f80fd5b50565b5f813590506100dc816100b8565b92915050565b5f602082840312156100f7576100f66100b4565b5b5f610104848285016100ce565b9150509291505056fea2646970667358221220b5c3075f2f2034d039a227fac6dd314b052ffb2b3da52c7b6f5bc374d528ed3664736f6c63430008140033"

    const deploy_tx = await walletClient.deployContract({
        abi,
        account: account,
        bytecode,
    })
    console.log("transction")
    console.log(deploy_tx)

}

function logAndFetch(url, options) {
  console.log('HTTP request', url, options);
  return fetch(url, options);
}

main().catch(console.error);
// main();