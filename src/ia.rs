use crate::memory::Memory;
use crate::controls::Controls;

pub struct Ia{
    pub mat :[[bool; 10]; 18],
    pub old_mat : [[bool; 10]; 18],
    pub tet : PieceType,
    pub inputs : [Input; 8], //TODO Check le nombre max de coup ?
    pub input_iterator : u8
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
    A,
    None
}

/*pub enum TetPattern { //Plz end me
    I(([u8;4],[u8;1],[u8;4],[u8;1])),
    J(([u8;3],[u8;2],[u8;3],[u8;2])),
    L(([u8;3],[u8;2],[u8;3],[u8;2])),
    O(([u8;2],[u8;2],[u8;2],[u8;2])),
    S(([u8;3],[u8;2],[u8;3],[u8;2])),
    Z(([u8;3],[u8;2],[u8;3],[u8;2])),
    T(([u8;3],[u8;2],[u8;3],[u8;2])),
}*/
/*
pub enum TetPattern { //Plz end me
    I([Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>]),
    J(([u8;3],[u8;2],[u8;3],[u8;2])),
    L(([u8;3],[u8;2],[u8;3],[u8;2])),
    O(([u8;2],[u8;2],[u8;2],[u8;2])),
    S(([u8;3],[u8;2],[u8;3],[u8;2])),
    Z(([u8;3],[u8;2],[u8;3],[u8;2])),
    T(([u8;3],[u8;2],[u8;3],[u8;2])),
}
*/
//tile: 47
//9802
//+0x1 pour l'axe x
//+0x20 pour l'axe y



impl Ia{

    pub fn default_inputs() -> [Input;8]{
        [Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None]
    }

    pub fn get_pattern (&self) -> [Vec<u8>;4]{
        match self.tet {
            PieceType::O => [vec![0; 2],vec![0; 2],vec![0; 2],vec![0; 2]],
            PieceType::T => [vec![1,0,1],vec![0,1],vec![0; 3],vec![1,0]],
            PieceType::S => [vec![0,0,1],vec![1,0],vec![0,0,1],vec![1,0]],
            PieceType::L => [vec![0,0,0],vec![2,0],vec![0,1,1],vec![0,0]],
            PieceType::J => [vec![0,0,0],vec![0,0],vec![1,1,0],vec![0,2]],
            PieceType::I => [vec![0,0,0,0],vec![0],vec![0,0,0,0],vec![0]],
            PieceType::Z => [vec![1,0,0],vec![0,1],vec![1,0,0],vec![0,1]],
            _ => { [vec![0; 2],vec![0; 2],vec![0; 2],vec![0; 2]]} //Never gonna happend but meh
        }
    }


