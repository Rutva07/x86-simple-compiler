use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

//types 
type Grammar = HashMap<&'static str, Vec<Vec<&'static str>>>;
type FirstMap = HashMap<&'static str, BTreeSet<&'static str>>;
type FollowMap = HashMap<&'static str, BTreeSet<&'static str>>;

#[derive(Debug)]
struct Node { label: String, children: Vec<usize> }

#[derive(Clone, Debug)]
struct Token {
    kind: String,
    value: String,
}

//some constants to access the LL table
const EPS: &str = "EPSILON";
const EXPR: usize = 0;
const EXPRDASH: usize = 1;
const TERM: usize = 2;
const TERMDASH: usize = 3;
const FACTOR: usize = 4;
const IDENTIFIER: usize = 0;
const NUMBER: usize = 1;
const PLUS: usize = 2; 
const STAR: usize = 3; 
const BOPEN: usize = 4; 
const BCLOSE: usize = 5; 
const DOLLAR: usize = 6;
const ROWS: usize = 5;
const COLS: usize = 7;


//check if the symbol is non-terminal
fn check_nonterminal(sym: &str, g: &Grammar) -> bool { g.contains_key(sym)}

//First function to construct LL table, don't care about terminals 
fn compute_first(g: &Grammar) -> FirstMap{
    let mut first: FirstMap = g.keys().map(|&nt| (nt, BTreeSet::new())).collect();
    let mut changed = true;
    while changed {
        changed = false;
        for (&a, prods) in g {
            for rule in prods {
                //Handle epsilon
                if rule.len() == 1 && rule[0] == EPS {
                    if first.get_mut(a).unwrap().insert(EPS) {changed = true;}
                    continue;
                }
                //Main logic
                let mut all_nullable = true;
                for &x in rule {
                    if check_nonterminal(x, g) {
                        let (nullable_x, to_add) = {
                            let fx = &first[x];
                            (fx.contains(EPS), fx.iter().copied().filter(|&t| t != EPS).collect::<Vec<_>>())
                        };
                        for t in to_add {
                            if first.get_mut(a).unwrap().insert(t) { changed = true; }
                        }
                        if !nullable_x { 
                            all_nullable = false; 
                            break;
                        }
                    } else {
                        // terminal
                        if first.get_mut(a).unwrap().insert(x) { changed = true; }
                        all_nullable = false;
                        break;
                    }
                }
                if all_nullable {
                    if first.get_mut(a).unwrap().insert(EPS) { changed = true; }
                }
            }
        }
    }
    first
}

//Follow-helper function
fn first_of_seq<'a>(seq: &[&'a str], g: &Grammar, firsts: &FirstMap) -> (BTreeSet<&'a str>, bool) {
    let mut out: BTreeSet<&str> = BTreeSet::new();
    if seq.is_empty() { return (out, true); }
    for &y in seq {
        if check_nonterminal(y, g) {
            let fy = &firsts[y];
            for &t in fy {
                if t != EPS { out.insert(t); }
            }
            //no eps then dont go further
            if !fy.contains(EPS) { return (out, false); }
        } else {
            out.insert(y);
            return (out, false);
        }
    }
    (out, true)
}

//Follow function to construct LL Table
fn compute_follow(g: &Grammar, firsts: &FirstMap, start: &str) -> FollowMap {
    let mut follow: FollowMap = g.keys().map(|&nt| (nt, BTreeSet::new())).collect();
    //Rule 1
    follow.get_mut(start).unwrap().insert("$");

    let mut changed = true;
    while changed {
        changed = false;
        for (&a, prods) in g {
            let follow_a = follow[a].clone();
            for prod in prods {
                for i in 0..prod.len() {
                    let b = prod[i];
                    if !check_nonterminal(b, g) { continue; }

                    let beta = &prod[i + 1..];
                    //Rule 2
                    let (first_beta, beta_nullable) = first_of_seq(beta, g, firsts);
                    for t in &first_beta {
                        if follow.get_mut(b).unwrap().insert(t) { changed = true; }
                    }
                    //Rule 3
                    if beta_nullable {
                        for t in &follow_a {
                            if follow.get_mut(b).unwrap().insert(t) { changed = true; }
                        }
                    }
                }
            }
        }
    }
    follow
}

