use ::rand::Rng;
use macroquad::prelude::*;
use std::collections::LinkedList;

#[derive(Clone, Copy)]
struct Branch {
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
}

impl Branch {
    fn new(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Self {
        Branch {
            start_x,
            start_y,
            end_x,
            end_y,
        }
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
        Tree {
            branches: first,
            branch_thickness,
            branch_color,
        }
    }
    fn next_step(&mut self) {
        if let Some(last_branch) = self.branches.back().clone() {
            match ::rand::thread_rng().gen_range(0..4) {
                0 => self.branches.push_back(Branch::new(
                    last_branch.end_x,
                    last_branch.end_y,
                    last_branch.end_x,
                    last_branch.end_y - 32.0,
                )),
                1 => self.branches.push_back(Branch::new(
                    last_branch.end_x,
                    last_branch.end_y,
                    last_branch.end_x,
                    last_branch.end_y + 32.0,
                )),
                2 => self.branches.push_back(Branch::new(
                    last_branch.end_x,
                    last_branch.end_y,
                    last_branch.end_x - 32.0,
                    last_branch.end_y,
                )),
                _ => self.branches.push_back(Branch::new(
                    last_branch.end_x,
                    last_branch.end_y,
                    last_branch.end_x + 32.0,
                    last_branch.end_y,
                )),
            }
        }
    }

    fn draw_all(&self, camera_pos_x: f32, camera_pos_y: f32, zoom_factor: f32) {
        for x in self.branches.iter() {
            draw_line(
                (x.start_x + camera_pos_x) * zoom_factor,
                (x.start_y + camera_pos_y) * zoom_factor,
                (x.end_x + camera_pos_x) * zoom_factor,
                (x.end_y + camera_pos_y) * zoom_factor,
                self.branch_thickness * zoom_factor,
                self.branch_color,
            );
        }
    }
}

#[macroquad::main("Random walk")]
async fn main() {
    let mut trees = vec![Tree::new(3.0, GREEN, 250., 250.)];
    let mut zoom_factor: f32 = 1.0;
    let mut camera_pos_x = 0.;
    let mut camera_pos_y = 0.;

    let mut current_color = RED;
    loop {
        clear_background(BLACK);

        for tree in trees.iter_mut() {
            tree.draw_all(camera_pos_x, camera_pos_y, zoom_factor);
            tree.next_step();
        }

        if is_key_down(KeyCode::Up) {
            camera_pos_y = camera_pos_y + 15.;
        } else if is_key_down(KeyCode::Down) {
            camera_pos_y = camera_pos_y - 15.;
        } else if is_key_down(KeyCode::Left) {
            camera_pos_x = camera_pos_x + 15.;
        } else if is_key_down(KeyCode::Right) {
            camera_pos_x = camera_pos_x - 15.;
        } else if is_key_down(KeyCode::Equal) {
            zoom_factor = zoom_factor - 0.01;
        } else if is_key_down(KeyCode::Minus) {
            zoom_factor = zoom_factor + 0.01;
        }

        if is_key_down(KeyCode::Key1) {
            current_color = RED;
        } else if is_key_down(KeyCode::Key2) {
            current_color = BLUE;
        } else if is_key_down(KeyCode::Key3) {
            current_color = GREEN;
        } else if is_key_down(KeyCode::Key4) {
            current_color = WHITE;
        }

        draw_text(
            "Colors, 1 - red, 2 - blue, 3 - green, 4 - white",
            200.,
            400.,
            20.,
            WHITE,
        );

        if is_mouse_button_released(MouseButton::Left) {
            let x = (mouse_position().0 / zoom_factor) - camera_pos_x;
            let y = (mouse_position().1 / zoom_factor) - camera_pos_y;
            trees.push(Tree::new(3.0, current_color, x, y));
        }

        next_frame().await;
    }
}
