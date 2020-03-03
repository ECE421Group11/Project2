// Our code base is adapted from: https://play.rust-lang.org/?gist=d65d605a48d38648737ad2ae38f46434&version=stable
#![allow(unused_must_use)]
use slab::Slab;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Index, IndexMut};
use std::cmp;
extern crate slab;
use crate::pretty_print;

use rustc_serialize::json;

impl<T: Debug + Copy + Display + std::cmp::Ord + rustc_serialize::Decodable> Debug for AVLTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {

        // recursivley print the tree in an ordered fashion
        fn write_recursive<T: Debug + Copy>(avltree: &AVLTree<T>, node: Pointer, f: &mut Formatter){
            if node.is_null(){
                write!(f, "").unwrap();
            }
            else{
                write_recursive(avltree, avltree[node].left, f);
                let left = avltree[node].left;
                let right = avltree[node].right;
                let parent = avltree[node].parent;

                write!(f, "(value = {:?},", avltree[node].value).unwrap();
                
                if left.is_null(){
                    write!(f, "left = NULL, ").unwrap();
                }
                else{
                    write!(f, "left = {:?}, ", avltree[left].value).unwrap();
                }

                if right.is_null(){
                    write!(f, "right = NULL, ").unwrap();
                }
                else{
                    write!(f, "right = {:?}, ", avltree[right].value).unwrap();
                }

                if parent.is_null(){
                    write!(f, "parent = NULL").unwrap();
                }
                else{
                    write!(f, "parent = {:?}", avltree[parent].value).unwrap();
                }

                write!(f, "), \n").unwrap();

                write_recursive(avltree, avltree[node].right, f);

            }
        }

        fn write_pretty(tree: &pretty_print::Tree<i32, f32>, f: &mut Formatter) {
            if tree.root.is_none() {
                write!(f, "[empty]\n");
            } else {
                let mut v: Vec<pretty_print::DisplayElement> = Vec::new();
                tree.display(tree.root, pretty_print::Side::Up, &mut v, f);
            }
        }

        let binary_tree: pretty_print::Tree<i32, f32> = self.create_binary_tree();
        write_pretty(&binary_tree, f);
        write!(f, "In order traversal:(\n")?;
        write_recursive(&self, self.root, f);
        write!(f, ")")?;
        
        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Pointer(usize);
impl Pointer {
    #[inline]
    fn null() -> Pointer {
        Pointer(!0)
    }
    
    #[inline]
    fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> Index<Pointer> for AVLTree<T> {
    type Output = Node<T>;
    
    fn index(&self, index: Pointer) -> &Node<T> {
        &self.slab[index.0]
    }
}
// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> IndexMut<Pointer> for AVLTree<T> {
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
        &mut self.slab[index.0]
    }
}

pub struct Node<T> {
    pub value: T,
    right: Pointer,
    left: Pointer,
    parent: Pointer,
}

pub struct AVLTree<T> {
    slab: Slab<Node<T>>,
    pub root: Pointer,
}

fn addBinaryNodes<T: std::cmp::PartialOrd + std::cmp::Ord + Copy + Debug + rustc_serialize::Decodable + Display + std::string::ToString>(mut encoded: &mut String, avltree: &AVLTree<T>, node: Pointer) {
    if node.is_null(){
        return;
    }
    else{
        addBinaryNodes(&mut encoded, avltree, avltree[node].right);

        let left = avltree[node].left;
        let right = avltree[node].right;
        let parent = avltree[node].parent;
        let Pointer(left_int) = left;
        let Pointer(right_int) = right;
        let Pointer(up_int) = parent;

        // start the node encoding
        encoded.push_str("{\"key\":");
        encoded.push_str(&avltree[node].value.to_string());
        encoded.push_str(",\"value\":3,");
        
        if left.is_null(){
            encoded.push_str("\"left\":null,");
        }
        else{
            encoded.push_str("\"left\":");
            encoded.push_str(&left_int.to_string());
            encoded.push_str(",");
        }

        if right.is_null(){
            encoded.push_str("\"right\":null,");
        }
        else{
            encoded.push_str("\"right\":");
            encoded.push_str(&right_int.to_string());
            encoded.push_str(",");
        }

        if parent.is_null() {
            encoded.push_str("\"up\":null");
        } else {
            encoded.push_str("\"up\":");
            encoded.push_str(&up_int.to_string());
        }         

        // node cap
        encoded.push_str("},");

        addBinaryNodes(&mut encoded, avltree, avltree[node].left);
        return;
    }
}

