

pub struct Particle2D 
{
pub pos: Vec2,
velocity: Vec2,
acceleration: Vec2,
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}


impl Particle2D {

    pub fn new(pos: Vec2, velocity: Vec2, acceleration: Vec2) -> Particle2D {
        
        Self {pos, velocity, acceleration}

    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }


    pub fn new_newtonian(pos: Vec2) -> Particle2D {

        Self { pos, velocity: Vec2 { x: 0.0, y: 0.0 }, acceleration: Vec2 { x: 0.0, y: 9.81 } }

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

    pub fn collision(&mut self) { // TODO add dampening to simulate energy loss

        self.velocity.x *= -1.0;
        self.velocity.y *= -1.0;

    }

}

