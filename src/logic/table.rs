use rand::Rng;

pub struct Board {
    pub tab: Vec<Vec<i8>>,
    pub cofre: (usize, usize),
    pub main: (usize, usize),
}

impl Board {
    // jardin
    pub fn new(mat: &Vec<Vec<i8>>) -> Board {      
        
        // desreferenciando clon
        let mut tab = mat.clone();
        
        // casa
        for r in 9..19 {
            for c in 3..8 {
                tab[r][c] = -3;
            }
        }

        // aleatorios
        tab = random_e(&-1, &1, &mut tab);
        tab = random_e(&-2, &15, &mut tab);
        tab = random_e(&-8, &1, &mut tab);
        tab = random_e(&9, &10, &mut tab);
        tab = random_e(&6, &1, &mut tab);


        let cofre = find_cords(&tab, &6);
        let main = find_cords(&tab, &-8);

        Board{
            tab,
            cofre,
            main,
        }
    }
}


fn random_e(val: &i8, u: &u8, mat: &mut Vec<Vec<i8>>)-> Vec<Vec<i8>>{
    let mut rng = rand::thread_rng();
    let mut cont = 0;
    loop {
        let randomy = rng.gen_range(0..mat.len());
        let randomx = rng.gen_range(0..mat[0].len());

        if mat[randomy][randomx] == 0 {
            mat[randomy][randomx] = *val;
            cont += 1;
            if cont == *u {
                break;
            }
        }
    }
    return mat.clone();
}


fn find_cords( mat: &Vec<Vec<i8>>, val: &i8 ) -> (usize, usize){

    let coords = (mat
        .into_iter()
        .position( 
            |variable| variable
            .into_iter()
            .any( |f| { f==val })).unwrap(),
            mat.into_iter().find(
                |x| 
                x.clone()
                .into_iter()
                .any( |x | {x == val} )).unwrap()
                .into_iter()
                .position( | pos | pos ==val ).unwrap()
        );

        return coords
}
