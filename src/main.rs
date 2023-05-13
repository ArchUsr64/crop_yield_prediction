use random_forest::*;
fn main() {
	let file_path = "preprocessed.dat";
	let file_str = std::fs::read_to_string(file_path).unwrap();
	let mut data_set = Instances::new();
	file_str.lines().for_each(|line| {
		let (mut features, mut result) = ([0; 4], 0);
		line.split_whitespace()
			.enumerate()
			.for_each(|(i, word)| match i {
				0 => features[0] = word.parse().unwrap(),
				1 => result = word.parse().unwrap(),
				i => features[i - 1] = word.parse().unwrap(),
			});
		data_set.add_entry(features, result);
	});
	let forest = RandomForest::new(&data_set, 10000, 5);
	let mut cum_error = 0f32;
	fn cost(desired: i32, result: i32) -> f32 {
		(desired as f32 - result as f32).powi(2) / desired as f32
	}
	const LEN: usize = 10000;
	(0..LEN).for_each(|i| {
		let (features, desired) = data_set.nth(i);
		let prediction = forest.predict(features);
		println!("Prediction: {prediction}, Desired: {desired}");
		cum_error += cost(desired, prediction);
	});
	println!("Error rate: {}", cum_error / LEN as f32);
}
