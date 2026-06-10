/**
 * Slate Agent SDK - A fluent JavaScript/Node.js library for AI Agents
 * 
 * This library allows AI agents to easily programmatically generate Slate JSON ASTs 
 * without needing the Rust compiler or dealing with raw JSON structures.
 * 
 * Usage:
 * const builder = new SlateBuilder();
 * builder.title("My Dashboard")
 *        .card("StatsCard", card => {
 *            card.stat("Active Users", { value: 1400, delta: "+10%" })
 *        })
 *        .flowchart("Approval", flow => {
 *            flow.node("Start", "circle").node("End", "circle")
 *            flow.edge("Start", "End", "Approve")
 *        });
 * 
 * const json = builder.toJSON();
 */

class AstNode {
    constructor(tag, id = null) {
        this.tag = tag;
        this.id = id;
        this.text = null;
        this.required = false;
        this.typeAssertion = null;
        this.config = {};
        this.children = [];
    }
}

class RelationshipNode {
    constructor(source, target) {
        this.tag = "relationship";
        this.source = source;
        this.target = target;
        this.config = {};
    }
}

class SlateBuilder {
    constructor() {
        this.nodes = [];
    }

    _add(node) {
        this.nodes.push(node);
        return this;
    }

    // Typography
    title(text) {
        let n = new AstNode("title");
        n.text = text;
        return this._add(n);
    }

    subtitle(text) {
        let n = new AstNode("subtitle");
        n.text = text;
        return this._add(n);
    }

    heading(text) {
        let n = new AstNode("heading");
        n.text = text;
        return this._add(n);
    }

    paragraph(text) {
        let n = new AstNode("text");
        n.text = text;
        return this._add(n);
    }

    divider() {
        return this._add(new AstNode("divider"));
    }

    spacer(height = 20) {
        let n = new AstNode("spacer");
        n.config.height = height;
        return this._add(n);
    }

    // Layout Containers
    section(id, align = "left", buildCallback) {
        let n = new AstNode("section", id);
        n.config.align = align;
        if (buildCallback) {
            let subBuilder = new SlateBuilder();
            buildCallback(subBuilder);
            n.children = subBuilder.nodes;
        }
        return this._add(n);
    }

    grid(id, cols, gap, buildCallback) {
        let n = new AstNode("grid", id);
        n.config.cols = cols;
        n.config.gap = gap;
        if (buildCallback) {
            let subBuilder = new SlateBuilder();
            buildCallback(subBuilder);
            n.children = subBuilder.nodes;
        }
        return this._add(n);
    }

    card(id, buildCallback) {
        let n = new AstNode("card", id);
        if (buildCallback) {
            let subBuilder = new SlateBuilder();
            buildCallback(subBuilder);
            n.children = subBuilder.nodes;
        }
        return this._add(n);
    }

    // Charts & Stats
    stat(id, value, delta) {
        let n = new AstNode("stat", id);
        n.config.value = value;
        if (delta) n.config.delta = delta;
        return this._add(n);
    }

    barChart(id, datasetName) {
        let n = new AstNode("bar-chart", id);
        n.config.data = datasetName;
        return this._add(n);
    }

    lineChart(id, datasetName) {
        let n = new AstNode("line-chart", id);
        n.config.data = datasetName;
        return this._add(n);
    }

    pieChart(id, datasetName) {
        let n = new AstNode("pie-chart", id);
        n.config.data = datasetName;
        return this._add(n);
    }

    donutChart(id, datasetName) {
        let n = new AstNode("donut-chart", id);
        n.config.data = datasetName;
        return this._add(n);
    }

    // Flowcharts
    flowchart(id, buildCallback) {
        let n = new AstNode("flowchart", id);
        if (buildCallback) {
            let subBuilder = new FlowchartBuilder();
            buildCallback(subBuilder);
            n.children = subBuilder.nodes;
        }
        return this._add(n);
    }

    // Forms
    form(id, action, method, buildCallback) {
        let n = new AstNode("form", id);
        n.config.action = action;
        n.config.method = method;
        if (buildCallback) {
            let subBuilder = new FormBuilder();
            buildCallback(subBuilder);
            n.children = subBuilder.nodes;
        }
        return this._add(n);
    }

    toJSON() {
        return JSON.stringify({
            version: "1.0",
            format: "slate-ast",
            nodes: this.nodes
        }, null, 2);
    }
}

class FlowchartBuilder {
    constructor() {
        this.nodes = [];
    }

    node(id, shape = "rect") {
        let n = new AstNode("node", id);
        n.text = id;
        n.config.shape = shape;
        this.nodes.push(n);
        return this;
    }

    edge(source, target, label = null) {
        let rel = new RelationshipNode(source, target);
        if (label) rel.config.label = label;
        this.nodes.push(rel);
        return this;
    }
}

class FormBuilder extends SlateBuilder {
    input(id, placeholder, required = false, typeAssertion = "String") {
        let n = new AstNode("input", id);
        n.config.placeholder = placeholder;
        n.required = required;
        n.typeAssertion = typeAssertion;
        return this._add(n);
    }

    email(id, placeholder, required = false) {
        let n = new AstNode("email", id);
        n.config.placeholder = placeholder;
        n.required = required;
        n.typeAssertion = "Email";
        return this._add(n);
    }

    password(id, placeholder, required = false) {
        let n = new AstNode("password", id);
        n.config.placeholder = placeholder;
        n.required = required;
        n.typeAssertion = "Password";
        return this._add(n);
    }

    select(id, items) {
        let n = new AstNode("select", id);
        n.children = items.map(text => {
            let item = new AstNode("item");
            item.text = text;
            return item;
        });
        return this._add(n);
    }

    submit(id, text, variant = "primary") {
        let n = new AstNode("submit", id);
        n.text = text;
        n.config.variant = variant;
        return this._add(n);
    }

    button(id, text, variant = "secondary") {
        let n = new AstNode("button", id);
        n.text = text;
        n.config.variant = variant;
        return this._add(n);
    }
}

if (typeof module !== 'undefined' && module.exports) {
    module.exports = { SlateBuilder };
} else {
    window.SlateBuilder = SlateBuilder;
}
