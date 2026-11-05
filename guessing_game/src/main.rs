use std::io; 
use std::cmp::*;

use rand::prelude::*;

fn main() {
	println!("¡Voy a adivinar tu número!");

	let secret_number = rand::rng().random_range(1..=100);

	loop {
		println!("Por favor, piensa un número");
		
		let mut numero = String::new();
	
		io::stdin()
			.read_line(&mut numero)
			.expect("Fallo al leer numero");
		
		let numero: u32 = match numero.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,	
		};

		println!("Has dicho: {numero}");

		match numero.cmp(&secret_number) {
			Ordering::Less => println!("Mu pequeño!"),
			Ordering::Greater => println!("Mu grande!"),
			Ordering::Equal => {
				println!("Es el mismo!");
				break;
			}	
		}
	}
}
