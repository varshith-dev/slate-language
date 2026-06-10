use crate::ast::{AstNode, AstNodeKind, Value};

pub struct Compiler {
    #[allow(dead_code)]
    live_reload: bool,
}

impl Compiler {
    pub fn new(live_reload: bool) -> Self {
        Self { live_reload }
    }

    pub fn compile(&self, nodes: &[AstNode]) -> String {
        let mut items = Vec::new();
        for node in nodes {
            items.push(self.render_node_to_json(node));
        }
        format!("{{\n  \"version\": \"1.0\",\n  \"format\": \"slate-ast\",\n  \"nodes\": [\n{}\n  ]\n}}", items.join(",\n"))
    }

    fn render_node_to_json(&self, node: &AstNode) -> String {
        match &node.kind {
            AstNodeKind::Element {
                tag,
                id,
                config,
                children,
                raw_text,
                is_required,
                type_assertion,
                ..
            } => {
                let mut props = Vec::new();

                props.push(format!("      \"tag\": \"{}\"", escape_json(tag)));

                if let Some(id_val) = id {
                    props.push(format!("      \"id\": \"{}\"", escape_json(id_val)));
                }

                if let Some(text) = raw_text {
                    props.push(format!("      \"text\": \"{}\"", escape_json(text)));
                }

                if *is_required {
                    props.push("      \"required\": true".to_string());
                }

                if let Some(ta) = type_assertion {
                    props.push(format!("      \"typeAssertion\": \"{}\"", escape_json(ta)));
                }

                // Serialize config properties
                if !config.properties.is_empty() {
                    let mut config_items = Vec::new();
                    for (key, val) in &config.properties {
                        let val_str = match val {
                            Value::String(s) => format!("\"{}\"", escape_json(s)),
                            Value::Identifier(s) => format!("\"{}\"", escape_json(s)),
                            Value::Number(n) => format!("{}", n),
                            Value::Boolean(b) => format!("{}", b),
                            _ => "null".to_string(),
                        };
                        config_items.push(format!("          \"{}\": {}", escape_json(key), val_str));
                    }
                    props.push(format!("      \"config\": {{\n{}\n      }}", config_items.join(",\n")));
                }

                // Serialize children recursively
                if !children.is_empty() {
                    let mut child_items = Vec::new();
                    for child in children {
                        child_items.push(self.render_node_to_json(child));
                    }
                    props.push(format!("      \"children\": [\n{}\n      ]", child_items.join(",\n")));
                }

                format!("    {{\n{}\n    }}", props.join(",\n"))
            }
            AstNodeKind::Relationship { source, target, config } => {
                let mut props = Vec::new();
                props.push("      \"tag\": \"relationship\"".to_string());
                props.push(format!("      \"source\": \"{}\"", escape_json(source)));
                props.push(format!("      \"target\": \"{}\"", escape_json(target)));

                if !config.properties.is_empty() {
                    let mut config_items = Vec::new();
                    for (key, val) in &config.properties {
                        let val_str = match val {
                            Value::String(s) => format!("\"{}\"", escape_json(s)),
                            Value::Identifier(s) => format!("\"{}\"", escape_json(s)),
                            Value::Number(n) => format!("{}", n),
                            Value::Boolean(b) => format!("{}", b),
                            _ => "null".to_string(),
                        };
                        config_items.push(format!("          \"{}\": {}", escape_json(key), val_str));
                    }
                    props.push(format!("      \"config\": {{\n{}\n      }}", config_items.join(",\n")));
                }

                format!("    {{\n{}\n    }}", props.join(",\n"))
            }
            AstNodeKind::Comment(text) => {
                format!("    {{\n      \"tag\": \"comment\",\n      \"text\": \"{}\"\n    }}", escape_json(text))
            }
            AstNodeKind::Section { name, children } => {
                let mut props = Vec::new();
                props.push("      \"tag\": \"section\"".to_string());
                props.push(format!("      \"id\": \"{}\"", escape_json(name)));
                if !children.is_empty() {
                    let mut child_items = Vec::new();
                    for child in children {
                        child_items.push(self.render_node_to_json(child));
                    }
                    props.push(format!("      \"children\": [\n{}\n      ]", child_items.join(",\n")));
                }
                format!("    {{\n{}\n    }}", props.join(",\n"))
            }
        }
    }
}

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out
}
