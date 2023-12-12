use crate::construction_projects::StopReason::MissionImpossible;
use crate::construction_projects::{directioner, Project,  StopReason, UnFinishedProject};
use robotics_lib::interface::{destroy, go, put, robot_map, Direction, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
use std::cmp::max;
use std::mem;
use std::ops::Deref;

pub mod construction_projects;
pub use construction_projects::Shape;
/// -----Welcommen in the Asphalt-inators house, entry pls!-----
/// # Tool: Asphaltinator
/// a useful tool used to plan and build Streets on the map to get where you want, when u want,
/// always on the road. The main aim of the robot is to trace some smart and pragmatic path to
/// connect the areas of its interest
///
/// ## Usage
/// you start by deciding which type of street type are you going to build. Once your Asphaltinator
/// has designed ur desired Project you are ready to see some streets get built. To sum up and add
/// some information on this marvelous piece of engineering, here are the main functionality:
///
/// -Designing Projects: just designing projects... and recycle unfinished one, but we will get on
///                     that later.
///
///
/// -Having FUN on ur Asphalt-inator: with the function asphalt u can see ur map starting to get more
///                                   and more "concrete", the climate change will like this
///
/// -Checks Design condition: Verify if it is possible or not to ASPHALT the designed area, and
///                           require a minimum amount of resources from the robot
///
///
/// The Asphaltinator Struct is pretty simple, let's get into this:
///
///    - project_number: it stores the value of the number of projects designed
///
///    - unfinished_projects: it stores in a vector all the Unfinished Project, that for some reason
///                           happened to end, before they were completed, and that they still can
///                           be continued with the enough resources
///
/// NB: This Tool does NOT aim to build the most efficient path, but is your robot that has to
/// figure out where is the best road to build in the area of most interest for your robot.
/// Also if u start asphalting a project this will happen to end before it finished, but don't worry
/// we got you, you will receive the unfinished project that you can start again whenever you want,
/// if that is not a MissionImpossible
///
///  # Example:
///
///            let mut asp=Asphaltinator::new();
///             let project= asp.design_project(Shape::Rectangle(3,3));
///             match project {
///                 Ok(project_ok) => {
///                     println!("Project created successfully");
///                     let work_result=asp.asfalting(
///                         self, world, project_ok,robot_map(world).unwrap());
///                     match work_result {
///                         Ok(_) => {println!("Asphaltinator built successfully your project")}
///                         Err(unfinished_project) => {
///                             println!("Well we didn't finished but you can still continue!");
///                         }
///                     }
///                 }
///                 Err(_) => {
///                     println!("Project not gone well");
///                 }
///             }
///
///
pub struct Asphaltinator {
    project_number: usize,
    unfinished_projects: Vec<UnFinishedProject>, //add that u can not save an unfinished project that ended with mission impossible to ur unfinsihedproject
}

impl Tools for Asphaltinator {}

impl Asphaltinator {
    pub fn new() -> Self {
        Asphaltinator {
            project_number: 0,
            unfinished_projects: vec![],
        }
    }
    pub fn save_unfinished_project(&mut self, un_finished_project: UnFinishedProject) -> Result<(), ()> {
        return if !(un_finished_project.stop_reason == MissionImpossible) {
            self.unfinished_projects.push(un_finished_project.clone());
            Ok(())
        } else {
            Err(())
        };
    }
    pub fn design_project(&mut self, shape: Shape) -> Result<Project, ()> {
        if !self.project_number > 2 * self.unfinished_projects.len() {
            self.project_number += 1;
            return Ok(Project {
                curves_action: shape.get_action_curve(),
                min_cost: 0,
                min_rocks: 0,
            });
        } else {
            Err(())
        }
    }
    pub fn check_project_here(
        robot: &impl Runnable,
        world: &World,
        project: &Project
    ) -> Result<(), StopReason> {

        //Just checks if the project is inside the map in the whole project
        let (x_position, y_position) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
        let map_size = robot_map(world).expect("need to check").len();
        let possible=Asphaltinator::check_directions_allowed(
            x_position,y_position,&project.curves_action, map_size);

        if !possible{
            return Err(MissionImpossible);
        }

        //checks if has minimum energy and rocks
        let robot_rocks = robot.get_backpack().get_contents();
        let robot_rocks = robot_rocks.get(&Content::Rock(0)).unwrap();
        let robot_energy = robot.get_energy().get_energy_level();
        if project.min_cost > robot_energy {
            return Err(StopReason::LowEnergy);
        }
        if project.min_rocks > *robot_rocks {
            return Err(StopReason::MissingRocks);
        }
        Ok(())
    }

    pub fn asfalting(
        &self,
        robot: &mut impl Runnable,
        world: &mut World,
        project: Project,
        map: Vec<Vec<Option<Tile>>>,
    ) -> Result<(), UnFinishedProject> {
        let mut sequence = project.curves_action.clone();
        let mut copy_sequence = project.curves_action.clone();
        for direction in copy_sequence.iter_mut() {
            let shift = directioner(&direction);
            if let Some(target_tile) = map[((robot.get_coordinate().get_col() as i32) + shift.0) as usize]
                [((robot.get_coordinate().get_row() as i32) + shift.1) as usize]
                .clone()
            {
                let target_tile_type = target_tile.tile_type;
                let mut stop_type = match target_tile_type {
                    | TileType::Street => Ok(0),
                    | TileType::DeepWater | TileType::Lava => put(robot, world, Content::Rock(0), 3, direction.clone()),
                    | TileType::ShallowWater => put(robot, world, Content::Rock(0), 2, direction.clone()),
                    | TileType::Sand | TileType::Grass | TileType::Snow | TileType::Hill => {
                        put(robot, world, Content::Rock(0), 1, direction.clone())
                    }
                    | TileType::Mountain => put(robot, world, Content::None, 1, direction.clone()),
                    | TileType::Teleport(_) | TileType::Wall => Err(LibError::OutOfBounds),
                };
                //if error is caused by MustDestroyContentFirst, it trys to destroy it
                if stop_type == Err(LibError::MustDestroyContentFirst) {
                    match destroy(robot, world, direction.clone()) {
                        | Ok(_) => {
                            stop_type = Ok(0);
                        }
                        | Err(destroying_error) => {
                            stop_type = match destroying_error {
                                | LibError::NotEnoughEnergy => Err(LibError::NotEnoughEnergy),
                                | LibError::OutOfBounds => Err(LibError::OutOfBounds), //?
                                | LibError::NotEnoughSpace(value) => Ok(value),
                                | LibError::CannotDestroy => Err(LibError::OutOfBounds),
                                // LibError::MustDestroyContentFirst => Err(MustDestroyContentFirst), //? just checked
                                | _ => {
                                    println!("UNEXPECTED SCENARIO TRYING TO DESTROY Err: {:?}", destroying_error);
                                    Err(LibError::OutOfBounds)
                                }
                            };
                        }
                    }
                    //try to place it again
                    stop_type = match target_tile_type {
                        | TileType::Street => Ok(0),
                        | TileType::DeepWater | TileType::Lava => {
                            put(robot, world, Content::Rock(0), 3, direction.clone())
                        }
                        | TileType::ShallowWater => put(robot, world, Content::Rock(0), 2, direction.clone()),
                        | TileType::Sand | TileType::Grass | TileType::Snow | TileType::Hill => {
                            put(robot, world, Content::Rock(0), 1, direction.clone())
                        }
                        | TileType::Mountain => put(robot, world, Content::None, 1, direction.clone()),
                        | _ => {
                            println!(
                                "UNEXPECTED TARGET_TILE_TYPE TRYING TO RE-PLACE STREET Err: {:?}",
                                target_tile_type
                            );
                            Err(LibError::OutOfBounds)
                        }
                    };
                }
                //there are no problems, just keep swimming <_>
                if !stop_type.is_err() {
                    stop_type = match go(robot, world, direction.clone()) {
                        | Ok(_) => Ok(0),
                        | Err(stop_type_error) => Err(stop_type_error),
                    }
                }
                //final match to check the final result of the operation
                match stop_type {
                    | Ok(_) => {
                        sequence.remove(0);
                        return Ok(());
                    }
                    | Err(error_type) => match error_type {
                        | LibError::NotEnoughEnergy => {
                            return Err(UnFinishedProject {
                                start_position: (robot.get_coordinate().get_row(), robot.get_coordinate().get_col()),
                                stop_reason: StopReason::LowEnergy,
                                curves_action: sequence,
                            })
                        }
                        | LibError::OutOfBounds | LibError::CannotDestroy | LibError::WrongContentUsed => {
                            return Err(UnFinishedProject {
                                start_position: (robot.get_coordinate().get_row(), robot.get_coordinate().get_col()),
                                stop_reason: StopReason::MissionImpossible,
                                curves_action: sequence,
                            })
                        }
                        | LibError::NotEnoughContentInBackPack => {
                            return Err(UnFinishedProject {
                                start_position: (robot.get_coordinate().get_row(), robot.get_coordinate().get_col()),
                                stop_reason: StopReason::MissingRocks,
                                curves_action: sequence,
                            })
                        }
                        //checked before
                        // MustDestroyContentFirst => {}
                        // LibError::CannotWalk => {}
                        | _ => {
                            println!("UNEXPECTED LAST STOP_TYPE ERROR Err: {:?}", error_type);
                            return Err(UnFinishedProject {
                                start_position: (0, 0),
                                stop_reason: StopReason::MissingRocks,
                                curves_action: vec![],
                            });
                        }
                    },
                }
            }
        }
        Ok(())
    }
    fn check_directions_allowed(robot_x: usize, robot_y: usize, curves: &Vec<Direction>, map_dim:usize)->bool{
        let mut virtual_position:(i32,i32) =(robot_x as i32, robot_y as i32);
        for direction in curves.iter(){
            match direction {
                | Direction::Up => {
                    virtual_position.0-=1;
                }
                | Direction::Down => {
                    virtual_position.0+=1;
                }
                | Direction::Left => {
                    virtual_position.1-=1;
                }
                | Direction::Right => {
                    virtual_position.1+=1;
                }
            }
            if virtual_position.0 <0 || virtual_position.1 <0 {
                return false;
            }
        }
        return true;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    //it compiles but not work as i thought lol

    #[test]
    fn it_works() {}
}

// ⡙⡔⢃⠲⡌⠦⣉⠖⡰⣉⠦⡙⢢⠜⡒⢌⡊⢜⡠⢍⠢⣉⠱⢂⠍⡰⢉⠆⡱⢈⠒⡄⢊⠔⡡⢂⠍⡔⢊⠔⡡⠑⡌⠰⡈⠔⡡⠘⡄⢃⠒⡄⢣⠐⢢⠉⢆⡘⠔⡡⠚⡄⣃⠣⡑⣂⠓⠤⡙⢢⠜⣠⠣⣑
// ⡔⡩⢔⡩⢆⡱⢌⡃⢧⠘⡥⠢⢍⡒⠔⣢⠱⡁⠎⡔⡡⡘⢄⡒⢌⠢⣁⠎⡄⢃⠆⡡⢊⠔⡡⢊⠔⡡⠊⡔⠡⢊⠄⠣⡘⠄⠣⠌⡑⢌⠢⣁⠣⠘⡄⢣⠘⠤⡉⢆⡉⢆⡘⠤⣁⠣⠔⢢⠑⡌⠤⡙⢄⠣⣡⠊⣄⠓⠤
// ⡑⠎⡜⡰⣑⢊⡔⢢⡑⠦⣉⠦⠩⡔⣉⠦⢌⠱⡀⠇⣌⠱⢠⠑⡌⢢⠘⡄⢣⠐⡌⠰⣁⠊⠔⡡⢘⠠⣁⠒⡠⢑⠠⠃⢌⠰⠁⡔⠨⢁⠆⣁⠂⠆⡐⠨⡁⠔⢂⠡⢂⡑⠄⡒⠤⠘⡐⠄⢣⠘⡄⢣⠘⠤⡑⢌⠒⡄⢣⡐⠩⠜
// ⠥⢚⡘⠴⡑⣂⠧⣈⠇⡜⢢⠡⢎⠱⡐⠢⠜⡠⢃⠜⡐⢢⠑⢢⠑⢌⠂⠥⡘⢠⠡⠌⢡⠀⠎⡐⡁⠆⡡⠄⢂⡁⠆⢂⠉⡄⣂⣥⣶⣷⣦⣬⣤⣘⠠⠌⢡⠐⡈⢂⠡⠂⡄⠣⢐⠂⡑⡈⠜⢠⠡⡘⣀⠃⢆⠱⢈⠒⡌⠢⢌⠱⡘
// ⠥⢣⠜⣡⠚⢤⠒⡡⢎⠰⣁⠣⠌⢆⡡⢍⠢⡑⢌⠢⡉⠔⡈⢆⡘⠠⡉⠔⡈⠆⢂⡉⠤⠘⠠⢁⠔⠡⠐⡈⠄⡐⠈⠄⢂⠄⡉⠛⠛⠯⣟⣿⣽⣻⢿⣶⣄⢂⠐⡀⢂⠡⠄⢃⠂⢡⠐⡐⢈⠂⠆⡑⢠⠊⠰⢈⠂⡅⢢⠑⡌⢢⠑
// ⡬⢡⠚⢤⡉⠦⣉⠴⣈⠱⡀⢇⠚⡄⠒⡌⠰⡁⠆⡱⠈⢆⠡⠂⢄⠃⠄⢃⠰⠈⡄⠐⡠⠁⠌⠠⠈⠄⠡⠐⠠⢀⠡⠈⠄⠂⠄⠡⢈⠐⠈⠙⠾⣽⣯⣟⡿⣷⡄⡐⢀⠂⠌⠠⠈⠄⠂⠄⠡⢈⠐⡈⠄⠌⣁⠂⡡⠐⢡⠈⡔⢂⠡
// ⡒⡡⢍⠢⠜⣐⠢⠒⡄⢣⠘⡄⠣⢌⡑⠨⡁⠔⠡⠄⡉⢄⠂⡉⠄⠨⠐⡈⠄⠡⢀⠡⢀⠡⠈⠄⠡⠈⠄⢁⠂⠄⠂⢁⣌⣤⣌⣤⣀⣈⣐⠈⠀⠙⢷⣯⢿⡽⣿⡄⠠⠈⡀⣡⣨⣤⣥⡌⠐⡀⢂⠐⡈⠐⡀⢂⠡⠘⠠⣁⠰⢈⠰
// ⠱⡘⠤⣉⠖⢡⢊⠱⡈⢆⠱⡈⢅⠢⠌⢡⠐⡉⠰⠈⠔⠂⠌⡐⢈⠁⢂⠐⡈⠐⡀⠂⠄⠐⠈⡀⠂⠁⡈⠀⠠⠀⣢⣿⣿⣻⣟⣿⣻⣟⡿⣿⣿⣶⣌⣿⣯⢿⡽⣇⣤⣷⣿⣟⣿⡿⠟⠛⢀⠐⠀⠂⠄⠡⠐⡀⢂⠁⢂⠄⢂⠌⠠
// ⡱⢘⡐⢢⠘⡄⢊⠔⡡⠌⡂⢌⢂⠒⠨⠄⠒⡈⠡⠘⠠⢁⠂⡐⠠⠈⠄⠂⠠⠁⢀⠂⠈⢀⠐⠀⠠⠁⠀⠀⣀⣿⣿⣻⣞⣷⣻⢞⡷⣯⣟⡷⣽⣞⣿⣻⢯⡿⣽⢿⣻⣽⣾⠛⣁⡀⠀⠀⠂⠈⢀⠁⠌⠐⢀⠐⠠⠈⠄⡈⠄⠌⢂⠐
// ⠰⢡⠘⡄⢣⠘⡄⢊⠔⡠⢁⠎⣀⠊⠡⠈⠔⠠⠁⠌⡁⢂⠐⠀⡁⠐⠀⠌⠀⠐⠀⠀⠠⠀⠀⠐⠀⠀⠀⣰⣿⣟⡾⠷⠛⠚⣉⣩⣭⣭⡿⣿⢿⣿⡾⣽⢯⣿⣽⣯⢷⣯⣟⡿⣿⢷⣾⣦⣆⠁⠀⠠⠀⠈⡀⠄⠂⢁⠐⠠⢈⠐⡀⢂⠐
// ⡑⢢⠑⡌⢂⠥⠘⣀⠒⠠⡁⢂⠄⠌⠡⠈⠄⠡⢈⠐⠠⠀⠂⠁⠀⠄⠁⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⡟⠋⠁⢠⣴⣾⡿⣟⡿⣽⢯⡿⣽⣻⡾⠽⠿⣟⣾⢯⣿⢿⡺⢽⣻⡽⣯⣟⣯⡟⠀⠀⠀⠀⠁⠀⠀⠐⠀⠠⠁⢀⠂⠐⡀⢂⠐
// ⡌⢢⠑⡈⠆⢌⠡⠄⢊⠡⠐⠂⠌⠠⠁⠌⠠⢁⠀⠂⠁⠐⠀⠁⢀⠠⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⢸⣿⣷⣻⣽⣻⡽⠛⠋⡵⠃⠀⠀⠀⠈⠻⣟⣾⣻⢿⣦⠈⠻⣷⣻⢾⠁⠀⠀⠀⠀⠀⠂⠈⠀⠠⠀⠂⠀⠄⠁⡀⢂⠐⠠⢁
// ⡘⠄⡃⠜⡈⠔⠂⠌⠂⡄⢃⠡⠈⠄⠡⠈⠄⠀⠂⠁⠠⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣾⠗⠋⠁⠀⣠⠞⠀⠀⠀⠀⠀⠀⠀⡜⣿⡽⣯⣟⣧⠀⠈⢿⣟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠈⠀⠐⠀⡀⠂⠁⠄⡈⠄⠡
// ⢀⠣⠘⡠⢁⠎⡈⠔⠡⠐⡀⢂⠡⠈⠄⠁⠠⠈⠀⠐⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⡿⠋⠀⠀⠀⠀⡴⠃⠀⠀⠀⠀⠀⠀⠀⢠⡇⢸⣿⣳⢯⣿⡇⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠈⠀⡀⠄⠠⠁⠂⠐⡈⠄⡁
// ⢁⠊⢅⡐⢂⠰⠀⡌⠠⢁⠐⡀⠂⢈⠀⠌⠀⠀⠂⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣞⣁⣀⡀⠀⠀⠀⠀⠀⠀⢸⠃⠀⠛⠋⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⢀⠠⠀⢁⠂⠐⠠⢀⠁⢂
// ⢁⠎⢠⠐⡈⠄⠡⢀⠁⠂⠄⠐⠈⡀⠠⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⠟⠛⠁⠀⠀⠀⠈⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠠⠀⠠⠈⠐⠠⠈⠄
// ⠃⡌⠄⢢⠁⠌⡐⠠⢈⠐⠈⡀⠁⢀⠀⠠⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⠟⢁⡀⠀⠀⠀⠀⠀⠀⠀⢀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀   ⠐⠈⠀⠀⠄⠁⡈⠄⢈⠐⡀
// ⠡⠐⠌⠂⠌⡐⠠⠁⠄⠂⠁⡀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⠟⠉⠉⠉⠓⠣⣦⡀⠀⠀⠀⠀⢸⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀Asphalt-inator  ⠂⠀⠐⠀⡀⠂⠠⠐
// ⠜⡈⡐⢉⠐⠠⠁⠌⢀⠂⠁⠀⠐⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣶⣶⣦⣤⣀⠘⣧⣠⣿⠟⠁⠀⠀⠀⣠⣄⡀⠈⣷⡀⠀⠀⠀⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ an Evil inc. Product   ⢀⠐⠀⠡⠀
// ⠐⠤⠐⠂⠌⠠⢁⠂⠠⠀⠌⠀⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠋⢙⣿⠛⠉⠉⢙⣿⣿⣿⡿⠋⠀⠀⠀⢀⣾⣽⣿⣿⣷⡜⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⡀⠄⠈⡀⠁
// ⢌⠠⠑⡈⠄⡁⠂⠠⠁⠠⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡟⠀⠀⠀⠀⣾⣿⣿⣿⠀⠀⠀⠀⠀⠈⠻⣿⣟⣾⠟⢱⣏⠀⠀⢸⠃⣠⠴⠒⠓⠒⢤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠂⢀⠁
// ⠤⠘⡐⠀⠆⠠⠁⠂⠄⠁⡀⠠⠀⠁⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡡⠀⠀⠀⠀⠈⠛⠻⣯⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⣼⠏⠀⠀⣼⠞⠁⠀⠀⠀⠀⠀⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⡀⠁⠐⠀⠂
// ⢂⠡⠄⠃⠌⠠⠁⠂⠐⡀⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠷⣄⣀⠀⣀⣀⣰⠴⠿⣄⠀⠀⠀⠀⠀⠀⠀⠀⣀⡞⠃⠀⠀⠀⠋⠀⠀⠀⠀⠀⠀⠀⣸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠐⠈⢀⠁
// ⠠⠂⠌⡐⠈⠄⠡⠈⡀⠄⠐⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⣹⠋⠀⠈⡠⠂⠙⠧⣆⣀⣀⣀⣠⡤⠾⣉⡠⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡴⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⠀⠁⠠⠈⠀⠄
// ⡄⢃⠰⢀⠡⠈⠄⢁⠠⠀⠂⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⡤⠤⠶⠖⠦⠳⠒⠒⠊⠀⠀⠀⠀⠀⠀⠀⣠⡴⠖⠋⠁⢠⣿⠀⠀⠈⡗⠦⠤⡤⠤⠖⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠈⠀⠄⠂⠁⠄
// ⠠⠌⡐⠠⢂⠡⢈⠀⠄⠂⢀⠁⠀⠠⠀⠀⡀⠀⠀⠀⠀⠀⠀⣀⡴⠖⠋⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⢻⠉⠀⠀⠀⠀⣼⣿⡇⠀⠀⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⠀⠄⠀⠂⠁⠄
// ⣁⠒⡈⠰⢀⠂⠄⡈⠄⠂⡀⠄⠐⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠋⠀⠀⠀⠀⠀⠀⠀⢀⣀⣠⠤⠔⠒⠋⠁⠀⣠⠖⠉⠀⡜⠀⠀⠀⠀⢠⣿⣿⠅⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⢀⠂⠀⠌⢀⠁⠂
// ⢀⢂⠡⠒⠠⢈⠐⠠⠐⡀⠄⢀⠂⠀⠈⠀⠀⠀⡀⣠⡟⠁⠀⠀⠀⣀⣠⡤⠶⠒⠋⢉⡏⠀⠀⠀⠀⢀⡠⠞⠁⠀⠀⠀⠇⠀⠀⠀⢀⣼⣿⣿⠀⠀⠀⠘⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠄⢀⠠⠈⠀⠄⡈⠐
// ⢃⠄⠢⢑⠈⡄⠈⠄⠡⢀⠂⠠⢀⠈⠀⠐⠀⢀⣼⣫⣤⠴⠖⠛⠋⠉⠀⠀⠀⠀⠀⢼⣃⣠⡤⠴⢾⠉⠀⠀⠀⠀⣀⣠⡤⠴⠒⠋⠉⢻⣿⠃⠀⠀⠀⠀⠙⢤⣄⣀⣀⣀⣀⣀⣴⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⢀⠀⠂⠀⠄⡁⠂⠄⡁
// ⠂⡌⡁⠆⢌⠠⠁⠌⡐⢀⠂⡁⠠⠐⠈⢀⠀⠂⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣠⢤⠶⠚⠋⠉⠀⠀⠀⠀⠀⢀⡴⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⣿⣿⣿⣯⠉⠻⡒⠲⠤⣀⡀⠀⠀⠀⠀⠀⠠⠐⠀⠀⠄⠀⠂⠁⠄⡐⢈⠐⡀
// ⢃⠰⢈⠔⠂⡌⢈⠔⡀⢂⠐⠠⢁⠐⠈⢀⠠⠀⢀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡞⠉⠁⠀⠊⠀⠀⠀⠀⠀⠀⣀⡤⠖⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⡄⠀⠈⠢⡀⠀⠉⠓⠦⣄⡀⠄⠀⠀⠐⠀⠄⢈⠠⠁⢂⠐⡀⢂⠐
// ⡁⠎⡰⢈⠒⠠⡁⢂⠐⠠⢈⠐⡀⢂⠡⠀⠠⠐⠀⢀⠀⢀⠠⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⡿⠤⠤⠤⠤⠤⠤⠔⠒⠒⠊⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⣄⣠⣀⣼⣿⣿⣿⣿⣿⡄⠀⠀⠑⡄⠀⠀⠀⠈⠙⢦⣄⠁⠠⠈⢀⠂⠐⡈⠄⢂⠐⠠⠌
// ⠱⡈⠔⣂⠩⠐⡄⠡⢈⢂⠁⡂⢐⠀⠂⠌⡀⠐⠈⡀⠠⠀⠀⠀⡀⠀⠀⠀⢾⡛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⠤⠴⠖⠚⠉⠉⠉⠉⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠀⠈⢄⠀⠀⠀⠀⠀⠈⠳⣄⠈⡀⠌⠐⠠⠈⠄⠌⢂⠌
// ⡱⢈⠒⠤⣁⠣⢀⢃⠂⢌⠐⡈⠤⢈⠐⠠⢀⠁⠂⠄⡐⠀⢁⠀⡀⠄⠁⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⠴⠖⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡞⠉⠛⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠈⣆⠀⠀⠀⠀⠀⠀⠙⢦⡐⡈⠄⡁⢊⠰⠈⡄⢊
// ⡄⢣⠘⡰⢀⠎⠰⡈⢌⡐⢂⠁⠆⡈⠤⢁⠂⠌⡐⠠⠐⢈⠀⠄⠀⠄⠠⠈⢠⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⠖⠋⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠟⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠸⡄⠀⠀⠀⠀⠀⠀⠈⢳⡐⠄⢂⡁⠢⢡⠐⡡
// ⡰⢁⠎⡰⢁⠎⣁⠒⡄⠢⢌⠘⡠⢁⠢⠄⣈⠐⠠⠁⠌⠠⠈⠄⡈⠠⠐⠀⣾⠁⠀⠀⠀⠀⠀⣀⣤⠶⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠻⡄⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⣳⠀⠀⠀⠀⠀⠀⠀⠀⢻⡌⠤⠐⡡⢂⠜⠠
// ⠱⣈⢒⢡⠊⡔⢠⠃⡔⠡⢌⠂⡅⢂⠆⠒⢠⠈⢂⠅⠊⠄⡁⠂⠄⡁⠂⡁⢿⡠⠀⣀⣠⡴⠚⠉⠀⡀⢀⢀⣴⠚⠲⠤⣤⣄⣀⣀⣄⣤⣌⡀⠀⠀⠁⠀⠄⠂⢀⠂⠀⣙⣦⡀⢰⣿⣿⣿⣿⣿⣿⣿⣿⡅⠀⠀⣠⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⡌⢡⠐⡌⢢⠑
// ⠱⣐⠊⡤⢃⠜⡠⢃⠌⡑⠢⡑⢨⠐⡌⡘⢠⠘⠠⠌⢂⠡⡐⢁⠂⠄⠡⠐⠠⠉⠍⡉⠀⠄⡐⠠⠁⡀⣰⠟⠘⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⢳⡌⠀⠈⡀⠄⠐⢰⡚⠋⠉⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠺⠥⢄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣀⠣⡘⢄⢃
// ⠵⢠⢋⠔⡡⢎⠰⡁⢎⠰⢡⠘⡄⢣⠐⡌⠰⣈⠡⡘⢠⠡⢐⡈⠰⠈⡄⠡⢂⠉⡐⠠⢃⠐⣠⣁⣐⣞⣡⠤⠤⢧⡉⠙⢒⠲⠤⡤⣤⠀⠀⠀⢷⡈⠀⠄⡐⠀⠂⣧⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⢹⠁⠀⠀⠀⠀⠀⢀⠀⠀⠘⣇⢆⠱⡈⢆
// ⡌⢣⠜⣨⠑⣌⢒⡉⢆⠱⢊⠔⣈⠆⡱⢈⠥⡐⢂⠅⢢⠘⡠⢐⠡⠒⢠⢁⠢⢌⣠⣿⠛⠉⠉⠉⠀⠀⠀⠀⠀⠈⠳⣌⢀⠂⡐⠠⢸⡇⠀⠀⠘⡇⠌⡐⠠⢁⠂⣿⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⢸⠀⡇⠀⠀⠀⠀⠸⠀⠀⠀⢹⡌⢆⡱⣈
// ⡔⣃⠚⡤⡙⢤⢊⠴⡉⢆⡃⢎⠰⢌⡰⢁⡒⢌⠢⡘⢄⠣⡐⢡⠂⡍⢄⢂⡶⠛⠁⠀⣿⠒⣶⠒⠒⠒⠒⣶⣆⠀⠀⠙⢦⡂⠄⠡⢘⡇⠀⠀⠀⣿⠐⠠⢁⠂⣼⠃⡇⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⠀⠀⠀⠀⠀⡸⠀⡇⠀⠀⠀⠀⠀⡇⠀⠀⠀⢿⠰⡐⢆
// ⡘⢤⠋⡴⢑⡊⣌⠒⣍⠢⢜⡨⢒⡌⠔⣡⠘⡄⢣⠘⡄⢣⠘⡄⢣⠐⣬⠟⠁⠀⠀⣼⠃⣾⠁⠀⠀⢠⡾⠁⡌⢷⡀⠀⠈⠳⣬⠥⠼⡇⠀⠀⠀⣿⢈⡁⠆⣸⠇⠀⣧⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⡇⠀⡇⠀⠀⠀⠀⠀⡇⠀⠀⠀⠸⡧⡙⢤
// ⢍⢢⡙⢤⠣⢜⡠⢋⠤⢋⡒⡔⢣⡘⠜⣠⠣⡘⠤⢃⡜⢠⠃⡜⢠⠣⣏⢀⣀⠤⠾⠧⠧⠿⠤⣀⣀⣿⠄⣷⢺⠛⢳⡀⠀⠀⠙⢆⢸⠃⠀⠀⢰⡇⠢⢌⢰⡟⠀⠀⢸⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⢀⠃⢠⠃⠀⠀⠀⠀⠀⣣⠀⠀⠀⠀⢳⡍⠦
// ⢌⡒⠜⡢⢍⢢⠱⣉⢌⢣⢒⡉⢖⡨⠓⡤⢃⠵⡉⠦⢌⠥⠚⣄⣣⡼⠋⠁⢳⣄⠀⠀⠀⠀⠀⠀⠀⠈⠙⠻⢤⣳⢤⣧⠀⠀⠀⠈⢻⠀⠀⠀⣿⢈⡑⢂⡾⠁⠀⠀⢸⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⢸⠀⢸⠀⠀⠀⠀⠀⠀⣹⠀⠀⠀⠀⠸⣎⠱
// ⢌⡌⢣⡑⢎⢢⠓⢬⡘⣂⠎⡜⢢⢡⢋⠴⡉⢦⠱⡉⠖⣌⢓⡼⠃⠀⠀⢀⡞⠈⢿⡓⡶⡖⠦⠤⣄⣀⠀⠀⠀⠈⠉⠳⡄⠀⠀⠀⢸⠀⠀⢸⡇⢆⡘⡾⠁⠀⠀⠀⢸⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⡇⠀⡞⠀⠀⠀⠀⠀⠀⣼⠀⠀⠀⠀⠀⣿⡐
// ⠲⢌⡱⠌⣆⠣⢚⡔⢢⡑⠎⡬⣑⠢⢍⢢⡙⠤⣃⠹⢌⠢⣿⠁⠀⢀⣤⣾⠀⠀⠀⠙⠳⠇⠀⠀⠀⣿⣿⠒⢦⣀⠀⠀⠘⠀⠀⠀⠀⠀⠀⣿⠌⣂⡾⠁⠀⠀⠀⠀⢸⠀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⢠⠁⢰⠇⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⢹⡔
// ⠲⣡⠒⡍⢤⢋⡒⢬⡁⢎⡱⠒⣌⠱⣊⠒⡬⢱⢨⡑⠎⣽⡷⠀⠀⢸⡇⢾⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⣯⣷⠀⠀⠀⠀⠀⠀⠀⢸⣇⢚⡾⠁⠀⠀⠀⠀⢀⣾⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⡘⠀⡞⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢸⡎
// ⢎⡱⢄⡋⡜⡰⢊⡜⡰⣘⠢⣅⠫⢄⠳⣈⠵⡘⢢⢅⡚⠜⡸⣇⠀⠀⠀⠻⡾⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣻⠀⠀⠀⠀⠀⠀⢀⡿⢢⡟⠁⠀⠀⠀⠀⢠⣟⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⢠⠃⡰⠁⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠘⡧
