use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<TreeNode<u32>>>;
type AVLTree= Option<Tree>;
struct TreeNode<T> {
 pub key: T,
 pub parent: AVLTree,
 left: AVLTree,
 right: AVLTree,
}

fn insert(){

}
fn delete(){

}
fn count(){

}
fn height(){

}
fn print(){
    
}
fn is_empty(){

}