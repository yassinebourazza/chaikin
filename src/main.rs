use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin".to_string(),
        window_width: 600,
        window_height: 400,
        ..Default::default()
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points = Vec::new();
    let last_point : Point ;
    loop {
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            points.push(Point{x,y});
            println!{"{:?}",points};
        }
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        if is_key_pressed(KeyCode::Backspace) {
            points.clear();
            println!{"{:?}",points};
        }
        
        for point in &points {
            draw_circle(point.x, point.y, 3.0, SKYBLUE);
        }

        for i in 0..points.len().saturating_sub(1) {
            draw_line(points[i].x, points[i].y,points[i + 1].x,points[i + 1].y,2.0,GRAY);
        }

       next_frame().await;
    }
}