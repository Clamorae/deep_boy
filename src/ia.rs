use pyo3::prelude::*;
use pyo3::types::PyTuple;


pub struct Ia{}

/*This function will take a position and then compute a score. Lower is the score better is the position.*/
fn compute_best_place(/*matrix,w1,w2,w3,w4*/){
    /*gaps = number of gap
    height_mean = mean of the heigths
    max_diff diff between highest and lowest
    max_side_diff = max dif between two side raw*/
    let gaps = 0, height_mean = 0, min_height = 15, max_height = 0,max_side_diff;
    let col_heigths =[0,0,0,0,0,0,0,0,0,0];
    for collumn in range 0..9{
        for raw in range 0..15{
            if matrix[raw][collumn]==0 && col_heigths[collumn]!=0{
                gaps+=1;
            }else if matrix[raw][collumn]==1 && col_heigths[collumn]==0{
                col_heigths[collumn]=(15-raw);
                height_mean+=(15-raw);
            }
        }
        if min_height > col_heigths[collumn]{
            min_height=col_heigths[collumn];
        }
        if max_height > col_heigths[collumn]{
            max_height=col_heigths[collumn];
        }
        if collumn >=1{
            if value.abs(col_heigths[collumn-1]-col_heigths[collumn])>max_side_diff{
                max_side_diff=value.abs(col_heigths[collumn-1]-col_heigths[collumn]);
            }
        }
    }
    height_mean = height_mean/10;
    let max_diff = max_height - min_height; 

    score=gaps*w1+height_mean*w2+max_diff*w3+max_side_diff*w4;
}

/*this function will check for each possiblr position of the tetromino which one is the better.
Then it will call function to move the piece to the right place*/
fn find_best_place(/* tetromino et sol*/){
    let score = 0;
    let best_score = 0;
    let pose = [0,0];
    for col in range 0..9{
        for rotate in range 0..3{
            score = compute_best_place(col,rotate);
            if score > best_score{
                best_score = score;
                pose[0] = col;
                pose[1] = rotate;
            }
        }
    }
    /*call action*/
}

fn genetic(){
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