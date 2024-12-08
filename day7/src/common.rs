use daggy::{Dag, Walker};

pub fn search_combination(result: u64, operands: Vec<u64>, operations: i8) -> u64 {
    let mut dag = Dag::<u64, u64>::new();
    let mut depth = 0;
    let mut curr = dag.add_node(operands[depth]);
    depth += 1;

    while !(depth == operands.len() && dag[curr] == result) {
        if dag[curr] > result || depth == operands.len() {
            if let Some((_, parent)) = dag.parents(curr).iter(&dag).next() {
                curr = parent;
                depth -= 1;
            } else {
                return 0;
            }
        }

        let child_count = dag.children(curr).iter(&dag).count() as i8;
        match child_count - operations {
            0 => {
                if let Some((_, parent)) = dag.parents(curr).iter(&dag).next() {
                    curr = parent;
                    depth -= 1;
                } else {
                    return 0;
                }
            }
            -3 => {
                let multiplier = 10_u64.pow(operands[depth].checked_ilog10().unwrap_or(0) + 1);
                let (_, new_child) =
                    dag.add_child(curr, 1, dag[curr] * multiplier + operands[depth]);
                curr = new_child;
                depth += 1;
            }
            -2 => {
                let (_, new_child) = dag.add_child(curr, 1, dag[curr] + operands[depth]);
                curr = new_child;
                depth += 1;
            }
            -1 => {
                let (_, new_child) = dag.add_child(curr, 1, dag[curr] * operands[depth]);
                curr = new_child;
                depth += 1;
            }
            _ => {
                panic! {};
            }
        }
    }

    return result;
}
