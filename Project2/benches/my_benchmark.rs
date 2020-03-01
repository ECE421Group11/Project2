use criterion::{black_box, criterion_group, criterion_main, Criterion};
use Project2::avl_tree::AVLTree;
use Project2::red_black_tree::RedBlackTree;
use Project2::binary_tree::BinaryTree;

const SCALEFACTOR: u32 = 100;

fn Binary(val : u32){
    let mut bintree = BinaryTree::<u32>::new();
    for x in 0..val{
        bintree.insert(x);
    }
    for y in 0..(val/10){
        bintree.get_node(y);
    }
}

fn RBTree(val : u32) {
    let mut rbtree = RedBlackTree::<u32>::new();
    for x in 0..val{
        rbtree.insert(x);
    }
    for y in 0..(val/10){
        rbtree.get_node(y);
    }
}

fn AVLTree(val : u32) {
    let mut avltree = AVLTree::<u32>::new();
    for x in 0..val{
        avltree.insert(x);
    }
    for y in 0..(val/10){
        avltree.get_node(y);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let vec = vec![10000,40000,70000,100000,130000];
    for tree_size in vec {
        c.bench_function("BinaryTree", |b| b.iter(|| Binary(tree_size/SCALEFACTOR)));
        c.bench_function("RBTree", |b| b.iter(|| RBTree(tree_size/SCALEFACTOR)));
        c.bench_function("AVLTree", |b| b.iter(|| AVLTree(tree_size/SCALEFACTOR)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
