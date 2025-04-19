
use particles::{Particle2D, vector2d::*};
use macroquad::{prelude::{clear_background, draw_circle, next_frame, RED, WHITE}, time::{draw_fps, get_frame_time}, window::{screen_height, screen_width}};


#[macroquad::main("Window")]
async fn main() {
    
    let width = screen_width();
    let height = screen_height();

    println!("Screen Width: {}\n Screen Height: {}", width, height);


    let mut p1 = Particle2D::new(
        Vec2 {x: (width / 2.0) - 50.0, y: height / 2.0 },
        Vec2 { x: 0.0, y: 0.0},
        Vec2 { x: 0.0, y: 0.0 },
        20.0,
        10.0,

    );

    let mut p2 = Particle2D::new(
        Vec2 {x: (width / 2.0) + 30.0, y: height / 2.0 },
        Vec2 { x: 0.0, y: 0.0},
        Vec2 { x: 0.0, y: 0.0 },
        20.0,
        10.0,

    );

    p1.set_velocity(Vec2 { x: 2.0, y: 3.0 });
    p2.set_velocity(Vec2 { x: -4.0, y: -1.0 });

    

    loop { // simulation loop
        let _delta = get_frame_time(); // FIXME

        p1.move_particle_from_velocity();
        p2.move_particle_from_velocity();
        
        clear_background(RED);
        draw_fps();

        draw_circle(p1.pos.x, p1.pos.y, p1.get_radius(), WHITE);
        draw_circle(p2.pos.x, p2.pos.y, p2.get_radius(), WHITE);


        // draw_text("Hello World!", 50.0, 50.0, 14.0, WHITE);
        
        if p1.pos.x >= width || p1.pos.x <= 1.0  || p1.pos.y >= height || p1.pos.y <= 1.0{
           p1.collision();
        }
        if p2.pos.x >= width || p2.pos.x <= 1.0 || p2.pos.y >= height || p2.pos.y <= 1.0{
            p2.collision();
         }

        // Inefficient test

        if p1.is_collided_other(&mut p2) {
            p1.handle_collision_from_other_particle(&mut p2);
        }


        // println!("Logging x: {}\nLogging y: {}", p1.pos.x, p1.pos.y);

        next_frame().await
    }
}
