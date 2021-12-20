use crate::memory::Memory;
use crate::controls::Controls;

pub struct Ia{
    pub mat :[[bool; 10]; 18],
    pub old_mat : [[bool; 10]; 18],
    pub tet : PieceType,
    //child : [u8; 4]
    //Jeux de coup ?
    //Je commente car flemme
}

pub enum PieceType{
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
    None,
}

pub enum Input{
    Left,
    Right,
    A
}

//tile: 47
//9802
//+0x1 pour l'axe x
//+0x20 pour l'axe y
impl Ia{

    pub fn get_field(&mut self, mem: &mut Memory){
        self.old_mat = self.mat;

        for i in 0..18 {
            for j in 0..10{
                self.mat[i][j] = (mem.read((0x9802+(i*0x20)+j) as u16)) != 47;
            }
        }
    }

    pub fn print_field(&mut self){
        println!("┌──────────┐");
        for i in 0..18{
            print!("|");
            for j in 0..10{
                if self.mat[i][j] {
                    print!("█");
                }else{
                    print!(" ");
                }
            }
            print!("|");
            println!("");
        }
        println!("└──────────┘");
    }

    pub fn get_next_tet(&mut self, mem: &mut Memory){
        match mem.read(0xC203){
            12 => self.tet = PieceType::O,
            24 => self.tet = PieceType::T,
            20 => self.tet = PieceType::S,
            4 => self.tet = PieceType::J,
            0 => self.tet = PieceType::L,
            8 => self.tet = PieceType::I,
            16 => self.tet = PieceType::Z,
            _ => self.tet = PieceType::None,

        }
    }

    pub fn print_tet(&self){
        print!("Current tet: ");
        match self.tet {
            PieceType::O => println!("O"),
            PieceType::T => println!("T"),
            PieceType::S => println!("S"),
            PieceType::L => println!("L"),
            PieceType::J => println!("J"),
            PieceType::I => println!("I"),
            PieceType::Z => println!("Z"),
            PieceType::None => println!("None"),
        }
    }

    pub fn get_inputs(&mut self) -> Controls {
        Controls{
            up: 1,
            down: 1,
            left: 1,
            right: 1,
            a: 1,
            b: 1,
            select: 1,
            start: 1,
        }
    }

}



