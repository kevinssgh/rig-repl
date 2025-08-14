// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.7.6;
pragma abicoder v2;

import './interfaces/IUniswapV3Factory.sol';
import './UniswapV3Pool.sol';

contract UniswapV3Factory is IUniswapV3Factory {
    PoolDeployer public immutable override poolDeployer;
    mapping(address => mapping(address => mapping(uint24 => address))) public override getPool;
    address[] public override allPools;

    constructor(PoolDeployer _poolDeployer) {
        poolDeployer = _poolDeployer;
    }

    function allPoolsLength() external view override returns (uint) {
        return allPools.length;
    }

    function createPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) external override returns (address pool) {
        require(tokenA != tokenB);
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        require(token0 != address(0));
        int24 tickSpacing = getTickSpacing(fee);
        require(tickSpacing != 0);
        require(getPool[token0][token1][fee] == address(0));
        
        pool = poolDeployer.deploy(address(this), token0, token1, fee, tickSpacing);
        getPool[token0][token1][fee] = pool;
        getPool[token1][token0][fee] = pool;
        allPools.push(pool);

        emit PoolCreated(token0, token1, fee, tickSpacing, pool);
    }

    function getTickSpacing(uint24 fee) public pure override returns (int24) {
        if (fee == 500) return 10;
        if (fee == 3000) return 60;
        if (fee == 10000) return 200;
        revert('Invalid fee');
    }
} 