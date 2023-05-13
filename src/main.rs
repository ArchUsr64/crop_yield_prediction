use random_forest::*;
use std::{
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
};

fn handle_connection(buf_reader: &mut BufReader<&mut TcpStream>) -> Features {
	let mut features = [0; 4];
	let mut buffer = String::new();
	let _ = buf_reader.read_line(&mut buffer).unwrap();
	buffer
		.split_whitespace()
		.map(|val| val.parse::<i32>().unwrap())
		.enumerate()
		.for_each(|(i, x)| features[i] = x);
	println!("Received raw: '{buffer}'");
	features
}

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
	let forest = RandomForest::new(&data_set, 10000, 500);
	println!("Training completed");

	let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
	for stream in listener.incoming() {
		let mut stream = stream.unwrap();
		println!("Connection Established with {stream:?}");
		let mut buf_reader = BufReader::new(&mut stream);
		let features = handle_connection(&mut buf_reader);
		println!("Received features: {features:#?}");
		let prediction = forest.predict(features);
		println!("Prediction: {prediction}\n");
		stream
			.write_all(format!("{prediction}").as_bytes())
			.unwrap();
	}
}
