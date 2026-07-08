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
    let last_point : Point ;
    let mut enterPresed=false;
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
        if !enterPresed{
        for i in 0..points.len().saturating_sub(1) {
            draw_line(points[i].x, points[i].y,points[i + 1].x,points[i + 1].y,2.0,GRAY);
        }
        }
       
        if is_key_pressed(KeyCode::Enter) ||enterPresed{
            enterPresed=true;
            
            chaikinalgo(&points);

            
            println!("Enter pressed!");
        }

       next_frame().await;
    }

}
fn chaikinalgo(tab: &Vec<Point>) {
    let mut res = tab.clone();

    for _ in 0..7 {
        let mut arr = Vec::new();

      arr.push(res[0].clone());
      
      for i in 0..res.len() - 1 {
          let p0 = &res[i];
          let p1 = &res[i + 1];
      
          let q = Point {
              x: 0.75 * p0.x + 0.25 * p1.x,
              y: 0.75 * p0.y + 0.25 * p1.y,
          };
      
          let r = Point {
              x: 0.25 * p0.x + 0.75 * p1.x,
              y: 0.25 * p0.y + 0.75 * p1.y,
          };
      
          arr.push(q);
          arr.push(r);
      }
      
      arr.push(res[res.len() - 1].clone());
      
      res = arr;

        clear_background(WHITE);
          
        for point in tab {
            draw_circle(point.x, point.y, 3.0, SKYBLUE);
        }

           for i in 0..res.len().saturating_sub(1) {
                 draw_line(
                  res[i].x,
                  res[i].y,
                  res[i + 1].x,
                  res[i + 1].y,
                  2.0,
                  GRAY,
                  );
              }
    }

 
}