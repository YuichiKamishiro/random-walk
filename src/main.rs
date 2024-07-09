use macroquad::prelude::*;
use ::rand::Rng;
use std::collections::LinkedList;

#[derive(Clone, Copy)]
struct Branch {
    start_x: f32,
    start_y: f32,
    end_x:  f32,
    end_y:  f32,
} 

impl Branch {
    fn new(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Self{
        Branch{start_x, start_y, end_x, end_y}
    }
}

struct Tree {
    branches: LinkedList<Branch>,
    branch_thickness: f32,
    branch_color: Color,
}

impl Tree {
    fn new(branch_thickness: f32, branch_color: Color, start_x: f32, start_y: f32) -> Self {
        let mut first: LinkedList<Branch> = LinkedList::new();
        first.push_back(Branch::new(start_x, start_y, start_x, start_y - 32.));
        Tree{branches: first, branch_thickness, branch_color}
    }
    fn next_step(&mut self) {
        let last_branch= self.branches.back().unwrap().clone();
        
        match ::rand::thread_rng().gen_range(0..4) {
            0 => self.branches.push_back(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x, last_branch.end_y - 32.0)),
            1 => self.branches.push_back(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x, last_branch.end_y + 32.0)),       
            2 => self.branches.push_back(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x - 32.0, last_branch.end_y)),       
            _ => self.branches.push_back(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x + 32.0, last_branch.end_y)),        
        }
    }

    fn draw_all(&self) {
        for x in self.branches.iter() {
            draw_line(x.start_x, x.start_y, x.end_x, x.end_y, self.branch_thickness, self.branch_color);
        }
    }
}

fn input(new_x: &mut f32, new_y: &mut f32, zoom_level: &mut f32) {
    if is_key_down(KeyCode::Left) {
        *new_x = *new_x - 32.;
    }
    if is_key_down(KeyCode::Right) {
        *new_x = *new_x + 32.;
    }
    if is_key_down(KeyCode::Up) {
        *new_y = *new_y + 32.;
    }
    if is_key_down(KeyCode::Down) {
        *new_y = *new_y - 32.;
    }
    if is_key_down(KeyCode::Minus) {
        *zoom_level = *zoom_level + 0.01;
    }

    if is_key_down(KeyCode::Equal) {
        *zoom_level = *zoom_level - 0.01;
    }
} 

#[macroquad::main("Random walk")]
async fn main() {
    let mut trees = vec![Tree::new(3.0, GREEN, 250., 250.)];
    
    let mut zoom_level: f32 = 1.0;
    let mut new_x = 0.;
    let mut new_y = 0.;
    let mut camera = Camera2D::from_display_rect(Rect{x:0.0, y:0.0, w:screen_width() * zoom_level, h:screen_height() * zoom_level});
    loop {
        camera = Camera2D::from_display_rect(Rect{x:new_x, y:new_y, w:screen_width() * zoom_level, h:screen_height() * zoom_level});

        set_camera(&camera);
    
        clear_background(BLACK); 
        println!("FPS:{} Frame Time:{} \t Trees:{}", get_fps(), get_frame_time(), trees.len());

        input(&mut new_x, &mut new_y, &mut zoom_level); 

        for tree in trees.iter_mut() {
            tree.draw_all();

            let start1 = std::time::Instant::now();
            tree.next_step();
        }
        
        set_default_camera();
        
        if is_mouse_button_released(MouseButton::Left) {
            trees.push(Tree::new(3.0, RED, 250., 250.));
        }
        
        next_frame().await
    } 
}
