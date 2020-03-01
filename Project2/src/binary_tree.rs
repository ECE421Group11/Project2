use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::cmp;
extern crate slab;

impl<T: fmt::Debug + Copy + fmt::Debug> fmt::Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn write_recursive<T: fmt::Debug + Copy>(binaryTree: &BinaryTree<T>, node: Pointer, f: &mut fmt::Formatter){
            if node.is_null(){
                write!(f, "").unwrap();
            }
            else{
                write_recursive(binaryTree, binaryTree[node].left, f);
                write!(f, "({:?}), ", binaryTree[node].value).unwrap();
                write_recursive(binaryTree, binaryTree[node].right, f);
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
impl<T> Index<Pointer> for BinaryTree<T> {
    type Output = Node<T>;
    
    fn index(&self, index: Pointer) -> &Node<T> {
        &self.slab[index.0]
    }
}
// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> IndexMut<Pointer> for BinaryTree<T> {
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

pub struct BinaryTree<T> {
    slab: Slab<Node<T>>,
    pub root: Pointer,
}

impl<T: PartialOrd + Copy + fmt::Debug> BinaryTree<T> {
    // Returns a new doubly linked list.
    pub fn new() -> Self {
        BinaryTree {
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
            }
            else{
                self.insert_below_node(val, left);
            }
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
}