use math::round;
use std::fs;

fn main() {
	let file_path = "../files/day-1-input.txt";

	let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
	let split = contents.split("\n");
	let values: Vec<i32> = split
		.into_iter()
		.map(|x| x.parse::<i32>().unwrap())
		.collect();
	fn fuel_calculation(x: &i32) -> i32 {
		return (round::floor((x / 3).into(), 0) as i32) - 2;
	}
	let calculated: Vec<i32> = values.iter().map(fuel_calculation).collect();
	let total = calculated.iter().fold(0, |acc, x| acc + x);

	fn sub_fuel_requirements(fuel: i32, acc: i32) -> i32 {
		let new_fuel = fuel_calculation(&fuel);
		if new_fuel <= 0 {
			return acc;
		}
		let new_acc = acc + new_fuel;
		return sub_fuel_requirements(new_fuel, new_acc);
	};

	let sub_calculated: Vec<i32> = calculated
		.into_iter()
		.map(|x| sub_fuel_requirements(x, 0))
		.collect();
	let subtotal = sub_calculated.iter().fold(0, |acc, x| acc + x);
	println!("{:#?}", subtotal + total);
}
