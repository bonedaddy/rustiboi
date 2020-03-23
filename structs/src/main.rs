struct Node {
    id: String,
    active_counts: u64,
    active: bool,
}

fn main() {
    let mut node1 = build_node(String::from("hello world"));
    println!("id = {}, active_counts = {}, active = {}", node1.id, node1.active_counts, node1.active);
    println!("{}", node1.id);
    let s = &node1.id;
    let node2 = Node {
        id: s.to_string(),
        active_counts: 100,
        active: false,
    };
    let node3 = Node {
        id: String::from("hello this is node 3"),
        ..node1
    };
    println!("printing node3");
    println!("id = {}, active_counts = {}, active = {}", node3.id, node3.active_counts, node3.active);
    println!("printing node1");
    println!("id = {}, active_counts = {}, active = {}", node1.id, node1.active_counts, node1.active);
    println!("setting node 1");
    node1.id = String::from("new");
    println!("printing node1");
    println!("id = {}, active_counts = {}, active = {}", node1.id, node1.active_counts, node1.active);
    println!("printing node2");
    println!("id = {}, active_counts = {}, active = {}", node2.id, node2.active_counts, node2.active);
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}


fn build_node(id: String) -> Node {
    return Node {
        id: id,
        active_counts: 1,
        active: true
    }
}