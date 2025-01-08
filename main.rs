const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn main() {
    println!("O valor de PI é: {}", PI);

    unsafe {
        println!("O valor da variável global é: {}", GLOBAL);
    }

    let variavel:i32 = 300;
    println!("O valor da variável é: {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("Decimal = {}", decimal);

    let booleana:bool = true;
    println!("Booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("tamanho do char = {}", std::mem::size_of_val(&letra));
}