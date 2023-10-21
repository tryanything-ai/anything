//! The sequencer essentially allows us to manipulate the graph
//! and interact with it.
//!
//! Eventually, this sequencer may become a player of nodes
use super::node::Node;
use daggy::*;

/// Get nodes in order based upon the dag
pub fn get_nodes_in_order<'a>(
    dag: &'a Dag<Node, ()>,
    start: &Vec<NodeIndex>,
    tree: &mut Vec<Vec<&'a Node>>,
) {
    let mut row: Vec<&Node> = vec![];

    for idx in start {
        for row in tree.iter_mut() {
            let pos = row.iter().position(|s| s.name == dag[*idx].name);
            if let Some(remove_idx) = pos {
                row.remove(remove_idx);
            }
        }
        let unique = !row.iter().any(|s| s.name == dag[*idx].name);
        if unique {
            row.push(&dag[*idx]);
        }
    }
    tree.push(row);

    let mut children: Vec<NodeIndex> = vec![];
    for parent in start.iter() {
        for (_, node_index) in dag.children(*parent).iter(&dag) {
            children.push(node_index);
        }
    }
    if children.len() != 0 {
        get_nodes_in_order(dag, &children, tree);
    }
}

pub fn find_node_recursive<'a>(
    dag: &'a Dag<Node, ()>,
    name: &str,
    start: NodeIndex,
) -> Option<(NodeIndex, &'a Node)> {
    if dag.children(start).iter(&dag).count() != 0 {
        if let Some((_, node)) = dag
            .children(start)
            .iter(&dag)
            .find(|(_, n)| &dag[*n].name == name)
        {
            return Some((node, &dag[node]));
        } else {
            for (_, child_node) in dag.children(start).iter(&dag) {
                if let Some(v) = find_node_recursive(dag, name, child_node) {
                    return Some(v);
                }
            }
            None
        }
    } else {
        None
    }
}

pub fn is_valid_subtree(dag: &Dag<Node, ()>, start: NodeIndex) -> bool {
    let mut allowed_deps = vec![];
    allowed_deps.push(&dag[start]);
    let mut processed_children = vec![start];

    while processed_children.len() != 0 {
        let mut next_to_process = vec![];

        for child_idx in processed_children.iter() {
            for (_, child_node) in dag.children(*child_idx).iter(&dag) {
                allowed_deps.push(&dag[*child_idx]);
                next_to_process.push(child_node);
            }
        }

        for child_idx in processed_children.iter() {
            if *child_idx != start {
                for (_, parent_node) in dag.parents(*child_idx).iter(&dag) {
                    if !allowed_deps
                        .iter()
                        .any(|node| node.name == dag[parent_node].name)
                    {
                        return false;
                    }
                }
            }
        }

        processed_children = next_to_process;
    }
    true
}

#[cfg(test)]
mod tests {

    use crate::test_helpers::test_helpers::*;

    use super::*;

    #[test]
    fn test_nodes_in_order() {
        let mut dag = Dag::<Node, ()>::new();
        let parent_node = make_node("root", &vec![]);
        let parent = dag.add_node(parent_node);

        let child1 = make_node("child1", &vec![]);
        let child2 = make_node("child2", &vec![]);
        dag.add_child(parent, (), child1);
        let (_, child2_idx) = dag.add_child(parent, (), child2);

        let sub_step = make_node("sub_step1", &vec![]);
        dag.add_child(child2_idx, (), sub_step);

        let expected = vec![vec!["root"], vec!["child2", "child1"], vec!["sub_step1"]];

        let mut actual = vec![];
        get_nodes_in_order(&dag, &vec![parent], &mut actual);

        let names = get_tree_names(actual);

        assert_eq!(names, expected);
    }

    #[test]
    fn test_nodes_find_ok() {
        let mut dag = Dag::<Node, ()>::new();
        let parent_node = make_node("root", &vec![]);
        let parent = dag.add_node(parent_node);

        let child1 = make_node("child1", &vec![]);
        let child2 = make_node("child2", &vec![]);
        let (_, child1_idx) = dag.add_child(parent, (), child1);
        let (_, child2_idx) = dag.add_child(parent, (), child2);

        let sub_step = make_node("sub_node1", &vec![]);
        let (_, subnode_idx) = dag.add_child(child2_idx, (), sub_step);

        // child
        let res = find_node_recursive(&dag, "child1", parent);
        assert!(res.is_some());
        let (found_idx, found_node) = res.unwrap();
        assert_eq!(found_idx, child1_idx);
        assert_eq!(found_node.name, "child1");

        // now grandchild
        let res = find_node_recursive(&dag, "sub_node1", parent);
        assert!(res.is_some());
        let (found_idx, found_node) = res.unwrap();
        assert_eq!(found_idx, subnode_idx);
        assert_eq!(found_node.name, "sub_node1");
    }

    #[test]
    fn test_nodes_find_nonexistant() {
        let mut dag = Dag::<Node, ()>::new();
        let parent_node = make_node("root", &vec![]);
        let parent = dag.add_node(parent_node);

        let child1 = make_node("child1", &vec![]);
        let child2 = make_node("child2", &vec![]);
        dag.add_child(parent, (), child1);
        let (_, child2_idx) = dag.add_child(parent, (), child2);

        let sub_node = make_node("sub_node1", &vec![]);
        dag.add_child(child2_idx, (), sub_node);

        // child
        let res = find_node_recursive(&dag, "not_really_a_task_at_all", parent);
        assert!(res.is_none());
    }

    #[test]
    fn test_nodes_is_valid_tree() {
        let mut dag = Dag::<Node, ()>::new();
        let parent_node = make_node("root", &vec![]);
        let parent = dag.add_node(parent_node);

        let child1 = make_node("child1", &vec![]);
        let child2 = make_node("child2", &vec![]);
        dag.add_child(parent, (), child1);
        let (_, child2_idx) = dag.add_child(parent, (), child2);

        let sub_node = make_node("sub_node1", &vec![]);
        dag.add_child(child2_idx, (), sub_node);

        // child
        let res = is_valid_subtree(&dag, parent);
        assert!(res);
    }

    #[test]
    fn test_nodes_is_invalid_tree() {
        let mut dag = Dag::<Node, ()>::new();
        let parent_node = make_node("root", &vec![]);
        let parent = dag.add_node(parent_node);

        let child1 = make_node("child1", &vec![]);
        let child2 = make_node("child2", &vec![]);
        let (_, child1_idx) = dag.add_child(parent, (), child1);
        let (_, child2_idx) = dag.add_child(parent, (), child2);

        let sub_node = make_node("sub_node1", &vec![]);
        let (_, sub_idx) = dag.add_child(child2_idx, (), sub_node);

        dag.add_edge(child1_idx, sub_idx, ()).ok().unwrap();

        assert_eq!(false, is_valid_subtree(&dag, child2_idx));
        assert_eq!(true, is_valid_subtree(&dag, parent));
    }
}
