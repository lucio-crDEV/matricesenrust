use rand::Rng;

fn main() {
    // apuntes, nada que ver con el ejercicio...
    // 32bits son 2 elevado a 32
    // for _i in 0..10{
    //     println!("Hello, world!");
    // }
    //u = unsigned que no lleva signo, postivo
    // let mut _edad: String = String::new();
    // std::io::stdin().read_line(&mut _edad ).unwrap();

    // println!("{}",_edad)

    let mut state = [[0i8; 12]; 20];

    for r in 9..19 {
        for c in 3..8 {
            state[r][c] = -3;
        }
    }

    state = random_e(-1, 1, state);
    state = random_e(-2, 15, state);
    state = random_e(-8, 1, state);
    state = random_e(9, 10, state);
    state = random_e(6, 1, state);

    for i in 0..state.len() {
        print!("|");
        for j in 0..state[i].len() {
            print!(" {} ", state[i][j]);
        }
        print!("|");
        println!("");
    }
    
}


fn random_e(val: i8, u: u8, mut mat: [[i8; 12]; 20]) -> [[i8; 12]; 20] {
    let mut rng = rand::thread_rng();
    let mut cont = 0;
    loop {
        let randomy = rng.gen_range(0..mat.len());
        let randomx = rng.gen_range(0..mat[0].len());

        if mat[randomy][randomx] == 0 {
            mat[randomy][randomx] = val;
            cont += 1;
            if cont == u {
                break;
            }
        }
    }
    return mat;
}