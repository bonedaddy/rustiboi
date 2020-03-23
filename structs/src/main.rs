struct Node {
    id: String,
    active_counts: u64,
    active: bool,
}

fn main() {
    let mut node1 = build_node(String::from("hello world"));
    println!("id = {}, active_counts = {}, active = {}", node1.id, node1.active_counts, node1.active);
    println!("{}", node1.id);
    let node2 = Node {
        id: node1.id,
        active_counts: 100,
        active: false,
    };
    node1.id = String::from("new");
    println!("id = {}, active_counts = {}, active = {}", node2.id, node2.active_counts, node2.active);
    println!("id = {}, active_counts = {}, active = {}", node1.id, node1.active_counts, node1.active);
}


fn build_node(id: String) -> Node {
    return Node {
        id: id,
        active_counts: 1,
        active: true
    }
}