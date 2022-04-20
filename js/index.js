const {
  LCDClient,
  MsgExecuteContract,
  Coin,
} = require("@terra-money/terra.js");

const { user, funder } = require("./key.json");
const address = require("./address.json");
const {getUserWallet, getFunderWallet} = require('./utils')

const terra = new LCDClient({
  URL: "https://bombay-lcd.terra.dev",
  chainID: "bombay-12",
});

const deposit = async(amount) => {
  const wallet = getUserWallet();
  console.log(wallet.key.accAddress)
  let message = new MsgExecuteContract(
    wallet.key.accAddress,
    address.stakingToken,
    {
      send: {
        contract: address.rewardPool,
        amount,
        msg: Buffer.from(JSON.stringify({
          deposit: {}
        })).toString('base64')
      },
    },
  );
  let signedTx = await wallet.createAndSignTx({
    msgs: [message],
  });
  const result = await terra.tx.broadcast(signedTx);
  console.log(`Deposit ${amount}: `, result.txhash);
}

const withdraw = async(amount) => {
  const wallet = getUserWallet();
  let message = new MsgExecuteContract(
    wallet.key.accAddress,
    address.rewardPool,
    {
      withdraw: {
        amount
      }
    },
  );
  let signedTx = await wallet.createAndSignTx({
    msgs: [message],
  });
  const result = await terra.tx.broadcast(signedTx);
  console.log(`Withdraw ${amount}: `, result.txhash);
}

const claim = async() => {
  const wallet = getUserWallet();
  let message = new MsgExecuteContract(
    wallet.key.accAddress,
    address.rewardPool,
    {
      claim: {
      }
    },
  );
  let signedTx = await wallet.createAndSignTx({
    msgs: [message],
  });
  const result = await terra.tx.broadcast(signedTx);
  console.log(`Claim: `, result.txhash);
}

const fund = async(amount) => {
  const wallet = getFunderWallet();
  let message = new MsgExecuteContract(
    wallet.key.accAddress,
    address.rewardPool,
    {
      fund: {
      }
    },
    [new Coin(address.rewardDenom, amount)]
  );
  let signedTx = await wallet.createAndSignTx({
    msgs: [message],
  });
  const result = await terra.tx.broadcast(signedTx);
  console.log(`Fund ${amount}: `, result.txhash);
}

const main = async() => {
  await deposit("100000000");
  await fund("100000000");
  await withdraw("50000000");
  await claim()
}

main();
