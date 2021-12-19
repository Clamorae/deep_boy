use pyo3::prelude::*;
use pyo3::types::PyTuple;


pub struct Ia{
    mat :[[u8; 144]; 160],
    piece : PieceType,
    child : [u8; 4]
    //Jeux de coup ?
    //
}

pub enum PieceType{
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
}


//fn calcul_best_place(u8 raw, u8 rotate){
    /*score=c1*w1+c2*w2...*/
//}

/*fn find_best_place(/* tetromino et sol*/){
    let score = 0;
    let best_score = 0;
    let pose = [0,0];
    for raw in range 0..9{
        for rotate in range 0..3{
            score = calcul_best_place(raw,rotate);
            if score > best_score{
                best_score = score;
                pose[0] = raw;
                pose[1] = rotate;
            }
        }
    }
    /*call action*/
}
*/
//fn genetic(){
    /*
    Generate the initial population
    Compute fitness
        launch ten game and get stats foreach
    REPEAT
        Selection
            keep how many participant?
        Crossover
            try try with crossover and mean value
        Mutation
            how to adapt?
        Compute fitness
            launch ten game and get stats foreach
    UNTIL population has converged
    */
}