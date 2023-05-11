pub struct Instance {
	features: [i32; 4],
	value: i32,
}
impl Instance {
	pub fn new(features: [i32; 4], value: i32) -> Self {
		Instance { features, value }
	}
}

#[derive(Clone, Debug)]
pub enum DecisionTreeNode {
	///The index of the feature that splitting is determined for
	SplitPoint {
		feature_index: usize,
		critical_value: i32,
		branch: (Box<DecisionTreeNode>, Box<DecisionTreeNode>),
	},
	Value(i32),
}

#[derive(Clone, Debug)]
pub struct DecisionTree {
	///The index of the feature that splitting is determined for
	root: DecisionTreeNode,
}
impl DecisionTree {
	fn traverse(&self, features: [i32; 4]) -> i32 {
		let mut start_node = self.root.clone();
		loop {
			use DecisionTreeNode::*;
			match start_node {
				Value(val) => return val,
				SplitPoint {
					feature_index,
					critical_value,
					branch,
				} => {
					start_node = if features[feature_index] < critical_value {
						*branch.0
					} else {
						*branch.1
					}
				}
			}
		}
	}
}

#[test]
fn decision_tree_traversal() {
	let root = DecisionTreeNode::SplitPoint {
		feature_index: 0,
		critical_value: 5,
		branch: (
			Box::new(DecisionTreeNode::SplitPoint {
				feature_index: 1,
				critical_value: 10,
				branch: (
					Box::new(DecisionTreeNode::Value(0)),
					Box::new(DecisionTreeNode::Value(1)),
				),
			}),
			Box::new(DecisionTreeNode::SplitPoint {
				feature_index: 2,
				critical_value: 11,
				branch: (
					Box::new(DecisionTreeNode::Value(2)),
					Box::new(DecisionTreeNode::SplitPoint {
						feature_index: 3,
						critical_value: 2,
						branch: (
							Box::new(DecisionTreeNode::Value(3)),
							Box::new(DecisionTreeNode::Value(4)),
						),
					}),
				),
			}),
		),
	};
	let tree = DecisionTree { root };

	//       Decision Tree
	//
	//           x < 5
	//          /     \
	//    y < 10      z < 11
	//   /      \    /      \
	//  0        1  2        w < 2
	//                      /     \
	//                     3       4

	assert_eq!(tree.traverse([4, 3, 0, 0]), 0);
	assert_eq!(tree.traverse([4, 10, 0, 0]), 1);
	assert_eq!(tree.traverse([5, 0, 10, 0]), 2);
	assert_eq!(tree.traverse([5, 0, 11, 1]), 3);
	assert_eq!(tree.traverse([5, 0, 11, 2]), 4);
	assert_eq!(tree.traverse([6, 0, 12, 3]), 4);
}