//Maping for col-index
fn term_index(col: &str) -> Option<usize> {
    match col {
        "IDENTIFIER" => Some(IDENTIFIER),
        "NUMBER"     => Some(NUMBER),
        "PLUS"       => Some(PLUS),
        "STAR"       => Some(STAR),
        "BOPEN"      => Some(BOPEN),
        "BCLOSE"     => Some(BCLOSE),
        "$"          => Some(DOLLAR),
        _ => None,
    }
}
//row index
fn nt_index(row: &str) -> Option<usize> {
    match row {
        "EXPR"     => Some(EXPR),
        "EXPRDASH" => Some(EXPRDASH),
        "TERM"     => Some(TERM),
        "TERMDASH" => Some(TERMDASH),
        "FACTOR"   => Some(FACTOR),
        _ => None,
    }
}

//Helper to build LL table
fn helper<'a>(seq: &[&'a str], g: &Grammar, firsts: &FirstMap) -> (BTreeSet<&'a str>, bool) {
    let mut out: BTreeSet<&str> = BTreeSet::new();
    if seq.is_empty() { return (out, true); }

    let mut all_nullable = true;
    for &y in seq {
        if y == EPS {
            continue;
        }
        if check_nonterminal(y, g) {
            let z = &firsts[y];
            for &t in z {
                if t != EPS { out.insert(t); }
            }
            if !z.contains(EPS) {
                all_nullable = false;
                break;
            }
        } else {
            // terminal
            out.insert(y);
            all_nullable = false;
            break;
        }
    }
    (out, all_nullable)
}

//Build LL table using follows and firsts
fn build_table(grammar: &Grammar, firsts: &FirstMap, follows: &FollowMap) -> [[Option<String>; COLS]; ROWS] {
    let mut table: [[Option<String>; COLS]; ROWS] =
        std::array::from_fn(|_| std::array::from_fn(|_| None));

    for (&a, prods) in grammar {
        let ai = nt_index(a).expect("unknown nonterminal");
        for prod in prods {
            let (falpha, nullable) = helper(prod, grammar, firsts);

            //Fill it up with firsts
            for &t in falpha.iter() {
                if let Some(ci) = term_index(t) {
                    let x = if prod.len() == 1 && prod[0] == EPS {
                        "EPSILON".to_string()
                    } else {
                        prod.join(" ")
                    };
                    if table[ai][ci].is_none() {
                        table[ai][ci] = Some(x);
                    }
                }
            }

            //follows
            if nullable {
                for &b in follows[a].iter() {
                    if let Some(ci) = term_index(b) {
                        let x = if prod.len() == 1 && prod[0] == EPS {
                            "EPSILON".to_string()
                        } else {
                            prod.join(" ")
                        };
                        if table[ai][ci].is_none() {
                            table[ai][ci] = Some(x);
                        }
                    }
                }
            }
        }
    }

    table
}

//Read the line into a vector 
fn scan_expr(line: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let characters: Vec<char> = line.chars().collect();
    let mut i = 0usize;

    while i < characters.len() {
        let c = characters[i];
        if c.is_whitespace() {
            i += 1; continue; 
        }

        match c {
            '+' => { tokens.push(Token { kind: "PLUS".into(), value: "+".into() }); i += 1; }
            '*' => { tokens.push(Token { kind: "STAR".into(), value: "*".into() }); i += 1; }
            '(' => { tokens.push(Token { kind: "BOPEN".into(), value: "(".into() }); i += 1; }
            ')' => { tokens.push(Token { kind: "BCLOSE".into(), value: ")".into() }); i += 1; }

            _ if c.is_ascii_alphabetic() => {
                let mut j = i + 1;
                while j < characters.len() && characters[j].is_ascii_alphabetic() { j += 1; }
                let word: String = characters[i..j].iter().collect();
                tokens.push(Token { kind: "IDENTIFIER".into(), value: word });
                i = j;
            }

            _ if c.is_ascii_digit() => {
                let mut j = i + 1;
                while j < characters.len() && characters[j].is_ascii_digit() { j += 1; }
                let num: String = characters[i..j].iter().collect();
                tokens.push(Token { kind: "NUMBER".into(), value: num });
                i = j;
            }
            _ => return Err(format!("character not accepted: '{}'", c)),
        }
    }

    tokens.push(Token { kind: "$".into(), value: "$".into() });
    Ok(tokens)
}

