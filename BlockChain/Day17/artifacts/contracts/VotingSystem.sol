// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Voting{

    struct Party{
        string name;
        uint vote;
    }

    struct Voter{
        bool isRegistered;
        uint age;
        bool hasVoted;
    }

    address public ElectionCommission;
    Party[] public politicalParty;

    mapping(address=>Voter) public Voters;

    constructor (){
        ElectionCommission = msg.sender;
    }

    modifier onlyOwner(){
        require(msg.sender==ElectionCommission,"Only Election Commission can call this function");
        _;
    }


    function registerParty(string memory _partyName) public onlyOwner{
    
        Party memory newParty = Party({
            name:_partyName,
            vote:0
        });
        
        politicalParty.push(newParty);
    }

    function registerVoter(address _voterAddress, uint _age) public onlyOwner{
     
     require(_age>=18,"You are not eligble for vote");
     require(!Voters[_voterAddress].isRegistered,"You are already registered");

     Voters[_voterAddress].isRegistered = true;
     Voters[_voterAddress].age = _age;

    }

    function voting(uint _partyIndex) public {
      
      require(Voters[msg.sender].isRegistered,"You are not registered");
      require(!Voters[msg.sender].hasVoted,"You have already voted");

      require(_partyIndex>=0 && _partyIndex<politicalParty.length,"Invalid Party Index");

      Voters[msg.sender].hasVoted = true;
      politicalParty[_partyIndex].vote++;

    }


    function getWinner() public view returns(string memory partyName){
     
     uint winnerVoteCount = 0;
     uint winnerIndex = 0;

     for(uint i=0;i<politicalParty.length;i++){
        if(politicalParty[i].vote>winnerVoteCount){
            winnerVoteCount = politicalParty[i].vote;
            winnerIndex = i;
        }
        }

        Party storage winner = politicalParty[winnerIndex];
        return winner.name;
        
    }

}