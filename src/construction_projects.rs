use crate::construction_projects::StopReason::MissionImpossible;
use robotics_lib::interface::Direction;

//The Asphalt-inator before building a new type of road must create a project to analise it
// and understand its need. the Project struct contains:
//
//       -action_curves:that describe the path the robot is going to follow to build the road
//
//        -cost: the minimum amount of energy it is going to take to build the whole project
//
//        -rock: the minimum amount of rock it is going to need to build the project

///Error type given to the user after checking a designed project and whenever there is an
///interaction in the road building function
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StopReason {
    LowEnergy,
    MissingRocks,
    MissionImpossible,
}

#[derive(Clone)]
pub struct Project {
    pub curves_action: Vec<Direction>,
    pub min_cost: usize,
    pub min_rocks: usize,
}
// Using this Tool your robot is likely to end up having UnFinishedProjects, what are them?
// An UnFinishedProject is almost like a project except that it does not store the minimum
// resources, instead, it stores the position where the robot stopped building, and the reason
// why it stopped, still having stored the missing action to complete its job. Most of the time the reason
// why the robot ended up having an Unfinished project in its robotics hand it will be because of
// the low energy level or not enough rocks, when the robot will have found the missing resources
// it can redesign a project from the unfinished project.Sometimes, though, the reason why it was
// not possible to
#[derive(Clone)]
pub struct UnFinishedProject {
    pub start_position: (usize, usize),
    pub stop_reason: StopReason,
    pub(crate) curves_action: Vec<Direction>,
}
impl From<UnFinishedProject> for Project {
    fn from(unfinished_project: UnFinishedProject) -> Project {
        let number_of_steps = unfinished_project.curves_action.len();
        if unfinished_project.stop_reason == MissionImpossible {
            return Project {
                curves_action: vec![],
                min_cost: 0,
                min_rocks: 0,
            };
        }
        Project {
            curves_action: unfinished_project.curves_action.clone(),
            min_cost: expected_min_cost_tick(number_of_steps),
            min_rocks: number_of_steps,
        }
    }
}
impl UnFinishedProject {
    pub fn new(
        curves_action: &Vec<Direction>,
        stop_reason: StopReason,
        end_position: (usize, usize),
    ) -> UnFinishedProject {
        UnFinishedProject {
            start_position: end_position,
            stop_reason,
            curves_action: curves_action.clone(),
        }
    }
}
/// Shapes are the key of the a project, these will then define the path the robot will follow to
/// place the Streets. There are multiple types of Shapes and we are aiming on updating and giving
/// more and more option for new and funny construction idea.
///
/// # A brief look into some of them:
///
/// ## Squares:
///
///
/// ## Roundabout:
///
///
/// ## LongLong:
///
///
/// ## TShape:
///
///

pub enum Shape {
    Square(usize),
    Rectangle(usize, usize),
    Roundabout(usize),
    Cross(usize), //its a roman cross
    LongLong(usize, Direction),
    TShape(usize, usize, Direction),
    LShape(usize, usize, Direction),
}
impl Shape {
    pub fn get_action_curve(&self) -> Vec<Direction> {
        let mut res: Vec<Direction> = vec![];
        match self {
            | Shape::Square(side_lenght) => {
                for _ in 0..side_lenght / 2 {
                    res.push(Direction::Right);
                }
                for _ in 0..*side_lenght {
                    res.push(Direction::Up);
                }
                for _ in 0..(*side_lenght as f32 / 2.0).ceil() as i32 {
                    res.push(Direction::Left);
                }
                let mut other = mirror_direction(&res, 2);
                res.append(&mut other);
            }
            | Shape::Rectangle(base, height) => {
                for _ in 0..base / 2 {
                    res.push(Direction::Right);
                }
                for _ in 0..*height {
                    res.push(Direction::Up);
                }
                for _ in 0..base / 2 {
                    res.push(Direction::Left);
                }
                res = mirror_direction(&res, 2);
            }
            | Shape::Roundabout(dimension) => {
                res = make_circle(*dimension);
            }
            | Shape::Cross(l) => {
                for _ in 0..*l {
                    res.push(Direction::Up);
                }
                for _ in 0..l / 2 {
                    res.push(Direction::Down)
                }
                for _ in 0..l / 2 {
                    res.push(Direction::Right)
                }
                for _ in 0..*l {
                    res.push(Direction::Left);
                }
            }
            | Shape::TShape(l, s, direction) => {
                for _ in 0..*l {
                    res.push(direction.clone())
                }
                for _ in 0..l / 2 {
                    res.push(opposite_direction(direction))
                }
                for _ in 0..*s {
                    let to_push = match direction {
                        | Direction::Up => Direction::Right,
                        | Direction::Down => Direction::Left,
                        | Direction::Left => Direction::Up,
                        | Direction::Right => Direction::Down,
                    };
                    res.push(to_push);
                }
            }
            | Shape::LongLong(lenght, direction) => {
                for _ in 0..*lenght {
                    res.push(direction.clone());
                }
            }
            | Shape::LShape(long_side, short_side, direction) => {
                for _ in 0..*long_side {
                    res.push(direction.clone())
                }
                for _ in 0..*short_side {
                    let to_push = match direction {
                        | Direction::Up => Direction::Left,
                        | Direction::Down => Direction::Right,
                        | Direction::Left => Direction::Down,
                        | Direction::Right => Direction::Up,
                    };
                    res.push(to_push);
                }
            }
        }
        res
    }
}