fn parse_with_table(tokens: &[Token],grammar: &Grammar,table: &[[Option<String>; COLS]; ROWS]) -> Result<Vec<Node>, String> {
    let mut nodes: Vec<Node> = Vec::new();
    let root = nodes.len();
    nodes.push(Node { label: "EXPR".into(), children: vec![] });

    //stack 
    let mut stack: Vec<(String, usize)> = Vec::new();
    stack.push(("$".into(), root));   
    stack.push(("EXPR".into(), root));

    let mut matches = 0usize;

    while let Some((sym, node_id)) = stack.pop() {
        let look = tokens.get(matches).ok_or_else(|| "Unexpected end of input".to_string())?;

        //Accept
        if sym == "$" {
            if look.kind == "$" { 
                return Ok(nodes); 
            } else { 
                return Err(format!("Extra input before end: '{}'", look.value)); 
            }
        }

        //Terminal, matching
        if sym != EPS && !check_nonterminal(&sym, grammar) {
            if sym == look.kind {
                if sym == "IDENTIFIER" {
                    nodes[node_id].label = format!("IDENTIFIER({})", look.value);
                } else if sym == "NUMBER" {
                    nodes[node_id].label = format!("NUMBER({})", look.value);
                } else {
                    nodes[node_id].label = sym.clone();
                }
                matches += 1;
                continue;
            } else {
                return Err(format!("Parse error: expected '{}', saw '{}'", sym, look.kind));
            }
        }

        //Nonterminal
        let ai = nt_index(&sym).ok_or_else(|| format!("Unknown nonterminal {}", sym))?;
        let ci = term_index(&look.kind).ok_or_else(|| format!("Unknown lookahead {}", look.kind))?;
        let current = table[ai][ci].as_ref()
            .ok_or_else(|| format!("No rule for ({}, {})", sym, look.kind))?;

        if current == "EPSILON" {
            let x = nodes.len();
            nodes.push(Node { label: "EPSILON".into(), children: vec![] });
            nodes[node_id].children.push(x);
        } else {
            let syms: Vec<&str> = current.split_whitespace().collect();
            let mut child_ids = Vec::with_capacity(syms.len());
            for &s in &syms {
                let id = nodes.len();
                nodes.push(Node { label: s.to_string(), children: vec![] });
                nodes[node_id].children.push(id);
                child_ids.push(id);
            }
            for (s, id) in syms.iter().rev().zip(child_ids.into_iter().rev()) {
                stack.push(((*s).to_string(), id));
            }
        }
    }

    Err("Stack emptied without accepting input".into())
}

// --------- AST construction ----------

enum AST{
    BinOp {op: char, left: Box<AST>, right: Box<AST>},
    IdentifierAst(String),
    NumberAst(i64),
}

//starting building. idea: from type ast to vec
fn build_ast(nodes: &[Node]) -> Result<AST, String>{
    expr_f(nodes, 0)
}

fn expr_f(nodes: &[Node], id: usize) -> Result<AST, String>{
    let id_one = child(nodes, id, 0)?;
    let id_two = child(nodes, id, 1)?;
    let left_subtree = term_f(nodes, id_one)?;
    exprdash_f(nodes, id_two, left_subtree)
}

fn exprdash_f(nodes: &[Node], id: usize, mut left: AST) -> Result<AST, String>{
    let kids = &nodes[id].children;
    if kids.len() == 1 && nodes[kids[0]].label == "EPSILON"{
        return Ok(left);
    }

    let term = term_f(nodes, kids[1])?;
    left = AST::BinOp { op: '+', left: Box::new(left), right: Box::new(term)};
    exprdash_f(nodes, kids[2], left)
}

