// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// Import the pre-built, audited ERC20 and Ownable contracts from the library.
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

// Our contract inherits all the functionality from ERC20 and Ownable.
contract RohitCoin is ERC20, Ownable {
    
    /**
     * @dev The constructor runs once when the contract is deployed.
     *      It sets the token's name, symbol, and initial owner.
     *      Then it mints the total supply to the initial owner.
     */
    constructor(address initialOwner) 
        ERC20("RohitCoin", "RHT")
        Ownable(initialOwner)
    {
        // Mint 1,000,000 tokens.
        // Because the ERC20 contract from OpenZeppelin uses 18 decimals by default,
        // we multiply by 10**18 to get the correct value in wei.
        _mint(initialOwner, 1_000_000 * (10**18));
    }
}