// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./IoraclePrice.sol";

contract vendingMachine{
   
   uint soda;
   address public owner;
   IoraclePrice public priceOracle;


   constructor(address _addressOracle){
        soda = 100;
        owner = msg.sender;
        priceOracle = IoraclePrice(_addressOracle);
   }

   function getBalance() public view returns(uint){
     
     return address(this).balance;
   }

   modifier onlyOwner(){
    require(msg.sender == owner, "You are not allowed");
    _;
   }

  function buySoda() public payable {
    uint _price = priceOracle.getPrice();
    require(msg.value == _price , "You must pay 0.01 ether");
    require(soda > 0, "Sorry, out of stock!");
    
    soda = soda - 1;
}

   function addStock(uint _soda) public onlyOwner{
    soda = soda + _soda;
   }
   
   function withdraw() public onlyOwner{
    
    payable(msg.sender).transfer(address(this).balance);
   }

}

