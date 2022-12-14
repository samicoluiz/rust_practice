const HORAS_NO_DIA: u8 = 24;

fn main() {
    let x: u8 = 6*HORAS_NO_DIA;
    println!("O valor de x é {x}");
    
    let x = 200;
    println!("O valor de x é {x}");
    
    let max_u8: isize = 2isize.pow(8)-1;
    println!("O número maximo em u8 é {max_u8}");
    
    let quotient: f32 = 2./3.;
    println!("O quociente pode ser escrito como 2. = {quotient}");

    let tup: (&str, u8) = ("oi", 8);
    let (saudacao, numero) = tup;
    println!("{saudacao}");
    println!("{numero}");
    println!("{}", tup.1);

    let array_teste: [isize; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array_teste)

}