fn recursive_reindex<T: std::cmp::PartialOrd + std::cmp::Ord + Copy + Debug + rustc_serialize::Decodable + Display + std::string::ToString>(avltree: &AVLTree<T>, node: Pointer, reindexed: &mut AVLTree<T>){
    if !node.is_null(){
        recursive_reindex(avltree, avltree[node].right, reindexed);
        reindexed.insert(avltree[node].value);
        recursive_reindex(avltree, avltree[node].left, reindexed);
    }
}

impl<T: std::cmp::PartialOrd + std::cmp::Ord + Copy + Debug + rustc_serialize::Decodable + std::fmt::Display + std::string::ToString> AVLTree<T> {
    // Returns a new doubly linked list.
    pub fn new() -> Self {
        AVLTree {
            slab: Slab::new(),
            root: Pointer::null(),
        }
    }

    // Returns true if tree is empty, false otherwise
    pub fn is_empty(&self) -> bool{
        return self.root.is_null();
    }

    // Returns height of tree
    pub fn get_height(&self) -> u32{
        return self.get_height_from_node(self.root);
    }

    // Returns balance factor used to determine balance of AVL tree. Neg number = left heavy, pos number = right heavy
    pub fn get_balance_factor(&self, node: Pointer) -> i32{
        return self.get_height_from_node(self[node].right) as i32 - self.get_height_from_node(self[node].left) as i32;
    }

    // Returns height below node passed as argument
    pub fn get_height_from_node(&self, node: Pointer) -> u32{
        if node.is_null(){
            return 0;
        }
        else{
            let left = self.get_height_from_node(self[node].left);
            let right = self.get_height_from_node(self[node].right);
            return cmp::max(left, right) + 1;
        }
    }

