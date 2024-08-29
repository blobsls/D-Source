use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Token types
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Identifier,
    Keyword,
    Operator,
    Literal,
    Separator,
    Comment,
    Whitespace,
}

// Token structure
#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    column: usize,
}

// Abstract Syntax Tree Node
#[derive(Debug)]
enum ASTNode {
    Program(Vec<ASTNode>),
    FunctionDeclaration {
        name: String,
        parameters: Vec<ASTNode>,
        return_type: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    VariableDeclaration {
        name: String,
        var_type: Box<ASTNode>,
        initializer: Option<Box<ASTNode>>,
    },
    Type(String),
    Block(Vec<ASTNode>),
    Expression(Box<ASTNode>),
    BinaryOperation {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    UnaryOperation {
        operator: String,
        operand: Box<ASTNode>,
    },
    Literal(String),
    Identifier(String),
}

// Parser structure
struct Parser {
    tokens: Vec<Token>,
    current: usize,
    ast: Option<ASTNode>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            ast: None,
        }
    }

    fn parse(&mut self) -> Result<ASTNode, String> {
        let mut program_nodes = Vec::new();

        while !self.is_at_end() {
            match self.parse_declaration() {
                Ok(node) => program_nodes.push(node),
                Err(e) => return Err(e),
            }
        }

        self.ast = Some(ASTNode::Program(program_nodes));
        Ok(self.ast.clone().unwrap())
    }

    fn parse_declaration(&mut self) -> Result<ASTNode, String> {
        if self.match_token(TokenType::Keyword, "fn") {
            self.parse_function_declaration()
        } else if self.match_token(TokenType::Keyword, "let") {
            self.parse_variable_declaration()
        } else {
            Err("Expected declaration".to_string())
        }
    }

