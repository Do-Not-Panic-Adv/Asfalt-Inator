use std::ops::Deref;

// use robotics_lib::interface::Direction;
//The SalviniTool before building a new type of road must create a project to analise it and understand its need.
//the Project struct contains:
//-action_curves:that describe the path the robot is going to follow to build the road
//-cost: the amount of energy it is going to take to build the whole project
//-rock: the amount of rock it is going to need to build the project
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}
//Error type given to the user after checking a designed project, and whever there is an
//interaction in the road building functon
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StopReason{
    LowEnergy,
    MissingRocks,
    PonteSulloStretto
}
pub struct Project{
    pub(crate) curves_action: Vec<Direction>,
    pub(crate) cost: u32,
    pub(crate) rock: u32,
}
//ragruppo maybe?
pub struct UnfinishedProject{
    pub(crate) start_position: (usize,usize),
    pub(crate) curves_action: Vec<Direction>,
    pub(crate) cost: u32,
    pub(crate) rock: u32,
}

pub enum Shape{
    Square(u32),
    Rectangle(i32, i32),
    Roundabout(i32),
    Cross(i32),
    LongLong(i32),
    LShape(i32, i32)
}

//this mirrors the shape(for the circle/roundabout, but could also be implemented for a
pub(crate) fn mirror_direction(vec: &Vec<Direction>, angle: u32)->Vec<Direction>{
    let mut v:Vec<Direction> = vec![];
    match angle{
        0 => {
            for d in vec.len()-1 ..0 {
                if vec[d] == Direction::Right{
                    v.push(Direction::Up);
                }
                if vec[d] == Direction::Up{
                    v.push(Direction::Right);
                }
            }
        }
        1 => {
            for d in 0..vec.len()-1 {
                if vec[d] == Direction::Right{
                    v.push(Direction::Left);
                }
            }
        }
        2 =>{
            for d in 0..vec.len()-1 {
                if vec[d] == Direction::Right{
                    v.push(Direction::Left);
                }
                println!("debug?");
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
pub(crate) fn complete_shape(vec: &mut Vec<Direction>){
    for i in 0..2 {
        vec.append(&mut mirror_direction(&vec, i));
    }
}
// impl From<UnfinishedProject> Project{
//     fn from(){
//
//     }
// }

impl Shape{
    pub fn get_action_curve(&self) ->Vec<Direction>{
        let mut v:Vec<Direction> = vec![];
        match self {
            Shape::Square(side_lenght) => {
                let x = *side_lenght as i32;
                for i in 0..x/2{
                    v.push(Direction::Right);
                }
                for i in 0..x {
                    v.push(Direction::Up);
                }
                for i in 0..(x as f32/2.0 ).ceil() as i32{
                    v.push(Direction::Left);
                }
                let mut other = mirror_direction(&v, 2);
                println!("{:?}", other);
                v.append(&mut other);
            }
            Shape::Rectangle(x, y) => {
                for i in 0..x/2{
                    v.push(Direction::Right);
                }
                for i in 0..*y{
                    v.push(Direction::Up);
                }
                for i in 0..x/2{
                    v.push(Direction::Left);
                }
                let other = mirror_direction(&v, 2);
            }
            Shape::Roundabout(d) => {todo!()}
            Shape::Cross(l) => {

            }
            Shape::LShape(l, s) =>{

            }
            Shape::LongLong(L) => {

            }
        }
        v
    }
}
