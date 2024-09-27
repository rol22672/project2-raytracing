
use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use ggez::graphics::{self, Color, Mesh}; // Import `Mesh` and `Color`
use nalgebra as na; // Import nalgebra


pub struct Player {
    pub position: na::Vector3<f32>, // The player’s position in 3D space
    velocity: na::Vector3<f32>, // Velocity for smooth movement
    speed: f32,                 // Movement speed
    pub direction: f32,             // Direction angle in radians
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: na::Vector3::new(5.0, 5.0, 1.0), // Start in the center of the grid
            velocity: na::Vector3::new(0.0, 0.0, 0.0),
            speed: 0.1,
            direction: 0.0, // Start facing along the positive x-axis
        }
    }

    pub fn update(&mut self) {
        // Update player position based on velocity
        self.position += self.velocity;
        
        // Dampen velocity (simulate friction)
        self.velocity *= 0.9;
    }

    pub fn handle_input(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::W => {
                self.velocity.x += self.direction.cos() * self.speed;
                self.velocity.y += self.direction.sin() * self.speed;
            }
            KeyCode::S => {
                self.velocity.x -= self.direction.cos() * self.speed;
                self.velocity.y -= self.direction.sin() * self.speed;
            }
            KeyCode::A => {
                self.direction -= 0.1; // Turn left
            }
            KeyCode::D => {
                self.direction += 0.1; // Turn right
            }
            _ => {}
        }
    }
}
