use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;

/*
Part 1: the types, nodes, etc
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind{
    //keywords
    Int,
    If,
    Else,
    While,
    Then,
    True,
    False,

    //complete the code
    Args,
    Return,
    Identifier,
    Number,
    Eof,

    //operations
    Plus,
    Minus,
    Star,
    AssignOp,
    Semicolon,
    Comma,
    LParen,
    RParen,
    LCurly,
    RCurly,
    Leq,
    Lt,
    Geq,
    Gt,
    Eq,
}

//nodes to build the st
#[derive(Debug, Clone)]
pub struct Token{
    pub kind: TokenKind,
    pub literal_value: String,
}

//to hold 4 parts of our program
#[derive(Debug)]
pub struct Program{
    pub args: Vec<String>,
    pub vars: Vec<String>,
    pub stmts: Vec<Stmt>,
    pub ret_var: String,
}

//to handle if,while
#[derive(Debug)]
pub enum Stmt {
    Assign { name: String, expr: Expr },
    If {
        cond: BoolExpr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    While {
        cond: BoolExpr,
        body: Vec<Stmt>,
    },
}

//basic stuff
#[derive(Debug)]
pub enum Expr{
    Var(String),
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

//for bools
#[derive(Debug)]
pub enum RelOp{
    Le,
    Lt,
    Ge,
    Gt,
    Eq,
}

#[derive(Debug)]
pub enum BoolExpr{
    True,
    False,
    Cmp {
        op: RelOp,
        left: Expr,
        right: Expr,
    },
}

//error handle
#[derive(Debug)]
pub struct ParseError{
    pub message: String,
}

impl fmt::Display for ParseError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "ParseError: {}", self.message)
    }
}

/*
Part 2: Lex
*/

pub fn lex_program(input: &str) -> Result<Vec<Token>, String>{
    use TokenKind::*; //dont need to use tokenkind everytime

    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable(); //for .next.print

    while let Some(&ch) = chars.peek(){
        if ch.is_whitespace(){
            chars.next();
            continue;
        }

        // Identifiers / keywords
        if ch.is_ascii_alphabetic(){
            let mut ident = String::new();
            while let Some(&c) = chars.peek(){
                if c.is_ascii_alphabetic(){
                    ident.push(c);
                    chars.next();
                } 
                else{
                    break;
                }
            }
            //match char
            let kind = match ident.as_str(){
                "args" => Args,
                "int" => Int,
                "if" => If,
                "else" => Else,
                "while" => While,
                "then" => Then,
                "return" => Return,
                "true" => True,
                "false" => False,
                _ => Identifier,
            };
            tokens.push(Token{
                kind,
                literal_value: ident,
            });
            continue;
        }

        //for int
        if ch.is_ascii_digit(){
            let mut num = String::new();
            while let Some(&c) = chars.peek(){
                if c.is_ascii_digit(){
                    num.push(c);
                    chars.next();
                } 
                else{
                    break;
                }
            }
            tokens.push(Token{
                kind: Number,
                literal_value: num,
            });
            continue;
        }

        // Operators and punctuation
        match ch{
            '+' =>{
                chars.next();
                tokens.push(Token{kind: Plus,literal_value: "+".into()});
            }
            '-' =>{
                chars.next();
                tokens.push(Token{kind: Minus, literal_value: "-".into()});
            }
            '*' =>{
                chars.next();
                tokens.push(Token{kind: Star, literal_value: "*".into()});
            }
            ';' => {
                chars.next();
                tokens.push(Token{kind: Semicolon, literal_value: ";".into()});
            }
            ',' =>{
                chars.next();
                tokens.push(Token{kind: Comma, literal_value: ",".into()});
            }
            '(' =>{
                chars.next();
                tokens.push(Token{kind: LParen, literal_value: "(".into()});
            }
            ')' =>{
                chars.next();
                tokens.push(Token{kind: RParen, literal_value: ")".into()});
            }
            '{' =>{
                chars.next();
                tokens.push(Token{kind: LCurly, literal_value: "{".into()});
            }
            '}' =>{
                chars.next();
                tokens.push(Token{kind: RCurly, literal_value: "}".into()});
            }
            //handle twos
            '=' =>{
                chars.next();
                if let Some(&'=') = chars.peek(){
                    chars.next();
                    tokens.push(Token{kind: Eq, literal_value: "==".into()});
                } 
                else{
                    tokens.push(Token{kind: AssignOp, literal_value: "=".into()});
                }
            }
            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek(){
                    chars.next();
                    tokens.push(Token{kind: Leq, literal_value: "<=".into()});
                } 
                else{
                    tokens.push(Token{kind: Lt, literal_value: "<".into()});
                }
            }
            '>' =>{
                chars.next();
                if let Some(&'=') = chars.peek(){
                    chars.next();
                    tokens.push(Token{kind: Geq, literal_value: ">=".into()});
                } 
                else{
                    tokens.push(Token{kind: Gt,literal_value: ">".into()});
                }
            }
            //error not gonna happen
            _ => {
                return Err(format!("Unknown character: '{}'", ch));
            }
        }
    }

    tokens.push(Token {kind: TokenKind::Eof,literal_value: "".into()});
    Ok(tokens)
}

