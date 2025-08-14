# Uniswap V2 Documentation

## Overview
Uniswap V2 is a decentralized exchange protocol built on Ethereum that enables automated token trading through smart contracts.

## Key Features
- Automated Market Making (AMM)
- ERC-20 token swaps
- Liquidity provision
- Flash swaps

## Core Contracts

### UniswapV2Factory
The factory contract creates and manages trading pairs.

### UniswapV2Pair
Each trading pair is represented by a pair contract that manages the liquidity pool.

### UniswapV2Router02
The router contract provides a user-friendly interface for swapping tokens and managing liquidity.

## Trading
Users can swap any ERC-20 token for another by calling the swap functions on the router contract.

## Liquidity Provision
Liquidity providers can add tokens to pools and earn trading fees proportional to their share of the pool. 