use std::rc::Rc;

type AvlTreeImpl<T> = Option<Rc<TreeNode<T>>>;

const BALANCE_FACTOR: u32 = 2;

#[derive(Clone, Debug)]
pub struct ImmutAvlTree<T: PartialOrd + Clone> {
    root: AvlTreeImpl<T>,
}

#[derive(Clone, Debug)]
struct TreeNode<T: PartialOrd + Clone> {
    val: T,
    height: u32,
    left: AvlTreeImpl<T>,
    right: AvlTreeImpl<T>,
}

impl<T: PartialOrd + Clone> TreeNode<T> {
    fn new(val: T, left: AvlTreeImpl<T>, right: AvlTreeImpl<T>) -> AvlTreeImpl<T> {
        let mut node = TreeNode {
            val,
            height: 1,
            left,
            right,
        };
        node.update_height();
        Some(Rc::new(node))
    }
    fn get_height(node: &AvlTreeImpl<T>) -> u32 {
        match node {
            Some(node) => node.height,
            None => 0,
        }
    }
    fn balance_tree(val: T, left: AvlTreeImpl<T>, right: AvlTreeImpl<T>) -> AvlTreeImpl<T> {
        let left_height = TreeNode::get_height(&left);
        let right_height = TreeNode::get_height(&right);
        if left_height > right_height + BALANCE_FACTOR {
            let left_node = left.as_ref().unwrap();
            let ll_height = TreeNode::get_height(&left_node.left);
            let lr_height = TreeNode::get_height(&left_node.right);
            if ll_height > lr_height {
                TreeNode::new(
                    left_node.val.clone(),
                    left_node.left.clone(),
                    TreeNode::new(val, left_node.right.clone(), right),
                )
            } else {
                let lr_node = left_node.right.as_ref().unwrap();
                TreeNode::new(
                    lr_node.val.clone(),
                    TreeNode::new(
                        left_node.val.clone(),
                        left_node.left.clone(),
                        lr_node.left.clone(),
                    ),
                    TreeNode::new(val, lr_node.right.clone(), right),
                )
            }
        } else if right_height > left_height + BALANCE_FACTOR {
            let right_node = right.as_ref().unwrap();
            let rl_height = TreeNode::get_height(&right_node.left);
            let rr_height = TreeNode::get_height(&right_node.right);
            if rr_height > rl_height {
                TreeNode::new(
                    right_node.val.clone(),
                    TreeNode::new(val, left, right_node.left.clone()),
                    right_node.right.clone(),
                )
            } else {
                let rl_node = right_node.left.as_ref().unwrap();
                TreeNode::new(
                    rl_node.val.clone(),
                    TreeNode::new(val, left, rl_node.left.clone()),
                    TreeNode::new(
                        rl_node.val.clone(),
                        rl_node.right.clone(),
                        right_node.right.clone(),
                    ),
                )
            }
        } else {
            TreeNode::new(val, left, right)
        }
    }
    fn do_insert(&self, val: T) -> AvlTreeImpl<T> {
        if val < self.val {
            if let Some(ln) = &self.left {
                TreeNode::balance_tree(self.val.clone(), ln.do_insert(val), self.right.clone())
            } else {
                TreeNode::balance_tree(self.val.clone(), TreeNode::new(val, None, None), self.right.clone())
            }
        } else if val > self.val {
            if let Some(rn) = &self.right {
                TreeNode::balance_tree(self.val.clone(), self.left.clone(), rn.do_insert(val))
            } else {
                TreeNode::balance_tree(self.val.clone(), self.left.clone(), TreeNode::new(val, None, None))
            }
        } else {
            TreeNode::new(val, self.left.clone(), self.right.clone())
        }
    }
    fn update_height(&mut self) {
        let left_height = TreeNode::get_height(&self.left);
        let right_height = TreeNode::get_height(&self.right);
        self.height = 1 + std::cmp::max(left_height, right_height);
    }
    fn contains(&self, val: T) -> bool {
        if val < self.val {
            if let Some(ln) = &self.left {
                ln.contains(val)
            } else {
                false
            }
        } else if val > self.val {
            if let Some(rn) = &self.right {
                rn.contains(val)
            } else {
                false
            }
        } else {
            true
        }
    }
}

impl<T: PartialOrd + Clone> ImmutAvlTree<T> {
    pub fn new() -> Self {
        ImmutAvlTree { root: None }
    }
    pub fn insert(&self, val: T) -> Self {
        match self.root {
            None => ImmutAvlTree {
                root: TreeNode::new(val, None, None),
            },
            Some(ref root) => ImmutAvlTree {
                root: root.as_ref().do_insert(val),
            },
        }
    }
    pub fn delete(&self, val: T) -> Self {
        ImmutAvlTree { root: None }
    }
    pub fn contains(&self, val: T) -> bool {
        if let Some(root) = &self.root {
            root.contains(val)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tree_node() {
        let node = TreeNode::new(1, None, None);
        assert_eq!(TreeNode::get_height(&node), 1);
        let node2 = node.as_ref().unwrap().do_insert(2);
        let node3 = node2.as_ref().unwrap().do_insert(3);
        for i in 1..4 {
            assert!(node3.as_ref().unwrap().as_ref().contains(i));
        }
        assert!(!node3.unwrap().as_ref().contains(0));
    }

    #[test]
    fn test_avl_tree() {
        let mut tree = ImmutAvlTree::new();
        for i in 1..100 {
            tree = tree.insert(i);
        }
        assert_eq!(TreeNode::get_height(&tree.root), 8);
        assert!(tree.contains(1));
        assert!(tree.contains(99));
        assert!(!tree.contains(100));
        //print!("{:#?}", tree);
    }
}