fn term_f(nodes: &[Node], id: usize) -> Result<AST, String>{
    let id_one = child(nodes, id, 0)?;
    let id_two = child(nodes, id, 1)?;
    let left = factor_f(nodes, id_one)?;
    termdash_f(nodes, id_two, left)
}

fn termdash_f(nodes: &[Node], id: usize, mut left: AST) -> Result<AST, String>{
    let kids = &nodes[id].children;
    if kids.len() == 1 && nodes[kids[0]].label == "EPSILON"{
        return Ok(left);
    }
    let fac = factor_f(nodes, kids[1])?;
    left = AST::BinOp { op: '*', left: Box::new(left), right: Box::new(fac) };
    termdash_f(nodes, kids[2], left)
}

fn factor_f(nodes: &[Node], id: usize) -> Result<AST, String>{
    let kids = &nodes[id].children;
    match kids.len(){
        1 => {
            let leaf = &nodes[kids[0]].label;
            if let Some(name) = extract_value(leaf, "IDENTIFIER("){
                Ok(AST::IdentifierAst(name))
            } 
            else if let Some(num) = extract_value(leaf, "NUMBER("){
                let n: i64 = num.parse().map_err(|_| format!("bad NUMBER literal: {}", num))?;
                Ok(AST::NumberAst(n))
            } 
            else{
                Err(format!("Unexpected leaf: {}", leaf)) 
            }
        }
        3 => {
            let expr_id = kids[1];
            expr_f(nodes, expr_id)
        }
        _ => Err("Not possible".to_string()),
    }
}

fn child(nodes: &[Node], id: usize, k: usize) -> Result<usize, String>{
    nodes[id].children.get(k).copied()
        .ok_or_else(|| format!("Node {} missing child {}", id, k))
}

fn extract_value(label: &str, prefix: &str) -> Option<String>{
    if label.starts_with(prefix) && label.ends_with(')'){
        Some(label[prefix.len()..label.len() - 1].to_string())
    } else {
        None
    }
}

//convert from AST
fn ast_to_nodes(ast: &AST) -> Vec<Node>{
    fn convert_to_vec(ast: &AST, nodes: &mut Vec<Node>) -> usize{
        match ast{
            AST::IdentifierAst(s) =>{
                let id = nodes.len();
                nodes.push(Node { label: format!("{}", s), children: vec![] });
                id
            }
            AST::NumberAst(n) =>{
                let id = nodes.len();
                nodes.push(Node { label: format!("{}", n), children: vec![] });
                id
            }
            AST::BinOp {op, left, right} =>{
                let id = nodes.len();
                nodes.push(Node { label: op.to_string(), children: vec![] });
                let l = convert_to_vec(left, nodes);
                let r = convert_to_vec(right, nodes);
                nodes[id].children.push(l);
                nodes[id].children.push(r);
                id
            }
        }
    }

    let mut nodes = Vec::<Node>::new();
    let root = convert_to_vec(ast, &mut nodes);
    debug_assert_eq!(root, 0);
    nodes
}


//DAG
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Offtype{
    LeafIdent(String),
    LeafNum(i64),
    AC {op: char, kids: Vec<usize>}
}

fn is_op(label: &str) -> bool { matches!(label, "+" | "*") }

fn leaf_gen(label: &str) -> Result<Offtype, String>{
    if label.chars().all(|c| c.is_ascii_digit()){
        let v: i64 = label.parse().map_err(|_| format!("Bad number literal: {}", label))?;
        Ok(Offtype::LeafNum(v))
    } 
    else if label.chars().all(|c| c.is_ascii_alphabetic()){
        Ok(Offtype::LeafIdent(label.to_string()))
    } 
    else{
        Err(format!("wrongleaf'{}'", label))
    }
}

