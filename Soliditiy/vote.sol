pragma solidity ^0.8.0;

contract VotingSystem {
    uint public propositionCount = 0;
    uint public voterCount = 0;

    address private owner;

    mapping(address => Voter) private voters;
    mapping(address => Proposition) private votes;

    mapping(uint => Proposition) public propositions;


    struct Proposition {
        uint _id;
        string _name;
        uint _voteCount;
    }

    struct Voter {
        uint _id;
        address _voter;
    }

    constructor() public {
        owner = msg.sender;
        createVoter(owner);
    }

    modifier isOwner() {
        require(msg.sender == owner);
        _;
    }

    // Make sure the user is allowed to vote
    modifier userAllowedToVote() {
        require(voters[msg.sender]._id != 0);
        _;
    }

    function vote(uint propID) public userAllowedToVote {
        Proposition memory prop = _findProposition(propID);

        // Update Proposition
        prop._voteCount += 1;
        propositions[propID] = prop;

        // Mark user voted for Proposition
        votes[msg.sender] = prop;
    }

    function createVoter(address voterAddress) public isOwner {
        require(voters[voterAddress]._id == 0);
        voterCount += 1;
        voters[voterAddress] = Voter(voterCount, voterAddress);
    }

    // Find Proposition user is voting on
    function _findProposition(uint id) private returns (Proposition memory) {
        Proposition memory proposition = propositions[id];
        require(proposition._id != 0);
        return proposition;
    }

    function createProposition(string memory propName) public isOwner {
        uint _id = propositionCount += 1;
        propositions[_id] = Proposition(_id, propName, 0);
    }
}

