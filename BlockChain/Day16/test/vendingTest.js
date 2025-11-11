const {expect} = require("chai");
const VendingModule = require("../ignition/modules/Deploy");
const {ethers,ignition }=require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-toolbox/network-helpers");
const { parseEther } = require("ethers");

describe("Testing our Vending Machine", function(){

    // deploy the contract
    async function vendingMachineDeploy(){
     
    // Ether chaiye hoga
    // total 20 account return karke dega jo free honge (array)
    const [owner,buyer]  = await ethers.getSigners();
    const {vendingMachine,sodaVendor} = await ignition.deploy(VendingModule);
      
      return {owner,vendingMachine,sodaVendor,buyer};
    }


    it("Should Correctly set the Deployer as owner", async function(){


       const {owner,vendingMachine} = await loadFixture(vendingMachineDeploy);

    //    const ownerOfVendingMachine = await vendingMachine.owner();

        expect(await vendingMachine.owner()).to.equal(owner.address);
    })


    it("Should Revert if some Payment is failed", async function () {

        const {buyer,vendingMachine,sodaVendor} = await loadFixture(vendingMachineDeploy);
        
        const price = 1;
        // 1 wei

        await expect(vendingMachine.connect(buyer).buySoda({value:price})).to.be.revertedWith("Incorrect Payment amount for Soda");
    })
})


