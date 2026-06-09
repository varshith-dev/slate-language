use crate::lexer::Lexer;
use crate::ast::{AstNode, AstNodeKind, Config, Value, Action};

pub struct Parser {
    lexer: Lexer,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let content = self.lexer.get_input_string();
        let mut nodes = Vec::new();
        let mut container_stack: Vec<AstNode> = Vec::new();

        let mut line_num = 0;
        for raw_line in content.lines() {
            line_num += 1;
            
            // Strip comments
            let clean_line = if let Some(idx) = raw_line.find("//") {
                &raw_line[..idx]
            } else {
                raw_line
            }.trim();

            if clean_line.is_empty() {
                continue;
            }

            // Check if it's a container tag: ::: [tag [id] [config]]
            if clean_line.starts_with(":::") {
                let rest = clean_line[3..].trim();
                if rest.is_empty() {
                    // Close current container
                    if let Some(popped) = container_stack.pop() {
                        if container_stack.is_empty() {
                            nodes.push(popped);
                        } else {
                            let len = container_stack.len();
                            if let AstNodeKind::Element { children, .. } = &mut container_stack[len - 1].kind {
                                children.push(popped);
                            }
                        }
                    } else {
                        self.errors.push(format!(
                            "[Line {}] Found closing container ':::' with no matching open container",
                            line_num
                        ));
                    }
                } else {
                    // Open new container
                    let mut parts = rest.split_whitespace();
                    if let Some(tag) = parts.next() {
                        let mut id = None;
                        let mut config_str = String::new();
                        
                        // Parse ID and config string
                        if let Some(second) = parts.next() {
                            if second.contains('=') || second.contains(':') {
                                config_str.push_str(second);
                                config_str.push(' ');
                            } else {
                                id = Some(second.to_string());
                            }
                        }
                        
                        for part in parts {
                            config_str.push_str(part);
                            config_str.push(' ');
                        }

                        let config = parse_properties(&config_str);
                        let container_node = AstNode::new(
                            AstNodeKind::Element {
                                tag: tag.to_string(),
                                id,
                                config,
                                children: Vec::new(),
                                raw_text: None,
                                actions: Vec::new(),
                                ai_operations: Vec::new(),
                                type_assertion: None,
                                is_required: false,
                            },
                            line_num,
                            1,
                        );
                        container_stack.push(container_node);
                    }
                }
                continue;
            }

            // Check if we are inside flowchart container
            let mut inside_flowchart = false;
            if let Some(last) = container_stack.last() {
                if let AstNodeKind::Element { tag, .. } = &last.kind {
                    if tag == "flowchart" || tag == "flow" {
                        inside_flowchart = true;
                    }
                }
            }

            if inside_flowchart {
                // Parse flowchart lines
                if clean_line.contains("->") {
                    // Relationship
                    let parts: Vec<&str> = clean_line.split("->").collect();
                    if parts.len() == 2 {
                        let source = parts[0].trim().to_string();
                        let right_side = parts[1].trim();
                        
                        let mut target = right_side.to_string();
                        let mut config = Config::new();
                        
                        // Look for label or properties
                        if let Some(colon_idx) = right_side.find(':') {
                            target = right_side[..colon_idx].trim().to_string();
                            let label = right_side[colon_idx + 1..].trim().to_string();
                            config.properties.push(("label".to_string(), Value::String(label)));
                        } else if let Some(paren_idx) = right_side.find('(') {
                            target = right_side[..paren_idx].trim().to_string();
                            let properties_str = right_side[paren_idx..].trim();
                            // parse parentheses properties
                            let clean_props = properties_str.trim_start_matches('(').trim_end_matches(')').trim();
                            let clean_props_normalized = clean_props.replace(":", "=");
                            config = parse_properties(&clean_props_normalized);
                        }
                        
                        let rel_node = AstNode::new(
                            AstNodeKind::Relationship { source, target, config },
                            line_num,
                            1,
                        );
                        
                        let len = container_stack.len();
                        if let AstNodeKind::Element { children, .. } = &mut container_stack[len - 1].kind {
                            children.push(rel_node);
                        }
                    }
                } else {
                    // Node definition: node_id (shape) or node_id
                    let mut node_id = clean_line.to_string();
                    let mut shape = "rect".to_string();
                    let mut config = Config::new();
                    
                    let mut node_def = clean_line;
                    if node_def.starts_with("node ") {
                        node_def = node_def[5..].trim();
                    }

                    if let Some(paren_idx) = node_def.find('(') {
                        node_id = node_def[..paren_idx].trim().to_string();
                        let inner = node_def[paren_idx + 1..].trim();
                        if let Some(end_paren) = inner.find(')') {
                            shape = inner[..end_paren].trim().to_string();
                        }
                    } else if let Some(space_idx) = node_def.find(' ') {
                        node_id = node_def[..space_idx].trim().to_string();
                        shape = node_def[space_idx + 1..].trim().to_string();
                    }
                    
                    config.properties.push(("shape".to_string(), Value::String(shape)));
                    
                    let node_el = AstNode::new(
                        AstNodeKind::Element {
                            tag: "node".to_string(),
                            id: Some(node_id.clone()),
                            config,
                            children: Vec::new(),
                            raw_text: Some(node_id),
                            actions: Vec::new(),
                            ai_operations: Vec::new(),
                            type_assertion: None,
                            is_required: false,
                        },
                        line_num,
                        1,
                    );
                    
                    let len = container_stack.len();
                    if let AstNodeKind::Element { children, .. } = &mut container_stack[len - 1].kind {
                        children.push(node_el);
                    }
                }
                continue;
            }

            // Check if we are inside select container
            let mut inside_select = false;
            if let Some(last) = container_stack.last() {
                if let AstNodeKind::Element { tag, .. } = &last.kind {
                    if tag == "select" {
                        inside_select = true;
                    }
                }
            }

            if inside_select {
                let mut value = clean_line;
                if value.starts_with("item ") {
                    value = value[5..].trim();
                }
                let item_el = AstNode::new(
                    AstNodeKind::Element {
                        tag: "item".to_string(),
                        id: None,
                        config: Config::new(),
                        children: Vec::new(),
                        raw_text: Some(value.to_string()),
                        actions: Vec::new(),
                        ai_operations: Vec::new(),
                        type_assertion: None,
                        is_required: false,
                    },
                    line_num,
                    1,
                );
                let len = container_stack.len();
                if let AstNodeKind::Element { children, .. } = &mut container_stack[len - 1].kind {
                    children.push(item_el);
                }
                continue;
            }

            // Parse standard line items
            let node = parse_line_element(clean_line, line_num);
            if let Some(n) = node {
                if container_stack.is_empty() {
                    nodes.push(n);
                } else {
                    let len = container_stack.len();
                    if let AstNodeKind::Element { children, .. } = &mut container_stack[len - 1].kind {
                        children.push(n);
                    }
                }
            }
        }

