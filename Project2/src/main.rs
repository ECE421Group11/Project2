#![allow(non_snake_case)]
//extern crate red_black_tree;
use Project2::red_black_tree::RedBlackTree;
use Project2::avl_tree::AVLTree;
use std::env;

# [test]
fn test_rb_insert(){
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);
    assert_eq!(rbtree.len(), 7);

    let node50 = rbtree.get_node(50);
    let node25 = rbtree.get_node(25);
    let node15 = rbtree.get_node(15);
    let node35 = rbtree.get_node(35);
    let node75 = rbtree.get_node(75);
    let node60 = rbtree.get_node(60);
    let node90 = rbtree.get_node(90);

}

# [test]
fn test_rb_get_node(){
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);

    assert_eq!(rbtree[rbtree.get_node(75)].value, 75);
    assert_eq!(rbtree[rbtree.get_node(50)].value, 50);
    assert_eq!(rbtree[rbtree.get_node(90)].value, 90);

}

# [test]
fn test_rb_empty(){
    let mut rbtree = RedBlackTree::<u32>::new();
    assert_eq!(rbtree.is_empty(), true);
    rbtree.insert(50);
    assert_eq!(rbtree.is_empty(), false);
}

# [test]
fn test_rb_height(){
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);
    assert_eq!(rbtree.get_height(), 4)
}

#[test]
fn simple_avl_insert(){
    let mut avltree = AVLTree::<u32>::new();
    avltree.insert(1);
    avltree.insert(2);
    avltree.insert(3);
    assert_eq!(avltree[avltree.root].value, 2);

    let mut avltree = AVLTree::<u32>::new();
    avltree.insert(3);
    avltree.insert(2);
    avltree.insert(1);
    assert_eq!(avltree[avltree.root].value, 2);
}

#[test]
fn avl_rebalance(){
    let mut avltree = AVLTree::<u32>::new();
    avltree.insert(1);
    avltree.insert(2);
    avltree.insert(3);
    avltree.insert(4);
    avltree.insert(5);
    avltree.insert(6);
    avltree.insert(7);
    assert_eq!(avltree.get_height(), 3);
    avltree.insert(8);
    assert_eq!(avltree.get_height(), 4);
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut avltree = AVLTree::<u32>::new();
    println!("{:?}",avltree.is_empty());
    avltree.insert(1);
    avltree.insert(2);
    avltree.insert(3);
    avltree.insert(4);
    avltree.insert(5);
    avltree.insert(6);
    avltree.insert(7);
    //avltree.insert(10);
    avltree.delete(5);
    println!("{:?}", avltree);
    println!("HEIGHT {:?}", avltree.get_height());
    //println!("{:?}", avltree.len());

    avltree.delete(6);
    println!("{:?}", avltree);
    println!("HEIGHT {:?}", avltree.get_height());
    //println!("{:?}", avltree.len());

    avltree.delete(7);
    println!("{:?}", avltree);
    println!("HEIGHT {:?}", avltree.get_height());
    println!("ROOT {:?}", avltree[avltree.root].value);

    avltree.delete(3);
    println!("{:?}", avltree);
    println!("HEIGHT {:?}", avltree.get_height());
    println!("ROOT {:?}", avltree[avltree.root].value);

    //avltree.delete(6);
    //println!("{:?}", avltree);
    //println!("HEIGHT {:?}", avltree.get_height());
    //println!("{:?}", avltree.len());

    //let mut rbtree = RedBlackTree::<u32>::new();
    /*rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);

    println!("{:?}", rbtree);

    println!("");
    
    let node25 = rbtree[rbtree.root].left;
    
    
    rbtree.right_rotate(node25);
    println!("After right rotate");
    println!("{:?}", rbtree);

    println!("");
    rbtree.left_rotate(node25);
    println!("After left rotate");
    println!("{:?}", rbtree);*/
}
