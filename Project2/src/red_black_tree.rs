// Disclaimer: Our ECE 421 group worked on this together
// Our code base is adapted from: https://play.rust-lang.org/?gist=d65d605a48d38648737ad2ae38f46434&version=stable

use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;
extern crate slab;

impl<T: fmt::Debug + Copy + fmt::Debug> fmt::Debug for RedBlackTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        let mut n = self.root;

        fn write_recursive<T: fmt::Debug + Copy>(rbTree: &RedBlackTree<T>, node: Pointer, f: &mut fmt::Formatter){
            if node.is_null(){
                write!(f, "");
            }
            else{
                write_recursive(rbTree, rbTree[node].left, f);
                let left = rbTree[node].left;
                let right = rbTree[node].right;
                let parent = rbTree[node].parent;

                write!(f, "(val = {:?}, color = {:?}, ", rbTree[node].value, rbTree[node].color);
                
                if left.is_null(){
                    write!(f, "left = NULL, ");
                }
                else{
                    write!(f, "left = {:?}, ", rbTree[left].value);
                }

                if right.is_null(){
                    write!(f, "right = NULL, ");
                }
                else{
                    write!(f, "right = {:?}, ", rbTree[right].value);
                }

                if parent.is_null(){
                    write!(f, "parent = NULL, ");
                }
                else{
                    write!(f, "parent = {:?},", rbTree[parent].value);
                }

                write!(f, "), \n");

                write_recursive(rbTree, rbTree[node].right, f);

            }
        }

        write!(f, "Tree(")?;
        write_recursive(&self, self.root, f);
        write!(f, ")")?;
        
        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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

impl<T: PartialOrd + Copy> RedBlackTree<T> {
    // Returns a new doubly linked list.
    pub fn new() -> Self {
        RedBlackTree {
            slab: Slab::new(),
            root: Pointer::null(),
        }
    }

    pub fn len(&self) -> usize{
        return self.slab.len();
    }

    pub fn is_empty(&self) -> bool{
        return self.len() == 0;
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
            self.insert_below_node(val, self.root);
        }
    }

    pub fn insert_below_node(&mut self, val: T, node: Pointer){
        let nodeValue = self[node].value;
        let left = self[node].left;
        let right = self[node].right;

        if val == nodeValue{
            panic!("Duplicate node values");
        }
        else if val > nodeValue{
            if right.is_null(){
                self[node].right = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                    color: NodeColor::Black,
                }));
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
                    color: NodeColor::Black,
                }));
            }
            else{
                self.insert_below_node(val, left);
            }
        }
    }

    pub fn left_rotate(&mut self, current: Pointer){
        let right = self[current].right;

        let right_left = self[right].left;
        let parent = self[current].parent;

        // set W's right child to be B
        self[current].right = right_left;
        self[right_left].parent = current;

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
            if self[parent_right].value == self[current].value{ // set V to parent right
                self[parent].right = right;
            }
            else{ // set V to parent left
                self[parent].left = right;
            }
        }
    }

    pub fn right_rotate(&mut self, current: Pointer){
        let left = self[current].left;

        let left_right = self[left].right;
        let parent = self[current].parent;

        // set V's left child to be B
        self[current].left = left_right;
        self[left_right].parent = current;

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
            if self[parent_left].value == self[current].value{ // set W to parent left
                self[parent].left = left;
            }
            else{ // set W to parent right
                self[parent].right = left;
            }
        }
    }

}



fn main() {
    
}