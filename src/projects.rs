use std::ops::Deref;
use robotics_lib::interface::Direction;
use robotics_lib::interface::Direction::{Right, Up};
use robotics_lib::world::coordinates::Coordinate;
// use robotics_lib::interface::Direction;
//The SalviniTool before building a new type of road must create a project to analise it and understand its need.
//the Project struct contains:
//-action_curves:that describe the path the robot is going to follow to build the road
//-cost: the amount of energy it is going to take to build the whole project
//-rock: the amount of rock it is going to need to build the project

//Error type given to the user after checking a designed project, and whever there is an
//interaction in the road building functon
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StopReason{
    LowEnergy,
    MissingRocks,
    PonteSulloStretto,
    NoStop
}

#[derive(Clone)]
pub struct Project{
    pub(crate) curves_action: Vec<Direction>,
    pub(crate) cost: u32,
    pub(crate) rock: u32,
}
#[derive(Clone)]
pub struct UnFinishedProject {
    pub start_position: (usize,usize),
    pub stop_reason: StopReason,
    pub(crate) curves_action: Vec<Direction>,
    pub(crate) cost: u32,
    pub(crate) rock: u32,
}
impl From<UnFinishedProject> for Project{
    fn from(unfinished_project: UnFinishedProject) ->Project{
        Project{
            curves_action: unfinished_project.curves_action.clone(),
            cost: unfinished_project.cost,
            rock: unfinished_project.rock,
        }
    }
}
impl UnFinishedProject{
    //do i pass the ownership to the function, if the project fail and i get and unfinished project do i delete the prev project?
    pub fn new(project: Project, stop_reason: StopReason, end_position: (usize, usize)) ->UnFinishedProject{
        UnFinishedProject {
            start_position: end_position,
            stop_reason,
            curves_action: project.curves_action.clone(),
            cost: project.cost,
            rock: project.rock,
        }
    }
}


pub enum Shape{
    Square(u32),
    Rectangle(i32, i32),
    Roundabout(u32),
    Cross(i32), //its a roman cross
    LongLong(i32, Direction),
    TShape(i32, i32, Direction),
    LShape(u32, u32, Direction)
}
impl Shape{
    pub fn get_action_curve(&self) ->Vec<Direction>{
        let mut res:Vec<Direction> = vec![];
        match self {
            Shape::Square(side_lenght) => {
                let x = *side_lenght as i32;
                for _ in 0..x/2{
                    res.push(Direction::Right);
                }
                for _ in 0..x {
                    res.push(Direction::Up);
                }
                for _ in 0..(x as f32/2.0 ).ceil() as i32{
                    res.push(Direction::Left);
                }
                let mut other = mirror_direction(&res, 2);
                res.append(&mut other);
            }
            Shape::Rectangle(base, height
            ) => {
                for i in 0..base/2{
                    res.push(Direction::Right);
                }
                for i in 0..*height {
                    res.push(Direction::Up);
                }
                for i in 0..base/2{
                    res.push(Direction::Left);
                }
                let other = mirror_direction(&res, 2);
            }
            Shape::Roundabout(dimension) => {
                res = make_circle(*dimension);
            }
            Shape::Cross(l) => {
                for _ in 0..*l {
                    res.push(Direction::Up);
                }
                for _ in 0..l/2 {
                    res.push(Direction::Down)
                }
                for _ in 0..l/2 {
                    res.push(Direction::Right)
                }
                for _ in 0..*l {
                    res.push(Direction::Left);
                }
            }
            Shape::TShape(l, s, direction) =>{
                for _ in 0..*l {
                    res.push(direction.clone())
                }
                for _ in 0..l/2 {
                    res.push(opposite_direction(direction))
                }
                for _ in 0..*s {
                    let to_push = match direction {
                        Direction::Up => {Direction::Right}
                        Direction::Down => {Direction::Left}
                        Direction::Left => {Direction::Up}
                        Direction::Right => {Direction::Down}
                    };
                    res.push(to_push);
                }
            }
            Shape::LongLong(lenght, direction) => {
                for i in 0..*lenght{
                    res.push(direction.clone());
                }
            }
            Shape::LShape(long_side, short_side, direction) => {

            }
        }
        res
    }
}

//----------------------------------------------------//
//Here are defined some, very, useful function to design some shapes
//----------------------------------------------------//

//directioner function takes in input a Direction and gives back
//the tuple representing the vector with the same direction and
//1 in module, in the cartesian notation
pub fn directioner(d: & Direction)->(i32, i32){
    match d {
        Direction::Up => {(1,0)}
        Direction::Down => {(-1,0)}
        Direction::Left => {(0,-1)}
        Direction::Right => {(0,1)}
    }
}

