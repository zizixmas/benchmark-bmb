// Binary trees benchmark
// Measures: memory allocation, tree traversal, recursion

struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(depth: i32) -> Box<TreeNode> {
        if depth > 0 {
            Box::new(TreeNode {
                left: Some(TreeNode::new(depth - 1)),
                right: Some(TreeNode::new(depth - 1)),
            })
        } else {
            Box::new(TreeNode {
                left: None,
                right: None,
            })
        }
    }

    fn check(&self) -> i32 {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => 1 + left.check() + right.check(),
            _ => 1,
        }
    }
}

fn main() {
    let min_depth = 4;
    let max_depth = 14;
    let stretch_depth = max_depth + 1;

    // Stretch tree
    let stretch_tree = TreeNode::new(stretch_depth);
    println!("stretch tree of depth {}\t check: {}", stretch_depth, stretch_tree.check());

    // Long-lived tree
    let long_lived_tree = TreeNode::new(max_depth);

    let mut depth = min_depth;
    while depth <= max_depth {
        let iterations = 1 << (max_depth - depth + min_depth);
        let mut check = 0;

        for _ in 0..iterations {
            let tree = TreeNode::new(depth);
            check += tree.check();
        }

        println!("{}\t trees of depth {}\t check: {}", iterations, depth, check);
        depth += 2;
    }

    println!("long lived tree of depth {}\t check: {}", max_depth, long_lived_tree.check());
}
