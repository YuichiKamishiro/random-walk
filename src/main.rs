use macroquad::prelude::*;
use miniquad::window::set_window_size;
use ::rand::Rng;

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
    branches: Vec<Branch>,
    branch_thickness: f32,
    branch_color: Color,
}

impl Tree {
    fn new(branch_thickness: f32, branch_color: Color) -> Self {
        Tree{branches: vec![Branch::new(240., 240., 240., 256.)], branch_thickness, branch_color}
    }
    fn next_step(&mut self) {
        let cloned_branches = self.branches.clone();
        let last_branch = cloned_branches.last().unwrap(); 
        
        match ::rand::thread_rng().gen_range(0..4) {
            0 => self.branches.push(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x, last_branch.end_y - 32.0)),
            1 => self.branches.push(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x, last_branch.end_y + 32.0)),       
            2 => self.branches.push(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x - 32.0, last_branch.end_y)),       
            _ => self.branches.push(Branch::new(last_branch.end_x, last_branch.end_y, last_branch.end_x + 32.0, last_branch.end_y)),        
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
        *zoom_level = *zoom_level - 0.5 * get_frame_time();
    }

    if is_key_down(KeyCode::Equal) {
        *zoom_level = *zoom_level + 0.5 * get_frame_time();
    }
} 

#[macroquad::main("Dammnit")]
async fn main() {
    let mut tree = Tree::new(3.0, GREEN);
    
    let mut zoom_level: f32 = 1.0;
    let mut new_x = 0.;
    let mut new_y = 0.;
    let mut camera = Camera2D::from_display_rect(Rect{x:0.0, y:0.0, w:screen_width() * zoom_level, h:screen_height() * zoom_level});
    
    loop {
        camera = Camera2D::from_display_rect(Rect{x:new_x, y:new_y, w:screen_width() * zoom_level, h:screen_height() * zoom_level});

        set_camera(&camera);
        
        clear_background(BLACK); 
        println!("FPS:{} Frame Time:{}", get_fps(), get_frame_time());

        input(&mut new_x, &mut new_y, &mut zoom_level); 
        tree.draw_all();
        tree.next_step();

        next_frame().await
    } 
}
