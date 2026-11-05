fn main() {

	let x = 5;
	
	let x = x + 1;

	{
		let x = x * 2;
		println!("El valor con alcance chico es {x}");
	}

	println!("El valor de x es {x}");


}
