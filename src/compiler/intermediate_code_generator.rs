use crate::compiler::*;

pub fn generate_intermediate_code(ast: &Vec<Option<Node>>) -> Vec<String> {
    let mut mid_commands: Vec<String> = Vec::new();
    for node in ast.iter() {
        match node {
            Some(node) => {
                traverse(&node, &mut mid_commands);
            }
            _ => {}
        }
    }
    mid_commands
}

fn traverse(node: &Node, mid_commands: &mut Vec<String>) {
    
    match node.kind {
        NodeKind::NdReturn => {
            match &node.left {
                Some(node_left) => {
                    traverse(&node_left, mid_commands);
                }
                _ => {}
            }
        mid_commands.push(String::from(format!("RETURN")));
        }
        
        NodeKind::NdNum => {
            mid_commands.push(format!("PUSH {}", node.val));
        }
        _ => {panic!("nokind")}
    }
}