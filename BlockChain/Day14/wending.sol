// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./IoraclePrice.sol";

contract sodaVendor is IoraclePrice{
    address public owner;
    uint public  price;


    modifier onlyOwner{
        require(msg.sender==owner,"You are not the owner");
        _;
    }

    constructor(){
        owner = msg.sender;
        price = 1 ether;
    }

    function getPrice() external view returns(uint) {
        return price;
    }

    function setPrice(uint _price) public onlyOwner {
        price = _price;
    }
}