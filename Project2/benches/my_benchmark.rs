use criterion::{black_box, criterion_group, criterion_main, Criterion};
use Project2::avl_tree::AVLTree;
use Project2::red_black_tree::RedBlackTree;

fn RBTree() {
    let mut vec = Vec::new();
    vec.push(10000);
    vec.push(40000);
    vec.push(70000);
    vec.push(100000);
    vec.push(130000);
    for tree_size in vec{
        let mut rbtree = RedBlackTree::<u32>::new();
        for x in 0..tree_size{
            rbtree.insert(x);
        }
        for y in 0..(tree_size/10){
            rbtree.get_node(y);
        } 
    }
}

fn AVLTree() {
    let mut vec = Vec::new();
    vec.push(10000);
    vec.push(40000);
    vec.push(70000);
    vec.push(100000);
    vec.push(130000);
    for tree_size in vec{
        let mut avltree = AVLTree::<u32>::new();
        for x in 0..tree_size{
            avltree.insert(x);
        }
        for y in 0..(tree_size/10){
            avltree.get_node(y);
        } 
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("RBTree", |b| b.iter(|| RBTree()));
    c.bench_function("AVLTree", |b| b.iter(|| AVLTree())); 
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);