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