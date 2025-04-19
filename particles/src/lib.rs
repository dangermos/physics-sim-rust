pub mod vector2d;

use crate::vector2d::*;
use core::f32;
use std::f32::consts::PI;


use macroquad::math::bool;

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


    pub fn get_radius(&self) -> f32{
        self.radius
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
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

    }

    pub fn collision(&mut self, other: &mut Particle2D) { // TODO add dampening to simulate energy loss

        todo!()
    }

}

