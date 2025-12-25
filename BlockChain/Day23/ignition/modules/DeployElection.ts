// File: ignition/modules/DeployElection.ts

import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const ElectionModule = buildModule("ElectionModule", (m) => {
  /**
   * The Election contract's constructor takes no arguments.
   * Ignition will automatically use the first account from your `hardhat.config.ts`
   * (the one derived from your PRIVATE_KEY) to send the deployment transaction.
   * The contract's constructor will then set this deployer address as the ElectionCommission.
   */

  // The 'm.contract()' function tells Ignition to deploy a contract.
  // The first argument is the exact name of the contract from your /contracts folder.
  // The second argument is an array of constructor arguments. Since there are
  // none, we provide an empty array [].
  const election = m.contract("Election", []);

  // The module returns an object containing a "future" for the deployed contract.
  // This is a best practice, allowing other scripts or tests to easily access
  // the result of this deployment.
  return { election };
});

export default ElectionModule;