# Slate Skills and Agent Integrations

This document describes the primary skills and capabilities available to autonomous agents interacting with the Slate ecosystem. The tools defined here allow AI systems to build structural layouts and flowcharts seamlessly without needing to interact with the underlying Rust compiler directly.

## Skill: Agent JSON SDK Generation (`slate-agent-sdk.js`)

Agents working in Node.js or browser environments can bypass the `slate.exe` Rust compiler entirely by using the `slate-agent-sdk.js` library. This is the preferred method for agents needing to construct visual interfaces dynamically.

### Capabilities

- Construct Abstract Syntax Trees (AST) using a fluent class-based API.
- Serialize AST structures directly to the JSON format expected by `slate.js`.
- Manage complex flowchart topologies via edge and node builders.
- Construct comprehensive form configurations with built-in type assertions.

### Implementation Example

Agents can use the `SlateBuilder` class to programmatically generate interfaces:

```javascript
const { SlateBuilder } = require('./slate-agent-sdk.js');

const builder = new SlateBuilder();

// Build a dynamic dashboard layout
builder.title("Agent Overview")
       .subtitle("Dynamically generated via Agent SDK")
       .divider()
       .grid("MainGrid", 2, 24, (grid) => {
           // Left Column: Statistics
           grid.card("StatsPanel", (card) => {
               card.heading("System Load")
                   .stat("CPU", "45%", "-2%")
                   .stat("Memory", "1.2GB", "+100MB");
           });
           
           // Right Column: Flowchart
           grid.card("ProcessMap", (card) => {
               card.heading("Current Process")
                   .flowchart("AgentFlow", (flow) => {
                       flow.node("Init", "circle")
                           .node("Fetch", "rect")
                           .node("Analyze", "diamond")
                           .node("Done", "circle")
                           
                           .edge("Init", "Fetch")
                           .edge("Fetch", "Analyze")
                           .edge("Analyze", "Done", "Pass");
                   });
           });
       });

const jsonOutput = builder.toJSON();
console.log(jsonOutput);
```

### Directives for Usage

1. No HTML Strings: Do not use string concatenation or template literals to generate UI or interfaces. Rely completely on the `SlateBuilder` API.
2. Form Validation: When building forms for data collection via the SDK, always specify the `typeAssertion` correctly (`String`, `Email`, `Password`, `Phone`) to ensure proper rendering logic on the client.
3. Flowchart Ranking: The agent does not need to worry about x/y coordinates when building flowcharts. Just define the `edge()` relationships, and the native `slate.js` browser engine will execute the topological ranking to determine vertical placement.

## Skill: Markdown to Slate Transpilation

Agents reading standard Markdown documents can convert them into Slate scripts (`.slt`) by adjusting blockquote syntax and replacing tables with Grid/Card structures.

1. Convert Markdown lists and paragraphs natively, as Slate supports them out-of-the-box.
2. Convert Markdown tables into `::: grid` components with nested `::: card` wrappers for individual data blocks to adhere to the flat visual system.
3. Replace fenced code blocks (` ``` `) detailing process logic with `::: flowchart` configurations for a better visual representation.
