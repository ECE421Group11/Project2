// Our code base is adapted from: https://play.rust-lang.org/?gist=d65d605a48d38648737ad2ae38f46434&version=stable

use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::cmp;
extern crate slab;

impl<T: fmt::Debug + Copy + fmt::Debug> fmt::Debug for AVLTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn write_recursive<T: fmt::Debug + Copy>(avlTree: &AVLTree<T>, node: Pointer, f: &mut fmt::Formatter){
            if node.is_null(){
                write!(f, "").unwrap();
            }
            else{
                write_recursive(avlTree, avlTree[node].left, f);
                write!(f, "({:?}), ", avlTree[node].value).unwrap();
                write_recursive(avlTree, avlTree[node].right, f);
            }
        }

        write!(f, "In order traversal: (\n")?;
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

impl<T: PartialOrd + Copy + fmt::Debug> AVLTree<T> {
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
                self.right_rotate(node);
            }
            else if bal > 1{
                // Right heavy so rotate left
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
                    self.root = self[remove].right;
                }
                else{
                    self.root = self[remove].left;
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
            let left = self.get_height_from_node(self[node].left);
            let right = self.get_height_from_node(self[node].right);
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