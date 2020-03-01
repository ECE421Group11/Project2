#![allow(non_snake_case)]
//extern crate red_black_tree;
use Project2::red_black_tree::RedBlackTree;
use Project2::red_black_tree::NodeColor;
use Project2::red_black_tree::Pointer;
use std::env;

use Project2::avl_tree::AVLTree;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
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

<<<<<<< HEAD
=======
<<<<<<< HEAD
<<<<<<< HEAD
>>>>>>> 5b74904bed9627a0c0aedc462cc04961af2fffc8
#[test]
fn simple_avl_insert(){
    let mut avltree = AVLTree::<u32>::new();
    avltree.insert(1);
    avltree.insert(2);
    avltree.insert(3);
    assert_eq!(avltree[avltree.root].value, 2);
<<<<<<< HEAD
=======
=======
# [test]
=======
#[test]
>>>>>>> 256595002533caae6087c11aae85143fa6367b76
fn test_rb_count_leafs(){
    let mut rbtree = RedBlackTree::<u32>::new();
    assert_eq!(rbtree.count_leaf_nodes(), 0);
>>>>>>> 5b74904bed9627a0c0aedc462cc04961af2fffc8

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

>>>>>>> 3e796668512b575bf975a252f96f302cdd5202e5


fn main() {
<<<<<<< HEAD
    env::set_var("RUST_BACKTRACE", "1");
=======
<<<<<<< HEAD

    let mut intro = 1;

    // used to test the library
    loop {
        if (intro == 1) {
            // ask user to test a tree type
            println!("\n\nType \"rbt\" or \"avl\" to select the type of tree to test.");
            println!("Type help at any time for a list of available commands.\n");
            intro = 0;
        }
        
        // print the users cursor
        print!("Select Tree > ");
        stdout().flush();

        // get the users input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // match the command to an action
        match command {
            "exit" => return,
            "rbt" => {
                test_rbt();
            },
            "avl" => {
                test_avl();
            },
            "help" => {
                println!("\nAvaliable commands:\n");
                println!("    exit - Exit the program.");
                println!("    rbt  - Test the red black tree library.");
                println!("    avl  - Test the avl tree library.");
                println!("    help - Show available commands.");
                println!();
            },
            command => {
                println!("\nCommand {:?} not recognized. Type help for available commands.\n", command);
            }
        }
    }
}

fn test_rbt() {}
fn test_avl() {}
=======
    env::set_var("RUST_BACKTRACE", "1");
    let mut rbtree = RedBlackTree::<u32>::new();
>>>>>>> 5b74904bed9627a0c0aedc462cc04961af2fffc8

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

<<<<<<< HEAD
    println!("{:?}", rbtree);
=======
    rbtree.print_in_order_traversal();

    rbtree.delete(50);
    rbtree.print_in_order_traversal();

>>>>>>> 5b74904bed9627a0c0aedc462cc04961af2fffc8

    println!("");
    
    let node25 = rbtree[rbtree.root].left;
    
    
    rbtree.right_rotate(node25);
    println!("After right rotate");
    println!("{:?}", rbtree);

<<<<<<< HEAD
    println!("");
    rbtree.left_rotate(node25);
    println!("After left rotate");
    println!("{:?}", rbtree);*/
}
=======
    let mut avltree = AVLTree::<u32>::new();
    avltree.insert(3);
    avltree.insert(2);
    avltree.insert(1);
    assert_eq!(avltree[avltree.root].value, 2);
}
>>>>>>> 3e796668512b575bf975a252f96f302cdd5202e5
>>>>>>> 5b74904bed9627a0c0aedc462cc04961af2fffc8