    /*Generate the matrix containing already set tetriminos*/
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
        let mut temp: Controls = Controls {
            up: 1,
            down: 1,
            left: 1,
            right: 1,
            a: 1,
            b: 1,
            select: 1,
            start: 1
        };
        match self.inputs[self.input_iterator as usize] {
            Input::Left => {
                self.inputs[self.input_iterator as usize] = Input::None;
                self.ready_next_move();
                temp = Controls {
                    up: 1,
                    down: 1,
                    left: 0,
                    right: 1,
                    a: 1,
                    b: 1,
                    select: 1,
                    start: 1,
                }
            },
            Input::Right => {
                self.inputs[self.input_iterator as usize] = Input::None;
                self.ready_next_move();
                temp = Controls {
                    up: 1,
                    down: 1,
                    left: 1,
                    right: 0,
                    a: 1,
                    b: 1,
                    select: 1,
                    start: 1,
                }
            },
            Input::A => {
                self.inputs[self.input_iterator as usize] = Input::None;
                self.ready_next_move();
                temp = Controls {
                    up: 1,
                    down: 1,
                    left: 1,
                    right: 1,
                    a: 0,
                    b: 1,
                    select: 1,
                    start: 1,
                }
            },
            Input::None => {
                self.ready_next_move();
                temp = Controls { //TODO METTRE EN DOWN ?
                    up: 1,
                    down: 1,
                    left: 1,
                    right: 1,
                    a: 1,
                    b: 1,
                    select: 1,
                    start: 1,
                }
            },
        }
        return temp;
    }

    /*Lunched at each new screen, this function will act as the routine for our Ia*/
    pub fn process_screen(&mut self, mem: &mut Memory) {
        self.get_field(mem); //Generating the new screen
        self.get_next_tet(mem);
        if self.mat != self.old_mat {
            print!("Hewo bebou");
            self.inputs = self.find_best_place();
            self.input_iterator = 0 //Reseting the parsing of inputs
        }
    }

    pub fn ready_next_move(&mut self) {
        if self.input_iterator == 7{
            self.input_iterator = 0
        }else{
            self.input_iterator += 1;
        }
    }


    /*This function will take a position and then compute a score. Lower is the score better is the position.*/
    //TODO Make a enum for rotate ?
    fn compute_best_place(matrix : &[[bool; 10]; 18]) -> u8{
        /*gaps = number of gap
        height_mean = mean of the heigths
        max_diff diff between highest and lowest
        max_side_diff = max dif between two side raw*/
        let mut gaps = 0;
        let mut height_mean :i8 = 0;
        let mut min_height :i8 = 18;
        let mut max_height:i8 = 0;
        let mut max_side_diff:i8 = 0;
        let mut col_height:[i8;10] =[0,0,0,0,0,0,0,0,0,0];
        for column in 0..9{
            for raw in 0..18{
                if matrix[raw][column]== false && col_height[column]!=0{
                    gaps+=1;
                }else if matrix[raw][column]== true && col_height[column]==0{
                    col_height[column]= (18 - raw) as i8;
                    //println!("{} {}",height_mean,raw);
                    height_mean+=(18-raw) as i8;
                }
            }
            if min_height > col_height[column]{
                min_height= col_height[column];
            }
            if max_height > col_height[column]{
                max_height= col_height[column];
            }
            if column >=1{
                if (col_height[column -1]- col_height[column]).abs() >max_side_diff{
                    max_side_diff =(col_height[column -1]- col_height[column]).abs();
                }
            }
        }
        height_mean = height_mean/10;
        let max_diff = max_height - min_height;

        (gaps + height_mean + max_diff + max_side_diff) as u8 // score
    }

    /*this function will check for each possible position of the tetromino which one is the better.
    Then it will call function to move the piece to the right place*/
    fn find_best_place(&mut self) -> [Input; 8] {
        /*This is a matrix storing array, in each array the down side of a piece is represented
        0 is the lowest point on the piece, 15 is an empty line,
        1 and 2 are the difference between this place of the piece and the lowest level of the piece
        the first array in each line represent the base piece shape
        each other array in line represent the base piece shape ater a counterclockwise rotation
        */
        let mut tet_pattern : [Vec<u8>;4] = self.get_pattern();
        let mut score :u8 = 0;
        let mut best_score = 0;
        let mut pose : [i8;2] = [0,0];
        let mut piece_shape : Vec<u8>;
        let mut new_mat : [[bool; 10]; 18];
        let mut is_placed;
        let mut heigth : usize;

        for col in 0..9{
            for rotate in  0..3{
                piece_shape = tet_pattern[rotate].clone();
                new_mat = self.mat;
                is_placed = 0;
                heigth = 0;
                if piece_shape.len()+col<9{
                    while is_placed == 0{
                        for i  in 0..piece_shape.len(){
                            println!("Piece shape {}",piece_shape.len());
                            println!("i {}",i);
                            println!("End me please {}",heigth+1-piece_shape[i] as usize);
                        if heigth+1 - piece_shape[i] == 18 {

                        }
                            if new_mat[heigth+1-piece_shape[i] as usize][col+i as usize]{
                                is_placed = 1;
                                for i in 0..piece_shape.len(){
                                    new_mat[heigth-piece_shape[i] as usize][col+i as usize] = true;
                                }
                                match self.tet {
                                    PieceType::O => {
                                        new_mat[heigth-1 as usize][col as usize] = true;
                                        new_mat[heigth-1 as usize][col+1 as usize] = true;
                                    },
                                    PieceType::T => {
                                        match rotate{
                                            0 => new_mat[heigth-1][col+1] = true,
                                            1 => {new_mat[heigth-1][col] = true;
                                                new_mat[heigth-2][col] = true;},
                                            2 => new_mat[heigth-1][col+1] = true,
                                            3 => {new_mat[heigth-1][col+1] = true;
                                                new_mat[heigth-2][col+1] = true;}
                                            _ => {},
                                        }
                                    }
                                    PieceType::S => {
                                        if rotate==0 || rotate==2{
                                            new_mat[heigth-2][col] = true;
                                            new_mat[heigth-1][col+1] = true;
                                        }else{
                                            new_mat[heigth-1][col+1] = true;
                                        }
                                    },
                                    PieceType::L => {
                                        match rotate{
                                            0 => new_mat[heigth-1][col+2] = true,
                                            1 => {new_mat[heigth-1][col+1] = true;
                                                new_mat[heigth-2][col+1] = true;},
                                            2 => new_mat[heigth-1][col] = true,
                                            3 => {new_mat[heigth-1][col] = true;
                                                new_mat[heigth-2][col] = true;},
                                            _ => ()
                                        }
                                    }
                                    PieceType::J => {
                                        match rotate{
                                            0 => new_mat[heigth-1][col] = true,
                                            1 => {new_mat[heigth-1][col+1] = true;
                                                new_mat[heigth-2][col+1] = true;},
                                            2 => new_mat[heigth-1][col+2] = true,
                                            3 => {new_mat[heigth-1][col] = true;
                                                new_mat[heigth-2][col] = true;},
                                            _ => ()
                                        }
                                    }
                                    PieceType::I => {
                                        if rotate==1 || rotate==3{
                                            new_mat[heigth-1][col] = true;
                                            new_mat[col][heigth-2] = true;
                                            new_mat[col][heigth-3] = true;
                                        }
                                    },
                                    PieceType::Z => {
                                        if rotate==0 || rotate== 2{
                                            new_mat[heigth-1][col] = true;
                                            new_mat[heigth-2][col+1] = true;
                                        }else{
                                            new_mat[heigth-1][col+1] = true;
                                        }
                                    },
                                    PieceType::None => {}
                                }
                            }else{
                                heigth +=1;
                            }
                        }
                    }
                }
                score = Ia::compute_best_place(&new_mat);
                if score > best_score{
                    best_score = score;
                    pose[0] = col as i8;
                    pose[1] = rotate as i8;
                }
            }
        }

        let mut future_inputs :[Input;8] = Ia::default_inputs();
        pose[0] = pose[0] - 4;
        for index in 0..pose[1]{
            future_inputs[index as usize] = Input::A;
        }

        if pose[0] > 0 {
            for index in pose[1]..pose[0]+pose[1]{
                future_inputs[index as usize] = Input::Right;
            }

        }else if pose[0]<0{
            for index in 0..(pose[0]+pose[1]).abs(){
                future_inputs[index as usize] = Input::Left;
            }
        }
        return future_inputs;
    }



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
