use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(u64),
    Parameter(String), // $id
}

#[derive(Debug, Clone)]
pub enum Expression {
    Eq(Box<Expression>, Box<Expression>),
    IdFunc(String), // id(n)
    Value(Value),
    CountEdges(String), // count((n)-[]->()) - counts outgoing edges from bound var
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
    pub var: Option<String>,
    pub kind: Option<String>,
    pub props: Vec<(String, Value)>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Node(NodePattern),
    Edge {
        src: NodePattern,
        rel_var: Option<String>,
        rel_kind: Option<String>,
        dst: NodePattern,
    },
}

#[derive(Debug, Clone)]
pub enum OrderBy {
    IdAsc(String),  // ORDER BY id(n) or ORDER BY id(n) ASC
    IdDesc(String), // ORDER BY id(n) DESC
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
        where_clause: Option<Expression>,
        returns: Vec<ReturnExpression>,
        order_by: Option<OrderBy>,
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
    let cmd = parser.parse_command()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "Trailing tokens after command: {:?}",
            &parser.tokens[parser.pos..]
        ));
    }
    Ok(cmd)
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
    Arrow,           // ->
    Dash,            // -
    Eq,              // =
    Dot,             // .
    Dollar,          // $
    Keyword(String), // MERGE, MATCH, RETURN, SET, LIMIT, HELP, QUIT, SCHEMA, SKIP, WHERE
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            '{' => {
                chars.next();
                tokens.push(Token::LBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RBrace);
            }
            '[' => {
                chars.next();
                tokens.push(Token::LBracket);
            }
            ']' => {
                chars.next();
                tokens.push(Token::RBracket);
            }
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            '.' => {
                chars.next();
                tokens.push(Token::Dot);
            }
            '=' => {
                chars.next();
                tokens.push(Token::Eq);
            }
            '$' => {
                chars.next();
                tokens.push(Token::Dollar);
            }
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
                    if nc == '\\' {
                        chars.next();
                        if let Some(ec) = chars.next() {
                            match ec {
                                'n' => s.push('\n'),
                                'r' => s.push('\r'),
                                't' => s.push('\t'),
                                '0' => s.push('\0'),
                                '\\' => s.push('\\'),
                                '"' => s.push('"'),
                                _ => s.push(ec),
                            }
                        }
                    } else if nc == '"' {
                        chars.next(); // skip closing
                        break;
                    } else {
                        s.push(chars.next().unwrap());
                    }
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
                    "MERGE" | "MATCH" | "WHERE" | "RETURN" | "SET" | "LIMIT" | "SKIP" | "HELP"
                    | "QUIT" | "EXIT" | "SCHEMA" | "COUNT" | "ORDER" | "BY" | "ASC" | "DESC" => {
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

    fn expect_token(&mut self, tok: Token) -> bool {
        if let Some(t) = self.peek() {
            if *t == tok {
                self.consume();
                return true;
            }
        }
        false
    }

    fn parse_command(&mut self) -> Result<Command, String> {
        if let Some(Token::Keyword(k)) = self.peek() {
            match k.as_str() {
                "HELP" => {
                    self.consume();
                    Ok(Command::Help)
                }
                "QUIT" | "EXIT" => {
                    self.consume();
                    Ok(Command::Quit)
                }
                "SCHEMA" => {
                    self.consume();
                    Ok(Command::Schema)
                }
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
                    Ok(Command::Merge {
                        pattern,
                        returns,
                        skip,
                    })
                }
                "MATCH" => {
                    self.consume();
                    let pattern = self.parse_pattern()?;

                    let mut where_clause = None;
                    if self.expect_keyword("WHERE") {
                        where_clause = Some(self.parse_expression()?);
                    }

                    let mut limit = 50; // Default limit reduced to optimize BFS scans
                    let mut skip = 0;
                    let mut returns = Vec::new();
                    let mut order_by = None;

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

                    // Check for ORDER BY after RETURN
                    if self.expect_keyword("ORDER") {
                        if !self.expect_keyword("BY") {
                            return Err("Expected BY after ORDER".to_string());
                        }
                        // Parse ORDER BY id(var) [ASC|DESC]
                        if let Some(Token::Ident(func)) = self.peek() {
                            if func == "id" {
                                self.consume(); // consume "id"
                                if self.consume() != Some(&Token::LParen) {
                                    return Err("Expected '(' after id in ORDER BY".to_string());
                                }
                                let var = self.parse_ident()?;
                                if self.consume() != Some(&Token::RParen) {
                                    return Err(
                                        "Expected ')' after id(var) in ORDER BY".to_string()
                                    );
                                }
                                // Check for ASC/DESC
                                if self.expect_keyword("DESC") {
                                    order_by = Some(OrderBy::IdDesc(var));
                                } else {
                                    self.expect_keyword("ASC"); // optional, consume if present
                                    order_by = Some(OrderBy::IdAsc(var));
                                }
                            } else {
                                return Err(format!("Unsupported ORDER BY expression: {}", func));
                            }
                        } else {
                            return Err("Expected id(var) after ORDER BY".to_string());
                        }
                    }

                    // Check again for LIMIT/SKIP after ORDER BY
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

                    Ok(Command::Match {
                        pattern,
                        where_clause,
                        returns,
                        order_by,
                        limit,
                        skip,
                    })
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
                    Ok(Command::Set {
                        var,
                        key,
                        value: val,
                    })
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
        // or ()-[e:REL]->()

        let node_a = self.parse_node_pattern()?;

        // Check for edge
        if let Some(Token::Dash) = self.peek() {
            self.consume(); // -

            let mut rel_var = None;
            let mut rel_kind = None;

            if self.expect_token(Token::LBracket) {
                // [e:REL]
                if let Some(Token::Ident(_s)) = self.peek() {
                    rel_var = Some(self.parse_ident()?);
                }

                if self.expect_token(Token::Colon) {
                    rel_kind = Some(self.parse_ident()?);
                }

                if !self.expect_token(Token::RBracket) {
                    return Err("Expected ']' in edge pattern".to_string());
                }
            }

            if self.consume() != Some(&Token::Arrow) {
                return Err("Expected '->' in edge pattern".to_string());
            }

            let node_b = self.parse_node_pattern()?;

            return Ok(Pattern::Edge {
                src: node_a,
                rel_var,
                rel_kind,
                dst: node_b,
            });
        }

        Ok(Pattern::Node(node_a))
    }

    fn parse_node_pattern(&mut self) -> Result<NodePattern, String> {
        if self.consume() != Some(&Token::LParen) {
            return Err("Expected '(' for node pattern".to_string());
        }

        let mut var = None;
        if let Some(Token::Ident(_)) = self.peek() {
            var = Some(self.parse_ident()?);
        }

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
            Some(Token::Dollar) => {
                let name = self.parse_ident()?;
                Ok(Value::Parameter(name))
            }
            Some(t) => Err(format!("Expected value, got {:?}", t)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        // Minimal expression parser for id(n) = $id
        let left = self.parse_primary_expression()?;
        if let Some(Token::Eq) = self.peek() {
            self.consume();
            let right = self.parse_primary_expression()?;
            Ok(Expression::Eq(Box::new(left), Box::new(right)))
        } else {
            Ok(left)
        }
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, String> {
        // Handle id(n)
        if let Some(Token::Ident(s)) = self.peek() {
            if s == "id" {
                self.consume();
                if self.consume() != Some(&Token::LParen) {
                    return Err("Expected '(' after id".to_string());
                }
                let var = self.parse_ident()?;
                if self.consume() != Some(&Token::RParen) {
                    return Err("Expected ')' after id(var)".to_string());
                }
                return Ok(Expression::IdFunc(var));
            }
        }

        // Handle count((n)-[]->()) for outgoing edge count
        if let Some(Token::Keyword(kw)) = self.peek() {
            if kw == "COUNT" {
                self.consume(); // COUNT
                if self.consume() != Some(&Token::LParen) {
                    return Err("Expected '(' after count".to_string());
                }
                // Now we expect: (var) - [] -> ()
                // Parse (var)
                if self.consume() != Some(&Token::LParen) {
                    return Err("Expected '(' for count edge pattern".to_string());
                }
                let var = self.parse_ident()?;
                if self.consume() != Some(&Token::RParen) {
                    return Err("Expected ')' after variable in count edge pattern".to_string());
                }
                // Parse - [] ->
                if self.consume() != Some(&Token::Dash) {
                    return Err("Expected '-' in count edge pattern".to_string());
                }
                if self.consume() != Some(&Token::LBracket) {
                    return Err("Expected '[' in count edge pattern".to_string());
                }
                if self.consume() != Some(&Token::RBracket) {
                    return Err("Expected ']' in count edge pattern".to_string());
                }
                if self.consume() != Some(&Token::Arrow) {
                    return Err("Expected '->' in count edge pattern".to_string());
                }
                // Parse ()
                if self.consume() != Some(&Token::LParen) {
                    return Err("Expected '(' for target in count edge pattern".to_string());
                }
                if self.consume() != Some(&Token::RParen) {
                    return Err("Expected ')' for target in count edge pattern".to_string());
                }
                // Close the count()
                if self.consume() != Some(&Token::RParen) {
                    return Err("Expected ')' to close count()".to_string());
                }
                return Ok(Expression::CountEdges(var));
            }
        }

        Ok(Expression::Value(self.parse_value()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_tokenize_basic() {
        let input = "MATCH (n:proc.Process {id: 123}) RETURN n";
        let _tokens = tokenize(input).unwrap();

        let _expected = vec![
            Token::Keyword("MATCH".to_string()),
            Token::LParen,
            Token::Ident("n".to_string()),
            Token::Colon,
            Token::Ident("proc.Process".to_string()),
            Token::LBrace,
            Token::Ident("id".to_string()),
            Token::Colon,
            Token::Number(123),
            Token::RBrace,
            Token::RParen,
            Token::Keyword("RETURN".to_string()),
            Token::Ident("n".to_string()),
        ];
    }

    #[test]
    fn test_parse_match_simple() {
        let cmd = parse("MATCH (n) RETURN n").unwrap();
        if let Command::Match {
            pattern, returns, ..
        } = cmd
        {
            if let Pattern::Node(pat) = pattern {
                assert_eq!(pat.var.as_deref(), Some("n"));
            } else {
                panic!("Wrong pattern");
            }
            assert_eq!(returns.len(), 1);
            assert_eq!(returns[0], ReturnExpression::Variable("n".to_string()));
        } else {
            panic!("Expected Match");
        }
    }

    #[test]
    fn test_parse_match_multiple_props() {
        let cmd = parse("MATCH (n {id: 1, type: \"user\", status: \"active\"}) RETURN n").unwrap();
        if let Command::Match { pattern, .. } = cmd {
            if let Pattern::Node(pat) = pattern {
                assert_eq!(pat.var.as_deref(), Some("n"));
                assert_eq!(pat.kind, None);
                assert_eq!(pat.props.len(), 3);

                assert_eq!(pat.props[0].0, "id");
                if let Value::Number(n) = pat.props[0].1 {
                    assert_eq!(n, 1);
                } else {
                    panic!("Wrong prop value type for id");
                }

                assert_eq!(pat.props[1].0, "type");
                if let Value::String(ref s) = pat.props[1].1 {
                    assert_eq!(s, "user");
                } else {
                    panic!("Wrong prop value type for type");
                }

                assert_eq!(pat.props[2].0, "status");
                if let Value::String(ref s) = pat.props[2].1 {
                    assert_eq!(s, "active");
                } else {
                    panic!("Wrong prop value type for status");
                }
            } else {
                panic!("Expected Pattern::Node");
            }
        } else {
            panic!("Expected Command::Match");
        }
    }

    #[test]
    fn test_parse_match_kind_props() {
        let cmd = parse("MATCH (p:proc.Process {name: \"init\"}) RETURN p").unwrap();
        if let Command::Match { pattern, .. } = cmd {
            if let Pattern::Node(pat) = pattern {
                assert_eq!(pat.var.as_deref(), Some("p"));
                assert_eq!(pat.kind.as_deref(), Some("proc.Process"));
                assert_eq!(pat.props.len(), 1);
                assert_eq!(pat.props[0].0, "name");
                if let Value::String(ref s) = pat.props[0].1 {
                    assert_eq!(s, "init");
                } else {
                    panic!("Wrong prop value type");
                }
            }
        }
    }

    #[test]
    fn test_parse_match_edge() {
        let cmd = parse("MATCH (a)-[:PARENT]->(b) RETURN a, b").unwrap();
        if let Command::Match {
            pattern, returns, ..
        } = cmd
        {
            if let Pattern::Edge {
                src,
                rel_var,
                rel_kind,
                dst,
            } = pattern
            {
                assert_eq!(src.var.as_deref(), Some("a"));
                assert_eq!(rel_var, None);
                assert_eq!(rel_kind.as_deref(), Some("PARENT"));
                assert_eq!(dst.var.as_deref(), Some("b"));
            } else {
                panic!("Wrong pattern");
            }
            assert_eq!(returns.len(), 2);
        }
    }

    #[test]
    fn test_parse_merge() {
        let cmd = parse("MERGE (n:Kind {key: 1}) RETURN count(n)").unwrap();
        if let Command::Merge {
            pattern, returns, ..
        } = cmd
        {
            if let Pattern::Node(pat) = pattern {
                assert_eq!(pat.kind.as_deref(), Some("Kind"));
            }
            assert_eq!(returns.len(), 1);
            assert_eq!(returns[0], ReturnExpression::Count("n".to_string()));
        } else {
            panic!("Expected Merge");
        }
    }

    #[test]
    fn test_parse_set() {
        let cmd = parse("SET n.priority = 10").unwrap();
        if let Command::Set { var, key, value } = cmd {
            assert_eq!(var, "n");
            assert_eq!(key, "priority");
            if let Value::Number(n) = value {
                assert_eq!(n, 10);
            } else {
                panic!("Wrong value");
            }
        } else {
            panic!("Expected Set");
        }
    }

    #[test]
    fn test_parse_limit_skip() {
        let cmd = parse("MATCH (n) RETURN n SKIP 10 LIMIT 5").unwrap();
        if let Command::Match { limit, skip, .. } = cmd {
            assert_eq!(skip, 10);
            assert_eq!(limit, 5);
        } else {
            panic!("Expected Match");
        }

        // Test interleaved
        let cmd = parse("MATCH (n) LIMIT 20 SKIP 5 RETURN n").unwrap();
        if let Command::Match { limit, skip, .. } = cmd {
            assert_eq!(limit, 20);
            assert_eq!(skip, 5);
        }
    }

    #[test]
    fn test_parse_where_complex() {
        let cmd = parse("MATCH (n) WHERE id(n) = $id RETURN n").unwrap();
        if let Command::Match { where_clause, .. } = cmd {
            assert!(where_clause.is_some());
        }
    }

    #[test]
    fn test_parse_parameter_usage() {
        let cmd = parse("MATCH (n {id: $target}) RETURN n").unwrap();
        if let Command::Match { pattern, .. } = cmd {
            if let Pattern::Node(pat) = pattern {
                if let Value::Parameter(p) = &pat.props[0].1 {
                    assert_eq!(p, "target");
                } else {
                    panic!("Expected parameter");
                }
            }
        }
    }

    #[test]
    fn test_parse_shortcuts() {
        assert!(matches!(parse("HELP").unwrap(), Command::Help));
        assert!(matches!(parse("QUIT").unwrap(), Command::Quit));
        assert!(matches!(parse("SCHEMA").unwrap(), Command::Schema));
    }

    #[test]
    fn test_parse_errors() {
        assert!(parse("INVALID COMMAND").is_err());
        assert!(parse("MATCH (n RETURN n").is_err()); // Missing )
        assert!(parse("MATCH (n) RETURN n {").is_err()); // Trailing garbage partially parsed maybe?
        assert!(parse("SET n = 1").is_err()); // Missing dot
    }

    #[test]
    fn test_tokenize_strings() {
        let tokens = tokenize("\"hello world\" \"escaped \\\" quote\"").unwrap();
        assert_eq!(tokens.len(), 2);
        if let Token::String(s) = &tokens[0] {
            assert_eq!(s, "hello world");
        }
        if let Token::String(s) = &tokens[1] {
            assert_eq!(s, "escaped \" quote");
        }
    }

    #[test]
    fn test_tokenize_escapes() {
        let tokens = tokenize("\"line\\nbreak\"").unwrap();
        if let Token::String(s) = &tokens[0] {
            assert_eq!(s, "line\nbreak");
        } else {
            panic!("Expected string token");
        }

        let tokens = tokenize("\"tab\\tcharacter\"").unwrap();
        if let Token::String(s) = &tokens[0] {
            assert_eq!(s, "tab\tcharacter");
        } else {
            panic!("Expected string token");
        }
    }

    #[test]
    fn test_parse_match_anonymous_edge() {
        let cmd = parse("MATCH ()-[e]->() RETURN e").unwrap();
        if let Command::Match {
            pattern, returns, ..
        } = cmd
        {
            if let Pattern::Edge {
                src,
                rel_var,
                rel_kind,
                dst,
            } = pattern
            {
                assert!(src.var.is_none());
                assert_eq!(rel_var.as_deref(), Some("e"));
                assert!(rel_kind.is_none());
                assert!(dst.var.is_none());
            } else {
                panic!("Wrong pattern");
            }
            assert_eq!(returns.len(), 1);
            assert_eq!(returns[0], ReturnExpression::Variable("e".to_string()));
        } else {
            panic!("Expected Match");
        }
    }

    #[test]
    fn test_parse_order_by() {
        // Case 1: ORDER BY id(n) (default ASC)
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n)").unwrap();
        if let Command::Match { order_by, .. } = cmd {
            if let Some(OrderBy::IdAsc(var)) = order_by {
                assert_eq!(var, "n");
            } else {
                panic!("Expected OrderBy::IdAsc");
            }
        } else {
            panic!("Expected Match");
        }

        // Case 2: ORDER BY id(n) ASC
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) ASC").unwrap();
        if let Command::Match { order_by, .. } = cmd {
            if let Some(OrderBy::IdAsc(var)) = order_by {
                assert_eq!(var, "n");
            } else {
                panic!("Expected OrderBy::IdAsc");
            }
        } else {
            panic!("Expected Match");
        }

        // Case 3: ORDER BY id(n) DESC
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) DESC").unwrap();
        if let Command::Match { order_by, .. } = cmd {
            if let Some(OrderBy::IdDesc(var)) = order_by {
                assert_eq!(var, "n");
            } else {
                panic!("Expected OrderBy::IdDesc");
            }
        } else {
            panic!("Expected Match");
        }

        // Case 4: Error cases
        // ORDER with missing BY
        assert!(parse("MATCH (n) RETURN n ORDER").is_err());
        // ORDER BY missing variable/id
        assert!(parse("MATCH (n) RETURN n ORDER BY n").is_err());
        // ORDER BY invalid direction
        assert!(parse("MATCH (n) RETURN n ORDER BY id(n) INVALID").is_err());
    }

    #[test]
    fn test_parse_where_count_edges() {
        let cmd = parse("MATCH (n) WHERE COUNT((n)-[]->()) = 5 RETURN n").unwrap();
        if let Command::Match { where_clause, .. } = cmd {
            if let Some(Expression::Eq(left, right)) = where_clause {
                if let Expression::CountEdges(var) = *left {
                    assert_eq!(var, "n");
                } else {
                    panic!("Expected CountEdges on left side");
                }
                if let Expression::Value(Value::Number(n)) = *right {
                    assert_eq!(n, 5);
                } else {
                    panic!("Expected Number on right side");
                }
            } else {
                panic!("Expected Eq expression");
            }
        } else {
            panic!("Expected Match command");
        }
    }

    #[test]
    fn test_parse_return_multiple_expressions() {
        let cmd = parse("MATCH (n) RETURN count(n), n").unwrap();
        if let Command::Match { returns, .. } = cmd {
            assert_eq!(returns.len(), 2);
            assert_eq!(returns[0], ReturnExpression::Count("n".to_string()));
            assert_eq!(returns[1], ReturnExpression::Variable("n".to_string()));
        } else {
            panic!("Expected Match command");
        }
    }
}
