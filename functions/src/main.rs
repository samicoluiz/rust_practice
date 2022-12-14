fn main() {
    let tres_par: bool  = par_ou_impar(3);
    println!("{}", tres_par);
}

fn par_ou_impar(numero: isize) -> bool {
    let se_par = numero % 2 == 0;
    se_par
}
