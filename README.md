How would a shortest path algorithm look like if implemented without looking at existing solutions?
  - setup a series of nodes
  - each node is connected to one or more nodes via an edge
    - each edge has a distance: the distance is the cost
    - a higher distance means a higher traversal cost

Steps:
  - create a random set of nodes
  - create a random set of edges connecting the nodes
    - the network should have random connections amongs nodes, providing different layers of depth
  - compute the shortest path from node 'A' to any random node
    - initially, all other nodes have infinite distance from node '0'
    - how can we compute the shortest path from node '0' to any other given node?

