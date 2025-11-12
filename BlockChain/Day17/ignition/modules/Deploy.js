// File: ignition/modules/Deploy.js

const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");
const { parseEther } = require("ethers");

// We define a module that describes the deployment of our contracts.
module.exports = buildModule("VendingModule", (m) => {
  // --- Deployment of the Price Oracle ---
  // We need to set an initial price when deploying the sodaVendor.
  // Let's set it to 0.01 ether. The `parseEther` function from ethers.js
  // is the standard way to convert a string like "0.01" into wei.
  const initialPrice = parseEther("0.01");

  // The 'm.contract()' function tells Ignition to deploy a contract.
  // First, we deploy our 'sodaVendor' contract.
  // The second argument is an array of constructor arguments.
  const sodaVendor = m.contract("sodaVendor", []);
  
  // Hardhat Ignition is still evolving, and setting the price
  // via a separate call is a common pattern for now.
  // We can add calls to our contracts after deployment.
  m.call(sodaVendor, "setPrice", [initialPrice]);


  // --- Deployment of the Vending Machine ---
  // The vendingMachine's constructor needs the address of the sodaVendor.
  // Ignition is smart. When we pass the 'sodaVendor' contract object
  // as an argument here, Ignition knows it needs to wait for sodaVendor
  // to be deployed first and then pass its address.
  const vendingMachine = m.contract("vendingMachine", [sodaVendor]);

  // The module returns an object with the deployed contracts,
  // so we can easily access them later in tests or other scripts.
  return { sodaVendor, vendingMachine };
});