        // Close any remaining unclosed containers
        while let Some(popped) = container_stack.pop() {
            if container_stack.is_empty() {
                nodes.push(popped);
            } else {
                let len = container_stack.len();
                if let AstNodeKind::Element { children, .. } = &mut container_stack[len - 1].kind {
                    children.push(popped);
                }
            }
        }

        nodes
    }
}

pub fn parse_properties(s: &str) -> Config {
    let mut config = Config::new();
    let chars: Vec<char> = s.chars().collect();
    let mut idx = 0;
    while idx < chars.len() {
        // Skip whitespace
        while idx < chars.len() && chars[idx].is_whitespace() {
            idx += 1;
        }
        if idx >= chars.len() {
            break;
        }
        
        // Read key
        let mut key = String::new();
        while idx < chars.len() && (chars[idx].is_alphanumeric() || chars[idx] == '-' || chars[idx] == '_' || chars[idx] == '@') {
            key.push(chars[idx]);
            idx += 1;
        }
        
        // Skip whitespace until = or :
        while idx < chars.len() && chars[idx].is_whitespace() {
            idx += 1;
        }
        
        if idx < chars.len() && (chars[idx] == '=' || chars[idx] == ':') {
            idx += 1; // consume = or :
            
            // Skip whitespace
            while idx < chars.len() && chars[idx].is_whitespace() {
                idx += 1;
            }
            
            if idx < chars.len() {
                let val = if chars[idx] == '"' || chars[idx] == '\'' {
                    let quote = chars[idx];
                    idx += 1; // consume quote
                    let mut val_str = String::new();
                    while idx < chars.len() && chars[idx] != quote {
                        val_str.push(chars[idx]);
                        idx += 1;
                    }
                    if idx < chars.len() {
                        idx += 1; // consume quote
                    }
                    Value::String(val_str)
                } else {
                    let mut val_str = String::new();
                    while idx < chars.len() && !chars[idx].is_whitespace() && chars[idx] != ',' {
                        val_str.push(chars[idx]);
                        idx += 1;
                    }
                    if val_str == "true" {
                        Value::Boolean(true)
                    } else if val_str == "false" {
                        Value::Boolean(false)
                    } else if let Ok(n) = val_str.parse::<f64>() {
                        Value::Number(n)
                    } else {
                        Value::Identifier(val_str)
                    }
                };
                
                if !key.is_empty() {
                    config.properties.push((key, val));
                }
            }
        } else if !key.is_empty() {
            // Positional value if no '=' or ':'
            let val = if key == "true" {
                Value::Boolean(true)
            } else if key == "false" {
                Value::Boolean(false)
            } else if let Ok(n) = key.parse::<f64>() {
                Value::Number(n)
            } else {
                Value::Identifier(key)
            };
            config.positional.push(val);
        }
        
        // Consume trailing comma or whitespace
        while idx < chars.len() && (chars[idx].is_whitespace() || chars[idx] == ',') {
            idx += 1;
        }
    }
    config
}