fn one_leaf(x: Offtype, dag: &mut Vec<Node>, changeoff: &mut HashMap<usize, Offtype>, revrback: &mut HashMap<Offtype, usize>) -> usize {
    if let Some(&idx) = revrback.get(&x) { return idx;}
    let (label, children) = match &x{
        Offtype::LeafIdent(name) => (name.clone(), vec![]),
        Offtype::LeafNum(v) => (v.to_string(), vec![]),
        Offtype::AC { .. } => unreachable!(),
    };
    let idx = dag.len();
    dag.push(Node {label, children});
    changeoff.insert(idx, x.clone());
    revrback.insert(x, idx);
    idx
}

fn flatten(child: usize, op: char, changeoff: &HashMap<usize, Offtype>, out: &mut Vec<usize>){
    if let Some(Offtype::AC { op: sop, kids }) = changeoff.get(&child){
        if *sop == op{
            out.extend(kids.iter().copied());
            return;
        }
    }
    out.push(child);
}

fn one_bin(op: char, l: usize,r: usize, dag: &mut Vec<Node>, changeoff: &mut HashMap<usize, Offtype>, revrback: &mut HashMap<Offtype, usize>) -> usize {
    let mut ops: Vec<usize> = Vec::new();
    flatten(l, op, changeoff, &mut ops);
    flatten(r, op, changeoff, &mut ops);

    ops.sort_unstable();
    let s = Offtype::AC {op, kids: ops.clone()};
    if let Some(&idx) = revrback.get(&s){
        return idx;
    }

    let idx = dag.len();
    dag.push(Node {label: op.to_string(), children: vec![l, r]});

    changeoff.insert(idx, s.clone());
    revrback.insert(s, idx);
    idx
}


fn build_dag_from_ast(ast_nodes: &[Node], ast_root: usize) -> Result<(Vec<Node>, usize), String>{
    let mut dag: Vec<Node> = Vec::new();
    let mut changeoff: HashMap<usize, Offtype> = HashMap::new(); 
    let mut revrback: HashMap<Offtype, usize> = HashMap::new();
    let mut mem_src: HashMap<usize, usize> = HashMap::new();

    fn goiter(i: usize,ast: &[Node],dag: &mut Vec<Node>, changeoff: &mut HashMap<usize, Offtype>, revrback: &mut HashMap<Offtype, usize>, mem_src: &mut HashMap<usize, usize>) -> Result<usize, String> {
        if let Some(&j) = mem_src.get(&i) { return Ok(j); }
        let n = &ast[i];

        let j = if is_op(&n.label){
            if n.children.len() != 2{
                return Err(format!("Operator '{}' not binary:{}", n.label, i));
            }
            let l = goiter(n.children[0], ast, dag, changeoff, revrback, mem_src)?;
            let r = goiter(n.children[1], ast, dag, changeoff, revrback, mem_src)?;
            let op = n.label.chars().next().unwrap();
            one_bin(op, l, r, dag, changeoff, revrback)
        } 
        else {
            let s = leaf_gen(&n.label)?;
            one_leaf(s, dag, changeoff, revrback)
        };

        mem_src.insert(i, j);
        Ok(j)
    }

    let root = goiter(ast_root, ast_nodes, &mut dag, &mut changeoff, &mut revrback, &mut mem_src)?;
    Ok((dag, root))
}


//IR handle
#[derive(Debug, Clone)]
struct IrInstr {
    dest: String,
    op: String,
    lhs: String,
    rhs: String,
}

#[derive(Debug, Clone)]
struct IrFunction {
    args: Vec<String>,
    instrs: Vec<IrInstr>,
    ret: String,
}

