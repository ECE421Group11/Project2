#![allow(non_snake_case)]
//extern crate red_black_tree;
use Project2::red_black_tree::RedBlackTree;

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
    assert_eq!(rbtree.get_height(), 3)
}

# [test]
fn test_root_rotations(){
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);

    assert_eq!(rbtree[rbtree.root].value, 50);

    rbtree.right_rotate(rbtree.root);
    assert_eq!(rbtree[rbtree.root].value, 25);

    rbtree.left_rotate(rbtree.root);
    assert_eq!(rbtree[rbtree.root].value, 50);

}

# [test]
fn middle_tree_rotation(){
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
    rbtree.insert(25);
    rbtree.insert(15);
    rbtree.insert(35);
    rbtree.insert(75);
    rbtree.insert(60);
    rbtree.insert(90);
    
    let node25 = rbtree[rbtree.root].left;
    
    rbtree.right_rotate(node25);

    assert_eq!(rbtree[node25].left.is_null(), true);
    assert_eq!(rbtree[rbtree[node25].right].value, 35);

    rbtree.left_rotate(node25);
    assert_eq!(rbtree[node25].left.is_null(), true);
    assert_eq!(rbtree[node25].right.is_null(), true);
}

fn main() {
    let mut rbtree = RedBlackTree::<u32>::new();

    rbtree.insert(50);
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
    println!("{:?}", rbtree);
}
