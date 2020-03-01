use criterion::{black_box, criterion_group, criterion_main, Criterion};
use Project2::avl_tree::AVLTree;
use Project2::red_black_tree::RedBlackTree;
use Project2::binary_tree::BinaryTree;

const SCALEFACTOR: i32 = 100;

fn Binary(){
    let mut vec = Vec::new();
    vec.push(10000/SCALEFACTOR);
    vec.push(40000/SCALEFACTOR);
    vec.push(70000/SCALEFACTOR);
    vec.push(100000/SCALEFACTOR);
    vec.push(130000/SCALEFACTOR);
    for tree_size in vec{
        let mut bintree = BinaryTree::<u32>::new();
        for x in 0..tree_size{
            bintree.insert(x);
        }
        for y in 0..(tree_size/10){
            bintree.get_node(y);
        }
    }
}

fn RBTree() {
    let mut vec = Vec::new();
    vec.push(10000/SCALEFACTOR);
    vec.push(40000/SCALEFACTOR);
    vec.push(70000/SCALEFACTOR);
    vec.push(100000/SCALEFACTOR);
    vec.push(130000/SCALEFACTOR);
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
    vec.push(10000/SCALEFACTOR);
    vec.push(40000/SCALEFACTOR);
    vec.push(70000/SCALEFACTOR);
    vec.push(100000/SCALEFACTOR);
    vec.push(130000/SCALEFACTOR);
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
    c.bench_function("BinaryTree", |b| b.iter(|| Binary()));
    c.bench_function("RBTree", |b| b.iter(|| RBTree()));
    c.bench_function("AVLTree", |b| b.iter(|| AVLTree()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