//opposite_direction takes in input a direction and gives as
//output the opposite direction
pub(crate) fn opposite_direction(d: &Direction) -> Direction{
    match d {
        Direction::Up => {Direction::Down}
        Direction::Down => {Direction::Up}
        Direction::Left => {Direction::Right}
        Direction::Right => {Direction::Left}
    }
}

// mirror_direction takes in input a vector of directions and mirror+add
// to the vector the mirrored directions based on an 'angle' that defines
// an axes of symmetry:
// - 0:
// - 1:
// - 2:
pub(crate) fn mirror_direction(vec: &Vec<Direction>, angle: u32)->Vec<Direction>{
    let mut v:Vec<Direction> = vec![];
    match angle{
        0 => {
            for d in (0..vec.len()).rev() {
                if vec[d] == Direction::Right{
                    v.push(Direction::Up);
                }
                if vec[d] == Direction::Up{
                    v.push(Direction::Right);
                }
            }
        }
        1 => {
            for d in (0..vec.len()).rev() {
                if vec[d] == Direction::Right{
                    v.push(Direction::Left);
                }
                else { v.push(vec[d].clone()); }
            }
        }
        2 =>{
            for d in 0..vec.len()-1 {
                if vec[d] == Direction::Right{
                    v.push(Direction::Left);
                }
                if vec[d] == Direction::Up{
                    v.push(Direction::Down);
                }
                if vec[d] == Direction::Down{
                    v.push(Direction::Up);
                }
                if vec[d] == Direction::Left{
                    v.push(Direction::Right);
                }
            }
        }
        _ => { }
    }
    v
}

// used to complete a shape, as it is now just circle
// given its 12,5%
pub(crate) fn complete_shape(vec: &mut Vec<Direction>){
    for i in 0..3 {
        vec.append(&mut mirror_direction(&vec, i));
    }
}

//describe a circle as a vector of directions
fn make_circle(size: u32)->Vec<Direction>{
    let mut res = vec![];
    let mut b = vec![];
    for i in 0..(size as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as i32{
        let mut a = vec![];
        for j in 0..size {
            a.push(0)
        }
        b.push(a);
    }
    circular_2x2(&mut b, size as i32);
    res = circular_to_direction(&mut b, size as i32);
    complete_shape(&mut res);
    res
}

//describe a circle (1/8 of a cicrle) as 1 in a 2x2 matrix
fn circular_2x2(pippo: &mut Vec<Vec<i32>>, k: i32){
    for riga in 0..(k as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as i32 {
        for colonna in 0..k -(k as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as i32 {
            // print!("{} = {} : ",riga, colonna);
            // if j == ((k*k-(i)*(i))as f32).sqrt().ceil() as i32 || j == ((k*k-(i)*(i))as f32).sqrt().floor() as i32 {pippo[j as usize][i as usize] = 1;}
            // print!("{} = {} : ",riga, (k as f32 - (-(colonna as f32).powi(2)+(k as f32).powi(2)).sqrt()).floor() as i32);
            if riga == ((k as f32) - (-((colonna + 1) as f32).powi(2)+(k as f32).powi(2)).sqrt()).floor() as i32{
                pippo[riga as usize][colonna as usize] = 1;
                // println!("true");
            }
            // else { println!("false"); }
        }
    }
}

//gives the direction from the 1/8 of circle
fn circular_to_direction(mat: &mut Vec<Vec<i32>>, k: i32)->Vec<Direction>{
    let mut res = vec![];
    let mut raw_index = 0;
    let start = 1;
    for col in start..(k as usize - (k as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as usize){
        if mat[raw_index][col] != 1 {
            raw_index += 1;
            res.push(Up);
        }
        res.push(Right);
    }
    res
}
//some tests
#[test]
fn circle(){
    let k = 5;
    let mut b = vec![];
    for i in 0..(k as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as i32{
        let mut a = vec![];
        for j in 0..k {
            a.push(0)
        }
        b.push(a);
    }

    for i in 0..(k as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as usize {
        for j in 0..k  {
            print!("{}", b[i][j]);
        }
        println!();
    }
    for _ in 0..6{
        println!("///////////////////////////////////////");
    }
    circular_2x2(&mut b, k as i32);
    for i in (0..(k as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as usize).rev() {
        for j in 0..(k -(k as f32*(1.0-(2.0_f32).sqrt()/2.0)).ceil() as usize)  {
            print!("{}", b[i][j]);
        }
        println!();
    }
    let mut direction = circular_to_direction(&mut b, k as i32);
    complete_shape(&mut direction);
    print!("{:?}", direction);


}