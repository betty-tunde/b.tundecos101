fn main() {
	let a:f64 = 450_000.00;
	let b:f64 = 1_500_000.00;
	let c:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let e:f64 = 250_000.00;

	//amout * quantity
	let f = a * 2.0;
	let g = b * 1.0;
	let h = c * 3.0;
	let i = d * 3.0;
	let j = e * 1.0;

	//sum 
	let sum = f + g + h + i + j;

	let total_quantity:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;

	let avarage = sum/total_quantity;

	println!("Avarage is {}, Sum is {}", avarage,sum);
}