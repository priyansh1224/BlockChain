// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title Election
 * @dev A smart contract to simulate a general election process.
 * The Election Commission (deployer) can register parties and voters.
 * Registered voters (18 or older) can cast one vote each.
 */
contract Election {
   
    // ===================================================================
    // STATE VARIABLES
    // ===================================================================

    address public electionCommission;

    struct Party {
        string name;
        uint totalVote;
    }

    struct Voter {
        bool isRegistered;
        uint age;
        bool hasVoted;
    }
   
    Party[] public politicalParties;
    mapping(address => Voter) public voters;
    
    // A mapping to efficiently check if a party name has already been registered.
    // We store a hash of the name to save gas.
    mapping(bytes32 => bool) private partyNameExists;

    // ===================================================================
    // EVENTS
    // ===================================================================

    event PartyRegistered(uint partyId, string name);
    event VoterRegistered(address indexed voterAddress, uint age);
    event VoteCasted(address indexed voterAddress, uint partyId);

    // ===================================================================
    // MODIFIER
    // ===================================================================

    modifier onlyElectionCommission {
        require(electionCommission == msg.sender, "Only the Election Commission can perform this action.");
        _;
    }

    // ===================================================================
    // CONSTRUCTOR
    // ===================================================================

    constructor() {
        electionCommission = msg.sender;
    }
   
    // ===================================================================
    // ADMINISTRATIVE FUNCTIONS (Commission Only)
    // ===================================================================

    /**
     * @dev Registers a new political party.
     * @param _name The name of the new party.
     */
    function registerPoliticalParty(string memory _name) public onlyElectionCommission {
        bytes32 nameHash = keccak256(abi.encodePacked(_name));
        
        // Use the mapping for an efficient O(1) check.
        require(!partyNameExists[nameHash], "Party with this name already exists.");

        partyNameExists[nameHash] = true;
        
        Party memory newParty = Party({
            name: _name,
            totalVote: 0
        });

        politicalParties.push(newParty);
        emit PartyRegistered(politicalParties.length - 1, _name);
    }

    /**
     * @dev Registers a new voter.
     * @param _voter The address of the new voter.
     * @param _age The age of the new voter.
     */
    function registerVoter(address _voter, uint _age) public onlyElectionCommission {
        require(!voters[_voter].isRegistered, "Voter is already registered.");
        require(_age >= 18, "Voter must be 18 or older.");
        require(_voter != address(0), "Cannot register the zero address.");

        voters[_voter].isRegistered = true;
        voters[_voter].age = _age;
        emit VoterRegistered(_voter, _age);
    }

    // ===================================================================
    // VOTING FUNCTION (Voters Only)
    // ===================================================================

    /**
     * @dev Allows a registered voter to cast a single vote for a party.
     * @param _partyIndex The index of the party in the politicalParties array.
     */
    function voting(uint _partyIndex) public {
        require(voters[msg.sender].isRegistered, "You are not a registered voter.");
        require(!voters[msg.sender].hasVoted, "You have already cast your vote.");
        require(_partyIndex < politicalParties.length, "Invalid party index.");

        politicalParties[_partyIndex].totalVote++;
        voters[msg.sender].hasVoted = true;
        emit VoteCasted(msg.sender, _partyIndex);
    }

    // ===================================================================
    // VIEW FUNCTIONS (Public Access)
    // ===================================================================

    /**
     * @dev Returns the name and vote count of the winning party.
     */
    function watchResult() public view returns(string memory winningPartyName, uint winningVoteCount) {
        uint winnerIndex = 0;

        // Find the index of the party with the most votes.
        for(uint i = 0; i < politicalParties.length; i++) {
            if(politicalParties[i].totalVote > politicalParties[winnerIndex].totalVote) {
                winnerIndex = i;
            }
        }

        Party storage winner = politicalParties[winnerIndex];
        return (winner.name, winner.totalVote);
    }

    /**
     * @dev Returns the total number of registered political parties.
     */
    function getNumberOfParties() public view returns (uint) {
        return politicalParties.length;
    }
}