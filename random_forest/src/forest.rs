use super::*;
use rand::prelude::*;
pub struct RandomForest {
	trees: Vec<DecisionTreeNode>,
	ensamble_size: usize,
}
impl RandomForest {
	pub fn new(data_set: &Instances, ensamble_size: usize, batch_size: usize) -> RandomForest {
		let mut result = RandomForest {
			trees: Vec::with_capacity(ensamble_size),
			ensamble_size,
		};
		(0..ensamble_size).for_each(|i| {
			let mut training_batch = Instances::new();
			(0..batch_size).for_each(|_| {
				let (feature, result) = data_set.nth(random::<usize>() % data_set.len());
				training_batch.add_entry(feature, result);
			});
			result
				.trees
				.push(DecisionTreeNode::construct(&training_batch));
		});
		result
	}
	pub fn predict(&self, features: Features) -> i32 {
		let mut predictions = Vec::with_capacity(self.ensamble_size);
		self.trees
			.iter()
			.for_each(|tree| predictions.push(tree.traverse(features)));
		predictions.iter().sum::<i32>() / self.ensamble_size as i32
	}
}
