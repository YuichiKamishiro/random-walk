use macroquad::prelude::*;
use miniquad::window::set_window_size;
use ::rand::Rng;

#[derive(Clone, Copy)]
struct Branch {
    x: f32,
    y: f32,
    x1: f32,
    y1: f32,
} 

impl Branch {
    fn new(x: f32, y: f32, x1: f32, y1: f32) -> Self{
        Branch{x, y, x1, y1}
    }
}

struct Tree {
    branches: Vec<Branch>,
    branch_thickness: f32,
}

impl Tree {
    fn new() -> Self {
        Tree{branches: vec![Branch::new(240., 240., 240., 256.)], branch_thickness: 3.0}
    }
    fn next_step(&mut self) {
        let choice = ::rand::thread_rng().gen_range(0..4);
        
        let mut branches = self.branches.clone();
        let last_pos = branches.last().clone().unwrap();

        if choice == 0 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1, last_pos.y1 - 32.0));       
        }
        if choice == 1 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1, last_pos.y1 + 32.0));       
        }
        if choice == 2 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1 - 32.0, last_pos.y1));       
        }
        if choice == 3 {
            self.branches.push(Branch::new(last_pos.x1, last_pos.y1, last_pos.x1 + 32.0, last_pos.y1));       
        }
    }

    fn draw_all(&self) {
        for x in self.branches.iter() {
            draw_line(x.x, x.y, x.x1, x.y1, self.branch_thickness, WHITE);
        }
    }

}

fn window_conf() -> Conf {
    Conf {
        window_title: "FPS Unlocked Example".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true, // unlock FPS limit
        ..Default::default()
    }
}

#[macroquad::main("Dammnit")]
async fn main() {
    let mut tree = Tree::new();
    set_window_size(1200, 700);
    let mut zoom_level: f32 = 1.0;
    let mut new_x = 0.;
    let mut new_y = 0.;
    let mut camera = Camera2D::from_display_rect(Rect{x:0.0, y:0.0, w:screen_width() * zoom_level, h:screen_height() * zoom_level});
    loop {
        camera = Camera2D::from_display_rect(Rect{x:new_x, y:new_y, w:screen_width() * zoom_level, h:screen_height() * zoom_level});

        set_camera(&camera);
        
        clear_background(BLACK); 
        println!("FPS:{}", get_fps());
        if is_key_down(KeyCode::Left) {
            new_x = new_x - 32.;
        }
        if is_key_down(KeyCode::Right) {
            new_x = new_x + 32.;
        }
        if is_key_down(KeyCode::Up) {
            new_y = new_y + 32.;
        }
        if is_key_down(KeyCode::Down) {
            new_y = new_y - 32.;
        }

        if is_key_down(KeyCode::Minus) {
            zoom_level = zoom_level - 0.01;
        }
        
        if is_key_down(KeyCode::Equal) {
            zoom_level = zoom_level + 0.01;
        }

        tree.draw_all();
        tree.next_step();

        next_frame().await
    } 
}