//-------------------------------------------------------------------//
//Here are defined some, very, useful function to design some shapes
//------------------------------------------------------------------//

//directioner function takes in input a Direction and gives back
//the tuple representing the vector with the same direction and
//1 in module, in the cartesian notation
pub fn directioner(d: &Direction) -> (i32, i32) {
    match d {
        | Direction::Up => (1, 0),
        | Direction::Down => (-1, 0),
        | Direction::Left => (0, -1),
        | Direction::Right => (0, 1),
    }
}

//opposite_direction takes in input a direction and gives as
//output the opposite direction
pub fn opposite_direction(d: &Direction) -> Direction {
    match d {
        | Direction::Up => Direction::Down,
        | Direction::Down => Direction::Up,
        | Direction::Left => Direction::Right,
        | Direction::Right => Direction::Left,
    }
}

// mirror_direction takes in input a vector of directions and mirror+add
// to the vector the mirrored directions based on an 'angle' that defines
// an axes of symmetry:
// - 0: mirrors at 45
// - 1: mirrors at 90
// - 2: mirrors at 180
pub fn mirror_direction(vec: &Vec<Direction>, angle: u32) -> Vec<Direction> {
    let mut v: Vec<Direction> = vec![];
    match angle {
        | 0 => {
            for d in (0..vec.len()).rev() {
                if vec[d] == Direction::Right {
                    v.push(Direction::Up);
                }
                if vec[d] == Direction::Up {
                    v.push(Direction::Right);
                }
            }
        }
        | 1 => {
            for d in (0..vec.len()).rev() {
                if vec[d] == Direction::Right {
                    v.push(Direction::Left);
                } else {
                    v.push(vec[d].clone());
                }
            }
        }
        | 2 => {
            for d in 0..vec.len() - 1 {
                if vec[d] == Direction::Right {
                    v.push(Direction::Left);
                }
                if vec[d] == Direction::Up {
                    v.push(Direction::Down);
                }
                if vec[d] == Direction::Down {
                    v.push(Direction::Up);
                }
                if vec[d] == Direction::Left {
                    v.push(Direction::Right);
                }
            }
        }
        | _ => {}
    }
    v
}

// used to complete a shape, as it is now just circle
// given its 12,5%
pub fn complete_shape(vec: &mut Vec<Direction>) {
    for i in 0..3 {
        vec.append(&mut mirror_direction(&vec, i));
    }
}

