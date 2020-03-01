// Disclaimer: Our ECE 421 group worked on this together
// Our code base is adapted from: https://play.rust-lang.org/?gist=d65d605a48d38648737ad2ae38f46434&version=stable

use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::cmp;
extern crate slab;

impl<T: fmt::Debug + Copy + fmt::Debug> fmt::Debug for RedBlackTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn write_recursive<T: fmt::Debug + Copy>(rbTree: &RedBlackTree<T>, node: Pointer, f: &mut fmt::Formatter){
            if node.is_null(){
                write!(f, "").unwrap();
            }
            else{
                write_recursive(rbTree, rbTree[node].left, f);
                let left = rbTree[node].left;
                let right = rbTree[node].right;
                let parent = rbTree[node].parent;

                write!(f, "(value = {:?}, color = {:?}, ", rbTree[node].value, rbTree[node].color).unwrap();
                
                if left.is_null(){
                    write!(f, "left = NULL, ").unwrap();
                }
                else{
                    write!(f, "left = {:?}, ", rbTree[left].value).unwrap();
                }

                if right.is_null(){
                    write!(f, "right = NULL, ").unwrap();
                }
                else{
                    write!(f, "right = {:?}, ", rbTree[right].value).unwrap();
                }

                if parent.is_null(){
                    write!(f, "parent = NULL").unwrap();
                }
                else{
                    write!(f, "parent = {:?}", rbTree[parent].value).unwrap();
                }

                write!(f, "), \n").unwrap();

                write_recursive(rbTree, rbTree[node].right, f);

            }
        }

        write!(f, "In order traversal:(\n")?;
        write_recursive(&self, self.root, f);
        write!(f, ")")?;
        
        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Pointer(usize);

