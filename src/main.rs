use shortest_path::Graph;

///////////////////////////////////////////////
// Suppose there's a graph with this structure
//
//                 5 - 7
//                /
//          1 -- 3
//        /       \
//  10 - 0         6
//        \       /
//          2 -- 4
//                \
//                 8 - 9
//
///////////////////////////////////////////////
fn main() {
    let mut graph = Graph::new();

    let node_0_id = graph.add_node(0.0, 0.0).unwrap();
    let node_1_id = graph.add_node(1.0, 1.0).unwrap();
    let node_2_id = graph.add_node(0.5, 5.0).unwrap();
    let node_3_id = graph.add_node(2.0, 4.5).unwrap();
    let node_4_id = graph.add_node(3.5, 1.0).unwrap();
    let node_5_id = graph.add_node(2.0, 5.0).unwrap();
    let node_6_id = graph.add_node(7.5, 1.0).unwrap();
    let node_7_id = graph.add_node(2.0, 5.5).unwrap();
    let node_8_id = graph.add_node(4.25, 3.5).unwrap();
    let node_9_id = graph.add_node(1.75, 3.5).unwrap();
    let node_10_id = graph.add_node(6.3, 2.1).unwrap();

    graph.create_edge(node_0_id, node_1_id).unwrap();
    graph.create_edge(node_1_id, node_3_id).unwrap();
    graph.create_edge(node_3_id, node_5_id).unwrap();
    graph.create_edge(node_3_id, node_6_id).unwrap();
    graph.create_edge(node_0_id, node_2_id).unwrap();
    graph.create_edge(node_2_id, node_4_id).unwrap();
    graph.create_edge(node_4_id, node_6_id).unwrap();
    graph.create_edge(node_5_id, node_7_id).unwrap();
    graph.create_edge(node_4_id, node_8_id).unwrap();
    graph.create_edge(node_8_id, node_9_id).unwrap();
    graph.create_edge(node_0_id, node_10_id).unwrap();

    let _sp = graph.find_shortest_path(node_3_id, node_0_id);
    // let _sp = graph.find_shortest_path(node_0_id, node_6_id);
    // println!();
    // println!("Shortest path: {:?}", sp.0);
    // println!("Distance: {:?}", sp.1);
}
