#![allow(non_snake_case)]
//extern crate red_black_tree;
//use red_black_tree::RedBlackTree;
use Project2::avl_tree::AVLTreeRoot;

fn main() {
    println!("Project 2");

    let mut AVLtree = AVLTreeRoot::<u32>::new();
    println!("Tree Empty? {}", AVLtree.is_empty());
    AVLtree.insert(1);
    AVLtree.insert(2);
    AVLtree.insert(0);
    AVLtree.print();
    println!("Tree Empty? {}", AVLtree.is_empty());


}
