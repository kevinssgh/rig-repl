# Uniswap V3 Documentation

## Overview
Uniswap V3 is the latest version of the Uniswap protocol, introducing concentrated liquidity and multiple fee tiers.

## Key Features
- Concentrated Liquidity
- Multiple Fee Tiers (0.05%, 0.3%, 1%)
- Non-fungible Liquidity Positions
- Advanced Oracle
- Improved Capital Efficiency

## Core Contracts

### UniswapV3Factory
Creates and manages Uniswap V3 pools with different fee tiers.

### UniswapV3Pool
Individual pool contracts that manage concentrated liquidity positions.

### UniswapV3Router
Provides a user-friendly interface for swapping tokens and managing concentrated liquidity positions.

### NonfungiblePositionManager
Manages NFT representations of concentrated liquidity positions.

## Concentrated Liquidity
Liquidity providers can concentrate their capital within custom price ranges, providing deeper liquidity and higher fees.

## Fee Tiers
- 0.05%: For stable pairs like USDC/USDT
- 0.3%: Standard fee tier for most pairs
- 1%: For exotic pairs with higher volatility 