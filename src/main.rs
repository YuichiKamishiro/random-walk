use macroquad::prelude::*;
use ::rand::Rng;
use std::thread;

#[derive(Clone, Copy)]
struct Branch {
    x: f32,
    y: f32,
    x1: f32,
    y1: f32,
} 

impl Branch {
    fn new(x: f32, y: f32, x1: f32, y1: f32) -> Self{
        Branch{x: x, y: y, x1: x1, y1: y1}
    }
}

struct Tree {
    branches: Vec<Branch>
}

impl Tree {
    fn new() -> Self {
        Tree{branches: vec![Branch::new(240., 240., 240., 256.)]}
    }
    fn next_step(&mut self) {
        let choice = ::rand::thread_rng().gen_range(0..4);
        
        let mut branches = self.branches.clone();
        let last_pos = branches.last().clone().unwrap();

        if choice == 0 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1, last_pos.y1 - 16.0));       
        }
        if choice == 1 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1, last_pos.y1 + 16.0));       
        }
        if choice == 2 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1 - 16.0, last_pos.y1));       
        }
        if choice == 3 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1 + 16.0, last_pos.y1));       
        }
        thread::sleep(std::time::Duration::from_millis(5));
    }

    fn draw_all(&self) {
        for x in self.branches.iter() {
            draw_line(x.x, x.y, x.x1, x.y1, 1.0, WHITE);
        }
    }
}

#[macroquad::main("Dammnit")]
async fn main() {
    let mut tree = Tree::new();
    loop {  
        
        if is_key_released(KeyCode::Space) {
        }
        tree.draw_all();
        tree.next_step();

        next_frame().await
    } 
}
