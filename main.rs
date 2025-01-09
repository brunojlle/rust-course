const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn soma(a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra(){
    let a = 123;

    {
        let b = 456;
        println!("Dentro B: {}", b);

        let a = 789;
        println!("Dentro A: {}", a);
    }

    println!("Fora A: {}", a);
}

fn escopo() {
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

fn main() {
    escopo();
    sombra();

    println!("Soma: {}", soma(2, 2));

    condicionais();
}

fn condicionais() {
    let idade: u8 = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >=18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };
    println!("É {} de idade", condicao);
}