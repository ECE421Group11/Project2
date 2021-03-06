#![allow(non_snake_case)]
#[allow(unused_imports)]
use Project2::red_black_tree::RedBlackTree;

#[allow(unused_imports)]
use Project2::red_black_tree::NodeColor;

#[allow(unused_imports)]
use Project2::binary_tree::BinaryTree;

#[allow(unused_imports)]
use Project2::red_black_tree::Pointer;
use Project2::command_line_interface::CommandLineInterface;

#[allow(unused_imports)]
use Project2::avl_tree::AVLTree;

#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use std::env;


#[test]
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

#[test]
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

#[test]
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

#[test]
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
}

# [test]
fn test_rb_count_leafs(){
    let rbtree = RedBlackTree::<u32>::new();
    assert_eq!(rbtree.count_leaf_nodes(), 0);

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

#[test]
fn avl_delete(){
    let mut avltree = AVLTree::<u32>::new();
    avltree.insert(1);
    avltree.insert(2);
    avltree.insert(3);
    avltree.insert(4);
    avltree.insert(5);
    avltree.insert(6);
    avltree.insert(7);
    avltree.delete(5);
    avltree.delete(6);
    avltree.delete(7);
    avltree.delete(3);
    assert_eq!(avltree.get_height(), 2);
}


fn main() {
    let command_line_interface = CommandLineInterface{};
    command_line_interface.run();
}
