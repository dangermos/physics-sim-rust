pub mod vector2d;

use crate::vector2d::*;
use core::f32;
use std::{f32::consts::PI, fmt::Display};
use rand::{prelude::*, random};


/// An object that simulates a circular particle in space under the laws of kinematics and momentum
///
///
///
pub struct Particle2D

{
    // Movement
    pub pos: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    angle_of_movement: f32, // Degrees


    // Properties
    radius: f32,
    collided: bool,
    mass: f32,

    // Computed Properties
    area: f32,
    momentum: Vec2,


}

/// Immovable wall 
pub struct Wall2D {
    pub pos: Vec2,
    length: f32,
    width: f32,

}




// TODO
// Add delta time as a calculation for the kinematics equations


impl Particle2D {

    pub fn new(pos: Vec2, velocity: Vec2, acceleration: Vec2, radius: f32, mass: f32) -> Particle2D {

        
        Self {
            pos,
            velocity,
            acceleration,
            radius,
            mass,
            angle_of_movement: 0.0,
            collided: false,
            area: radius.powf(2.0) * PI,
            momentum: velocity * mass,
        }

    }
    
    pub fn new_newtonian(pos: Vec2, radius: f32, mass: f32) -> Particle2D {

        let velocity = Vec2 { x: 0.0, y: 0.0 };

        Self { 
            pos,
            velocity,
            acceleration: Vec2 { x: 0.0, y: 9.81 },
            mass,
            radius,
            angle_of_movement: 270.0,
            collided: false,
            area: radius.powf(2.0) * PI,
            momentum: velocity * mass,
        }

    }

    pub fn new_random(screen_size: (f32, f32)) -> Particle2D {
        let x: f32 = rand::random_range(0.0 ..= screen_size.0);
        let y: f32 = rand::random_range(0.0 ..= screen_size.1);

        

    }

    pub fn get_radius(&self) -> f32{
        self.radius
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn rev_velocity(&mut self) {
        self.velocity = self.velocity * -1.0;
    }

    pub fn is_collided_other(&mut self, other: &mut Particle2D) -> bool {

        let dist_x: f32 = self.pos.x - other.pos.x;
        let dist_y : f32 = self.pos.y - other.pos.y;
        let distance = ( dist_x.powi(2) + dist_y.powi(2) ).sqrt();
        let radii_sum = self.radius + other.radius;

        distance < radii_sum
    }

    pub fn handle_collision_from_other_particle(&mut self, other: &mut Particle2D) {

        // if not within some range, just pass the function
        // as this can be called every frame
        let range: f32 = 1.0; // TODO

        self.collided = true;
        other.collided = true;
        self.collision(other);

    }


    pub fn move_particle(&mut self, dx: f32, dy: f32) -> () {
        self.pos.x += dx;
        self.pos.y += dy;
    }

    pub fn move_particle_from_velocity(&mut self) {
        let new_velocity_x = (self.velocity.x) + self.acceleration.x / 2.0;
        let new_velocity_y = (self.velocity.y) + self.acceleration.y / 2.0;

        self.pos.x += new_velocity_x;
        self.pos.y += new_velocity_y;

        self.velocity = Vec2 {x: new_velocity_x, y: new_velocity_y}

    }

    fn collision(&mut self, other: &mut Particle2D) { 

        // m1 * v1 = -m2 * v2
        // v1f = (-m2 * v2) / m1
        // v2f = (m1 * v1) / -m2
        println!("Prev V1: {}", &self.velocity);
        println!("Prev V2: {}", &self.velocity);


        let new_v1 = other.momentum / self.mass;
        let new_v2 = self.momentum / other.mass;

        println!("New V1: {}", &new_v1);
        println!("New V2: {}", &new_v2);


        self.set_velocity(new_v1);
        other.set_velocity(new_v2);

        self.collided = false;
        other.collided = false;


    }

}

impl Display for Particle2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos: {}
        Velocity: {}\n
        Acceleration: {}\n
        Angle: {}\n
        Radius: {}\n
        Colliding: {}\n
        Mass: {}\n
        Area: {}\n
        Momentum: {}\n", &self.pos, &self.velocity, &self.acceleration, 
        &self.angle_of_movement, &self.radius, &self.collided, &self.mass,
        &self.area, &self.momentum)
    }
}
#[cfg(test)]
mod tests {
    use crate::{vector2d::Vec2, Particle2D};

    

#[test]

fn particle_test() {
    let mut p1 = Particle2D::new
    ( Vec2 {x:0.0, y:0.0},
        Vec2 {x:0.0, y:0.0}, 
        Vec2 { x: 0.0, y: 0.0 },
        1.0, 
        1.0);

    assert!(p1.velocity == Vec2 {x: 0.0, y: 0.0});

    p1.set_velocity(Vec2 { x: 10.0, y: 10.0 });

    assert!(p1.velocity == Vec2 { x: 10.0, y: 10.0 });

}


}