fn extract_actions_and_clean_config(config: &mut Config) -> Vec<Action> {
    let mut actions = Vec::new();
    let mut clean_props = Vec::new();
    for (k, v) in config.properties.drain(..) {
        if k == "click" || k == "onclick" || k == "@click" {
            let body = match v {
                Value::String(s) => s,
                Value::Identifier(s) => s,
                _ => format!("{:?}", v),
            };
            actions.push(Action { name: "click".to_string(), body });
        } else {
            clean_props.push((k, v));
        }
    }
    config.properties = clean_props;
    actions
}

fn parse_line_element(line: &str, line_num: usize) -> Option<AstNode> {
    if line.is_empty() {
        return None;
    }

    // Headers: # Title
    if line.starts_with('#') {
        let mut level = 0;
        let chars: Vec<char> = line.chars().collect();
        while level < chars.len() && chars[level] == '#' {
            level += 1;
        }
        let text = line[level..].trim().to_string();
        let tag = match level {
            1 => "title",
            2 => "subtitle",
            _ => "heading",
        };
        return Some(AstNode::new(
            AstNodeKind::Element {
                tag: tag.to_string(),
                id: None,
                config: Config::new(),
                children: Vec::new(),
                raw_text: Some(text),
                actions: Vec::new(),
                ai_operations: Vec::new(),
                type_assertion: None,
                is_required: false,
            },
            line_num,
            1,
        ));
    }

    // Divider: --- or ***
    if line == "---" || line == "***" {
        return Some(AstNode::new(
            AstNodeKind::Element {
                tag: "divider".to_string(),
                id: None,
                config: Config::new(),
                children: Vec::new(),
                raw_text: None,
                actions: Vec::new(),
                ai_operations: Vec::new(),
                type_assertion: None,
                is_required: false,
            },
            line_num,
            1,
        ));
    }

    // Split first word
    let first_space = line.find(' ').unwrap_or(line.len());
    let first_word = &line[..first_space];
    let remaining = line[first_space..].trim();

    // Match keywords
    match first_word {
        "stat" => {
            let id_space = remaining.find(' ').unwrap_or(remaining.len());
            let id = remaining[..id_space].trim().to_string();
            let props_str = remaining[id_space..].trim();
            let config = parse_properties(props_str);
            Some(AstNode::new(
                AstNodeKind::Element {
                    tag: "stat".to_string(),
                    id: Some(id),
                    config,
                    children: Vec::new(),
                    raw_text: None,
                    actions: Vec::new(),
                    ai_operations: Vec::new(),
                    type_assertion: None,
                    is_required: false,
                },
                line_num,
                1,
            ))
        }
        "bar-chart" | "line-chart" | "pie-chart" | "donut-chart" => {
            let id_space = remaining.find(' ').unwrap_or(remaining.len());
            let id = remaining[..id_space].trim().to_string();
            let props_str = remaining[id_space..].trim();
            let config = parse_properties(props_str);
            Some(AstNode::new(
                AstNodeKind::Element {
                    tag: first_word.to_string(),
                    id: Some(id),
                    config,
                    children: Vec::new(),
                    raw_text: None,
                    actions: Vec::new(),
                    ai_operations: Vec::new(),
                    type_assertion: None,
                    is_required: false,
                },
                line_num,
                1,
            ))
        }
        "spacer" => {
            let height = remaining.parse::<f64>().unwrap_or(20.0);
            let mut config = Config::new();
            config.properties.push(("height".to_string(), Value::Number(height)));
            Some(AstNode::new(
                AstNodeKind::Element {
                    tag: "spacer".to_string(),
                    id: None,
                    config,
                    children: Vec::new(),
                    raw_text: None,
                    actions: Vec::new(),
                    ai_operations: Vec::new(),
                    type_assertion: None,
                    is_required: false,
                },
                line_num,
                1,
            ))
        }
        "button" => {
            let id_space = remaining.find(' ').unwrap_or(remaining.len());
            let id = remaining[..id_space].trim().to_string();
            let rest = remaining[id_space..].trim();
            
            let mut props_str = rest;
            let mut label = String::new();
            if let Some(colon_idx) = rest.find(':') {
                props_str = rest[..colon_idx].trim();
                label = rest[colon_idx + 1..].trim().to_string();
            }
            
            let mut config = parse_properties(props_str);
            let actions = extract_actions_and_clean_config(&mut config);
            Some(AstNode::new(
                AstNodeKind::Element {
                    tag: "button".to_string(),
                    id: Some(id),
                    config,
                    children: Vec::new(),
                    raw_text: Some(label),
                    actions,
                    ai_operations: Vec::new(),
                    type_assertion: None,
                    is_required: false,
                },
                line_num,
                1,
            ))
        }
        "email" | "input" | "password" | "*required" => {
            let mut is_required = false;
            let mut tag = first_word.to_string();
            let mut current_rem = remaining;

            if first_word == "*required" {
                is_required = true;
                let next_space = remaining.find(' ').unwrap_or(remaining.len());
                tag = remaining[..next_space].trim().to_string();
                current_rem = remaining[next_space..].trim();
            }

            let id_space = current_rem.find(' ').unwrap_or(current_rem.len());
            let id = current_rem[..id_space].trim().to_string();
            let rest = current_rem[id_space..].trim();

            let mut props_str = rest;
            let mut type_assertion = None;
            if let Some(double_colon_idx) = rest.find("::") {
                props_str = rest[..double_colon_idx].trim();
                type_assertion = Some(rest[double_colon_idx + 2..].trim().to_string());
            }

            let config = parse_properties(props_str);
            Some(AstNode::new(
                AstNodeKind::Element {
                    tag,
                    id: Some(id),
                    config,
                    children: Vec::new(),
                    raw_text: None,
                    actions: Vec::new(),
                    ai_operations: Vec::new(),
                    type_assertion,
                    is_required,
                },
                line_num,
                1,
            ))
        }
        "submit" => {
            let id_space = remaining.find(' ').unwrap_or(remaining.len());
            let id = remaining[..id_space].trim().to_string();
            let rest = remaining[id_space..].trim();

            let mut props_str = rest;
            let mut label = String::new();
            if let Some(colon_idx) = rest.find(':') {
                props_str = rest[..colon_idx].trim();
                label = rest[colon_idx + 1..].trim().to_string();
            }

            let config = parse_properties(props_str);
            Some(AstNode::new(
                AstNodeKind::Element {
                    tag: "submit".to_string(),
                    id: Some(id),
                    config,
                    children: Vec::new(),
                    raw_text: Some(label),
                    actions: Vec::new(),
                    ai_operations: Vec::new(),
                    type_assertion: None,
                    is_required: false,
                },
                line_num,
                1,
            ))
        }
        _ => {
            Some(AstNode::new(
                AstNodeKind::Element {
                    tag: "text".to_string(),
                    id: None,
                    config: Config::new(),
                    children: Vec::new(),
                    raw_text: Some(line.to_string()),
                    actions: Vec::new(),
                    ai_operations: Vec::new(),
                    type_assertion: None,
                    is_required: false,
                },
                line_num,
                1,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_markdown_parser() {
        let input = "\n# My Page\n::: grid Analytics cols=2\n  ::: card Stats\n    stat ActiveUsers value=100 delta=+5%\n    button ClickMe variant=primary onclick=\"navigate('/here')\": Press Me\n  :::\n:::\n";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let nodes = parser.parse();
        assert_eq!(parser.errors.len(), 0);
        assert_eq!(nodes.len(), 2); // Title and Grid container
        
        let grid = &nodes[1];
        if let AstNodeKind::Element { tag, id, config, children, .. } = &grid.kind {
            assert_eq!(tag, "grid");
            assert_eq!(id.as_deref(), Some("Analytics"));
            assert_eq!(config.get_number_property("cols"), Some(2.0));
            assert_eq!(children.len(), 1);
            
            let card = &children[0];
            if let AstNodeKind::Element { tag: card_tag, id: card_id, children: card_children, .. } = &card.kind {
                assert_eq!(card_tag, "card");
                assert_eq!(card_id.as_deref(), Some("Stats"));
                assert_eq!(card_children.len(), 2);
                
                let stat = &card_children[0];
                if let AstNodeKind::Element { tag: stat_tag, id: stat_id, config: stat_config, .. } = &stat.kind {
                    assert_eq!(stat_tag, "stat");
                    assert_eq!(stat_id.as_deref(), Some("ActiveUsers"));
                    assert_eq!(stat_config.get_number_property("value"), Some(100.0));
                    assert_eq!(stat_config.get_string_property("delta").as_deref(), Some("+5%"));
                } else {
                    panic!("Expected stat element");
                }
            } else {
                panic!("Expected card element");
            }
        } else {
            panic!("Expected grid element");
        }
    }
}
