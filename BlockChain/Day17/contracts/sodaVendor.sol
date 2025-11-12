// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./IoraclePrice.sol";
import "./Ownable.sol";

contract sodaVendor is IoraclePrice, Ownable{
    
    uint public  price;


    constructor(){
        price = 1 ether;
    }

    function getPrice() external view returns(uint) {
        return price;
    }

    function setPrice(uint _price) public onlyOwner {
        price = _price;
    }
}