/*This function will take a position and then compute a score. Lower is the score better is the position.*/
//TODO Make a enum for rotate ?
fn compute_best_place(matrix : &[[bool; 144]; 160],col : u8) -> u8{
    /*gaps = number of gap
    height_mean = mean of the heigths
    max_diff diff between highest and lowest
    max_side_diff = max dif between two side raw*/
    let mut gaps = 0; height_mean = 0; min_height = 18; max_height = 0; max_side_diff = 0;
    let mut col_heigth =[0,0,0,0,0,0,0,0,0,0];
    for column in 0..9{
        for raw in 0..18{
            if matrix[raw][column]== false && col_heigth[column]!=0{
                gaps+=1;
            }else if matrix[raw][column]== true && col_heigth[column]==0{
                col_heigth[column]=(18-raw);
                height_mean+=(18-raw);
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

/*this function will check for each possible position of the tetromino which one is the better.
Then it will call function to move the piece to the right place*/
fn find_best_place(mat : &[[bool; 144]; 160], piece : &piece) -> [Input] {
    /*This is a matrix storing array, in each array the down side of a piece is represented
    0 is the lowest point on the piece, 15 is an empty line, 
    1 and 2 are the difference between this place of the piece and the lowest level of the piece
    the first array in each line represent the base piece shape
    each other array in line represent the base piece shape ater a counterclockwise rotation 
    */
    let mut tet_patern;
    match self.tet {
        O => tet_patern = [[0,0],[0,0],[0,0],[0,0]],
        T => tet_patern = [[1,0,1],[0,1],[0,0,0],[1,0]],
        S => tet_patern = [[0,0,1],[1,0],[0,0,1],[1,0]],
        L => tet_patern = [[0,0,0],[2,0],[0,1,1],[0,0]],
        J => tet_patern = [[0,0,0],[0,0],[1,1,0],[0,2]],
        I => tet_patern = [[0,0,0,0],[0],[0,0,0,0],[0]],
        Z => tet_patern = [[1,0,0],[0,1],[1,0,0],[0,1]]
    }
    let mut score :u8 = 0;
    let mut best_score = 0;
    let mut pose = [0,0];
    let mut piece_shape;
    let mut new_mat;
    let mut is_placed;
    let mut heigth

    for col in 0..9{
        for rotate in  0..3{
            piece_shape = tet_patern[rotate];
            new_mat = mat;
            is_placed = 0;
            heigth = 15;
            if piece_shape.len()+col<9{
                while is_placed == 0{
                    for i in range 0..piece_shape.len(){
                        if new_mat[col+i][heigth+1-piece_shape[i]]==1{
                            is_placed = 1;
                            for i in range 0..piece_shape.len(){
                                new_mat[col+i][heigth-piece_shape[i]] = 1;
                            }
                            match self.tet {
                                O => {
                                    new_mat[col][heigth-1] = 1;
                                    new_mat[col+1][heigth-1] = 1;
                                    },
                                T => {
                                    match rotate{
                                        0 => new_mat[col+1][heigth-1] = 1,
                                        1 => {new_mat[col][heigth-1] = 1;
                                              new_mat[col][heigth-2] = 1;},
                                        2 => new_mat[col+1][heigth-1] = 1,
                                        3 => {new_mat[col+1][heigth-1] = 1;
                                            new_mat[col+1][heigth-2] = 1;}
                                    }
                                }
                                S => {
                                     if rotate==0 || rotate==2{
                                         new_mat[col][heigth-2] = 1;
                                         new_mat[col+1][heigth-1] = 1;
                                     }else{
                                         new_mat[col+1][heigth-1] = 1;
                                     }
                                    },
                                L => {
                                        match rotate{
                                            0 => new_mat[col+2][heigth-1] = 1,
                                            1 => {new_mat[col+1][heigth-1] = 1;
                                                  new_mat[col+1][heigth-2] = 1;},
                                            2 => new_mat[col][heigth-1] = 1,
                                            3 => {new_mat[col][heigth-1] = 1;
                                                new_mat[col][heigth-2] = 1;}
                                        }
                                    }
                                J => {
                                    match rotate{
                                        0 => new_mat[col][heigth-1] = 1,
                                        1 => {new_mat[col+1][heigth-1] = 1;
                                              new_mat[col+1][heigth-2] = 1;},
                                        2 => new_mat[col+2][heigth-1] = 1,
                                        3 => {new_mat[col][heigth-1] = 1;
                                            new_mat[col][heigth-2] = 1;}
                                    }
                                }
                                I => {
                                    if rotate==1 || rotate==3{
                                        new_mat[col][heigth-1] = 1;
                                        new_mat[col][heigth-2] = 1;
                                        new_mat[col][heigth-3] = 1;
                                    }
                                   },
                                Z => {
                                    if rotate==0 || rotate== 2{
                                        new_mat[col][heigth-1] = 1;
                                        new_mat[col+1][heigth-2] = 1;
                                    }else{
                                        new_mat[col+1][heigth-1] = 1;
                                    }
                                   }
                            }
                        }else{
                            heigth +=1;
                        }
                    }
                }
            }
            score = compute_best_place(new_mat);
            if score > best_score{
                best_score = score;
                pose[0] = col;
                pose[1] = rotate;
            }
        }
    }

    let mut future_inputs[pose[0]+pose[1]];
    let mut index = 0;
    pose[0] = pose[0] - 4;
    for i in range 0..rotate{
        future_inputs[index] = Input.A;
            i+=1
    }

    if pose[0] > 0 {
        for i in range 0..pose[0]{
            future_inputs[index] = Input.Right;
            i+=1
        }

    }else if pose[0]<0{
        for i in range 0..value.abs(pose[0]){
            future_inputs[index] = Input.Left;
            i+=1
        }
    }
 Return future_inputs;
}

/*fn genetic(){
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
}*/
*/
