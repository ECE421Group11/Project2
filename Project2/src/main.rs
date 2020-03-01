#![allow(non_snake_case)]
//extern crate red_black_tree;
use Project2::red_black_tree::RedBlackTree;
use Project2::avl_tree::AVLTree;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::env;

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

fn main() {

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