/*
Part 3: Parse
*/
pub struct Parser{
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser{
    //first helpers
    pub fn new(tokens: Vec<Token>) -> Self{
        Parser {tokens, pos: 0}
    }

    fn current_token(&self) -> &Token{
        &self.tokens[self.pos.min(self.tokens.len() - 1)] //doesn't go out of range
    }

    //making peek
    fn peek(&self) -> TokenKind{
        self.current_token().kind
    }

    fn advance(&mut self) -> TokenKind{
        let kind = self.peek();
        if self.pos < self.tokens.len(){
            self.pos += 1;
        }
        kind
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError>{
        let kind = self.peek();
        if kind == expected{
            self.advance();
            Ok(())
        } 
        else{
            Err(ParseError{
                message: format!("want {:?}, got {:?}", expected, kind),
            })
        }
    }

    //take string and num as whole
    fn consume_ident(&mut self) -> Result<String, ParseError>{
        match self.peek(){
            TokenKind::Identifier => {
                let name = self.current_token().literal_value.clone();
                self.advance();
                Ok(name)
            }
            other => Err(ParseError{
                message: format!("Expected IDENTIFIER, found {:?}", other),
            }),
        }
    }

    fn consume_number(&mut self) -> Result<i64, ParseError>{
        match self.peek() {
            TokenKind::Number =>{
                let txt = self.current_token().literal_value.clone();
                self.advance();
                //converting to i64
                txt.parse::<i64>().map_err(|e| ParseError{
                    message: format!("Invalid number '{}': {}", txt, e),
                })
            }
            other => Err(ParseError{
                message: format!("Expected num, found {:?}", other),
            }),
        }
    }

    //parser starts form top-level
    // PROG → ARGDECL TYPEDECL STMTS RET
    pub fn parse_prog(&mut self) -> Result<Program, ParseError>{
        //divide into 4
        let args = self.parse_argdecl()?;
        let vars = self.parse_typedecl()?;
        let stmts = self.parse_stmts()?;
        let ret_var = self.parse_ret()?;
        //end on EOF
        self.expect(TokenKind::Eof)?;
        Ok(Program{
            args,
            vars,
            stmts,
            ret_var,
        })
    }

    // RET → return IDENTIFIER ;
    fn parse_ret(&mut self) -> Result<String, ParseError>{
        //end with ;
        self.expect(TokenKind::Return)?;
        let name = self.consume_ident()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(name)
    }

    // ARGDECL → args IDENTIFIER ARGDECLTAIL
    // ARGDECLTAIL → ; | , IDENTIFIER ARGDECLTAIL
    fn parse_argdecl(&mut self) -> Result<Vec<String>, ParseError>{
        self.expect(TokenKind::Args)?;
        let mut args = Vec::new();
        let first = self.consume_ident()?;
        args.push(first);

        loop{
            match self.peek(){
                TokenKind::Comma =>{
                    self.advance();
                    let name = self.consume_ident()?;
                    args.push(name);
                }
                TokenKind::Semicolon =>{
                    self.advance();
                    break;
                }
                other =>{
                    return Err(ParseError {message: format!("wrong format, found {:?}", other)});
                }
            }
        }
        Ok(args)
    }

    // TYPEDECL → int IDENTIFIER TYPEDECLTAIL
    // TYPEDECLTAIL → ; | , IDENTIFIER TYPEDECLTAIL
    fn parse_typedecl(&mut self) -> Result<Vec<String>, ParseError>{
        self.expect(TokenKind::Int)?;
        let mut vars = Vec::new();
        let first = self.consume_ident()?;
        vars.push(first);

        loop{
            match self.peek(){
                TokenKind::Comma =>{
                    self.advance();
                    let name = self.consume_ident()?;
                    vars.push(name);
                }
                TokenKind::Semicolon =>{
                    self.advance();
                    break;
                }
                other =>{
                    return Err(ParseError {message: format!("wrong statement, found {:?}", other)});
                }
            }
        }
        Ok(vars)
    }
    //got args and vars

    // STMTS → STMT STMTS | EPSILON
    fn parse_stmts(&mut self) -> Result<Vec<Stmt>, ParseError>{
        let mut stmts = Vec::new();
        loop{
            match self.peek(){
                TokenKind::Identifier | TokenKind::If | TokenKind::While =>{
                    let s = self.parse_stmt()?; //not simple then run the complex
                    stmts.push(s);
                }
                //or else ending in } or with return statement
                TokenKind::Return | TokenKind::RCurly => break,
                other =>{
                    return Err(ParseError{message: format!("unexpected symbol {:?}", other)});
                }
            }
        }
        Ok(stmts)
    }

    // STMT → ASSIGN | IFTHENELSE | WHILE
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError>{
        match self.peek(){
            TokenKind::Identifier => self.parse_assign(),
            TokenKind::If => self.parse_ifthenelse(),
            TokenKind::While => self.parse_while(),
            //safety net
            other => Err(ParseError{message: format!("In STMT, expected IDENTIFIER | if | while, found {:?}", other)}),
        }
    }

    // ASSIGN → IDENTIFIER '=' EXPR ';'
    fn parse_assign(&mut self) -> Result<Stmt, ParseError>{
        let name = self.consume_ident()?;
        self.expect(TokenKind::AssignOp)?;
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Assign { name, expr })
    }

    // IFTHENELSE → if BOOL then { STMTS } else { STMTS }
    fn parse_ifthenelse(&mut self) -> Result<Stmt, ParseError>{
        //handle 3: cond, then, else
        self.expect(TokenKind::If)?;
        let cond = self.parse_bool()?;
        self.expect(TokenKind::Then)?;
        self.expect(TokenKind::LCurly)?;
        let then_body = self.parse_stmts()?;
        self.expect(TokenKind::RCurly)?;
        self.expect(TokenKind::Else)?;
        self.expect(TokenKind::LCurly)?;
        let else_body = self.parse_stmts()?;
        self.expect(TokenKind::RCurly)?;
        Ok(Stmt::If {
            cond,
            then_body,
            else_body,
        })
    }

    // WHILE → while BOOL then { STMTS }
    fn parse_while(&mut self) -> Result<Stmt, ParseError>{
        //handle 2
        self.expect(TokenKind::While)?;
        let cond = self.parse_bool()?;
        self.expect(TokenKind::Then)?;
        self.expect(TokenKind::LCurly)?;
        let body = self.parse_stmts()?;
        self.expect(TokenKind::RCurly)?;
        Ok(Stmt::While { cond, body })
    }

    // BOOL-> true | false | EXPR <= EXPR | EXPR < EXPR | EXPR >= EXPR | EXPR > EXPR | EXPR == EXPR
    fn parse_bool(&mut self) -> Result<BoolExpr, ParseError>{
        match self.peek(){
            TokenKind::True =>{
                self.advance();
                Ok(BoolExpr::True)
            }
            TokenKind::False =>{
                self.advance();
                Ok(BoolExpr::False)
            }
            //handle ( too
            TokenKind::Identifier | TokenKind::Number | TokenKind::LParen =>{
                let left = self.parse_expr()?;
                let op = match self.peek(){
                    TokenKind::Leq =>{
                        self.advance();
                        RelOp::Le
                    }
                    TokenKind::Lt =>{
                        self.advance();
                        RelOp::Lt
                    }
                    TokenKind::Geq =>{
                        self.advance();
                        RelOp::Ge
                    }
                    TokenKind::Gt =>{
                        self.advance();
                        RelOp::Gt
                    }
                    //can't be =
                    TokenKind::Eq =>{
                        self.advance();
                        RelOp::Eq
                    }
                    other =>{
                        return Err(ParseError{message: format!("wrong relop, found {:?}", other)})
                    }
                };
                let right = self.parse_expr()?;
                Ok(BoolExpr::Cmp { op, left, right })
            }
            //safety, can't happen though
            other => Err(ParseError {message: format!("wrong format {:?}", other)}),
        }
    }

    //back to basic
    //EXPR → TERM EXPR'
    fn parse_expr(&mut self) -> Result<Expr, ParseError>{
        let node = self.parse_term()?;
        self.parse_exprdash(node)
    }

    //EXPR' → + TERM EXPR' | - TERM EXPR' | eps
    fn parse_exprdash(&mut self, left: Expr) -> Result<Expr, ParseError>{
        match self.peek(){
            TokenKind::Plus =>{
                self.advance();
                let rhs = self.parse_term()?;
                let node = Expr::Add(Box::new(left), Box::new(rhs));
                self.parse_exprdash(node)
            }
            TokenKind::Minus =>{
                self.advance();
                let rhs = self.parse_term()?;
                let node = Expr::Sub(Box::new(left), Box::new(rhs));
                self.parse_exprdash(node)
            }
            _ => Ok(left),
        }
    }

    //TERM → FACTOR TERM'
    fn parse_term(&mut self) -> Result<Expr, ParseError>{
        let node = self.parse_factor()?;
        self.parse_termdash(node)
    }

    //TERM' → * FACTOR TERM' | eps
    fn parse_termdash(&mut self, left: Expr) -> Result<Expr, ParseError>{
        match self.peek(){
            TokenKind::Star =>{
                self.advance();
                let rhs = self.parse_factor()?;
                let node = Expr::Mul(Box::new(left), Box::new(rhs));
                self.parse_termdash(node)
            }
            _ => Ok(left),
        }
    }

    // FACTOR → IDENTIFIER | NUMBER | ( EXPR )
    fn parse_factor(&mut self) -> Result<Expr, ParseError>{
        match self.peek(){
            TokenKind::Identifier =>{
                let name = self.consume_ident()?;
                Ok(Expr::Var(name))
            }
            TokenKind::Number =>{
                let n = self.consume_number()?;
                Ok(Expr::Number(n))
            }
            TokenKind::LParen =>{
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            other => Err(ParseError{message: format!("wrong factor, found {:?}", other)}),
        }
    }
}

//Last Part: writing the IR
struct CodeGeneration {
    lines: Vec<String>,
    temp_counter: usize,
    if_counter: usize,
    while_counter: usize,
    allocas: HashMap<String, String>, //to get %name.alloca
}

impl CodeGeneration {
    fn new() -> Self {
        //init all to base values
        CodeGeneration{
            lines: Vec::new(),
            temp_counter: 1,
            if_counter: 0,
            while_counter: 0,
            allocas: HashMap::new(),
        }
    }

    //to push everything easily
    fn emit<S: Into<String>>(&mut self, s: S){
        self.lines.push(s.into());
    }

    //getting the labels according to the count
    fn fresh_temp(&mut self) -> String{
        let name = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn if_label_new(&mut self) -> (String, String, String){
        let idx = self.if_counter;
        self.if_counter += 1;
        if idx == 0{
            (format!("if.then"), format!("if.else"), format!("if.end"))
        } 
        else{
            (
                format!("if.then{}", idx),
                format!("if.else{}", idx),
                format!("if.end{}", idx),
            )
        }
    }

    fn while_label_new(&mut self) -> (String, String, String){
        let idx = self.while_counter;
        self.while_counter += 1;
        if idx == 0{
            (
                format!("while.cond"),
                format!("while.body"),
                format!("while.end"),
            )
        } 
        else{
            (
                format!("while.cond{}", idx),
                format!("while.body{}", idx),
                format!("while.end{}", idx),
            )
        }
    }

    //fill hash_map of allocas
    fn alloca_for_var(&mut self, name: &str) -> String{
        if let Some(a) = self.allocas.get(name){
            return a.clone();
        }
        let alloc_name = format!("%{}.alloc", name);
        self.allocas.insert(name.to_string(), alloc_name.clone());
        alloc_name
    }

    //expressions
    fn generate_expr(&mut self, expr: &Expr) -> String{
        match expr {
            //load
            Expr::Number(n) => n.to_string(),
            Expr::Var(name) => {
                let alloca = self.alloca_for_var(name);
                let tmp = self.fresh_temp();
                self.emit(format!("  {} = load i64, ptr {}", tmp, alloca));
                tmp
            }
            //add
            Expr::Add(a, b) =>{
                let lhs = self.generate_expr(a);
                let rhs = self.generate_expr(b);
                let tmp = self.fresh_temp();
                self.emit(format!("  {} = add i64 {}, {}", tmp, lhs, rhs));
                tmp
            }
            //sub, smae as add but change to sub
            Expr::Sub(a, b) =>{
                let lhs = self.generate_expr(a);
                let rhs = self.generate_expr(b);
                let tmp = self.fresh_temp();
                self.emit(format!("  {} = sub i64 {}, {}", tmp, lhs, rhs));
                tmp
            }
            //mul, also the same
            Expr::Mul(a, b) =>{
                let lhs = self.generate_expr(a);
                let rhs = self.generate_expr(b);
                let tmp = self.fresh_temp();
                self.emit(format!("  {} = mul i64 {}, {}", tmp, lhs, rhs));
                tmp
            }
        }
    }

    //bool expr
    fn gen_bool(&mut self, b: &BoolExpr) -> String{
        match b{
            BoolExpr::True => "true".to_string(),
            BoolExpr::False => "false".to_string(),
            BoolExpr::Cmp { op, left, right } => {
                let lhs = self.generate_expr(left);
                let rhs = self.generate_expr(right);
                let tmp = self.fresh_temp();
                let cond_str = match op{
                    RelOp::Le => "ule",
                    RelOp::Lt => "ult",
                    RelOp::Ge => "uge",
                    RelOp::Gt => "ugt",
                    RelOp::Eq => "eq",
                };
                self.emit(format!("  {} = icmp {} i64 {}, {}",tmp, cond_str, lhs, rhs));
                tmp
            }
        }
    }

    //statement using exprs
    fn gen_stmt(&mut self, stmt: &Stmt){
        //match which statement
        match stmt{
            Stmt::Assign { name, expr } =>{
                let alloca = self.alloca_for_var(name);
                let val = self.generate_expr(expr);
                self.emit(format!("  store i64 {}, ptr {}", val, alloca));
            }
            Stmt::If{
                cond,
                then_body,
                else_body,
            } => {
                let cond_val = self.gen_bool(cond);
                let (l_then, l_else, l_end) = self.if_label_new();

                self.emit(format!("  br i1 {}, label %{}, label %{}", cond_val, l_then, l_else));

                //then
                self.emit(String::new());
                self.emit(format!("{}:", l_then));
                for s in then_body{
                    self.gen_stmt(s);
                }
                self.emit(format!("  br label %{}", l_end));

                //else
                self.emit(String::new());
                self.emit(format!("{}:", l_else));
                for s in else_body{
                    self.gen_stmt(s);
                }
                self.emit(format!("  br label %{}", l_end));

                // end
                self.emit(String::new());
                self.emit(format!("{}:", l_end));
            }
            Stmt::While { cond, body } =>{
                let (l_cond, l_body, l_end) = self.while_label_new();

                //jump to while.cond
                self.emit(format!("  br label %{}", l_cond));

                //cond
                self.emit(String::new());
                self.emit(format!("{}:", l_cond));
                let cond_val = self.gen_bool(cond);
                self.emit(format!("  br i1 {}, label %{}, label %{}", cond_val, l_body, l_end));

                //body
                self.emit(String::new());
                self.emit(format!("{}:", l_body));
                for s in body{
                    self.gen_stmt(s);
                }
                self.emit(format!("  br label %{}", l_cond));

                //end
                self.emit(String::new());
                self.emit(format!("{}:", l_end));
            }
        }
    }

    //final, the top level
    fn gen_program(&mut self, prog: &Program){
        let params: Vec<String> = prog.args.iter().map(|a| format!("i64 %{}", a)).collect();
        let param_list = params.join(", ");

        //entry point
        self.emit(format!("define i64 @foo({}) {{", param_list));
        self.emit("entry:");

        //all args + vars
        for name in prog.args.iter().chain(prog.vars.iter()){
            let alloc_name = self.alloca_for_var(name);
            self.emit(format!("  {} = alloca i64", alloc_name));
        }

        //store args in allocas
        for arg in &prog.args{
            let alloc_name = self.alloca_for_var(arg);
            self.emit(format!("  store i64 %{}, ptr {}", arg, alloc_name));
        }

        //gen statement
        for stmt in &prog.stmts{
            self.gen_stmt(stmt);
        }

        //return
        let ret_alloc = self.alloca_for_var(&prog.ret_var);
        let tmp = self.fresh_temp();
        self.emit(format!("  {} = load i64, ptr {}", tmp, ret_alloc));
        self.emit(format!("  ret i64 {}", tmp));
        self.emit("}");
    }
}

pub fn generate_llvm_ir(prog: &Program) -> String{
    let mut temp_data = CodeGeneration::new();
    temp_data.gen_program(prog);
    temp_data.lines.join("\n")
}


struct X86Gen {
    lines: Vec<String>,
    var_offsets: HashMap<String, i32>,
    arg_regs: HashMap<String, String>,
    stack_size: i32,
    if_counter: usize,
    while_counter: usize,
}

impl X86Gen{
    fn new(prog: &Program) -> Self{
        let mut var_offsets = HashMap::new();
        let mut arg_regs = HashMap::new();

        //arguments to register
        let regs = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
        for (i, a) in prog.args.iter().enumerate(){
            if i >= regs.len(){
                panic!("More than 6 args not supported");
            }
            arg_regs.insert(a.clone(), regs[i].to_string());
        }

        //temp in stack
        let total_slots = prog.vars.len() as i32;
        let stack_size = 8 * total_slots;

        let mut offset = -8;
        for name in &prog.vars{
            var_offsets.insert(name.clone(), offset);
            offset -= 8;
        }

        X86Gen{
            lines: Vec::new(),
            var_offsets,
            arg_regs,
            stack_size,
            if_counter: 0,
            while_counter: 0,
        }
    }

    fn emit<S: Into<String>>(&mut self, s: S){
        self.lines.push(s.into());
    }

    fn offset(&self, name: &str) -> i32{
        *self.var_offsets.get(name).unwrap_or_else(|| panic!("Unknown local variable `{}` in offset()", name))
    }

    fn fresh_if_labels(&mut self) -> (String, String, String){
        let id = self.if_counter;
        self.if_counter += 1;
        if id == 0{
            ("ifthen".into(), "ifelse".into(), "ifend".into())
        } 
        else{
            (
                format!("ifthen{}", id),
                format!("ifelse{}", id),
                format!("ifend{}", id),
            )
        }
    }

    fn fresh_while_labels(&mut self) -> (String, String, String){
        let id = self.while_counter;
        self.while_counter += 1;
        if id == 0{
            ("whilecond".into(), "whilebody".into(), "whileend".into())
        } 
        else{
            (
                format!("whilecond{}", id),
                format!("whilebody{}", id),
                format!("whileend{}", id),
            )
        }
    }

    fn gen_program(mut self, prog: &Program) -> String{
        self.emit(".text");
        self.emit("  .global foo");
        self.emit("");
        self.emit("foo:");
        self.emit("  pushq %rbp");
        self.emit("  movq %rsp, %rbp");
        self.emit("  pushq %rbx");

        if self.stack_size > 0{
            self.emit(format!("  subq ${}, %rsp", self.stack_size));
        }

        // stmts
        for stmt in &prog.stmts{
            self.gen_stmt(stmt);
        }

        //return var %rax
        let ret_name = &prog.ret_var;
        if let Some(reg) = self.arg_regs.get(ret_name){
            self.emit(format!("  movq {}, %rax", reg));
        } 
        else{
            let ret_off = self.offset(ret_name);
            self.emit(format!("  movq {}(%rbp), %rax", ret_off));
        }

        if self.stack_size > 0{
            self.emit(format!("  addq ${}, %rsp", self.stack_size));
        }
        self.emit("  popq %rbx");
        self.emit("  popq %rbp");
        self.emit("  ret");
        self.emit("");
        self.emit(".section .note.GNU-stack,\"\",@progbits");

        self.lines.join("\n")
    }

    fn gen_stmt(&mut self, stmt: &Stmt){
        match stmt{
            Stmt::Assign { name, expr } =>{
                self.gen_expr(expr);
                if let Some(reg) = self.arg_regs.get(name){
                    self.emit(format!("  movq %rax, {}", reg));
                } 
                else{
                    let off = self.offset(name);
                    self.emit(format!("  movq %rax, {}(%rbp)", off));
                }
            }
            Stmt::If{
                cond,
                then_body,
                else_body,
            } =>{
                let (l_then, l_else, l_end) = self.fresh_if_labels();
                self.gen_bool_jump(cond, &l_then, &l_else);

                self.emit("");
                self.emit(format!("{}:", l_then));
                for s in then_body {
                    self.gen_stmt(s);
                }
                self.emit(format!("  jmp {}", l_end));

                self.emit("");
                self.emit(format!("{}:", l_else));
                for s in else_body {
                    self.gen_stmt(s);
                }
                self.emit(format!("  jmp {}", l_end));

                self.emit("");
                self.emit(format!("{}:", l_end));
            }
            Stmt::While { cond, body } =>{
                let (l_cond, l_body, l_end) = self.fresh_while_labels();

                self.emit(format!("  jmp {}", l_cond));

                self.emit("");
                self.emit(format!("{}:", l_cond));
                self.gen_bool_jump(cond, &l_body, &l_end);

                self.emit("");
                self.emit(format!("{}:", l_body));
                for s in body {
                    self.gen_stmt(s);
                }
                self.emit(format!("  jmp {}", l_cond));

                self.emit("");
                self.emit(format!("{}:", l_end));
            }
        }
    }

    // expressions: result in %rax
    fn gen_expr(&mut self, expr: &Expr){
        match expr{
            Expr::Number(n) =>{
                self.emit(format!("  movq ${}, %rax", n));
            }
            Expr::Var(name) =>{
                // If it's an argument, read from its register; otherwise from stack.
                if let Some(reg) = self.arg_regs.get(name){
                    self.emit(format!("  movq {}, %rax", reg));
                } 
                else{
                    let off = self.offset(name);
                    self.emit(format!("  movq {}(%rbp), %rax", off));
                }
            }
            Expr::Add(a, b) =>{
                self.gen_expr(a);
                self.emit("  pushq %rax");
                self.gen_expr(b);
                self.emit("  popq %rbx");
                self.emit("  addq %rbx, %rax");
            }
            Expr::Sub(a, b) =>{
                self.gen_expr(a);
                self.emit("  pushq %rax");
                self.gen_expr(b);
                self.emit("  movq %rax, %rbx"); // right
                self.emit("  popq %rax");       // left
                self.emit("  subq %rbx, %rax"); // left - right
            }
            Expr::Mul(a, b) =>{
                self.gen_expr(a);
                self.emit("  pushq %rax");
                self.gen_expr(b);
                self.emit("  popq %rbx");
                self.emit("  imulq %rbx, %rax");
            }
        }
    }

    // BoolExpr to conditional jumps
    fn gen_bool_jump(&mut self, b: &BoolExpr, label_true: &str, label_false: &str){
        match b{
            BoolExpr::True =>{
                self.emit(format!("  jmp {}", label_true));
            }
            BoolExpr::False =>{
                self.emit(format!("  jmp {}", label_false));
            }
            BoolExpr::Cmp { op, left, right } =>{
                self.gen_expr(left);
                self.emit("  pushq %rax");
                self.gen_expr(right);
                self.emit("  movq %rax, %rbx");
                self.emit("  popq %rax");
                self.emit("  cmpq %rbx, %rax");

                let jmp = match op{
                    RelOp::Lt => "jb",
                    RelOp::Le => "jbe",
                    RelOp::Gt => "ja",
                    RelOp::Ge => "jae",
                    RelOp::Eq => "je",
                };

                self.emit(format!("  {} {}", jmp, label_true));
                self.emit(format!("  jmp {}", label_false));
            }
        }
    }
}

fn generate_x86_from_ast(prog: &Program) -> String{
    let gene = X86Gen::new(prog);
    gene.gen_program(prog)
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let src = match fs::read_to_string(filename){
        Ok(s) => s,
        Err(e) =>{
            eprintln!("Error reading {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    //lex
    let tokens = match lex_program(&src){
        Ok(ts) => ts,
        Err(e) => {
            eprintln!("Lex error: {}", e);
            std::process::exit(1);
        }
    };

    //parse
    let mut parser = Parser::new(tokens);
    let prog = match parser.parse_prog(){
        Ok(prog) => prog,
        Err(e) =>{
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    //generate IR
    /*
    let ir = generate_llvm_ir(&prog);
    let mut ir_path = std::path::PathBuf::from(filename);
    ir_path.set_extension("ll");
    if let Err(e) = fs::write(&ir_path, &ir) {
        eprintln!("Error writing {}: {}", ir_path.display(), e);
        std::process::exit(1);
    }
    */

    //generate x86-64 assembly
    let asm = generate_x86_from_ast(&prog);
    let mut asm_path = std::path::PathBuf::from(filename);
    let stem = asm_path.file_stem().and_then(|s| s.to_str()).unwrap_or("out");

    let new_name = format!("{}_2.s", stem);
    asm_path.set_file_name(new_name);
    if let Err(e) = fs::write(&asm_path, &asm){
        eprintln!("Error writing {}: {}", asm_path.display(), e);
        std::process::exit(1);
    }
}

