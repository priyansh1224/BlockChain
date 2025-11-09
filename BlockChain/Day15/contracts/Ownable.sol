// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Ownable{

    address public owner;
    
    constructor() {
        owner = msg.sender;
    }

    event OwnerShipTransfer(address indexed oldOwner, address indexed newOwner);

    modifier onlyOwner(){
        require(msg.sender==owner, "You are not a Owner");
        _;
    }

    function transferOwnership(address _newOwner) public virtual onlyOwner{
        
        require(_newOwner != address(0), "New Owner address should not be zero address");
        emit OwnerShipTransfer(owner, _newOwner);
        owner = _newOwner; 
    }

}