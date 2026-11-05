fn main() {
    println!("Hello, world!");
	mi_primera_funcion();
	el_doble(5);
	suma(4,82);
}

fn mi_primera_funcion() {
	println!("Hola desde la funcion");
}

fn el_doble(x: i32){
	let doble = x * 2;
	println!("El doble de {x} es {doble}");
}

fn suma(x: i32, y: i32){
	let suma = x + y;
	println!("La suma de {x}+{y} = {suma}");

}
