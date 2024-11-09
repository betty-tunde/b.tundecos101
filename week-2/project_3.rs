fn main() {
	let p:f64 = 210_000.00;
	let t:f64 = 3.0;
	let r:f64 = 5.0;

	//compound interesr of depreciation
	let a = p * (1.0 - (r / 100.0)).powf(t);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest of Depreciation is {}", ci);
}