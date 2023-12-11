use crate::projects::{directioner, Project, Shape, StopReason, UnFinishedProject};
use robotics_lib::interface::{destroy, go, put, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
//use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;

mod projects;

//
pub struct Asfaltinator {
    project_number: i32,
    unfinished_projects: Vec<UnFinishedProject>,
}

impl Tools for Asfaltinator {}

impl Asfaltinator {
    pub fn new() -> Self {
        Asfaltinator {
            project_number: 0,
            unfinished_projects: vec![],
        }
    }
    pub fn save_unfinished_project(&mut self, un_finished_project: UnFinishedProject) {
        self.unfinished_projects.push(un_finished_project);
    }
    //ti dice qunanto vai a sependere (non so se fare passando il robot o passando la cacca addosso), non mi serve a nulla self i guess
    pub fn design_project(&mut self, shape: Shape) -> Project {
        self.project_number += 1;
        return Project {
            curves_action: shape.get_action_curve(),
            cost: 0,
            rock: 0,
        };
    }

    //This function of the SalviniBot, takes in input a Project, and return a result
    //-Ok if the project was successfully built
    //-Err if the project was stopped, inside the Err there is an UnFinishedProject that contains
    //the values of a Project + the reason why the project didnt successfully end and the position
    //where it stopped
    pub fn asfalting(
        &self,
        robot: &mut impl Runnable,
        world: &mut World,
        project: Project,
        map: Vec<Vec<Option<Tile>>>,
    ) -> Result<(), UnFinishedProject> {
        // let number_of_rocks = robot.get_backpack().get_contents();
        // let robot_energy = robot.get_energy().get_energy_level();
        let mut sequence = project.curves_action.clone();
        //i dont when i delete elements from the sequence (when a full-cycle is done) interfere with the iter_mut()
        let mut copy_sequence = project.curves_action.clone();
        for direction in copy_sequence.iter_mut() {
            let shift = directioner(&direction);
            //this crash when robot on edge of the map, there is no boundaries check before put and go interfaces
            if let Some(target_tile) = map
                [((robot.get_coordinate().get_col() as i32) + shift.0) as usize]
                [((robot.get_coordinate().get_row() as i32) + shift.1) as usize]
                .clone()
            {
                let target_tile_type = target_tile.tile_type;
                let mut stop_type = match target_tile_type {
                    TileType::Street => Ok(0),
                    TileType::DeepWater | TileType::Lava => {
                        put(robot, world, Content::Rock(0), 3, direction.clone())
                    }
                    TileType::ShallowWater => {
                        put(robot, world, Content::Rock(0), 2, direction.clone())
                    }
                    TileType::Sand | TileType::Grass | TileType::Snow | TileType::Hill => {
                        put(robot, world, Content::Rock(0), 1, direction.clone())
                    }
                    TileType::Mountain => put(robot, world, Content::None, 1, direction.clone()),
                    TileType::Teleport(_) | TileType::Wall => Err(LibError::OutOfBounds),
                };
                //if error is caused by MustDestroyContentFirst, it trys to destroy it
                if stop_type == Err(LibError::MustDestroyContentFirst) {
                    match destroy(robot, world, direction.clone()) {
                        Ok(_) => {
                            stop_type = Ok(0);
                        }
                        Err(destroying_error) => {
                            stop_type = match destroying_error {
                                LibError::NotEnoughEnergy => Err(LibError::NotEnoughEnergy),
                                LibError::OutOfBounds => Err(LibError::OutOfBounds), //?
                                LibError::NotEnoughSpace(value) => Ok(value),
                                LibError::CannotDestroy => Err(LibError::OutOfBounds),

                                // LibError::MustDestroyContentFirst => Err(MustDestroyContentFirst), //? just checked
                                _ => {
                                    println!(
                                        "UNEXPECTED SCENARIO TRYING TO DESTROY Err: {:?}",
                                        destroying_error
                                    );
                                    Err(LibError::OutOfBounds)
                                }
                            };
                        }
                    }
                    //try to place it again
                    stop_type = match target_tile_type {
                        TileType::Street => Ok(0),
                        TileType::DeepWater | TileType::Lava => {
                            put(robot, world, Content::Rock(0), 3, direction.clone())
                        }
                        | TileType::ShallowWater => put(robot, world, Content::Rock(0), 2, direction.clone()),
                        | TileType::Sand | TileType::Grass | TileType::Snow | TileType::Hill => {
                            put(robot, world, Content::Rock(0), 1, direction.clone())
                        }
                        TileType::Mountain => {
                            put(robot, world, Content::None, 1, direction.clone())
                        }
                        _ => {
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
                        Ok(_) => Ok(0),
                        Err(stop_type_error) => Err(stop_type_error),
                    }
                }
                //final match to check the final result of the operation
                match stop_type {
                    Ok(_) => {
                        sequence.remove(0);
                        return Ok(());
                    }
                    Err(error_type) => match error_type {
                        LibError::NotEnoughEnergy => {
                            return Err(UnFinishedProject {
                                start_position: (
                                    robot.get_coordinate().get_row(),
                                    robot.get_coordinate().get_col(),
                                ),
                                stop_reason: StopReason::LowEnergy,
                                curves_action: sequence,
                                cost: 0,
                                rock: 0,
                            })
                        }
                        LibError::OutOfBounds
                        | LibError::CannotDestroy
                        | LibError::WrongContentUsed => {
                            return Err(UnFinishedProject {
                                start_position: (
                                    robot.get_coordinate().get_row(),
                                    robot.get_coordinate().get_col(),
                                ),
                                stop_reason: StopReason::MissionImpossible,
                                curves_action: sequence,
                                cost: 0,
                                rock: 0,
                            })
                        }
                        LibError::NotEnoughContentInBackPack => {
                            return Err(UnFinishedProject {
                                start_position: (
                                    robot.get_coordinate().get_row(),
                                    robot.get_coordinate().get_col(),
                                ),
                                stop_reason: StopReason::MissingRocks,
                                curves_action: sequence,
                                cost: 0,
                                rock: 0,
                            })
                        }
                        //checked before
                        // MustDestroyContentFirst => {}
                        // LibError::CannotWalk => {}
                        _ => {
                            println!("UNEXPECTED LAST STOP_TYPE ERROR Err: {:?}", error_type);
                            return Err(UnFinishedProject {
                                start_position: (0, 0),
                                stop_reason: StopReason::MissingRocks,
                                curves_action: vec![],
                                cost: 0,
                                rock: 0,
                            });
                        }
                    },
                }
            }
        }
        Ok(())
    }
    pub fn check_project(project: Project) -> Result<Project, StopReason> {
        unimplemented!();
        let robot_energy = 0;
        let robot_rock = 0;
        if project.cost > robot_energy {
            return Err(StopReason::LowEnergy);
        }
        if project.rock > robot_rock {
            return Err(StopReason::MissingRocks);
        }
        Ok(project)
        // Err(StopReason::PonteSulloStretto)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    //it compiles but not work as i thought lol

    #[test]
    fn it_works() {}
}
