use std::fmt::{Display, Formatter};
// use std::ops::Deref;
use robotics_lib::interface::{destroy, go, put, Tools};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError;
use robotics_lib::utils::LibError::{CannotWalk, NotEnoughEnergy, OutOfBounds};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
use crate::projects::{directioner, Project, Shape, StopReason, UnFinishedProject};
mod projects;

pub struct Asfaltinator {
    project_number: i32,
    unfinished_projects: Vec<UnFinishedProject>
}

impl Tools for Asfaltinator {

}

impl Asfaltinator {
    pub fn new() -> Self{
        Asfaltinator { project_number: 0, unfinished_projects: vec![] }
    }
    //ti dice qunanto vai a sependere (non so se fare passando il robot o passando la cacca addosso), non mi serve a nulla self i guess
    pub fn design_project(&self, posizione: (usize, usize), shape: Shape) ->Result<Project, Project>{
        let mut rock_needs:u32 = 0;
        let mut energy_needs: usize = 0;
        match shape {
            Shape::Square(x) => {

            }
            Shape::Rectangle(x, y) => {}
            Shape::Roundabout(d) => {}
            Shape::Cross(l) => {}
            Shape::LongLong(l) => {}
            Shape::LShape(l, s) => {}
        }
        todo!()
    }
    //need to add the checks in the robot forse accantonerei la cosa
    pub fn check_project(project: Project)->Result<Project, StopReason>{
        let robot_energy = 0;
        let robot_rock = 0;
        if project.cost > robot_energy{ return Err(StopReason::LowEnergy); }
        if project.rock > robot_rock { return Err(StopReason::MissingRocks) }
        Ok(project)
        // Err(StopReason::PonteSulloStretto)
    }

    //This function of the SalviniBot, takes in input a Project, and return a result
    //-Ok if the project was successfully built
    //-Err if the project was stopped, inside the Err there is an UnFinishedProject that contains
    //the values of a Project + the reason why the project didnt successfully end and the position
    //where it stopped
    pub fn asfalting(&self, robot: &mut impl Runnable, world: &mut World, mut project: Project, map: Vec<Vec<Option<Tile>>>) ->Result<(), UnFinishedProject>{
        let number_of_rocks = robot.get_backpack().get_contents();
        let robot_energy = robot.get_energy().get_energy_level();
        let n= 0;
        for direction in project.curves_action.clone() {
            let shift = directioner(&direction);
            let res = match map[((robot.get_coordinate().get_col() as i32)+ shift.0)as usize][((robot.get_coordinate().get_row() as i32)+shift.1) as usize ] {
                None => {panic!()}
                Some(ref X) => { 
                    let put_result = match X.tile_type {

                        TileType::DeepWater | TileType::Lava => {
                            put(robot, world, Content::Rock(0), 3, direction.clone())
                        }
                        TileType::ShallowWater => {
                            put(robot, world, Content::Rock(0), 2, direction.clone())
                        }
                        TileType::Sand | TileType::Grass | TileType::Snow | TileType::Hill => {
                            put(robot, world, Content::Rock(0), 1, direction.clone())
                        }
                        TileType::Street => {
                            Ok(1)
                        }
                        TileType::Mountain => {
                            // destroy(robot, world, direction.clone())?; ???
                            put(robot, world, Content::None, 1, direction.clone())
                        }
                        TileType::Teleport(_) | TileType::Wall => { return Err(UnFinishedProject::new(project, StopReason::PonteSulloStretto, (robot.get_coordinate().get_row(),robot.get_coordinate().get_col())));//i do not
                            //assign any value to put_result dont know if this will lead to a crash of the program
                             }

                    };
                    //need to make a match
                    match put_result {
                        Ok(_) => {}
                        Err(OutOfBounds) | Err(LibError::WrongContentUsed) => {
                            return Err(UnFinishedProject::new(project, StopReason::PonteSulloStretto,
                                                              (robot.get_coordinate().get_row(),robot.get_coordinate().get_col())));
                        }
                        Err(NotEnoughEnergy) => {
                            return Err(UnFinishedProject::new(project, StopReason::LowEnergy,
                                                              (robot.get_coordinate().get_row(), robot.get_coordinate().get_col())));
                        }
                        Err(LibError::NotEnoughContentInBackPack) => {
                            return Err(UnFinishedProject::new(project, StopReason::MissingRocks,
                                                              (robot.get_coordinate().get_row(), robot.get_coordinate().get_col())));
                        }
                        Err(LibError::MustDestroyContentFirst) => {
                            todo!()
                        }
                        Err(X) => {print!("it is missing this unexpected case scenario {:?}", X)}
                    }
                }
            };
            // let put_result = put()
            let go_result = go(robot, world, direction);
            if go_result == Err(CannotWalk) || go_result == Err(OutOfBounds){ return Err(UnFinishedProject::new(project, StopReason::PonteSulloStretto, (robot.get_coordinate().get_row(),robot.get_coordinate().get_col()))); }//ive just built a road so i must be able to walk on it
            if go_result == Err(NotEnoughEnergy) {return Err(UnFinishedProject::new(project, StopReason::LowEnergy, (robot.get_coordinate().get_row(),robot.get_coordinate().get_col()))); }
            project.curves_action.pop();//or use refcell maybe idk need to try it out someday
        };
   

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    //it compiles but not work as i thought lol

    #[test]
    fn it_works() {
        println!("it does not");
        let newshape = Shape::Square(5);
        print!("{:?}",newshape.get_action_curve());

    }
}

