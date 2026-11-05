fn main() {

    let entero_8 : u8 = 5;
	println!("Esto es un entero de 8 bits {entero_8}");

    let entero_8_err : i8 = -5;
	println!("Esto es un entero de 8 bits {entero_8_err}");

    let entero_8_overflow : u8 = 255;
	println!("Esto es un entero de 8 bits {entero_8_overflow}");

	let x : f32 = 0.2;
	let y : f32 = 0.3;
	
	let x = x+y;
	println!("Suma de decimales! {x}");
	
	let tupla: (i32, bool, char, f64) = (-32,true,'🙈',3.14159);
	let (x,y,z,a) = tupla;
	println!("{x}, {y}, {z}, {a}");
	println!("{:?}",tupla);

//This panics
	let array : [i32; 5] = [1,2,3,4,5];
	println!("{}", array[10]);

	let array = ["a"; 5];
	println!("{:?}", array);

}
