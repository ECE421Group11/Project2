#![allow(non_snake_case)]
//extern crate red_black_tree;
use Project2::red_black_tree::RedBlackTree;
use Project2::avl_tree::AVLTree;

# [test]
fn test_insert(){
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);
    assert_eq!(rbtree.len(), 7);
}

# [test]
fn test_empty(){
    let mut rbtree = RedBlackTree::<u32>::new();
    assert_eq!(rbtree.is_empty(), true);
    rbtree.insert(50);
    assert_eq!(rbtree.is_empty(), false);
}

# [test]
fn test_height(){
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
    let mut avltree = AVLTree::<u32>::new();
    avltree.insert(1);
    avltree.insert(2);
    avltree.insert(3);
    avltree.insert(4);
    avltree.insert(5);
    avltree.insert(6);
    avltree.insert(7);
    avltree.insert(10);

    println!("{:?}", avltree);
    println!("{:?}", avltree.get_height());

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
