
use crate::compiler::*;

pub fn parsing(tokens: &Vec<Token>) -> Vec<Option<Node>> {
    let mut idx: usize = 0;
    let program = program(&tokens, &mut idx);
    program
}

fn program(tokens: &Vec<Token>, idx: &mut usize) -> Vec<Option<Node>> {
    let mut vec: Vec<Option<Node>> = Vec::new();
    loop {
        let node = function(&tokens, idx);
        match node {
            Some(node_) => {
                vec.push(Some(node_));
                continue;
            }
            _ => {
                vec.push(None);
                break;
            }
        }
    }
    vec
}

fn function(tokens: &Vec<Token>, idx: &mut usize) -> Option<Node> {
    if consume(&tokens, idx, &"int") { 
        if consume(&tokens, idx, &"main") {
            if consume(&tokens, idx, &"(")  {
                if consume(&tokens, idx, &")") {
                    if consume(&tokens, idx, &"{") {
                        let node = stmt(&tokens, idx);
                        match node {
                            Some(node) => {
                                if consume(&tokens, idx, &"}") {Some(node)} 
                                else {panic!("no rbrace")}
                            }                     
                            None => { panic!("no stmt")}
                        }
                    } else { panic!("no lbrace"); } 
                } else { panic!("no )"); }
            } else { panic!("no ("); }
        } else { panic!("no main"); }
    } else { None }
}

fn stmt(tokens: &Vec<Token>, idx: &mut usize) -> Option<Node> {
    let node;
    if consume(&tokens, idx, &"return") {
        node = expr(&tokens, idx);
        if consume(&tokens, idx, &";") {
            return create_node(&"return", NodeKind::NdReturn, node, None);
        } else {
            None
        }
    } else {
        let node = expr(&tokens, idx);
        if consume(&tokens, idx, &";") {
            return node;
        } else {
            None
        }
    }
}

fn expr(tokens: &Vec<Token>, idx: &mut usize) -> Option<Node> {
    let idx_: usize = *idx;
    if tokens.len() <= idx_ {
        return None;
    }
    match tokens[idx_].kind {
        TokenKind::TkNum => {
            *idx += 1;
            return create_node(&tokens[idx_].val, NodeKind::NdNum, None, None);
        }
        _ => {panic!("no!")}
    }
}






fn create_node(val: &str, kind: NodeKind, left: Option<Node>, right: Option<Node>) -> Option<Node> {
    match (left, right) {
        (Some(node_left), Some(node_right)) => Some(Node {
            val: String::from(val),
            kind: kind,
            left: Some(Box::new(node_left)),
            right: Some(Box::new(node_right)),
        }),
        (Some(node_left), None) => Some(Node {
            val: String::from(val),
            kind: kind,
            left: Some(Box::new(node_left)),
            right: None,
        }),
        (None, Some(node_right)) => Some(Node {
            val: String::from(val),
            kind: kind,
            left: None,
            right: Some(Box::new(node_right)),
        }),
        (None, None) => Some(Node {
            val: String::from(val),
            kind: kind,
            left: None,
            right: None,
        }),
    }
}

fn consume(tokens: &Vec<Token>, idx: &mut usize, target_val: &str) -> bool {
    let idx_: usize = *idx;

    if tokens.len() <= idx_ {
        return false;
    }

    if tokens[idx_].val == target_val {
        *idx += 1;
        true
    } else {
        false
    }
}