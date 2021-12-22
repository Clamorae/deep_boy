use crate::memory::Memory;
use crate::controls::Controls;
use std::cmp;

pub struct Ia{
    pub mat :[[bool; 10]; 18],
    pub old_mat : [[bool; 10]; 18],
    pub tet : PieceType,
    pub inputs : [Input; 30], //TODO Check le nombre max de coup ?
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
    None,
    Down,
    End,
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



    pub fn default_inputs() -> [Input;30]{
        [Input::End,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,
        Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,
        Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,
        Input::None,Input::None,Input::None,Input::None,Input::None,Input::None]
    }

    pub fn get_tet_coord(tet: &PieceType, rot: u8) -> [[u8; 2];4]{
        match tet {
            PieceType::O => [[0,0],[1,0],[0,1],[1,1]],
            PieceType::T => match rot {
                0 => [[0,1],[1,1],[2,1],[1,2]],
                1 => [[1,0],[1,1],[1,2],[0,1]],
                2 => [[0,1],[1,1],[2,1],[1,0]],
                _ => [[1,0],[1,1],[1,2],[2,1]],
            },
            PieceType::S => match rot {
                0 => [[2,1],[1,1],[1,2],[0,2]],
                1 => [[0,0],[0,1],[1,1],[1,2]],
                2 => [[2,1],[1,1],[1,2],[0,2]],
                _ => [[0,0],[0,1],[1,1],[1,2]],
            },
            PieceType::L => match rot {
                0 => [[0,1],[1,1],[1,2],[0,2]],
                1 => [[0,0],[1,0],[1,1],[1,2]],
                2 => [[2,0],[2,1],[1,1],[0,1]],
                _ => [[1,0],[1,1],[1,2],[2,2]],
            },
            PieceType::J => match rot {
                0 => [[0,1],[1,1],[2,1],[2,2]],
                1 => [[1,0],[1,1],[1,2],[0,2]],
                2 => [[0,0],[0,1],[1,1],[2,1]],
                _ => [[2,0],[1,0],[1,1],[1,2]],
            },
            PieceType::I => match rot {
                0 => [[0,0],[1,0],[2,0],[3,0]],
                1 => [[0,0],[0,1],[0,2],[0,3]],
                2 => [[0,0],[1,0],[2,0],[3,0]],
                _ => [[0,0],[0,1],[0,2],[0,3]],
            },
            _ => match rot { //Z
                0 => [[0,1],[1,1],[1,2],[2,2]],
                1 => [[1,0],[1,1],[0,1],[0,2]],
                2 => [[0,1],[1,1],[1,2],[2,2]],
                _ => [[1,0],[1,1],[0,1],[0,2]],
            },
        }
    }

    pub fn print_field(&mut self, mat : &[[bool; 10]; 18]){
        println!("┌──────────┐");
        for i in 0..18{
            print!("|");
            for j in 0..10{
                if mat[i][j] {
                    print!("@");
                }else{
                    print!(" ");
                }
            }
            print!("|");
            println!("");
        }
        println!("└──────────┘");
    }

    pub fn get_offset(tet: &PieceType, rot: u8) -> i8 {
        match tet{
            PieceType::O => 4,
            PieceType::I => match rot {
                0 | 2 => 3,
                _ => 4,
            },
            PieceType::L | PieceType::J | PieceType::T => match rot {
                3 => 4,
                _ => 3,
            },
            _ => 3,
        }
    }

    pub fn check_state(&self, tet: &PieceType, rot: u8, x:i8, y:i8) -> bool {
        let tet_coord : [[u8; 2];4] = Ia::get_tet_coord(&tet, rot);
        for i in 0..4{
            if self.mat[(y + (tet_coord[i][1])as i8) as usize][(x + (tet_coord[i][0]) as i8) as usize] {
                return false;
            }

        }
        return true;
    }

