# Slate: Universal Visual Scripting Language and Rendering Engine

Slate is a lightweight, brace-free scripting language designed for creating structural layouts, data visualizations, process flowcharts, and interactive forms. It compiles `.slt` scripts into a clean, human-readable Abstract Syntax Tree (AST) in JSON format. 

This JSON output is then natively parsed and rendered directly in the browser using the standalone JavaScript engine `slate.js`, avoiding the need for complex HTML templates, SVGs, or heavy front-end frameworks.

## Core Architecture and Philosophy

Slate separates the content definition from the visual rendering entirely:

1. **The Language and Compiler**: The `slate` CLI (written in Rust) acts solely as a parser and compiler. It takes the `.slt` text file, tokenizes it using a zero-regex, character-level lexer, builds an AST using a recursive descent parser, and serializes that AST to a standard JSON format. 
2. **The Rendering Engine**: The visual presentation is handled exclusively by `slate.js`, a standalone, zero-dependency browser script. It consumes the JSON AST and programmatically constructs DOM elements, applying a consistent, premium, flat grayscale design system.

This architecture ensures that the Slate compiler remains purely functional and decoupled from front-end layout constraints, while the browser engine handles responsive grids, chart plotting, and flowchart topologies natively.

## Language Syntax and Grammar

Slate relies on visual keywords, indentation-agnostic blocks, and standard Markdown conventions. The language requires no brackets or complex closures.

### Typography and Text

Markdown-style headers and plain text are supported. Unordered lists use dashes or asterisks. Blockquotes use the greater-than symbol. Task lists are supported with standard checkbox notation.

```slate
# Page Title
## Section Subtitle
### Card Heading

Standard paragraph text describing the current section.

- Bullet item one
- Bullet item two

[x] Completed task
[ ] Pending task

> A callout or blockquote.
```

### Structural Layouts

Layout containers are defined using the `:::` block syntax. They take an element type, an optional identifier, and optional key-value configuration properties.

**Grid Container**
Grids organize children into responsive columns.
```slate
::: grid DashboardGrid cols=2 gap=24
  // children elements
:::
```

**Cards**
Cards provide a bordered visual wrapper around content, featuring a flat grayscale aesthetic without shadows.
```slate
::: card SummaryCard
  ### Summary
  // children elements
:::
```

**Structural Utilities**
Dividers and spacers manage vertical rhythm.
```slate
---
spacer 24
```

### Data Visualization

Slate provides native components for data visualization, automatically handling axes, legends, and scaling via the JavaScript engine.

**Statistical KPI Components**
```slate
stat ActiveUsers value=14820 delta=+12.4%
```

**Charts**
Charts reference dataset identifiers. The `slate.js` engine natively renders bar, line, pie, and donut charts.
```slate
bar-chart RevenueByMonth data=monthly_revenue
line-chart GrowthTrend data=user_growth
pie-chart TrafficSources data=default
donut-chart Budget Allocation data=default
```

### Flowcharts and Process Diagrams

Flowcharts are defined as blocks containing nodes and directional relationships. The `slate.js` engine dynamically calculates the topological rank of nodes and renders a vertical, auto-routed SVG diagram directly in the DOM.

```slate
::: flowchart DevOpsPipeline
  Code (rect)
  Build (rect)
  Test (diamond)
  Prod (circle)

  Code -> Build
  Build -> Test
  Test -> Prod: Pass
:::
```
Supported shapes are `rect`, `circle`, and `diamond`.

### Interactive Form Mockups

Form elements allow the rapid prototyping of user interfaces and data collection views.

```slate
::: form RegistrationForm action="/register" method=POST
  *required input FullName placeholder="John Doe" :: String
  *required email UserEmail placeholder="name@domain.com" :: Email
  
  ::: select Department
    item Engineering
    item Sales
  :::
  
  submit Register variant=primary: Create Account
:::
```

## Setup and Usage

The project provides a Rust-based compiler and a JavaScript rendering engine.

### Building the Compiler

To build the `slate` CLI from source:
```bash
cargo build --release
```
The compiled executable will be located in `target/release/slate.exe`.

### Compiling Slate Files

To compile a `.slt` script into its JSON AST representation:
```bash
slate compile example.slt -o example.json
```
If the output flag `-o` is omitted, the compiler defaults to creating a `.json` file with the same base name as the input file.

### Browser Rendering

To render the compiled JSON in a browser, include `slate.js` in your HTML environment and call the render function, passing the JSON AST and a target DOM element.

```html
<!DOCTYPE html>
<html>
<head>
    <script src="slate.js"></script>
</head>
<body>
    <div id="slate-container"></div>
    <script>
        // Assuming astJson is the loaded JSON output from the compiler
        Slate.render(astJson, document.getElementById('slate-container'));
    </script>
</body>
</html>
```

Alternatively, `slate.js` provides a convenience method to fetch and render a JSON file via XHR:
```javascript
Slate.renderFromUrl('example.json', document.getElementById('slate-container'));
```

## Directory Structure

- `src/`: Rust source code for the compiler.
  - `lexer.rs`: Custom zero-regex character tokenizer.
  - `parser.rs`: Recursive descent parser.
  - `compiler.rs`: JSON AST serialization module.
  - `ast.rs`: Abstract Syntax Tree definitions.
  - `main.rs`: CLI entry point.
  - `server.rs`: Live-reload local development server.
- `slate.js`: Native JavaScript DOM rendering engine.
- `demo.slt`, `Firstcode.slt`: Example Slate scripts demonstrating syntax.
- `slate-vscode/`: VS Code extension for syntax highlighting.
