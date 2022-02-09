use std::io;

pub fn idade() -> u8 {
    let mut cibele = String::new();
    let mut camila = String::new();
    let mut celeste = String::new();

    io::stdin()
        .read_line(&mut cibele)
        .expect("falha ao ler a linha");

    let cibele: u8 = match cibele.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    io::stdin()
        .read_line(&mut camila)
        .expect("falha ao ler a linha");

    let camila: u8 = match camila.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    io::stdin()
        .read_line(&mut celeste)
        .expect("falha ao ler a linha");

    let celeste: u8 = match celeste.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut idades = vec!(cibele, camila, celeste);

    if !idades.iter().all(|&x| x>= 5 && x <= 100) {
        panic!("as idades devem estar entre cinco e cem");
    }

    idades.sort();

    match idades.get(1) {
        Some(n) => *n,
        None => panic!("erro ao ler o valor"),
    }
}
