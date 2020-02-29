use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;

type Tree = Rc<RefCell<TreeNode<u32>>>;

type AVLTree<T:Ord + std::fmt::Debug> = Option<Box<TreeNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct TreeNode<T> {
 pub key: T,
 pub parent: AVLTree<T>,
 left: AVLTree<T>,
 right: AVLTree<T>,
}

pub struct AVLTreeRoot<T:Ord + std::fmt::Debug>{
    root: AVLTree<T>,
}

impl<T:Ord + std::fmt::Debug> AVLTreeRoot<T>{
    pub fn new() -> Self{
        Self{root:None}
    }
    pub fn insert(&mut self, value:T){
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree{
            // If less if value is less than current nodes vale, right if value is greater 
            // than current node value, panic if value already exists. Until get to leaf
            match current_node.key.cmp(&value){
                Ordering::Less => current_tree = &mut current_node.right,
                Ordering::Equal => panic!("Value already in tree"),
                Ordering::Greater => current_tree = &mut current_node.left,
            }
        }
        // Now at leaf so create new
        *current_tree = Some(Box::new(TreeNode{
            key: value,
            parent: None, 
            left: None,
            right: None,
        }));
    }

    pub fn delete(){

    }
    
    pub fn count(){

    }

    pub fn height(){

    }

    pub fn print(&mut self){
        println!("PRINT TREE");
        
        let mut current_tree = &mut self.root;
        if let Some(current_node) = current_tree{
        println!("{:?}", current_node);
        }
    }

    pub fn is_empty(){

    }
}