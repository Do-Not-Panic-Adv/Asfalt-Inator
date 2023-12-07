use std::ops::Deref;
// use robotics_lib::interface::Tools;
use crate::projects::{Project, Shape, StopReason};
mod projects;

pub struct SalviniBot{
}

impl SalviniBot{
    pub fn new() -> Self{
        SalviniBot{ }
    }
    //ti dice qunanto vai a sependere (non so se fare passando il robot o passando la cacca addosso), non mi serve a nulla self i guess
    pub fn design_project(&self, posizione: (usize, usize), shape: Shape, map_size: usize) ->Result<Project, Project>{
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
    //need to add the checks in the robot
    pub fn check_project(project: Project)->Result<Project, StopReason>{
        let robot_energy = 0;
        let robot_rock = 0;
        if project.cost > robot_energy{ return Err(StopReason::LowEnergy); }
        if project.rock > robot_rock { return Err(StopReason::MissingRocks) }
        Ok(project)
        // Err(StopReason::PonteSulloStretto)
    }
    pub fn road_building(){
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    //it compiles but not work as i thought lol

    #[test]
    fn it_works() {
        println!("it does not");
        let newshape = Shape::Square(6);
        print!("{:?}",newshape.get_action_curve());

    }
}

