use zad4::*;

fn main() {
    let mut matrix = AdjMatrix::new();
    let mut list = AdjList::new();

    matrix.add_node(1);
    matrix.add_node(2);
    matrix.add_node(3);
    matrix.add_node(4);
    matrix.add_node(5);

    list.add_node(1);
    list.add_node(2);
    list.add_node(3);
    list.add_node(4);
    list.add_node(5);

    matrix.connect(0, 1, 4);
    matrix.connect(1, 2, 3);
    matrix.connect(2, 3, 9);
    matrix.connect(2, 4, 5);
    matrix.connect(3, 4, 1);
    matrix.connect(3, 5, 2);
    matrix.connect(4, 5, 6);

    list.connect(0, 1, 4);
    list.connect(1, 2, 3);
    list.connect(2, 3, 9);
    list.connect(2, 4, 5);
    list.connect(3, 4, 1);
    list.connect(3, 5, 2);
    list.connect(4, 5, 6);

    assert_eq!(matrix.dijkstra(0), list.dijkstra(0));

    dbg!(matrix.dijkstra(0));
    dbg!(list.dijkstra(0));
}
