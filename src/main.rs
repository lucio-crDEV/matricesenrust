// use rand::Rng;

// use rust::logic;

// fn main() {
// apuntes, nada que ver con el ejercicio...
// 32bits son 2 elevado a 32
// for _i in 0..10{
//     println!("Hello, world!");
// }
//u = unsigned que no lleva signo, postivo
// let mut _edad: String = String::new();
// std::io::stdin().read_line(&mut _edad ).unwrap();

// println!("{}",_edad)

//     // jardin
//     let mut state = vec![vec![0i8; 12]; 20];

//     // casa
//     for r in 9..19 {
//         for c in 3..8 {
//             state[r][c] = -3;
//         }
//     }

//     // aleatorios
//     state = random_e(-1, 1, state);
//     state = random_e(-2, 15, state);
//     state = random_e(-8, 1, state);
//     state = random_e(9, 10, state);
//     state = random_e(6, 1, state);

//     // orden

//     for i in state.iter(){
//         print!("|");
//         for &j in i.iter(){
//             if j < 0 {
//                 print!(" {} ",j);
//             } else {
//                 print!("  {} ",j);
//             }
//         }
//         print!("|");
//         println!("");
//     }

// }

// fn random_e(val: i8, u: u8, mut mat: Vec<Vec<i8>>)-> Vec<Vec<i8>>{
//     let mut rng = rand::thread_rng();
//     let mut cont = 0;
//     loop {
//         let randomy = rng.gen_range(0..mat.len());
//         let randomx = rng.gen_range(0..mat[0].len());

//         if mat[randomy][randomx] == 0 {
//             mat[randomy][randomx] = val;
//             cont += 1;
//             if cont == u {
//                 break;
//             }
//         }
//     }
//     return mat;
// }

use rust::logic::table::Board;

fn main() {

    // jardin
    let mut _tablero = Board::new(&vec![vec![0i8; 12]; 20]);

    
    let (row, col) = _tablero.main; 
    let (row2,col2 ) = _tablero.cofre; 
    

    
    println!("  Coordenadas del jugador: columna: {},  fila: {}", col, row );
    println!("  Coordenadas del cofre1:  columna: {},  fila: {}", col2, row2 );


    // orden

    for i in _tablero.tab.iter(){
        print!("|");
        for &j in i.iter(){
            if j < 0 {
                print!(" {} ",j);
            } else {
                print!("  {} ",j);
            }
        }
        print!("|");
        println!("");
    }

}