//describe a circle as a vector of directions
pub fn make_circle(size: usize) -> Vec<Direction> {
    let mut b = vec![];
    for _ in 0..(size as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as i32 {
        let mut a = vec![];
        for _ in 0..size {
            a.push(0)
        }
        b.push(a);
    }
    circular_2x2(&mut b, size);
    let mut res = circular_to_direction(&mut b, size);
    complete_shape(&mut res);
    res
}

//describe a circle (1/8 of a circle) as 1 in a 2x2 matrix
pub fn circular_2x2(pippo: &mut Vec<Vec<i32>>, k: usize) {
    for riga in 0..(k as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as i32 {
        for colonna in 0..k - (k as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as usize {
            if riga == ((k as f32) - (-((colonna + 1) as f32).powi(2) + (k as f32).powi(2)).sqrt()).floor() as i32 {
                pippo[riga as usize][colonna as usize] = 1;
                // println!("true");
            }
            // else { println!("false"); }
        }
    }
}

//gives the direction from the 1/8 of circle
pub fn circular_to_direction(mat: &mut Vec<Vec<i32>>, k: usize) -> Vec<Direction> {
    let mut res = vec![];
    let mut raw_index = 0;
    let start = 1;
    for col in start..(k as usize - (k as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as usize) {
        if mat[raw_index][col] != 1 {
            raw_index += 1;
            res.push(Direction::Up);
        }
        res.push(Direction::Right);
    }
    res
}
//----------------------------------------------------------------//
//                  here just a platypus                          //
//                  (he loves asphalting)                         //
//----------------------------------------------------------------//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⣴⠶⠶⣒⣛⡚⠛⠛⠛⠋⠛⠛⠒⠚⠒⠒⠒⠒⠦⠤⢤⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⢸⢠⠞⠉⣴⣿⢳⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⣾⣄⠓⠤⠬⠥⠚⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡀⠀⠀⠀⠀⠀⠀⣀⣠⠤⠀⠤⣄⣀⠀⠀⠀⠀
// ⠀⠀⢀⣀⣠⣴⡇⠈⠳⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⠔⠀⠀⣽⠓⠒⠒⠖⡒⠉⠉⠀⠠⣀⠀⠀⠘⡎⠉⠲⣄⠀
// ⢠⠞⠋⠉⢉⣉⡁⠀⠀⠀⠉⢲⠀⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⡴⠋⠀⠀⠀⠀⡏⠀⠦⡀⠀⢨⠑⠢⣄⠀⠀⠉⠓⠢⢼⣀⣠⣤⠇
// ⠈⠙⠛⠋⠉⠉⠉⠓⠒⠒⢲⠟⣠⠾⡆⠀⡾⠒⠒⠛⠛⠒⠒⠛⢦⣀⢀⡮⡟⠉⠙⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠚⠋⠉⠁⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠶⠃⠠⠷⠿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠘⠶⠽⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//----------------------------------------------------------------//
//
//
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣦⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⡟⠁⠈⠉⠻⢶⣄⡀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢰⣯⣥⣄⣀⣤⣄⣤⣽⣿⣶⣞⠛⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀           it PERRY THE PLATYPUS⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢿⣿⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⢠⣤⣤⣤⣶⣾⣿⣿⣿⣿⣿⡿⠟⠛⣻⣟⣭⣿⠿⠿⠿⡿⠟⣟⢿⣿⣿⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⢨⣞⣿⡿⠉⠉⣳⡀⠻⣷⣿⣿⠀⠀⢠⠇⠀⣿⡼⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠈⠻⠿⢥⠤⠒⡟⠳⣀⠈⠛⠦⠴⠚⠁⠀⠀⢯⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⢀⣀⡠⠤⠤⠼⠦⣤⡇⠀⠈⠳⣄⣀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⣴⣞⣉⡀⠀⣀⣀⣀⣀⠀⠀⠠⠤⠤⠤⠉⠻⡆⠀⠀⠀⢸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠉⠉⠉⠁⠀⠈⡟⠓⠲⠤⠤⠤⠤⠶⠚⠁⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢱⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⢀⡴⢾⡋⠉⠀⠀⠀⠀⠀⠈⠉⠓⠦⢄⡀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣴⠋⠀⣀⡹⠶⡖⠊⠉⠉⠉⠉⠉⠉⠉⢉⣿
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣍⠀⠀⠀⠘⢦⡀⠀⠀⠀⠀⢀⡼⠋⠁
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢦⡀⢀⣀⡀⠽⢦⠤⣠⠔⠉⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⢰⠃⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡗⠦⣄⠀⠱⡄⠀⠀⠀⡼⠋⠁⠀⠀⠀⠀⠀
// ⠀⠀⠀⣠⣤⣴⣶⣤⠏⢀⠞⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠈⠳⣄⠘⣆⣤⢿⡶⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⢀⣝⣻⣿⢿⡟⢠⠏⠀⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣷⡖⠒⢂⣈⣿⡿⢿⣯⣤⣄⣀⠀⠀⠀⠀⠀
// ⠀⠀⠛⠛⠻⣿⠀⣩⠏⠀⠀⢻⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⢹⡿⣿⡿⠳⢶⣤⣿⣝⠛⠚⠃⠀⠀⠀⠀
// ⠀⠀⠀⠀⢀⣼⣰⠋⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⣻⠎⠀⠀⠀⠈⢹⣿⡇⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠘⢿⠃⠀⠀⢀⡴⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢧⡴⠉⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⣠⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⣼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⢿⠀⠀⠀⠀⣤⣤⠤⠤⠤⠤⠤⠤⠤⣤⣄⣀⠀⠀⠀⠀⠀⣹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠳⡀⠀⠸⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢹⡆⠀⠀⢠⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⢀⣀⠤⠤⠒⠛⢦⣤⢧⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡇⢀⣴⣋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠞⠯⣄⣀⠀⠀⢀⡠⠔⠚⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣎⣉⠙⠋⠉⠉⠉⠓⠢⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠈⡉⠓⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠲⢤⣤⠤⠔⠊⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀
//
//
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀

//function to get expected cost: will be updated for sure
fn expected_min_cost_tick(number_steps: usize) -> usize {
    return ((number_steps as f32) * 2.5) as usize;
}

#[test]
fn circle() {
    let k = 5;
    let mut b = vec![];
    for _ in 0..(k as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as i32 {
        let mut a = vec![];
        for _ in 0..k {
            a.push(0)
        }
        b.push(a);
    }

    for i in 0..(k as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as usize {
        for j in 0..k {
            print!("{}", b[i][j]);
        }
        println!();
    }

    circular_2x2(&mut b, k);
    for i in (0..(k as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as usize).rev() {
        for j in 0..(k - (k as f32 * (1.0 - (2.0_f32).sqrt() / 2.0)).ceil() as usize) {
            print!("{}", b[i][j]);
        }
        println!();
    }
    let mut direction = circular_to_direction(&mut b, k);
    complete_shape(&mut direction);
    print!("{:?}", direction);
}
