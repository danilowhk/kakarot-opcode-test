import ethers, { ContractFactory, providers, Wallet } from 'ethers'
import fetch from 'node-fetch';
import nock from 'nock';

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
];
const bytecode = "608060405234801561000f575f80fd5b506101438061001d5f395ff3fe608060405234801561000f575f80fd5b5060043610610034575f3560e01c80632e64cec1146100385780636057361d14610056575b5f80fd5b610040610072565b60405161004d919061009b565b60405180910390f35b610070600480360381019061006b91906100e2565b61007a565b005b5f8054905090565b805f8190555050565b5f819050919050565b61009581610083565b82525050565b5f6020820190506100ae5f83018461008c565b92915050565b5f80fd5b6100c181610083565b81146100cb575f80fd5b50565b5f813590506100dc816100b8565b92915050565b5f602082840312156100f7576100f66100b4565b5b5f610104848285016100ce565b9150509291505056fea2646970667358221220b5c3075f2f2034d039a227fac6dd314b052ffb2b3da52c7b6f5bc374d528ed3664736f6c63430008140033";
const privateKey = '';
const rpcUrl = 'http://0.0.0.0:3030';

function hashSignedTransaction(signedTransaction) {
    const hashedTx = ethers.utils.keccak256(signedTransaction);
    console.log('Hashed transaction: ', hashedTx);
    return hashedTx;
  }


const provider = new providers.JsonRpcProvider(rpcUrl);

// Event listener for logging requests
provider.on("pending", (tx) => {
    provider.getTransaction(tx).then((transaction) => {
      console.log('RPC request', transaction);
    });
  });

const signer = new Wallet(privateKey, provider)
//const signer = provider.getSigner(address)

const contract = new ContractFactory(abi, bytecode, signer)



async function main() {

    nock.recorder.rec({
        logging: function(content) {
          console.log('HTTP request', content);
        }
      });
    
    //   const tx = await contract.deploy();
    //   console.log('tx', tx);
    
      nock.recorder.play();

    const hash = "0x02f901c2844b4b5254118459682f008459682f0288016345785d8a00008080b90160608060405234801561000f575f80fd5b506101438061001d5f395ff3fe608060405234801561000f575f80fd5b5060043610610034575f3560e01c80632e64cec1146100385780636057361d14610056575b5f80fd5b610040610072565b60405161004d919061009b565b60405180910390f35b610070600480360381019061006b91906100e2565b61007a565b005b5f8054905090565b805f8190555050565b5f819050919050565b61009581610083565b82525050565b5f6020820190506100ae5f83018461008c565b92915050565b5f80fd5b6100c181610083565b81146100cb575f80fd5b50565b5f813590506100dc816100b8565b92915050565b5f602082840312156100f7576100f66100b4565b5b5f610104848285016100ce565b9150509291505056fea2646970667358221220b5c3075f2f2034d039a227fac6dd314b052ffb2b3da52c7b6f5bc374d528ed3664736f6c63430008140033c001a0e048660b5ff46fa85ae2df500de40dd0bfeb77d39f18aa94a58be3ad3522f4bca014b0c48bbc34a6443b61c6f607d8ab3c7a2f1364e2cad6e60e1c336c1fbbd56a";
    hashSignedTransaction(hash);

}


// main().catch(console.error);
main();