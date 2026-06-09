# Slate: Universal Visual Scripting Language & SVG Compiler

Slate is a flat, brace-free, Markdown-inspired scripting language designed for creating premium visual layouts, charts, forms, and diagrams. The compiler translates Slate scripts (`.slt`) directly into self-contained vector graphic (`.svg`) files. 

Combined with its dedicated **VS Code Extension**, Slate provides syntax coloring and a real-time, side-by-side visual editor preview without requiring an external web browser.

---

## 🎨 Design Philosophy & Visual Spec

Slate is built to produce visually stunning, professional outputs. The rendering engine adheres to a **premium, gradient-free light theme**:

- **Color Palette**: Solid HSL/Hex tailored colors. No gradients, shadows are replaced by clean `#E2E8F0` borders.
- **Background**: Soft off-white base `#F8FAFC`.
- **Card Elements**: Pure white background `#FFFFFF`, thin `#E2E8F0` border, `rx="12"` rounded corners.
- **Typography**: Premium font stacks using **Outfit** (for headings) and **Inter** (for UI/text labels).
- **Controls & Accents**: Solid Indigo `#4F46E5` for primary buttons, Teal `#0D9488` for success/process actions, Pink `#DB2777` for decision markers, and Soft Red `#FEF2F2` / Crimson `#DC2626` for warning elements.

---

## 🚀 Key Features

1. **Direct SVG Compilation**: Slate compiles visual elements (grids, columns, cards, stat panels, charts, flowcharts, and interactive inputs) into a single, self-contained, optimized vector graphic (`.svg`) file.
2. **Dynamic Flow Layout**: A built-in vertical flow layout solver dynamically calculates coordinate offsets, column distributions, card sizing, and page height.
3. **Built-in Visualizations**:
   - **Charts**: Embed bar, line, pie, and donut charts mapped directly to data models.
   - **Flowcharts**: Auto-ranked nodes (circles, diamonds, rounded rectangles) linked by smooth cubic bezier arrows.
4. **Interactive Mockups**: Complete form elements, including required text/email/password boxes with type-tags, select dropdowns, and buttons.
5. **No Regex Policy**: The custom lexer, recursive-descent parser, and SVG layout compiler are implemented in pure, highly-maintainable Rust with **zero regular expressions**.
6. **VS Code Extension**: Provides syntax coloring and a side-by-side, auto-updating webview preview inside the editor.

---

## 📁 Directory Structure

```
├── src/
│   ├── ast.rs          # Abstract Syntax Tree representation
│   ├── lexer.rs        # Hand-written character-level tokenizer (Zero-Regex)
│   ├── parser.rs       # Recursive descent parser (Zero-Regex)
│   ├── compiler.rs     # Vertical flow layout SVG compilation engine
│   ├── main.rs         # Command line interface commands (compile, watch, help)
│   ├── lib.rs          # Project library interface
│   └── bin/
│       └── setup.rs    # CLI installer executable source (compiles to slate-init.exe)
├── slate-vscode/       # VS Code extension source directory
│   ├── package.json    # Extension manifest and commands registration
│   ├── language-configuration.json # Bracket closing & comment character config
│   ├── src/
│   │   └── extension.js # Webview preview panel and auto compile-on-save watcher
│   └── syntaxes/
│       └── slate.tmLanguage.json # TextMate grammar for editor syntax coloring
├── slate-setup.cs      # WinForms graphic installer code (gradient-free light GUI)
├── install_extension.ps1 # Local installation script for the VS Code extension
├── Cargo.toml          # Rust package manager configuration
└── demo.slt            # Visual script demonstration source file
```

---

## 🛠️ Installation & Setup

You can install the Slate Compiler and VS Code Extension using the automated script or compiled installers.

### Method 1: Local Setup Script (Recommended)
Compile the compiler and install the VS Code Extension in a single command using PowerShell:

1. Build the release binaries:
   ```powershell
   cargo build --release
   ```
2. Run the local extension installation script:
   ```powershell
   powershell -ExecutionPolicy Bypass -File .\install_extension.ps1
   ```
3. Restart or reload VS Code (`Ctrl+R` or search `Developer: Reload Window` in command palette).

