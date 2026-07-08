use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin".to_string(),
        window_width: 600,
        window_height: 400,
        ..Default::default()
    }
}

#[derive(Debug,Clone)]
struct Point {
    x: f32,
    y: f32
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points = Vec::new();
    let mut copy_points = Vec::new();

    let mut enterPresed=false;
    let mut iteration = 0;
    let mut last_update = 0.0;

    loop {
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) &&!enterPresed {
            let (x, y) = mouse_position();
            points.push(Point{x,y});
            println!{"{:?}",points};
        }
        
        if is_key_pressed(KeyCode::Escape) {
            enterPresed=false;
            points = Vec::new();
            iteration = 0;
            last_update = 0.0;

        }
        
        if is_key_pressed(KeyCode::Backspace) {
            points.clear();
            println!{"{:?}",points};
        }
       
         for point in &points {
            draw_circle(point.x, point.y, 3.0, SKYBLUE);
        }
        if !enterPresed{
        for i in 0..points.len().saturating_sub(1) {
            draw_line(points[i].x, points[i].y,points[i + 1].x,points[i + 1].y,2.0,GRAY);
        }
        }
       
        if is_key_pressed(KeyCode::Enter) && !enterPresed &&points.len()>0{
         copy_points=points.clone();
          enterPresed = true;
          iteration = 0;
       }
       if enterPresed && iteration <= 7{

         if iteration <= 7&& get_time() - last_update > 0.7{
              copy_points = chaikin_step(&copy_points);
             iteration += 1;
            last_update = get_time();
         }
           if iteration==7{
             copy_points=points.clone();
             iteration=0;
            }
     
         for i in 0..copy_points.len().saturating_sub(1) {
            
             draw_line(
                 copy_points[i].x,
                 copy_points[i].y,
                 copy_points[i + 1].x,
                 copy_points[i + 1].y,
                 2.0,
                 GRAY,
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