//build IR directly
fn build_ir_from_dag(dag_nodes: &[Node], dag_root: usize) -> io::Result<IrFunction> {
    let mut ids: Vec<String> = Vec::new();
    let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
    for n in dag_nodes {
        if n.children.is_empty() && n.label.chars().all(|c| c.is_ascii_alphabetic()) {
            if seen_ids.insert(n.label.clone()) {
                ids.push(n.label.clone());
            }
        }
    }

    let mut instrs: Vec<IrInstr> = Vec::new();
    let mut name_of: HashMap<usize, String> = HashMap::new();
    let mut temp_counter: usize = 1;

    fn ir_gen_emit(i: usize,nodes: &[Node], name_of: &mut HashMap<usize, String>, instrs: &mut Vec<IrInstr>, temp_counter: &mut usize) -> io::Result<String> {
        if let Some(s) = name_of.get(&i) {
            return Ok(s.clone());
        }
        let n = &nodes[i];

        //leaf
        if n.children.is_empty() {
            let x = if n.label.chars().all(|c| c.is_ascii_alphabetic()) {
                format!("%{}", n.label)
            } 
            else if n.label.chars().all(|c| c.is_ascii_digit()) {
                n.label.clone()
            } 
            else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("wrong leaf '{}'", n.label),
                ));
            };
            name_of.insert(i, x.clone());
            return Ok(x);
        }

        //internal handle
        if n.children.len() != 2 || !matches!(n.label.as_str(), "+" | "*") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Wrong node '{}' with {} children", n.label, n.children.len()),
            ));
        }

        let lhs = ir_gen_emit(n.children[0], nodes, name_of, instrs, temp_counter)?;
        let rhs = ir_gen_emit(n.children[1], nodes, name_of, instrs, temp_counter)?;

        let temp = format!("%t{}", *temp_counter);
        *temp_counter += 1;

        let op = match n.label.as_str() {
            "+" => "add".to_string(),
            "*" => "mul".to_string(),
            _ => unreachable!(),
        };

        instrs.push(IrInstr {
            dest: temp.clone(),
            op,
            lhs,
            rhs,
        });

        name_of.insert(i, temp.clone());
        Ok(temp)
    }

    let ret_name = ir_gen_emit(dag_root, dag_nodes, &mut name_of, &mut instrs, &mut temp_counter)?;

    Ok(IrFunction {
        args: ids,
        instrs,
        ret: ret_name,
    })
}


//X86 gen
fn load_operand(operand: &str, asm: &mut Vec<String>, var_off: &HashMap<String, i64>, arg_locs: &HashMap<String, &'static str>) -> io::Result<()> {
    if operand.starts_with('%') {
        //arg or temp
        if let Some(&reg) = arg_locs.get(operand){
            asm.push(format!("  movq {}, %rax", reg));
        } 
        else if let Some(off) = var_off.get(operand){
            // temp on stack
            asm.push(format!("  movq {}(%rbp), %rax", off));
        } 
        else{
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown IR variable '{}'", operand),
            ));
        }
    } 
    else{
        asm.push(format!("  movq ${}, %rax", operand));
    }
    Ok(())
}

