# Feedback

```
>>> swap 30 eth to usdc of 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
Claude: Processing Request...
Sorry there was an error processing your input: CompletionError: ProviderError: {"type":"error","error":{"type":"rate_limit_error","message":"This request would exceed the rate limit for your organization (2bb8eb5f-6dce-4d20-ac3c-ac5f465a83a1) of 80,000 input tokens per minute. For details, refer to: https://docs.anthropic.com/en/api/rate-limits. You can see the response headers for current usage. Please reduce the prompt length or the maximum tokens requested, or try again later. You may also contact sales at https://www.anthropic.com/contact-sales to discuss your options for a rate limit increase."}}
>>>               
```
The Claude agent in the rig-repl is hitting Anthropic's rate limits (80,000 input tokens per minute) during normal usage, causing the agent to fail with `CompletionError: ProviderError: rate_limit_error` messages.

## Root Cause Analysis

The agent is consuming excessive tokens due to several factors that accumulate with each conversation turn:

#### 1. **MCP Tool Schemas** (~3,000-5,000 tokens)
- Complete JSON schemas for all 8 MCP tools are sent with every request
- Each tool includes detailed parameter descriptions and examples
- These schemas are loaded once during initialization but resent with every prompt

#### 2. **Conversation History** (~5,000-15,000 tokens)
- Full conversation history accumulates in the `history: Vec<Message>` vector
- No truncation or summarization implemented
- Each turn adds user input, Claude responses, and tool call results

#### 3. **RAG Context** (~1,000-3,000 tokens per query)
- Relevant documentation chunks are added to every query
- Context includes full source code and documentation content
- No filtering or length limiting implemented

#### 4. **Tool Call Results** (~1,000-5,000 tokens per call)
- Complete tool responses (web search results, API data, etc.) are included
- No filtering of relevant data from tool outputs
- Debug logs and verbose responses add to token count

#### 5. **Debug Logs** (~2,000-5,000 tokens)
- Extensive debug output from MCP server may be included in context
- Connection pooling messages, transaction details, etc.

## Impact

- **Agent becomes unusable** after a few conversation turns
- **User experience degraded** with frequent rate limit errors
- **Development workflow interrupted** when testing agent functionality
- **Cost implications** from inefficient token usage



**Preamble** - Added during Agent Construction
```rust
// In src/rig_agent.rs:84
let agent = agent_builder
    .preamble(&cfg.preamble)  // ← Preamble added here
    .build();
```

**MCP Tool Schemas** - Added during Agent Construction
```rust
// In src/rig_agent.rs:76-80
agent_builder = tools_list
    .tools
    .into_iter()
    .fold(agent_builder, |builder, tool| {
        builder.mcp_tool(tool, mcp_client.clone())  // ← Tool schemas added here
    });
```

**RAG Context** - Added in REPL Loop
```rust
// In src/rig_agent.rs:108-109
// Process through RAG middleware
let query = self.query_rag(&line).await?;  // ← RAG context added here

// In src/rag_middleware.rs:25-32
// Attach relevant information to the query for the Agent to use
Ok(format!("You have access to the following relevant documentation: \n\n{context}\n\n --- \n\nUser: {query}"))
```

**Conversation History** - Added in REPL Loop
```rust
// In src/rig_agent.rs:111-116
match self
    .agent
    .prompt(query)
    .multi_turn(20)
    .with_history(&mut history)  // ← Full conversation history added here
    .await
```

**Tool Call Results** - Added by Rig Framework
The `rig` framework automatically includes tool call results in the conversation history. When tools like `web_search` or `get_quote` return data, the entire response gets added to the `history` vector and sent with the next request.

**Debug Logs** - Potentially Added by Rig Framework
The extensive debug output from the MCP server might be getting included in the context through the rig framework's internal handling of tool responses.
