
pub struct Ia{
    mat :[[bool; 144]; 160],
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

/*This function will take a position and then compute a score. Lower is the score better is the position.*/
//TODO Make a enum for rotate ?
fn compute_best_place(matrix : &[[bool; 144]; 160],col : u8, rotate : u8, ) -> u8{
    /*gaps = number of gap
    height_mean = mean of the heigths
    max_diff diff between highest and lowest
    max_side_diff = max dif between two side raw*/
    let mut gaps = 0; height_mean = 0; min_height = 15; max_height = 0; max_side_diff;
    let mut col_heigth =[0,0,0,0,0,0,0,0,0,0];
    for column in 0..9{
        for raw in ra0..15{
            if matrix[raw][column]== false && col_heigth[column]!=0{
                gaps+=1;
            }else if matrix[raw][column]== true && col_heigth[column]==0{
                col_heigth[column]=(15-raw);
                height_mean+=(15-raw);
            }
        }
        if min_height > col_heigth[column]{
            min_height=col_heigth[column];
        }
        if max_height > col_heigth[column]{
            max_height=col_heigth[column];
        }
        if column >=1{
            if value.abs(col_heigth[column -1]-col_heigth[column])>max_side_diff{
                max_side_diff=value.abs(col_heigth[column -1]-col_heigth[column]);
            }
        }
    }
    height_mean = height_mean/10;
    let max_diff = max_height - min_height;

    score=gaps*w1+height_mean*w2+max_diff*w3+max_side_diff*w4;
}

/*this function will check for each possiblr position of the tetromino which one is the better.
Then it will call function to move the piece to the right place*/
fn find_best_place(mat : &[[bool; 144]; 160], piece : &piece){
    let mut score :u8 = 0;
    let mut best_score = 0;
    let mut pose = [0,0];
    for col in 0..9{
        for rotate in  0..3{
            score = compute_best_place(mat,col,rotate);
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