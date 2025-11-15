// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract cryptoCurrency {
    
    address private owner;
    uint256 private _totalSupply;
    mapping(address => uint256) private balances;
    mapping(address => mapping(address => uint256)) private allowed;
    
    event Transfer(address indexed _from, address indexed _to, uint256 _value);
    event Approval(address indexed _owner, address indexed _spender, uint256 _value);
    
    string private _name;
    string private _symbol;
    uint8 private _decimal;

    // Add missing modifier
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    constructor() {
        _name = "DEVIL";
        _symbol = "D";
        _decimal = 10;
        owner = msg.sender;
        _totalSupply = 1000000 * (10 ** uint256(_decimal)); 
        balances[owner] = _totalSupply;
        emit Transfer(address(0), owner, _totalSupply);
    }
    
    function name() public view returns (string memory) {
        return _name;
    }

    function symbol() public view returns (string memory) {
        return _symbol;
    }

    function decimals() public view returns (uint8) {
        return _decimal;
    }

    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    function balanceOf(address _owner) public view returns (uint256 balance) {
        return balances[_owner];
    }

    function mint(uint amount) public onlyOwner {
        balances[owner] += amount;
        emit Transfer(address(0), owner, amount);  // Fixed event
        _totalSupply += amount;
    }

    function burn(uint amount) public onlyOwner {
        require(balances[owner] >= amount, "Insufficient balance to burn");
        balances[owner] -= amount;  // Fixed: using balances mapping
        emit Transfer(owner, address(0), amount);  // Fixed address
        _totalSupply -= amount;
    }

    function transfer(address _to, uint256 _value) public returns (bool success) {
        require(_to != address(0), "ERC20: transfer to the zero address");
        require(balances[msg.sender] >= _value, "Insufficient balance");
        balances[msg.sender] -= _value;
        balances[_to] += _value;
        emit Transfer(msg.sender, _to, _value);
        return true;
    }

    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        require(balances[_from] >= _value, "Insufficient balance");
        require(allowed[_from][msg.sender] >= _value, "Allowance exceeded");
        balances[_from] -= _value;
        balances[_to] += _value;
        allowed[_from][msg.sender] -= _value; 
        emit Transfer(_from, _to, _value);
        return true;
    }

    function approve(address _spender, uint256 _value) public returns (bool success) {
        require(_spender != address(0), "ERC20: approve to the zero address");
        require(balances[msg.sender] >= _value, "Insufficient balance");
        allowed[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }
    
    function allowance(address _owner, address _spender) public view returns (uint256 remaining) {
        return allowed[_owner][_spender];
    }
}