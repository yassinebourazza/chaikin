use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin".to_string(),
        window_width: 600,
        window_height: 400,
        ..Default::default()
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points = Vec::new();
    let mut copy_points = Vec::new();
    let mut enter_pressed = false;
    let mut iteration = 0;
    let mut last_update = 0.0;
    let mut err_msg = false;

    loop {
        clear_background(DARKGRAY);
        draw_text(
            "[   ] : Exit | [         ] : Clear | [     ] : Apply",
            10.0,
            15.0,
            16.0,
            LIGHTGRAY
        );
        draw_text(
            "[ESC]          [BACKSPACE]           [ENTER]        ",
            10.0,
            15.0,
            16.0,
            SKYBLUE
        );

        if is_mouse_button_pressed(MouseButton::Left) && !enter_pressed {
            err_msg = false;
            let (x, y) = mouse_position();
            points.push(Point { x, y });
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Backspace) {
            enter_pressed = false;
            points.clear();
            copy_points.clear();
            last_update = 0.0;
            iteration = 0;
        }

        for point in &points {
            draw_circle_lines(point.x, point.y, 2.0, 1.0, SKYBLUE);
        }
        if !enter_pressed {
            for i in 0..points.len().saturating_sub(1) {
                draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, 2.0, GRAY);
            }
        }

        if is_key_pressed(KeyCode::Enter) && !enter_pressed && points.len() > 0 {
            if points.len() == 1 {
                err_msg = true;
            } else {
                copy_points = points.clone();
                enter_pressed = true;
                iteration = 0;
            }
        }

        if err_msg {
            draw_text("Two points are required", 10.0, 30.0, 16.0, RED);
        }

        if enter_pressed && iteration <= 7 {
            if iteration <= 7 && get_time() - last_update > 0.7 {
                copy_points = chaikin_step(&copy_points);
                iteration += 1;
                last_update = get_time();
            }

            draw_text(format!("Step: {}", iteration ), 10.0, 50.0, 20.0, WHITE);

            if iteration > 7 {
                copy_points = points.clone();
                iteration = 0;
            }

            for i in 0..copy_points.len().saturating_sub(1) {
                draw_line(
                    copy_points[i].x,
                    copy_points[i].y,
                    copy_points[i + 1].x,
                    copy_points[i + 1].y,
                    2.0,
                    GRAY
                );
            }
        }

        next_frame().await;
    }
}

fn chaikin_step(tab: &Vec<Point>) -> Vec<Point> {
    let mut arr = Vec::new();

    arr.push(tab[0].clone());

    for i in 0..tab.len() - 1 {
        let p0 = &tab[i];
        let p1 = &tab[i + 1];

        arr.push(Point {
            x: 0.75 * p0.x + 0.25 * p1.x,
            y: 0.75 * p0.y + 0.25 * p1.y,
        });

        arr.push(Point {
            x: 0.25 * p0.x + 0.75 * p1.x,
            y: 0.25 * p0.y + 0.75 * p1.y,
        });
    }

    arr.push(tab[tab.len() - 1].clone());

    arr
}