    fn parse_function_declaration(&mut self) -> Result<ASTNode, String> {
        let name = self.expect_identifier()?;
        self.expect_token(TokenType::Separator, "(")?;
        let parameters = self.parse_parameters()?;
        self.expect_token(TokenType::Separator, ")")?;
        self.expect_token(TokenType::Separator, "->")?;
        let return_type = Box::new(self.parse_type()?);
        let body = Box::new(self.parse_block()?);

        Ok(ASTNode::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_parameters(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut parameters = Vec::new();

        if !self.check(TokenType::Separator, ")") {
            loop {
                let param_name = self.expect_identifier()?;
                self.expect_token(TokenType::Separator, ":")?;
                let param_type = self.parse_type()?;
                parameters.push(ASTNode::VariableDeclaration {
                    name: param_name,
                    var_type: Box::new(param_type),
                    initializer: None,
                });

                if !self.match_token(TokenType::Separator, ",") {
                    break;
                }
            }
        }

        Ok(parameters)
    }

    fn parse_type(&mut self) -> Result<ASTNode, String> {
        let type_name = self.expect_identifier()?;
        Ok(ASTNode::Type(type_name))
    }

    fn parse_block(&mut self) -> Result<ASTNode, String> {
        self.expect_token(TokenType::Separator, "{")?;
        let mut statements = Vec::new();

        while !self.check(TokenType::Separator, "}") && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.expect_token(TokenType::Separator, "}")?;
        Ok(ASTNode::Block(statements))
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if self.match_token(TokenType::Keyword, "let") {
            self.parse_variable_declaration()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        let name = self.expect_identifier()?;
        self.expect_token(TokenType::Separator, ":")?;
        let var_type = Box::new(self.parse_type()?);

        let initializer = if self.match_token(TokenType::Operator, "=") {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.expect_token(TokenType::Separator, ";")?;

        Ok(ASTNode::VariableDeclaration {
            name,
            var_type,
            initializer,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_expression()?;
        self.expect_token(TokenType::Separator, ";")?;
        Ok(ASTNode::Expression(Box::new(expr)))
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_equality()?;

        if self.match_token(TokenType::Operator, "=") {
            let value = self.parse_assignment()?;
            match expr {
                ASTNode::Identifier(name) => {
                    Ok(ASTNode::BinaryOperation {
                        left: Box::new(ASTNode::Identifier(name)),
                        operator: "=".to_string(),
                        right: Box::new(value),
                    })
                }
                _ => Err("Invalid assignment target".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_equality(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_comparison()?;

        while self.match_any(&[
            (TokenType::Operator, "=="),
            (TokenType::Operator, "!="),
        ]) {
            let operator = self.previous().value.clone();
            let right = self.parse_comparison()?;
            expr = ASTNode::BinaryOperation {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_term()?;

        while self.match_any(&[
            (TokenType::Operator, ">"),
            (TokenType::Operator, ">="),
            (TokenType::Operator, "<"),
            (TokenType::Operator, "<="),
        ]) {
            let operator = self.previous().value.clone();
            let right = self.parse_term()?;
            expr = ASTNode::BinaryOperation {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_factor()?;

        while self.match_any(&[
            (TokenType::Operator, "+"),
            (TokenType::Operator, "-"),
        ]) {
            let operator = self.previous().value.clone();
            let right = self.parse_factor()?;
            expr = ASTNode::BinaryOperation {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_unary()?;

        while self.match_any(&[
            (TokenType::Operator, "*"),
            (TokenType::Operator, "/"),
        ]) {
            let operator = self.previous().value.clone();
            let right = self.parse_unary()?;
            expr = ASTNode::BinaryOperation {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<ASTNode, String> {
        if self.match_any(&[
            (TokenType::Operator, "!"),
            (TokenType::Operator, "-"),
        ]) {
            let operator = self.previous().value.clone();
            let right = self.parse_unary()?;
            Ok(ASTNode::UnaryOperation {
                operator,
                operand: Box::new(right),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        if self.match_token(TokenType::Literal, "") {
            Ok(ASTNode::Literal(self.previous().value.clone()))
        } else if self.match_token(TokenType::Identifier, "") {
            Ok(ASTNode::Identifier(self.previous().value.clone()))
        } else if self.match_token(TokenType::Separator, "(") {
            let expr = self.parse_expression()?;
            self.expect_token(TokenType::Separator, ")")?;
            Ok(expr)
        } else {
            Err("Expected expression".to_string())
        }
    }

    fn match_token(&mut self, token_type: TokenType, value: &str) -> bool {
        if self.check(token_type.clone(), value) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, tokens: &[(TokenType, &str)]) -> bool {
        for (token_type, value) in tokens {
            if self.check(token_type.clone(), value) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType, value: &str) -> bool {
        if self.is_at_end() {
            false
        } else {
            let token = &self.tokens[self.current];
            token.token_type == token_type && (value.is_empty() || token.value == value)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn expect_token(&mut self, token_type: TokenType, value: &str) -> Result<(), String> {
        if self.check(token_type.clone(), value) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected token: {:?} '{}'", token_type, value))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        if self.match_token(TokenType::Identifier, "") {
            Ok(self.previous().value.clone())
        } else {
            Err("Expected identifier".to_string())
        }
    }
}

// Lexer structure
struct Lexer {
    input: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::Separator,
            value: "EOF".to_string(),
            line: self.line,
            column: self.current,
        });

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' | ')' | '{' | '}' | ',' | ';' | ':' => self.add_token(TokenType::Separator),
            '+' | '-' | '*' | '/' => self.add_token(TokenType::Operator),
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Operator);
                } else {
                    self.add_token(TokenType::Operator);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Operator);
                } else {
                    self.add_token(TokenType::Operator);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Operator);
                } else {
                    self.add_token(TokenType::Operator);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Operator);
                } else {
                    self.add_token(TokenType::Operator);
                }
            }
            '"' => self.string()?,
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Operator);
                }
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.input.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.input.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input.chars().nth(self.current).unwrap()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.input[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            value: text.to_string(),
            line: self.line,
            column: self.start,
        });
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        self.advance();
        let value = &self.input[self.start + 1..self.current - 1];
        self.add_token(TokenType::Literal);
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.input.chars().nth(self.current + 1).unwrap().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token(TokenType::Literal);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.input[self.start..self.current];
        let token_type = match text {
            "fn" | "let" | "if" | "else" | "while" | "return" => TokenType::Keyword,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }
}

fn main() -> io::Result<()> {
    println!("D++ C Parser Initialization");
    println!("---------------------------");

    // Read input file
    let file_path = "input.dpp";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut input = String::new();

    for line in reader.lines() {
        input.push_str(&line?);
        input.push('\n');
    }

    // Initialize lexer
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return Ok(());
        }
    };

    println!("Tokenization complete. Found {} tokens.", tokens.len());

    // Initialize parser
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return Ok(());
        }
    };

    println!("Parsing complete. AST generated.");

    // Print AST (for demonstration purposes)
    println!("Abstract Syntax Tree:");
    println!("{:#?}", ast);

    println!("D++ C Parser initialization complete.");
    Ok(())
}

// Helper function to read keywords from a file
fn read_keywords(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut keywords = Vec::new();

    for line in reader.lines() {
        keywords.push(line?);
    }

    Ok(keywords)
}

// Helper function to generate symbol table
fn generate_symbol_table(ast: &ASTNode) -> HashMap<String, String> {
    let mut symbol_table = HashMap::new();

    fn traverse_ast(node: &ASTNode, table: &mut HashMap<String, String>) {
        match node {
            ASTNode::VariableDeclaration { name, var_type, .. } => {
                if let ASTNode::Type(type_name) = **var_type {
                    table.insert(name.clone(), type_name);
                }
            }
            ASTNode::FunctionDeclaration { name, parameters, return_type, .. } => {
                let mut param_types = Vec::new();
                for param in parameters {
                    if let ASTNode::VariableDeclaration { var_type, .. } = param {
                        if let ASTNode::Type(type_name) = **var_type {
                            param_types.push(type_name);
                        }
                    }
                }
                let ret_type = if let ASTNode::Type(type_name) = **return_type {
                    type_name
                } else {
                    "void".to_string()
                };
                table.insert(name.clone(), format!("fn({}) -> {}", param_types.join(", "), ret_type));
            }
            ASTNode::Program(nodes) | ASTNode::Block(nodes) => {
                for node in nodes {
                    traverse_ast(node, table);
                }
            }
            _ => {}
        }
    }

    traverse_ast(ast, &mut symbol_table);
    symbol_table
}

// Helper function to perform semantic analysis
fn semantic_analysis(ast: &ASTNode, symbol_table: &HashMap<String, String>) -> Result<(), String> {
    fn check_node(node: &ASTNode, table: &HashMap<String, String>) -> Result<(), String> {
        match node {
            ASTNode::BinaryOperation { left, operator, right } => {
                check_node(left, table)?;
                check_node(right, table)?;
                // Add type checking for binary operations
            }
            ASTNode::UnaryOperation { operator, operand } => {
                check_node(operand, table)?;
                // Add type checking for unary operations
            }
            ASTNode::Identifier(name) => {
                if !table.contains_key(name) {
                    return Err(format!("Undefined variable: {}", name));
                }
            }
            ASTNode::FunctionDeclaration { name, parameters, body, .. } => {
                // Check function body
                check_node(body, table)?;
            }
            ASTNode::Program(nodes) | ASTNode::Block(nodes) => {
                for node in nodes {
                    check_node(node, table)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    check_node(ast, symbol_table)
}

// Helper function to optimize AST
fn optimize_ast(ast: &mut ASTNode) {
    fn optimize_node(node: &mut ASTNode) {
        match node {
            ASTNode::BinaryOperation { left, operator, right } => {
                optimize_node(left);
                optimize_node(right);
                // Perform constant folding and other optimizations
            }
            ASTNode::UnaryOperation { operand, .. } => {
                optimize_node(operand);
            }
            ASTNode::Program(nodes) | ASTNode::Block(nodes) => {
                for node in nodes {
                    optimize_node(node);
                }
            }
            _ => {}
        }
    }

    optimize_node(ast);
}

// Helper function to generate intermediate representation (IR)
fn generate_ir(ast: &ASTNode) -> Vec<String> {
    let mut ir = Vec::new();

    fn generate_node_ir(node: &ASTNode, ir: &mut Vec<String>) {
        match node {
            ASTNode::FunctionDeclaration { name, parameters, body, .. } => {
                ir.push(format!("function {}:", name));
                for param in parameters {
                    if let ASTNode::VariableDeclaration { name, .. } = param {
                        ir.push(format!("  param {}", name));
                    }
                }
                generate_node_ir(body, ir);
                ir.push("end_function".to_string());
            }
            ASTNode::Block(statements) => {
                for stmt in statements {
                    generate_node_ir(stmt, ir);
                }
            }
            ASTNode::VariableDeclaration { name, initializer, .. } => {
                if let Some(init) = initializer {
                    generate_node_ir(init, ir);
                    ir.push(format!("store {}", name));
                }
            }
            ASTNode::BinaryOperation { left, operator, right } => {
                generate_node_ir(left, ir);
                generate_node_ir(right, ir);
                ir.push(format!("{} {}", operator, operator));
            }
            ASTNode::UnaryOperation { operator, operand } => {
                generate_node_ir(operand, ir);
                ir.push(format!("{}", operator));
            }
            ASTNode::Literal(value) => {
                ir.push(format!("push {}", value));
            }
            ASTNode::Identifier(name) => {
                ir.push(format!("load {}", name));
            }
            _ => {}
        }
    }

    generate_node_ir(ast, &mut ir);
    ir
}

// Helper function to generate target code (e.g., x86 assembly)
fn generate_target_code(ir: &[String]) -> Vec<String> {
    let mut asm = Vec::new();
    
    for instruction in ir {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        match parts[0] {
            "function" => {
                asm.push(format!("{}:", parts[1].trim_end_matches(':')));
                asm.push("    push rbp".to_string());
                asm.push("    mov rbp, rsp".to_string());
            }
            "end_function" => {
                asm.push("    mov rsp, rbp".to_string());
                asm.push("    pop rbp".to_string());
                asm.push("    ret".to_string());
            }
            "param" => {
                // Handle parameter passing
            }
            "push" => {
                asm.push(format!("    push {}", parts[1]));
            }
            "load" => {
                asm.push(format!("    mov rax, [{}]", parts[1]));
                asm.push("    push rax".to_string());
            }
            "store" => {
                asm.push("    pop rax".to_string());
                asm.push(format!("    mov [{}], rax", parts[1]));
            }
            "+" | "-" | "*" | "/" => {
                asm.push("    pop rbx".to_string());
                asm.push("    pop rax".to_string());
                match parts[0] {
                    "+" => asm.push("    add rax, rbx".to_string()),
                    "-" => asm.push("    sub rax, rbx".to_string()),
                    "*" => asm.push("    imul rax, rbx".to_string()),
                    "/" => {
                        asm.push("    xor rdx, rdx".to_string());
                        asm.push("    idiv rbx".to_string());
                    }
                    _ => {}
                }
                asm.push("    push rax".to_string());
            }
            _ => {
                // Handle other instructions
            }
        }
    }

    asm
}