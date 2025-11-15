import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const DevilCoinModule = buildModule("DevilCoinModule", (m) => {
  // The constructor for RohitCoin needs an 'initialOwner' address.
  // m.getAccount(0) gets the address of the wallet whose private key
  // is configured in hardhat.config.js (that's you!).
  const initialOwner = m.getAccount(0);

  // We deploy the contract and pass 'initialOwner' as the constructor argument.
  const DevilCoin = m.contract("DevilCoin", [initialOwner]);

  return { DevilCoin };
});

export default DevilCoinModule;