    pub fn get_best_inputs(&mut self) -> [i8; 2] {
        let mut best_move: i8 = 0;
        let mut best_rot: i8 = 0;
        let mut best_score: u16 = 65534;
        let mut score: u16;
        let mut tet_coord: [[u8; 2];4] = [[0;2];4];
        let (mut x_min, mut x_max, mut y_max, mut x_min_win): (u8,u8,u8,u8) = (0,0,0,0);
        let mut run:bool;
        let mut y_ite;
        let mut x :i8= 0;
        let mut dummy_mat:[[bool; 10]; 18] = self.mat;
        let mut best_mat:[[bool; 10]; 18] = self.mat;

        //For every rotation
        for rot in 0..4{
            //get tet_coord
            tet_coord = Ia::get_tet_coord(&self.tet, rot);
            //get x_min, x_max and y_max
            x_min = cmp::min(tet_coord[0][0],cmp::min(tet_coord[1][0],cmp::min(tet_coord[2][0],tet_coord[3][0])));
            x_max = cmp::max(tet_coord[0][0],cmp::max(tet_coord[1][0],cmp::max(tet_coord[2][0],tet_coord[3][0])));
            y_max = cmp::max(tet_coord[0][1],cmp::max(tet_coord[1][1],cmp::max(tet_coord[2][1],tet_coord[3][1])));
            //for every position possible
            for j in 10-x_min..20-x_max{
                x = j as i8 - 10;
                run = true;
                //segment a refaire
                y_ite = 0;

                while (y_ite < 18-y_max) && (self.check_state(&self.tet,rot,x as i8,y_ite as i8)){
                    y_ite += 1;
                }

                dummy_mat = self.mat;

                for i in 0..4{
                    dummy_mat[(y_ite-1+tet_coord[i][1]) as usize][(x+(tet_coord[i][0] as i8)) as usize] = true;
                }
                score = Ia::compute_score(&dummy_mat);
                if score <= best_score{
                    best_score = score;
                    best_move = x as i8;
                    best_rot = rot as i8;
                    best_mat = dummy_mat;
                    x_min_win = x_min;
                }
            }
        }
        self.print_field(&best_mat);
        println!("x:{}  move:{}  rot:{}",best_move,best_move - Ia::get_offset(&self.tet,best_rot as u8), best_rot);
        return [best_move - Ia::get_offset(&self.tet,best_rot as u8), best_rot as i8];
    }

    /*
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
    */

    //Ici commence le refactor
    /*Fonction pour definir */


    /*Generate the matrix containing already set tetriminos*/
    pub fn get_field(&mut self, mem: &mut Memory){
        self.old_mat = self.mat;

        for i in 0..18 {
            for j in 0..10{
                self.mat[i][j] = (mem.read((0x9802+(i*0x20)+j) as u16)) != 47;
            }
        }
    }


    pub fn get_next_tet(&mut self, mem: &mut Memory){
        match mem.read(0xC213){
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
        //println!("{}",self.input_iterator/2);
        match self.inputs[(self.input_iterator/2) as usize] {
            Input::Left => {

                //self.inputs[self.input_iterator as usize] = Input::None;
                println!("Left");
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
                //self.inputs[(self.input_iterator) as usize] = Input::None;
                println!("Right");
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
                //self.inputs[(self.input_iterator) as usize] = Input::None;
                println!("A");
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
                //println!("None");
                self.ready_next_move();
                temp = Controls {
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
            Input::Down => {
                println!("Down");
                self.ready_next_move();
                temp = Controls {
                    up: 1,
                    down: 0,
                    left: 1,
                    right: 1,
                    a: 1,
                    b: 1,
                    select: 1,
                    start: 1,
                }
            },
            Input::End => {
                //self.ready_next_move();
                temp = Controls {
                    up: 1,
                    down: 0,
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

    pub fn duet_to_input(&mut self, duet: &[i8;2]) {
        let mut dir = duet[0];
        let mut rot = duet[1];
        for i in 4..14{
            if rot != 0 {
                self.inputs[i*2+1 as usize] = Input::A;
                rot -= 1;
            }else if dir < 0{
                self.inputs[i*2+1 as usize] = Input::Left;
                dir += 1;
            }else if dir > 0 {
                self.inputs[i*2+1 as usize] = Input::Right;
                dir -= 1;
            }else{
                self.inputs[i*2+1 as usize] = Input::Down;
            }

            self.inputs[i*2+2 as usize] = Input::None;
        }
        self.inputs[0] = Input::Down;
        self.inputs[1] = Input::None;
        self.inputs[3] = Input::Down;
        self.inputs[29] = Input::End;
    }

    /*Lunched at each new screen, this function will act as the routine for our Ia*/
    pub fn process_screen(&mut self, mem: &mut Memory) {
        let mut best;

        self.get_field(mem); //Generating the new screen
        if self.mat != self.old_mat {
            self.get_next_tet(mem);
            //print!("\x1B[2J\x1B[1;1H");
            best = self.get_best_inputs();
            self.duet_to_input(&best);
            self.input_iterator = 0 //Reseting the parsing of inputs
        }
    }

    pub fn ready_next_move(&mut self) {
        if self.input_iterator == 255{
            self.input_iterator = 0
        }else{
            self.input_iterator += 1;
        }
    }


    /*This function will take a position and then compute a score. Lower is the score better is the position.*/
    //TODO Make a enum for rotate ?
    fn compute_score(matrix : &[[bool; 10]; 18]) -> u16{
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

        (gaps + height_mean + max_diff + max_side_diff) as u16 // score
    }
}
    /*

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
                            if heigth as u8+1 - piece_shape[i] == 17 {
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
                            }else if new_mat[heigth+1-piece_shape[i] as usize][col+i as usize]{
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
                self.print_field(&new_mat);
                score = Ia::compute_score(&new_mat);

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
*/



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
