#!/bin/bash

# Set up environment variables for rig-repl

# MCP Server configuration
export MCP_SERVER_ADDRESS="0.0.0.0"
export MCP_SERVER_PORT="4000"

# API Keys

# OpenAI API Key - you'll need to set this
if [ -z "$OPENAI_API_KEY" ]; then
    echo "Warning: OPENAI_API_KEY not set. Please set it before running the application."
    echo "You can set it with: export OPENAI_API_KEY='your-openai-api-key'"
fi

# ETH RPC URL
export ETH_RPC_URL="http://localhost:8545"

# RIG Preamble
export RIG_PREAMBLE="You are a helpful AI assistant with access to Uniswap documentation and source code. You can help users understand Uniswap protocols, answer questions about smart contracts, and provide guidance on DeFi development."

# Uniswap directories - create placeholder directories if they don't exist
export UNISWAP_DOCS_DIR_V2="./docs/v2"
export UNISWAP_DOCS_DIR_V3="./docs/v3"
export UNISWAP_SOURCE_DIR_V2="./source/v2"
export UNISWAP_SOURCE_DIR_V3="./source/v3"

# Create directories if they don't exist
mkdir -p "$UNISWAP_DOCS_DIR_V2"
mkdir -p "$UNISWAP_DOCS_DIR_V3"
mkdir -p "$UNISWAP_SOURCE_DIR_V2"
mkdir -p "$UNISWAP_SOURCE_DIR_V3"

echo "Environment variables set up!"
echo "MCP_SERVER_ADDRESS: $MCP_SERVER_ADDRESS"
echo "MCP_SERVER_PORT: $MCP_SERVER_PORT"
echo "ANTHROPIC_API_KEY: ${ANTHROPIC_API_KEY:0:20}..."
echo "OPENAI_API_KEY: ${OPENAI_API_KEY:0:20}..."
echo "RIG_PREAMBLE: $RIG_PREAMBLE"
echo "Uniswap directories created:"
echo "  - $UNISWAP_DOCS_DIR_V2"
echo "  - $UNISWAP_DOCS_DIR_V3"
echo "  - $UNISWAP_SOURCE_DIR_V2"
echo "  - $UNISWAP_SOURCE_DIR_V3" 