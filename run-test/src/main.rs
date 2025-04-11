
use particles::*;
use macroquad::{prelude::{clear_background, draw_circle, next_frame, BLACK, RED, WHITE}, window::{screen_height, screen_width}};

#[macroquad::main("Window")]
async fn main() {
    
    let width = screen_width();
    let height = screen_height();

    println!("Screen Width: {}\n Screen Height: {}", width, height);


    let mut p1 = Particle2D::new(
        Vec2 {x: width / 2.0, y: height / 2.0 },
        Vec2 { x: 0.0, y: 0.0},
        Vec2 { x: 0.0, y: 0.0 }

    );

    p1.set_velocity(Vec2 { x: 1.0, y: 0.0 });


    

    loop { // simulation loop
        p1.move_particle_from_velocity();
        p1.set_velocity(v);

        
        clear_background(RED);

        draw_circle(p1.pos.x, p1.pos.y, 20.0, WHITE);

        // draw_text("Hello World!", 50.0, 50.0, 14.0, WHITE);
        
        if p1.pos.x >= width || p1.pos.x <= 1.0 {
            p1.collision();
        }


        println!("Logging x: {}\nLogging y: {}", p1.pos.x, p1.pos.y);

        next_frame().await
    }
}
