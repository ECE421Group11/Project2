use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;
extern crate slab;

impl<T: fmt::Debug + Copy + fmt::Debug> fmt::Debug for AVLTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        let mut n = self.root;

        fn write_recursive<T: fmt::Debug + Copy>(avlTree: &AVLTree<T>, node: Pointer, f: &mut fmt::Formatter){
            if node.is_null(){
                write!(f, "");
            }
            else{
                write_recursive(avlTree, avlTree[node].left, f);
                write!(f, "({:?}), ", avlTree[node].value);
                write_recursive(avlTree, avlTree[node].right, f);
            }
        }

        write!(f, "Tree(")?;
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

impl<T: PartialOrd + Copy> AVLTree<T> {
    // Returns a new doubly linked list.
    pub fn new() -> Self {
        AVLTree {
            slab: Slab::new(),
            root: Pointer::null(),
        }
    }

    // Returns total number of nodes in tree
    pub fn len(&self) -> usize{
        return self.slab.len();
    }

    // Returns true if tree is empty, false otherwise
    pub fn is_empty(&self) -> bool{
        return self.len() == 0;
    }

    // Returns height of tree
    pub fn get_height(&self) -> u32{
        return self.get_height_from_node(self.root);
    }

    // Returns balance factor used to determine balance of AVL tree. Neg number = left heavy, pos number = right heavy
    pub fn get_balance_factor(&self, node: Pointer) -> i32{
        return (self.get_height_from_node(self[node].right) as i32 - self.get_height_from_node(self[node].left) as i32);
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
        //self.rebalance();
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
                }));
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
        while(!node.is_null()){
            println!("Balance Facor {:?}", self.get_balance_factor(node));
            if self.get_balance_factor(node) < -1{
                // Left heavy so rotate right
                self.right_rotate(node);
            }
            else if self.get_balance_factor(node) > 1{
                // Right heavy so rotate left
                self.left_rotate(node);
            }
            node = self[node].parent;
        }
    }

    pub fn delete(&mut self, val: T){
        
    }

}