//gen x86 from IR
fn generate_x86_from_ir(ir: &IrFunction, path: &str) -> io::Result<()>{
    let arg_regs = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
    if ir.args.len() > arg_regs.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "more than 6 arguments not supported",
        ));
    }

    let mut arg_locs: HashMap<String, &'static str> = HashMap::new();
    for (i, a) in ir.args.iter().enumerate() {
        let key = format!("%{}", a); 
        arg_locs.insert(key, arg_regs[i]);
    }

    let mut temp_names: Vec<String> = Vec::new();
    for instr in &ir.instrs {
        if !temp_names.contains(&instr.dest) {
            temp_names.push(instr.dest.clone());
        }
    }

    let mut var_off: HashMap<String, i64> = HashMap::new();
    for (i, name) in temp_names.iter().enumerate() {
        let off = -8 * ((i as i64) + 1);
        var_off.insert(name.clone(), off);
    }
    let stack_size: i64 = (temp_names.len() as i64) * 8;

    let mut asm: Vec<String> = Vec::new();

    //pre
    asm.push(".text".to_string());
    asm.push("  .global foo".to_string());
    asm.push("".to_string());
    asm.push("foo:".to_string());
    asm.push("  pushq %rbp".to_string());
    asm.push("  movq %rsp, %rbp".to_string());

    if stack_size > 0 {
        asm.push(format!("  subq ${}, %rsp", stack_size));
    }

    //handle with rax and stack
    for instr in &ir.instrs{
        load_operand(&instr.lhs, &mut asm, &var_off, &arg_locs)?;

        if instr.rhs.starts_with('%'){
            if let Some(&reg) = arg_locs.get(&instr.rhs){
                match instr.op.as_str(){
                    "add" => asm.push(format!("  addq {}, %rax", reg)),
                    "mul" => asm.push(format!("  imulq {}, %rax", reg)),
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("unknown op '{}'", instr.op),
                        ))
                    }
                }
            } 
            else if let Some(off) = var_off.get(&instr.rhs){
                match instr.op.as_str(){
                    "add" => asm.push(format!("  addq {}(%rbp), %rax", off)),
                    "mul" => asm.push(format!("  imulq {}(%rbp), %rax", off)),
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("unknown op '{}'", instr.op),
                        ))
                    }
                }
            } 
            else{
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown IR variable '{}'", instr.rhs),
                ));
            }
        } 
        else{
            match instr.op.as_str(){
                "add" => asm.push(format!("  addq ${}, %rax", instr.rhs)),
                "mul" => asm.push(format!("  imulq ${}, %rax", instr.rhs)),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("unknown op '{}'", instr.op),
                    ))
                }
            }
        }

        if let Some(off) = var_off.get(&instr.dest){
            asm.push(format!("  movq %rax, {}(%rbp)", off));
        } 
        else{
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("no stack slot for dest '{}'", instr.dest),
            ));
        }
    }

    //load return 
    load_operand(&ir.ret, &mut asm, &var_off, &arg_locs)?;

    //last 
    if stack_size > 0 {
        asm.push("  movq %rbp, %rsp".to_string());
    }
    asm.push("  popq %rbp".to_string());
    asm.push("  ret".to_string());
    asm.push("".to_string());
    asm.push(".section .note.GNU-stack,\"\",@progbits".to_string());

    let mut f = File::create(path)?;
    let text = asm.join("\n");
    f.write_all(text.as_bytes())?;
    Ok(())
}



fn main() -> io::Result<()> {
    //Grammar
    let grammar: Grammar = HashMap::from([
        ("EXPR",     vec![vec!["TERM", "EXPRDASH"]]),
        ("EXPRDASH", vec![vec!["PLUS", "TERM", "EXPRDASH"], vec![EPS]]),
        ("TERM",     vec![vec!["FACTOR", "TERMDASH"]]),
        ("TERMDASH", vec![vec!["STAR", "FACTOR", "TERMDASH"], vec![EPS]]),
        ("FACTOR",   vec![
            vec!["IDENTIFIER"],
            vec!["NUMBER"],
            vec!["BOPEN", "EXPR", "BCLOSE"],
        ]),
    ]);

    let firsts = compute_first(&grammar);
    let follows = compute_follow(&grammar, &firsts, "EXPR");

    let table = build_table(&grammar, &firsts, &follows);

    //Reading file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./scanparse <filename>");
        std::process::exit(1);
    }
    let mut file = File::open(&args[1])?;
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    for line in input_string.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let tokens = match scan_expr(line) {
            Ok(ts) => ts,
            Err(e) => { eprintln!("{}", e); std::process::exit(1); }
        };
        
        let nodes_info = match parse_with_table(&tokens, &grammar, &table) {
            Ok(nodes) => nodes,
            Err(e) => { eprintln!("{}", e); std::process::exit(1); }
        };

        let ast_nodes = match build_ast(&nodes_info) {
            Ok(ast) => ast_to_nodes(&ast),
            Err(e) => { eprintln!("AST build error: {}", e); std::process::exit(1); }
        };

        let (dag_nodes, dag_root) = match build_dag_from_ast(&ast_nodes, 0) {
            Ok(v) => v,
            Err(e) => { eprintln!("DAG error: {}", e); std::process::exit(1); }
        };

        //build IR and then x86
        let ir = match build_ir_from_dag(&dag_nodes, dag_root) {
            Ok(ir) => ir,
            Err(e) => { eprintln!("IR build error: {}", e); std::process::exit(1); }
        };

        let in_path = Path::new(&args[1]);
        let asm_path = in_path.with_extension("s");
        generate_x86_from_ir(&ir, asm_path.to_string_lossy().as_ref())?;
    }
    Ok(())
}
