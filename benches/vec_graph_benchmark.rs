use aoc_helper::graph::{vec_graph::VecGraph, Graph};
use criterion::{Criterion, criterion_group, criterion_main, black_box};



// TODO: better benchmarks
pub fn node_add(c: &mut Criterion) {
    let mut graph = VecGraph::new();

    c.bench_function("node_add", |b| {
        b.iter(|| {
            let n0 = graph.add_node("one");
            let n1 = graph.add_node("two");
            let n2 = graph.add_node("three");
            let n3 = graph.add_node("four");
            let n4 = graph.add_node("five");
            let n5 = graph.add_node("six");

            black_box(&[n0, n1, n2, n3, n4, n5]);
        });

        graph = VecGraph::new(); // Reset graph so that it does not grow while iterating
    });
}

pub fn edge_add(c: &mut Criterion) {
    let mut graph = VecGraph::new();

    let n0 = graph.add_node("one");
    let n1 = graph.add_node("two");
    let n2 = graph.add_node("three");
    let n3 = graph.add_node("four");
    let n4 = graph.add_node("five");
    let n5 = graph.add_node("six");

    let nodes = &[n0, n1, n2, n3, n4, n5];

    c.bench_function("edge_add", |b| {
        b.iter(|| {
            for n1 in nodes {
                for n2 in nodes {
                    graph.add_edge(*n1, *n2);
                } 
            }
        });
    });
}

criterion_group!(benches, node_add, edge_add);
criterion_main!(benches);