### Method 2: GUI Installer (`slate-setup.exe`)
A standalone C# WinForms graphic installer utility is provided in `slate-setup.exe`. Running this installer will:
- Copy the built `slate.exe` compiler to your home directory (`~/.slate/bin/slate.exe`).
- Copy the logo to `~/.slate/logo.png`.
- Configure your local User `PATH` environment variable.
- Broadcast changes globally so the `slate` CLI command is immediately active in terminal sessions.

---

## 💻 CLI Usage

Once installed, use the `slate` CLI tool to compile and watch Slate files:

- **Compile to SVG**:
  ```bash
  slate compile demo.slt -o demo.svg
  ```
  *(If `-o` is omitted, the compiler outputs to `<filename>.svg` by default)*

- **Interactive Live Server (Deprecated)**:
  ```bash
  slate watch demo.slt --port 8080
  ```

---

## 🔌 VS Code Extension & Live Previewer

The Slate extension enables a seamless design feedback loop inside VS Code:

- **Syntax Highlighting**: Elements like blocks (`:::`), directives (`stat`, `bar-chart`, `flowchart`), strings, numbers, flow arrows (`->`), comments (`//`), and headers (`#`, `##`, `###`) are highlighted using a custom TextMate grammar.
- **Live Visual Preview**:
  1. Open a Slate script file (`.slt`).
  2. Click the **Show Visual Preview** icon on the top-right editor tab toolbar, or open the command palette (`Ctrl+Shift+P`) and select `Slate: Show Visual Preview`.
  3. A side-by-side Webview panel will open, displaying the vector SVG graphic.
  4. Edit the `.slt` script and save (`Ctrl+S`). The extension automatically triggers the compiler and updates the preview instantly!

---

## 📝 Syntax Reference & Grammar Guidelines

Slate is written using visual keywords, blocks, and Markdown headers. Comments can be written on any line using `//`.

### 1. Typography
Markdown-style headers and raw text:
```slate
# Main Title (compiled to Slate Title in Outfitters, 32px Bold)
## Subtitle description text (compiled to Slate Subtitle in Inter, 16px)
### Element Heading label (compiled to Slate Card Heading in Outfit, 18px Bold)
Standard descriptive body text (compiled to Slate Text in Inter, 14px)
```

### 2. Layouts
All layout elements are blocks starting with `::: <type> <id> [config]` and closed with `:::`.

* **Grid Container**: Groups elements into columns (e.g. 2 columns) with custom gaps.
  ```slate
  ::: grid MyGrid cols=2 gap=24
    // Children cards go here
  :::
  ```
* **Cards**: Visual block wrappers with rounded corners and light borders.
  ```slate
  ::: card MetricsCard
    ### Daily Metrics
    // Card body content
  :::
  ```
* **Dividers & Spacers**:
  ```slate
  ---          // Horizontal divider line
  spacer 24    // Vertical empty space of 24 pixels
  ```

### 3. Visualizations
* **Stats Card**: Renders KPI labels, primary values, and comparative indicators.
  ```slate
  stat ActiveUsers value=14820 delta=+12.4%
  stat TotalRevenue value=$94k delta=+8.7%
  ```
* **Charts**: Render interactive, clean bar charts, line charts, pie charts, and donut charts. Connects to internal mock dataset IDs.
  ```slate
  bar-chart MonthlyRevenue data=monthly_revenue color=#6366F1
  line-chart UserGrowth data=user_growth color=#10B981
  donut-chart UsersShare data=user_types
  ```
* **Flowcharts**: Standard visual diagramming container with custom nodes and connections.
  ```slate
  ::: flowchart LogisticsPipeline
    Start (circle)
    Validate (rect)
    Route (diamond)
    Success (circle)

    Start -> Validate
    Validate -> Route
    Route -> Success: Pass
  :::
  ```
  *Shapes supported: `circle`, `rect`, `diamond`.*

### 4. Interactive Forms
Generate clean input form mockups with validation types.
```slate
::: form ContactForm action="/api/contact" method=POST
  *required email UserEmail placeholder="enter your email" :: Email Address
  ::: select Category
    item General Inquiry
    item Sales Department
    item Technical Support
  :::
  submit SendRequest variant=primary: Submit Form
:::
```

---

## 🔧 Developer Setup

### Rebuilding Binary
If you modify the parser or compiler source code, run the following:
```bash
cargo build --release
```

### Run Parser Tests
```bash
cargo test
```
*(Tests check tokenizer token emissions, custom character scanning, and Markdown-script structural hierarchy parsing).*
