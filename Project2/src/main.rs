#![allow(non_snake_case)]
//extern crate red_black_tree;
use Project2::red_black_tree::RedBlackTree;
use Project2::red_black_tree::NodeColor;
use Project2::red_black_tree::Pointer;
use std::env;

use Project2::avl_tree::AVLTree;

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

    let node50 = rbtree.get_node(50);
    let node25 = rbtree.get_node(25);
    let node15 = rbtree.get_node(15);
    let node35 = rbtree.get_node(35);
    let node75 = rbtree.get_node(75);
    let node60 = rbtree.get_node(60);
    let node90 = rbtree.get_node(90);

    assert_eq!(rbtree[node50].value, 50);
    assert_eq!(rbtree[node50].color, NodeColor::Red);
    assert_eq!(rbtree[node50].left, node35);
    assert_eq!(rbtree[node50].right, node75);
    assert_eq!(rbtree[node50].parent, node25);

    assert_eq!(rbtree[node25].value, 25);
    assert_eq!(rbtree[node25].color, NodeColor::Black);
    assert_eq!(rbtree[node25].left, node15);
    assert_eq!(rbtree[node25].right, node50);
    assert_eq!(rbtree[node25].parent, Pointer::null());

    assert_eq!(rbtree[node15].value, 15);
    assert_eq!(rbtree[node15].color, NodeColor::Black);
    assert_eq!(rbtree[node15].left, Pointer::null());
    assert_eq!(rbtree[node15].right, Pointer::null());
    assert_eq!(rbtree[node15].parent, node25);

    assert_eq!(rbtree[node35].value, 35);
    assert_eq!(rbtree[node35].color, NodeColor::Black);
    assert_eq!(rbtree[node35].left, Pointer::null());
    assert_eq!(rbtree[node35].right, Pointer::null());
    assert_eq!(rbtree[node35].parent, node50);

    assert_eq!(rbtree[node75].value, 75);
    assert_eq!(rbtree[node75].color, NodeColor::Black);
    assert_eq!(rbtree[node75].left, node60);
    assert_eq!(rbtree[node75].right, node90);
    assert_eq!(rbtree[node75].parent, node50);

    assert_eq!(rbtree[node60].value, 60);
    assert_eq!(rbtree[node60].color, NodeColor::Red);
    assert_eq!(rbtree[node60].left, Pointer::null());
    assert_eq!(rbtree[node60].right, Pointer::null());
    assert_eq!(rbtree[node60].parent, node75);

    assert_eq!(rbtree[node90].value, 90);
    assert_eq!(rbtree[node90].color, NodeColor::Red);
    assert_eq!(rbtree[node90].left, Pointer::null());
    assert_eq!(rbtree[node90].right, Pointer::null());
    assert_eq!(rbtree[node90].parent, node75);
}

# [test]
fn test_rb_delete(){
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);

    rbtree.delete(50);
    

    let node25 = rbtree.get_node(25);
    let node15 = rbtree.get_node(15);
    let node35 = rbtree.get_node(35);
    let node75 = rbtree.get_node(75);
    let node60 = rbtree.get_node(60);
    let node90 = rbtree.get_node(90);

    assert_eq!(rbtree[node25].value, 25);
    assert_eq!(rbtree[node25].color, NodeColor::Black);
    assert_eq!(rbtree[node25].left, node15);
    assert_eq!(rbtree[node25].right, node60);
    assert_eq!(rbtree[node25].parent, Pointer::null());

    assert_eq!(rbtree[node15].value, 15);
    assert_eq!(rbtree[node15].color, NodeColor::Black);
    assert_eq!(rbtree[node15].left, Pointer::null());
    assert_eq!(rbtree[node15].right, Pointer::null());
    assert_eq!(rbtree[node15].parent, node25);

    assert_eq!(rbtree[node35].value, 35);
    assert_eq!(rbtree[node35].color, NodeColor::Black);
    assert_eq!(rbtree[node35].left, Pointer::null());
    assert_eq!(rbtree[node35].right, Pointer::null());
    assert_eq!(rbtree[node35].parent, node60);

    assert_eq!(rbtree[node75].value, 75);
    assert_eq!(rbtree[node75].color, NodeColor::Black);
    assert_eq!(rbtree[node75].left, Pointer::null());
    assert_eq!(rbtree[node75].right, node90);
    assert_eq!(rbtree[node75].parent, node60);

    assert_eq!(rbtree[node60].value, 60);
    assert_eq!(rbtree[node60].color, NodeColor::Red);
    assert_eq!(rbtree[node60].left, node35);
    assert_eq!(rbtree[node60].right, node75);
    assert_eq!(rbtree[node60].parent, node25);

    assert_eq!(rbtree[node90].value, 90);
    assert_eq!(rbtree[node90].color, NodeColor::Red);
    assert_eq!(rbtree[node90].left, Pointer::null());
    assert_eq!(rbtree[node90].right, Pointer::null());
    assert_eq!(rbtree[node90].parent, node75);
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

# [test]
fn test_rb_count_leafs(){
    let mut rbtree = RedBlackTree::<u32>::new();
    assert_eq!(rbtree.count_leaf_nodes(), 0);

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);

    assert_eq!(rbtree.count_leaf_nodes(), 4);
}



fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rbtree = RedBlackTree::<u32>::new();
    
    println!("{:?}", rbtree.count_leaf_nodes());

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);

    println!("{:?}", rbtree);
    println!("{:?}", rbtree.count_leaf_nodes());

    rbtree.delete(50);
    println!("{:?}", rbtree);

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