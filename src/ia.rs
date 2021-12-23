use crate::memory::Memory;
use crate::controls::Controls;
use std::cmp;

pub struct Ia{
    pub mat :[[bool; 10]; 18],
    pub old_mat : [[bool; 10]; 18],
    pub tet : PieceType,
    pub inputs : [Input; 30], //TODO Check le nombre max de coup ?
    pub input_iterator : u8
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


impl Ia{



    pub fn default_inputs() -> [Input;30]{
        [Input::End,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,
        Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,
        Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,Input::None,
        Input::None,Input::None,Input::None,Input::None,Input::None,Input::None]
    }

    pub fn get_tet_coord(tet: &PieceType, rot: u8) -> [[u8; 2];4]{
        /* 
            This function will take a tetromino and a rotation.
            It will return an array filled with the coordinates of the tetromino in the good orientation 
        */
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
                0 => [[0,1],[1,1],[2,1],[0,2]],
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
        /*
            This function was used for the debugging, ti will print the playground 
        */
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
        /*
            This function will return for each piece the number of column between the piece and the left wall
        */
        match tet{
            PieceType::O => 4,
            PieceType::I => match rot {
                0 | 2 => 3,
                _ => 4,
            },
            PieceType::L | PieceType::J | PieceType::T => match rot {
                3 => 3,
                _ => 3,
            },
            _ => 3,
        }
    }

    pub fn check_state(&self, tet: &PieceType, rot: u8, x:i8, y:i8) -> bool {//TODO comment
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
        let mut best_score: f32 = 65534.0;
        let mut score: f32;
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

                score = Ia::compute_score(&dummy_mat,1.0,0.5,0.5,0.5,0.5);
                println!("score:{},rot:{},col:{}",score,rot,j);
                if score < best_score{
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
        /*
            Get the shape of the next tetromino by checking the memory
        */
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
        /*
            Another function used for the debugging: printing the tetromino shape 
        */
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
       
        match self.inputs[(self.input_iterator/2) as usize] {
            Input::Left => {

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
        /*
            This function will take a duet made with two element
            The first one is the piece position
            The second one contain the piece orientation
            The function translates the data into a list of input
        */
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
        self.inputs[0] = Input::None;
        self.inputs[1] = Input::None;
        self.inputs[3] = Input::None;
        self.inputs[29] = Input::End;
    }

    /*Launched at each new screen, this function will act as the routine for our Ia*/
    pub fn process_screen(&mut self, mem: &mut Memory) {
        let mut best;

        self.get_field(mem); //Generating the new screen
        if self.mat != self.old_mat {
            self.get_next_tet(mem);
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
    fn compute_score(matrix : &[[bool; 10]; 18], w1:f32,w2:f32,w3:f32,w4:f32,w5:f32) -> f32{
        /*
        The final score depends on the following parameter
        gaps = number of gap
        height_mean = mean of the heigths
        max_diff diff between highest and lowest
        max_side_diff = max dif between two side raw
        standart_deviation = standart deviation between the heights
        */
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
        let mut standart_deviation :f32 = 0.0;
        for i in 0..9{
            standart_deviation+=(col_height[i]-height_mean).pow(2) as f32;
        }
        standart_deviation=(standart_deviation*0.1).sqrt();
        let max_diff = max_height - min_height;

        (gaps as f32 * w1 + height_mean as f32 * w2 + max_diff as f32 * w3 + max_side_diff as f32 *w4 +standart_deviation*w5) as f32 // score
    }

    /*fn create_child(parent1:[f32;6],parent2:[f32;6])->[f32;6]{
        /*
            This function take two IA "parent" and create a child with the mean of them.
            The last case in the child is for his future score.
        */
        let mut child : [f32;6];
        for i in 0..4{
            child[i] = (parent1[i] + parent2[i])/2;
        }
        return child;
    }*/


}

/*fn genetic(){
    /*

    Generate the initial population :
    - first step : générer un individu.
    - Second step : ajouter l'individu a un "tableau" d'individu (appelé population).
    - Third step : répéter l'étape 1 et 2 autant de fois que voulue. 

    Compute fitness : 
    - First step : On donne notre population initial a une fonction de score/classement. 
                   Elle retourne une population classé : de l'IA qui a le plus grand score tetris à celle qui à le moins bon.
                   On a donc une Population classé. 

    REPEAT => do while => idée ? 

    Selection made by hitler
    - First stape : On a en entré notre population classé => 
                    on va créer une nouvelle population composé des 3 première IA (3 ou plus ça dépend de ce qu'on veut) de notre population initial : 
                    le meilleur individu de notre population initial sera copié dans une nouvelle population avant de passer au crossover. 
    - Second step : Crossover : La fonction crossover prend en entré notre nouvelle population et notre population initiale 
                                et va mixé notre IA0/IA1 , IA1/IA2 , IA0/IA2  avec du 50/50 
    - Conclusion : on a une nouvelle population composé de notre meilleur individu de notre ancienne population et de trois nouveau individu issue du crossover.

    Mutation
        how to adapt?
        Compute fitness
        launch ten game and get stats foreach
    UNTIL population has converged
    */
}*/
