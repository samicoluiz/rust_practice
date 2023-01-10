fn main() {
    let n: u8 = 6;
    let mut nfib: u8 = 0;
    let mut n1: u8 = 0;
    let mut n2: u8;

    for number in 0..n+1 {
        if number == 0 {
            continue;
        } else if number == 1 {
            nfib = 1;
        } else {
            let tempn = n1;
            n1 = nfib;
            n2 = tempn;
            nfib = n1 + n2;
        }
    }

    println!("O número na posição 6 da sequência de fibonnaci é {nfib}")
}
