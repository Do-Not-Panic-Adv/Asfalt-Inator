use std::ops::Deref;
use robotics_lib::interface::Direction;
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
pub fn directioner(d: & Direction)->(i32, i32){
    match d {
        Direction::Up => {(1,0)}
        Direction::Down => {(-1,0)}
        Direction::Left => {(0,-1)}
        Direction::Right => {(0,1)}
    }
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

impl Shape{
    pub fn get_action_curve(&self) ->Vec<Direction>{
        let mut v:Vec<Direction> = vec![];
        match self {
            Shape::Square(side_lenght) => {
                let x = *side_lenght as i32;
                for _ in 0..x/2{
                    v.push(Direction::Right);
                }
                for _ in 0..x {
                    v.push(Direction::Up);
                }
                for _ in 0..(x as f32/2.0 ).ceil() as i32{
                    v.push(Direction::Left);
                }
                let mut other = mirror_direction(&v, 2);

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
                todo!()
            }
            Shape::LShape(l, s) =>{
                todo!()
            }
            Shape::LongLong(L) => {
                todo!()
            }
        }
        v
    }
}
