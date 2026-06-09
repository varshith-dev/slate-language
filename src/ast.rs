#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Expression(String), // e.g. ${ user.name }
    Array(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub properties: Vec<(String, Value)>,
    pub positional: Vec<Value>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            positional: Vec::new(),
        }
    }

    pub fn get_property(&self, key: &str) -> Option<&Value> {
        for (k, v) in &self.properties {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn get_string_property(&self, key: &str) -> Option<String> {
        match self.get_property(key) {
            Some(Value::String(s)) => Some(s.clone()),
            Some(Value::Identifier(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_number_property(&self, key: &str) -> Option<f64> {
        match self.get_property(key) {
            Some(Value::Number(n)) => Some(*n),
            _ => None,
        }
    }

    pub fn get_bool_property(&self, key: &str) -> Option<bool> {
        match self.get_property(key) {
            Some(Value::Boolean(b)) => Some(*b),
            _ => None,
        }
    }

    pub fn get_first_positional_string(&self) -> Option<String> {
        match self.positional.first() {
            Some(Value::String(s)) => Some(s.clone()),
            Some(Value::Identifier(s)) => Some(s.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub name: String, // e.g. click, hover
    pub body: String, // e.g. navigate("/start")
}

#[derive(Debug, Clone, PartialEq)]
pub struct AiOperation {
    pub name: String, // e.g. summarize, generate
    pub config: Config,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNodeKind {
    Element {
        tag: String, // e.g. page, button, title
        id: Option<String>,
        config: Config,
        children: Vec<AstNode>,
        raw_text: Option<String>,
        actions: Vec<Action>,
        ai_operations: Vec<AiOperation>,
        type_assertion: Option<String>,
        is_required: bool,
    },
    Relationship {
        source: String,
        target: String,
        config: Config,
    },
    Section {
        name: String,
        children: Vec<AstNode>,
    },
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub line: usize,
    pub col: usize,
}

impl AstNode {
    pub fn new(kind: AstNodeKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}
