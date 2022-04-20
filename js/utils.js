const {
  LCDClient,
  MsgInstantiateContract,
  MsgMigrateContract,
  MsgSend,
  MnemonicKey,
  MsgExecuteContract,
  Coin,
} = require("@terra-money/terra.js");

const { user, funder } = require("./key.json");
const address = require("./address.json");

const terra = new LCDClient({
  URL: "https://bombay-lcd.terra.dev",
  chainID: "bombay-12",
});

const getWalletFromMnemonic = (mnemonic) => {
  return terra.wallet(new MnemonicKey({
    mnemonic,
  }))
}
const getUserWallet = () => {
  return getWalletFromMnemonic(user)
}

const getFunderWallet = () => {
  return getWalletFromMnemonic(funder)
}

module.exports = {
  getUserWallet,
  getFunderWallet
}