impl Pointer {
    #[inline]
    pub fn null() -> Pointer {
        Pointer(!0)
    }
    
    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> Index<Pointer> for RedBlackTree<T> {
    type Output = Node<T>;
    
    fn index(&self, index: Pointer) -> &Node<T> {
        &self.slab[index.0]
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> IndexMut<Pointer> for RedBlackTree<T> {
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
        &mut self.slab[index.0]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub right: Pointer,
    pub left: Pointer,
    pub parent: Pointer,
    pub color: NodeColor,
}

pub struct RedBlackTree<T> {
    pub slab: Slab<Node<T>>,
    pub root: Pointer,
}

impl<T: PartialOrd + Copy + fmt::Debug> RedBlackTree<T> {
    // Returns a new doubly linked list.
    pub fn new() -> Self {
        RedBlackTree {
            slab: Slab::new(),
            root: Pointer::null(),
        }
    }

    pub fn is_empty(&self) -> bool{
        return self.root.is_null();
    }
    
    pub fn print_in_order_traversal(&self){
        println!("{:?}", self);
    }

    pub fn get_node(&self, val: T) -> Pointer{
        let node = self.get_node_from_node(self.root, val);

        if node.is_null(){
            println!("Node does not exist!")
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
    
    pub fn get_height(&self) -> u32{
        return self.get_height_from_node(self.root);
    }


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

    pub fn count_leaf_nodes(&self) -> u32{
        return self.count_leaf_nodes_from_node(self.root);
    }

    pub fn count_leaf_nodes_from_node(&self, node: Pointer) -> u32{
        if node.is_null(){
            return 0;
        }
        else{
            let left = self.get_height_from_node(self[node].left);
            let right = self.get_height_from_node(self[node].right);
            if left == right && left == 0{
                return 1;
            }
            return left + right;
        }
    }

    pub fn get_uncle(&self, node: Pointer) -> Pointer{
        let parent = self[node].parent;
        if parent.is_null(){
            return Pointer::null();
        }

        let grandparent = self[parent].parent;

        if grandparent.is_null(){
            return Pointer::null();
        }

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;
        
        if grandparent_left.is_null(){
            return Pointer::null();
        }

        if grandparent_right.is_null(){
            return Pointer::null();
        }

        if self[parent].value == self[grandparent_left].value{
            return grandparent_right;
        }

        return grandparent_left;

    }

    pub fn insert_fixup(&mut self, node: Pointer){
        let parent = self[node].parent;
        if self[node].parent.is_null(){
            return self.insert_case1(node);
        }
        
        if self[parent].color == NodeColor::Black{
            return self.insert_case2(node)
        }

        let uncle = self.get_uncle(node);

        if uncle.is_null(){
            return self.insert_case4(node);
        }
        if self[uncle].color == NodeColor::Black{
            return self.insert_case4(node);
        }

        return self.insert_case3(node)

    }

    pub fn insert_case1(&mut self, node: Pointer){
        self[node].color = NodeColor::Black;
    }

    pub fn insert_case2(&mut self, _node: Pointer){
        return
    }

    pub fn insert_case3(&mut self, node: Pointer){
        let parent = self[node].parent;
        let uncle = self.get_uncle(node);
        let grandparent = self[parent].parent;

        self[parent].color = NodeColor::Black;
        self[uncle].color = NodeColor::Black;
        self[grandparent].color = NodeColor::Red;

        self.insert_fixup(grandparent);
    }

    pub fn transfer_and_remove_ownership(&mut self, val: T){
        let mut newTree = RedBlackTree::new();
        for i in 0..self.slab.len(){
            if self.slab[i].value != val{
                newTree.insert(self.slab[i].value);
            }
        }
        self.slab = newTree.slab;
        self.root = newTree.root;
    }

    pub fn insert_case4(&mut self, node: Pointer){

        let parent = self[node].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;
        let parent_right = self[parent].right;

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;

        let mut n = node;

        if !parent_right.is_null() && !grandparent_left.is_null() && (self[n].value == self[parent_right].value) && (self[parent].value == self[grandparent_left].value){
            self.left_rotate(parent);
            n = self[n].left;
        }
        else if !parent_left.is_null() && !grandparent_right.is_null() && (self[n].value == self[parent_left].value) && (self[parent].value == self[grandparent_right].value){
            self.right_rotate(parent);
            n = self[n].right;
        }

        self.insert_case4_part2(n);
    }

    pub fn insert_case4_part2(&mut self, node: Pointer){
        let parent = self[node].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;

        if !parent_left.is_null() && self[node].value == self[parent_left].value{
            self.right_rotate(grandparent);
        }
        else{
            self.left_rotate(grandparent);
        }

        self[parent].color = NodeColor::Black;
        self[grandparent].color = NodeColor::Red;
    }

    pub fn delete(&mut self, val: T){
        if !self.get_node(val).is_null(){
            self.transfer_and_remove_ownership(val);
        }
    }

    pub fn insert(&mut self, val: T){
        if self.root.is_null(){
            self.root = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: Pointer::null(),
                color: NodeColor::Black,
            }));
        }
        else{
            let new_node = self.insert_below_node(val, self.root);
            if !new_node.is_null(){
                self.insert_fixup(new_node);
            }
        }
    }

    pub fn insert_below_node(&mut self, val: T, node: Pointer) -> Pointer{
        let nodeValue = self[node].value;
        let left = self[node].left;
        let right = self[node].right;

        if val == nodeValue{
            println!("Duplicate node values");
            return Pointer::null();
        }
        else if val > nodeValue{
            if right.is_null(){
                self[node].right = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                    color: NodeColor::Red,
                }));
                return self[node].right;
            }
            else{
                return self.insert_below_node(val, right);
            }
        }
        else{
            if left.is_null(){
                self[node].left = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                    color: NodeColor::Red,
                }));
                return self[node].left;
            }
            else{
                return self.insert_below_node(val, left);
            }
        }
    }

    pub fn left_rotate(&mut self, current: Pointer){
        let right = self[current].right;

        if right.is_null(){
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

}