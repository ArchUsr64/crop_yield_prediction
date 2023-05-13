#![allow(unused)]
pub mod forest;
pub use self::forest::*;
pub type Features = [i32; 4];

#[derive(Clone, Debug)]
pub struct Instances {
	x: Vec<i32>,
	y: Vec<i32>,
	z: Vec<i32>,
	w: Vec<i32>,
	result: Vec<i32>,
}
impl Instances {
	pub fn new() -> Self {
		Self {
			x: Vec::new(),
			y: Vec::new(),
			z: Vec::new(),
			w: Vec::new(),
			result: Vec::new(),
		}
	}
	pub fn add_entry(&mut self, features: Features, result: i32) {
		self.x.push(features[0]);
		self.y.push(features[1]);
		self.z.push(features[2]);
		self.w.push(features[3]);
		self.result.push(result);
	}
	fn variance(&self) -> [f32; 4] {
		fn sample_variance(data: &[i32]) -> f32 {
			// println!("{data:#?}");
			let sum_of_squares: i64 = data.iter().map(|i| (*i as i64).pow(2)).sum();
			let square_of_sum: i64 = data.iter().map(|i| *i as i64).sum::<i64>().pow(2);
			let n: f32 = data.len() as f32;
			(sum_of_squares as f32 - (square_of_sum as f32 / n)) / (n - 1.)
		}
		[
			sample_variance(&self.x),
			sample_variance(&self.y),
			sample_variance(&self.z),
			sample_variance(&self.w),
		]
	}
	fn sort_by_feature(&mut self, feature_index: usize) {
		assert!(
			(0..4).contains(&feature_index),
			"Request feature index out of bounds"
		);
		let feature_vec = match feature_index {
			0 => self.x.clone(),
			1 => self.y.clone(),
			2 => self.z.clone(),
			_ => self.w.clone(),
		};
		let mut feature_vec_indexed = feature_vec.iter().enumerate().collect::<Vec<_>>();
		feature_vec_indexed.sort_by(|a, b| a.1.cmp(b.1));
		let new_index = feature_vec_indexed.iter().map(|(i, _)| i);
		let len = self.result.len();
		let mut sorted_instance = Self {
			x: Vec::with_capacity(len),
			y: Vec::with_capacity(len),
			z: Vec::with_capacity(len),
			w: Vec::with_capacity(len),
			result: Vec::with_capacity(len),
		};
		new_index.for_each(|new| {
			sorted_instance.x.push(self.x[*new]);
			sorted_instance.y.push(self.y[*new]);
			sorted_instance.z.push(self.z[*new]);
			sorted_instance.w.push(self.w[*new]);
			sorted_instance.result.push(self.result[*new])
		});
		*self = sorted_instance
	}
	pub fn nth(&self, index: usize) -> (Features, i32) {
		(
			[self.x[index], self.y[index], self.z[index], self.w[index]],
			self.result[index],
		)
	}
	fn len(&self) -> usize {
		self.result.len()
	}
	fn split_at(&self, split_index: usize) -> (Self, Self) {
		let x = self.x.split_at(split_index);
		let y = self.y.split_at(split_index);
		let z = self.z.split_at(split_index);
		let w = self.w.split_at(split_index);
		let result = self.result.split_at(split_index);
		(
			Self {
				x: x.0.to_vec(),
				y: y.0.to_vec(),
				z: z.0.to_vec(),
				w: w.0.to_vec(),
				result: result.0.to_vec(),
			},
			Self {
				x: x.1.to_vec(),
				y: y.1.to_vec(),
				z: z.1.to_vec(),
				w: w.1.to_vec(),
				result: result.1.to_vec(),
			},
		)
	}
}

#[derive(Clone, Debug)]
enum DecisionTreeNode {
	///The index of the feature that splitting is determined for
	SplitPoint {
		feature_index: usize,
		critical_value: i32,
		branch: (Box<DecisionTreeNode>, Box<DecisionTreeNode>),
	},
	Value(i32),
}
impl DecisionTreeNode {
	fn traverse(&self, features: Features) -> i32 {
		let mut start_node = self.clone();
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
	fn construct(instances: &Instances) -> Self {
		let len = instances.len();
		if len == 1 {
			return Self::Value(instances.result[0]);
		}
		let mut feature_variance = instances
			.variance()
			.into_iter()
			.enumerate()
			.collect::<Vec<_>>();
		feature_variance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
		let most_varied_feature_index = feature_variance[0].0;
		let mut instances = instances.clone();
		instances.sort_by_feature(most_varied_feature_index);
		let (mut left, mut right) = instances.split_at(len / 2);
		let root = Self::SplitPoint {
			feature_index: most_varied_feature_index,
			critical_value: instances.nth(len / 2).0[most_varied_feature_index],
			branch: (
				Box::new(Self::construct(&mut left)),
				Box::new(Self::construct(&mut right)),
			),
		};
		root
	}
}

#[test]
fn decision_tree_construction() {
	let mut instances = Instances::new();
	instances.add_entry([1, 5, 9, 3], 7);
	instances.add_entry([2, 6, 0, 4], 8);
	instances.add_entry([3, 7, 1, 5], 9);
	instances.add_entry([4, 8, 2, 6], 0);
	instances.add_entry([7, 50, 10, 3], -10);
	let root = DecisionTreeNode::construct(&instances);
	(0..instances.len()).for_each(|i| {
		assert_eq!(
			root.traverse([
				instances.x[i],
				instances.y[i],
				instances.z[i],
				instances.w[i]
			]),
			instances.result[i]
		)
	});
	panic!("{instances:#?}");
	// panic!("{root:#?}");
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

	//       Decision Tree
	//
	//           x < 5
	//          /     \
	//    y < 10      z < 11
	//   /      \    /      \
	//  0        1  2        w < 2
	//                      /     \
	//                     3       4

	assert_eq!(root.traverse([4, 3, 0, 0]), 0);
	assert_eq!(root.traverse([4, 10, 0, 0]), 1);
	assert_eq!(root.traverse([5, 0, 10, 0]), 2);
	assert_eq!(root.traverse([5, 0, 11, 1]), 3);
	assert_eq!(root.traverse([5, 0, 11, 2]), 4);
	assert_eq!(root.traverse([6, 0, 12, 3]), 4);
}
