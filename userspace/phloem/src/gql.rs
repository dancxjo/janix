use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnExpression {
    Variable(String),
    Count(String),
}

impl ReturnExpression {
    pub fn to_string(&self) -> String {
        match self {
            ReturnExpression::Variable(s) => s.clone(),
            ReturnExpression::Count(s) => format!("count({})", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodePattern {
    pub var: String,
    pub kind: Option<String>,
    pub props: Vec<(String, Value)>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Node(NodePattern),
    Edge {
        src_var: String,
        rel: String,
        dst_var: String,
    },
}

#[derive(Debug, Clone)]
pub enum Command {
    Merge {
        pattern: Pattern,
        returns: Vec<ReturnExpression>,
        skip: usize,
    },
    Match {
        pattern: Pattern,
        returns: Vec<ReturnExpression>,
        limit: usize,
        skip: usize,
    },
    Set {
        var: String,
        key: String,
        value: Value,
    },
    Help,
    Quit,
    Schema,
}

pub fn parse(input: &str) -> Result<Command, String> {
    let tokens = tokenize(input)?;
    let mut parser = Parser { tokens, pos: 0 };
    parser.parse_command()
}

#[derive(Debug, PartialEq)]
enum Token {
    Ident(String),
    String(String),
    Number(u64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Arrow, // ->
    Dash,  // -
    Eq,    // =
    Dot,   // .
    Keyword(String), // MERGE, MATCH, RETURN, SET, LIMIT, HELP, QUIT, SCHEMA, SKIP
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            '(' => { chars.next(); tokens.push(Token::LParen); }
            ')' => { chars.next(); tokens.push(Token::RParen); }
            '{' => { chars.next(); tokens.push(Token::LBrace); }
            '}' => { chars.next(); tokens.push(Token::RBrace); }
            '[' => { chars.next(); tokens.push(Token::LBracket); }
            ']' => { chars.next(); tokens.push(Token::RBracket); }
            ':' => { chars.next(); tokens.push(Token::Colon); }
            ',' => { chars.next(); tokens.push(Token::Comma); }
            '.' => { chars.next(); tokens.push(Token::Dot); }
            '=' => { chars.next(); tokens.push(Token::Eq); }
            '-' => {
                chars.next();
                if let Some(&'>') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Dash);
                }
            }
            '"' => {
                chars.next(); // skip opening quote
                let mut s = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc == '"' {
                        chars.next(); // skip closing
                        break;
                    }
                    s.push(chars.next().unwrap());
                }
                tokens.push(Token::String(s));
            }
            c if c.is_alphabetic() => {
                let mut s = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' {
                        s.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match s.to_uppercase().as_str() {
                    "MERGE" | "MATCH" | "RETURN" | "SET" | "LIMIT" | "SKIP" | "HELP" | "QUIT" | "EXIT" | "SCHEMA" | "COUNT" => {
                        tokens.push(Token::Keyword(s.to_uppercase()));
                    }
                    _ => tokens.push(Token::Ident(s)),
                }
            }
            c if c.is_ascii_digit() => {
                let mut s = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() {
                        s.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Ok(n) = s.parse::<u64>() {
                    tokens.push(Token::Number(n));
                } else {
                    return Err(format!("Invalid number: {}", s));
                }
            }
            _ => {
                return Err(format!("Unexpected character: {}", c));
            }
        }
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let t = &self.tokens[self.pos];
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn expect_keyword(&mut self, kw: &str) -> bool {
        if let Some(Token::Keyword(k)) = self.peek() {
            if k == kw {
                self.consume();
                return true;
            }
        }
        false
    }

    fn parse_command(&mut self) -> Result<Command, String> {
        if let Some(Token::Keyword(k)) = self.peek() {
            match k.as_str() {
                "HELP" => { self.consume(); Ok(Command::Help) }
                "QUIT" | "EXIT" => { self.consume(); Ok(Command::Quit) }
                "SCHEMA" => { self.consume(); Ok(Command::Schema) }
                "MERGE" => {
                    self.consume();
                    let pattern = self.parse_pattern()?;
                    let mut returns = Vec::new();
                    let mut skip = 0;
                    if self.expect_keyword("RETURN") {
                        returns = self.parse_return_vars()?;
                    }
                    if self.expect_keyword("SKIP") {
                        if let Some(Token::Number(n)) = self.consume() {
                            skip = *n as usize;
                        }
                    }
                    Ok(Command::Merge { pattern, returns, skip })
                }
                "MATCH" => {
                    self.consume();
                    let pattern = self.parse_pattern()?;
                    let mut limit = 1000; // Default limit increased
                    let mut skip = 0;
                    let mut returns = Vec::new();

                    // Support LIMIT/SKIP anywhere
                    loop {
                        if self.expect_keyword("LIMIT") {
                            if let Some(Token::Number(n)) = self.consume() {
                                limit = *n as usize;
                            }
                            continue;
                        }
                        if self.expect_keyword("SKIP") {
                            if let Some(Token::Number(n)) = self.consume() {
                                skip = *n as usize;
                            }
                            continue;
                        }
                        break;
                    }

                    if self.expect_keyword("RETURN") {
                        returns = self.parse_return_vars()?;
                    }

                    // Check again after return
                    loop {
                        if self.expect_keyword("LIMIT") {
                            if let Some(Token::Number(n)) = self.consume() {
                                limit = *n as usize;
                            }
                            continue;
                        }
                        if self.expect_keyword("SKIP") {
                            if let Some(Token::Number(n)) = self.consume() {
                                skip = *n as usize;
                            }
                            continue;
                        }
                        break;
                    }

                    Ok(Command::Match { pattern, returns, limit, skip })
                }
                "SET" => {
                    self.consume();
                    // SET n.prop = val
                    let var_key = self.parse_ident()?; // This will now handle "n.prop"
                    let parts: Vec<&str> = var_key.split('.').collect();
                    if parts.len() != 2 {
                        return Err("Expected 'var.key' format for SET".to_string());
                    }
                    let var = parts[0].to_string();
                    let key = parts[1].to_string();

                    if self.consume() != Some(&Token::Eq) {
                        return Err("Expected '=' in SET".to_string());
                    }
                    let val = self.parse_value()?;
                    Ok(Command::Set { var, key, value: val })
                }
                _ => Err(format!("Unknown command: {}", k)),
            }
        } else {
            Err("Expected command keyword".to_string())
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        // (n:Kind { ... })
        // or (a)-[:REL]->(b)

        let node_a = self.parse_node_pattern()?;

        // Check for edge
        if let Some(Token::Dash) = self.peek() {
            self.consume(); // -
            if self.consume() != Some(&Token::LBracket) {
                return Err("Expected '[' in edge pattern".to_string());
            }
            if self.consume() != Some(&Token::Colon) {
                return Err("Expected ':' in edge pattern".to_string());
            }
            let rel = self.parse_ident()?;
            if self.consume() != Some(&Token::RBracket) {
                return Err("Expected ']' in edge pattern".to_string());
            }
            if self.consume() != Some(&Token::Arrow) {
                return Err("Expected '->' in edge pattern".to_string());
            }

            let node_b = self.parse_node_pattern()?;

            return Ok(Pattern::Edge {
                src_var: node_a.var,
                rel,
                dst_var: node_b.var,
            });
        }

        Ok(Pattern::Node(node_a))
    }

    fn parse_node_pattern(&mut self) -> Result<NodePattern, String> {
        if self.consume() != Some(&Token::LParen) {
            return Err("Expected '(' for node pattern".to_string());
        }

        let var = self.parse_ident()?;
        let mut kind = None;
        let mut props = Vec::new();

        if let Some(Token::Colon) = self.peek() {
            self.consume();
            kind = Some(self.parse_ident()?);
        }

        if let Some(Token::LBrace) = self.peek() {
            self.consume();
            loop {
                if let Some(Token::RBrace) = self.peek() {
                    self.consume();
                    break;
                }
                let key = self.parse_ident()?;
                if self.consume() != Some(&Token::Colon) {
                    return Err("Expected ':' in property map".to_string());
                }
                let val = self.parse_value()?;
                props.push((key, val));

                if let Some(Token::Comma) = self.peek() {
                    self.consume();
                }
            }
        }

        if self.consume() != Some(&Token::RParen) {
            return Err("Expected ')' closing node pattern".to_string());
        }

        Ok(NodePattern { var, kind, props })
    }

    fn parse_return_vars(&mut self) -> Result<Vec<ReturnExpression>, String> {
        let mut exprs = Vec::new();
        loop {
            if self.expect_keyword("COUNT") {
                if self.consume() != Some(&Token::LParen) {
                    return Err("Expected '(' after COUNT".to_string());
                }
                let var = self.parse_ident()?;
                if self.consume() != Some(&Token::RParen) {
                    return Err("Expected ')' after COUNT(var)".to_string());
                }
                exprs.push(ReturnExpression::Count(var));
            } else {
                exprs.push(ReturnExpression::Variable(self.parse_ident()?));
            }

            if let Some(Token::Comma) = self.peek() {
                self.consume();
            } else {
                break;
            }
        }
        Ok(exprs)
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        let mut s = match self.consume() {
            Some(Token::Ident(s)) => s.clone(),
            _ => return Err("Expected identifier".to_string()),
        };

        while let Some(Token::Dot) = self.peek() {
            self.consume();
            match self.consume() {
                Some(Token::Ident(part)) => {
                    s.push('.');
                    s.push_str(part);
                }
                _ => return Err("Expected identifier after '.'".to_string()),
            }
        }
        Ok(s)
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        match self.consume() {
            Some(Token::String(s)) => Ok(Value::String(s.clone())),
            Some(Token::Number(n)) => Ok(Value::Number(*n)),
            Some(t) => Err(format!("Expected value, got {:?}", t)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}
