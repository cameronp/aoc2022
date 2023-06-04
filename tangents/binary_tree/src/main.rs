
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct TreeNode {
  val: i32,
  left: Option<TreeNodeRef>,
  right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val: val,
            left: None,
            right: None
        }
    }

    // return the value of the node
    fn get_val(&self) -> i32 {
        self.val
    }

    //return the value of the left child
    fn get_left_val(&self) -> i32 {
        self.left.as_ref().unwrap().borrow().val
    }

    //return the value of the right child
    fn get_right_val(&self) -> i32 {
        self.right.as_ref().unwrap().borrow().val
    }

    fn set_left(&mut self, t: TreeNode ) -> () {
        self.left = Some(Rc::new(RefCell::new(t)));
    }

    fn set_right(&mut self, t: TreeNode ) -> () {
        self.right = Some(Rc::new(RefCell::new(t)));
    }

    fn get_left(&self) -> Option<TreeNodeRef> {
        self.left.clone()
    }

    fn get_right(&self) -> Option<TreeNodeRef> {
        self.right.clone()
    }

}

#[test]
fn test_set_get_new() {
    let mut root = TreeNode::new(5);
    let mut n1: TreeNode = TreeNode::new(3);
    let mut n2: TreeNode = TreeNode::new(7);

    root.set_left(n1);
    root.set_right(n2);

    assert_eq!(root.val, 5);
    assert_eq!(root.get_left().unwrap().borrow().val, 3);
    assert_eq!(root.get_right().unwrap().borrow().val, 7);

}

fn main() {
    println!("Hello, world!");
}
