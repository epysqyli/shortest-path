mod graph;
use graph::Graph;
///////////////////////////////////////////////
//
//                 5 - 7
//                 /
//          1 --- 3
//         /       \
//   10 - 0         6
//         \       /
//          2 --- 4
//         /       \
//        11        8 - 9
//       /         /
//      12 ------ 15
//        \      /
//        13 -- 14
//          \  /
//           16
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
    let node_11_id = graph.add_node(15.5, 12.9).unwrap();
    let node_12_id = graph.add_node(15.1, 11.0).unwrap();
    let node_13_id = graph.add_node(13.25, 9.2).unwrap();
    let node_14_id = graph.add_node(13.20, 9.8).unwrap();
    let node_15_id = graph.add_node(5.6, 2.8).unwrap();
    let node_16_id = graph.add_node(8.6, 4.2).unwrap();
    let node_17_id = graph.add_node(10.0, 11.05).unwrap();

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
    graph.create_edge(node_16_id, node_13_id).unwrap();
    graph.create_edge(node_16_id, node_14_id).unwrap();
    graph.create_edge(node_14_id, node_15_id).unwrap();
    graph.create_edge(node_13_id, node_14_id).unwrap();
    graph.create_edge(node_13_id, node_12_id).unwrap();
    graph.create_edge(node_15_id, node_12_id).unwrap();
    graph.create_edge(node_11_id, node_2_id).unwrap();
    graph.create_edge(node_15_id, node_8_id).unwrap();

    match graph.find_shortest_path(node_16_id, node_17_id) {
        Some(sp) => {
            println!("Shortest path: {:?}", sp.0);
            println!("Distance: {:?}", sp.1);
        }
        None => println!("No path found"),
    }

    match graph.find_shortest_path(node_10_id, node_16_id) {
        Some(sp) => {
            println!("Shortest path: {:?}", sp.0);
            println!("Distance: {:?}", sp.1);
        }
        None => println!("No path found"),
    }
}
