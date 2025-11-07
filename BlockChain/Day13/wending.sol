// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract VendingMachine{
    uint soda;
    address owner;

    constructor(){
        soda=100;
        owner=msg.sender;
    }
    function buy() public payable{
        require(msg.value==1 ether, "You should have 1 ether");

        require(soda>0, "out of stock");
        soda=soda-1;
    }
    function fillSoda(uint _soda) public {
        require(msg.sender==owner,"you are not the owner");
        soda=soda+ _soda;
    }

    function withdrawBalance() public{
        require(msg.sender == owner,"you are not the owner");

        payable(owner).transfer(address(this).balance);
    }
}