    // Insert node with value val into tree
    pub fn insert(&mut self, val: T){
        if self.root.is_null(){
            self.root = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: Pointer::null(),
            }));
        }
        else{
            self.insert_below_node(val, self.root);
        }
    }

    pub fn insert_below_node(&mut self, val: T, node: Pointer){
        let nodeValue = self[node].value;
        let left = self[node].left;
        let right = self[node].right;

        if val == nodeValue{
            println!("Duplicate node values");
            return;
        }
        else if val > nodeValue{
            if right.is_null(){
                self[node].right = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                }));
                // Fix tree
                self.rebalance(node);
            }
            else{
                self.insert_below_node(val, right);
            }
        }
        else if val < nodeValue{
            if left.is_null(){
                self[node].left = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                }));
                // Fix tree
                self.rebalance(node);
            }
            else{
                self.insert_below_node(val, left);
            }
        }
    }

    pub fn left_rotate(&mut self, current: Pointer){
        let right = self[current].right;

        if right.is_null(){
            println!("NULL");
            return;
        }

        let right_left = self[right].left;
        let parent = self[current].parent;

        // set W's right child to be B
        self[current].right = right_left;

        if !right_left.is_null(){
            self[right_left].parent = current;
        }

        // setting W's parent to be V
        self[current].parent = right;
        self[right].left = current;

        // Set V's parent to be W's old parent
        self[right].parent = parent;

        if parent.is_null(){
            self.root = right;
        }
        else{
            let parent_right = self[parent].right;
            if !parent_right.is_null(){
                if self[parent_right].value == self[current].value{ // set V to parent right
                    self[parent].right = right;
                }
                else{ // set V to parent left
                    self[parent].left = right;
                }
            }
            else{ // set V to parent left
                self[parent].left = right;
            }
        }
    }

    pub fn right_rotate(&mut self, current: Pointer){
        let left = self[current].left;

        if left.is_null(){
            return;
        }

        let left_right = self[left].right;
        let parent = self[current].parent;

        // set V's left child to be B
        self[current].left = left_right;

        if !left_right.is_null(){
            self[left_right].parent = current;
        }

        // setting V's parent to be W
        self[current].parent = left;
        self[left].right = current;

        // Set W's parent to be V's old parent
        self[left].parent = parent;

        if parent.is_null(){
            self.root = left;
        }
        else{
            let parent_left = self[parent].left;
            if !parent_left.is_null(){
                if self[parent_left].value == self[current].value{ // set W to parent left
                    self[parent].left = left;
                }
                else{ // set W to parent right
                    self[parent].right = left;
                }
            }
            else{ // set W to parent right
                self[parent].right = left;
            }
        }
    }

    // Rebalance to ensure AVL tree properties are maintained
    pub fn rebalance(&mut self, mut node: Pointer){
        while !node.is_null(){
            let bal = self.get_balance_factor(node);
            if bal < -1{                
                // Left heavy so rotate right
                let y = self[node].left;
                let bal_y = self.get_balance_factor(y);
                if bal_y > 0 {
                    // Need left-right rotate
                    self.left_rotate(y);
                }
                self.right_rotate(node);
            }
            else if bal > 1{
                // Right heavy so rotate left
                let y = self[node].right;
                let bal_y = self.get_balance_factor(y);
                if bal_y < 0 {
                    // Need right-left rortate
                    self.right_rotate(y);
                }
                self.left_rotate(node);
            }
            node = self[node].parent;
        }
    }

    pub fn get_node(&self, val: T) -> Pointer{
        let node = self.get_node_from_node(self.root, val);

        if node.is_null(){
            println!("Node does not exist!");
            return Pointer::null();
        }
        return node;
    }

    pub fn get_node_from_node(&self, node: Pointer, val:T) -> Pointer{
        if node.is_null(){
            return Pointer::null();
        }
        else{
            if self[node].value == val{
                return node;
            }
            else if val > self[node].value{
                return self.get_node_from_node(self[node].right, val);
            }
            else{
                return self.get_node_from_node(self[node].left, val);
            }
        }
    }

    // Delete node with value val
    pub fn delete(&mut self, val: T) /*-> T*/{
        let remove = self.get_node(val);
        if remove.is_null(){
            return;
        }
        let parent = self[remove].parent;
        // Three cases no children, 1 children, 2 children
        if self[remove].left.is_null() && self[remove].right.is_null(){
            // No children just delete node
            if parent.is_null(){
                self.root = Pointer::null();
            }
            else if self[self[remove].parent].left == remove{
                self[parent].left = Pointer::null();
            }
            else{
                self[parent].right = Pointer::null();
            }
        }
        else if !self[remove].left.is_null() && !self[remove].right.is_null(){
            // Two childre need to find replacement node
            let replace = self.min_of_right(remove);
            if parent.is_null(){
                let lefttree = self[remove].left;
                self.root = replace;
                self[replace].parent = Pointer::null();
                self[replace].left = self[remove].left;
                self[lefttree].parent = replace;
                if self[remove].right == replace{
                    self[replace].right = Pointer::null();
                }
                else{
                    let righttree = self[remove].right;
                    self[replace].right = self[remove].right;
                    self[righttree].parent = replace;
                }
            }
            else if self[parent].left == remove{
                let lefttree = self[remove].left;
                self[parent].left = replace;
                self[replace].parent = parent;
                self[replace].left = self[remove].left;
                self[lefttree].parent = replace;
                if self[remove].right == replace{
                    self[replace].right = Pointer::null();
                }
                else{
                    let righttree = self[remove].right;
                    self[replace].right = self[remove].right;
                    self[righttree].parent = replace;
                }
            }
            else{
                let lefttree = self[remove].left;
                self[parent].right = replace;
                self[replace].parent = parent;
                self[replace].left = self[remove].left;
                self[lefttree].parent = replace;
                if self[remove].right == replace{
                    self[replace].right = Pointer::null();
                }
                else{
                    let righttree = self[remove].right;
                    self[replace].right = self[remove].right;
                    self[righttree].parent = replace;
                }
            }
        }
        else{
            // One child, replace remove with child
            if parent.is_null(){
                if self[remove].left.is_null(){
                    let right = self[remove].right;
                    self.root = right;
                    self[right].parent = Pointer::null();
                }
                else{
                    let left = self[remove].left;
                    self.root = self[remove].left;      
                    self[left].parent = Pointer::null();
                }
            }
            else if !self[remove].left.is_null(){
                if self[self[remove].parent].left == remove{
                    let left = self[remove].left;
                    self[parent].left = left;
                    self[left].parent = parent;
                }
                else{
                    let left = self[remove].left;
                    self[parent].right = left;
                    self[left].parent = parent;
                }
            }
            else{
                if self[self[remove].parent].left == remove{
                    let right = self[remove].right;
                    self[parent].left = right;
                    self[right].parent = parent;
                }
                else{
                    let right = self[remove].right;
                    self[parent].right = right;
                    self[right].parent = parent;
                }
            }
        }        
        self.rebalance(parent);
    }

    // Return number of leaf nodes is tree
    pub fn count_leaf_nodes(&self) -> u32{
        return self.count_leaf_nodes_from_node(self.root);
    }

    pub fn count_leaf_nodes_from_node(&self, node: Pointer) -> u32{
        if node.is_null(){
            return 0;
        }
        else{
            let left = self.count_leaf_nodes_from_node(self[node].left);
            let right = self.count_leaf_nodes_from_node(self[node].right);
            if left == right && left == 0{
                return 1;
            }
            return left + right;
        }
    }

    // Print tree from left to right
    pub fn print_in_order_traversal(&self){
        println!("{:?}", self);
    }

    pub fn reIndex(&self) -> Self{
        let mut reindexed = AVLTree {
            slab: Slab::new(),
            root: Pointer::null(),
        };

        recursive_reindex(&self, self.root, &mut reindexed);
        return reindexed
    }

    pub fn create_binary_tree(&self) -> pretty_print::Tree<i32, f32> {
        // if the tree is empty, return an empty encoded binary tree
        if self.root.is_null() {
            let encoded =  r#"{"root":null,"store":[]}"#;
            let tree: pretty_print::Tree<i32, f32> = json::decode(&encoded).unwrap();
            return tree;
        }

        // must re-index the tree from largest to smallest
        let avltree = self.reIndex();

        // create the encoded string starting from root
        let mut encoded = String::from("{\"root\":");
        let Pointer(root_int) = avltree.root;
        encoded.push_str(&root_int.to_string());
        encoded.push_str(",\"store\":[");

        // recurse to add rest of nodes
        addBinaryNodes(&mut encoded, &avltree, avltree.root);

        // remove last ","
        encoded.truncate(encoded.len() - 1);

        // cap formatting
        encoded.push_str("]}");
        let tree: pretty_print::Tree<i32, f32> = json::decode(&encoded).unwrap();
        return tree;
    }

    // Find smallest value in right tree of head, used for delete
    pub fn min_of_right(&self, head: Pointer)-> Pointer{
        let mut current = self[head].right;
        if current.is_null(){
            return current;
        }

        while !self[current].left.is_null(){
            current = self[current].left;
        }
        return current;
    }

}