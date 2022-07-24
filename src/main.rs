mod math;
use std::f32::consts::PI;

use math::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
fn graph_fn(x:f32) -> f32{ // Converts X coordinate to Y coordinate
    tan(x)
}
fn graph_para(t:f32) -> (f32, f32){ //Converts Time to X and Y coordinates
    let x = t.tan();
    let y = t.cos();
    (x, y)
}
fn graph_polar(th:f32) -> f32{ //Converts Theta to Radius
    th.sin()
}
fn graph_seq(hist:Vec<f32>) -> f32{ //Converts History to Y coordinate
    hist[0]+1.
}
const GRAPH_MODE:&str = "para"; //Graph mode
const GRAPH_WIDTH:u32 = 800; //Graph width
const GRAPH_HEIGHT:u32 = 600; //Graph height
const GRAPH_CENTER_Y:u32 = GRAPH_HEIGHT/2; //Graph center Y coordinate
const GRAPH_CENTER_X:u32 = GRAPH_WIDTH/2; //Graph center X coordinate

const GRAPH_START_FN:f32 = -10.;
const GRAPH_END_FN:f32 = 10.;
const GRAPH_STEP_FN:f32 = 0.1;

const GRAPH_START_PARA:f32 = 0.;
const GRAPH_END_PARA:f32 = 2.*PI;
const GRAPH_STEP_PARA:f32 = 0.1;

const GRAPH_START_POLAR:f32 = 0.;
const GRAPH_END_POLAR:f32 = 2.*PI;
const GRAPH_STEP_POLAR:f32 = 0.1;

const GRAPH_START_SEQ:f32 = 0.;
const GRAPH_LEN_SEQ:u32 = 100; //Plot 32 iterations of the graph

const IPS:u64 = 1000/30; //Iterations per second
const SCALE:f32 = 10.; //Scale of the graph
fn main(){
    //Initialize an sdl2 window and canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Graph", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut evp = sdl_context.event_pump().unwrap();
    
    let fun = match GRAPH_MODE {
        "fn" | "function" | "functional" | "x" => "fn",
        "para" | "parametric" | "t" | "xy"=> "para",
        "polar" | "polarcoord" | "r" | "radius" | "th" | "theta" => "polar",
        "seq" | "sequential" | "hist" | "history" => "seq",
        _ => {
            println!("Invalid graph mode, defaulting to fn");
            "fn"
        }
    };

    let start = match fun {
        "fn" => GRAPH_START_FN,
        "para" => GRAPH_START_PARA,
        "polar" => GRAPH_START_POLAR,
        "seq" => GRAPH_START_SEQ,
        _ => {
            println!("Invalid graph mode, defaulting to fn");
            GRAPH_START_FN
        }
    };
    let end = match fun {
        "fn" => GRAPH_END_FN,
        "para" => GRAPH_END_PARA,
        "polar" => GRAPH_END_POLAR,
        "seq" => GRAPH_START_SEQ + GRAPH_LEN_SEQ as f32,
        _ => {
            println!("Invalid graph mode, defaulting to fn");
            GRAPH_END_FN
        }
    };
    let step = match fun {
        "fn" => GRAPH_STEP_FN,
        "para" => GRAPH_STEP_PARA,
        "polar" => GRAPH_STEP_POLAR,
        "seq" => 1.,
        _ => {
            println!("Invalid graph mode, defaulting to fn");
            GRAPH_STEP_FN
        }
    };
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut last_point = None;

    if fun != "seq"{
        let mut n = start;
        while n < end{
            let x;
            let y;
            if fun == "fn"{
                x = n;
                y = graph_fn(n);
            }else if fun == "para"{
                (x, y) = graph_para(n);
            }else if fun == "polar"{
                let r = graph_polar(n);
                x = r*n.cos();
                y = r*n.sin();
            }else{
                println!("Invalid graph mode, defaulting to fn");
                x = n;
                y = graph_fn(n);
            }
            let x_coord = x*SCALE;
            let y_coord = y*SCALE;
            let x_coord_center = GRAPH_CENTER_X as i32 + x_coord as i32;
            let y_coord_center = GRAPH_CENTER_Y as i32 - y_coord as i32; //+Y = up
            let pt = Point::new(x_coord_center, y_coord_center);
            canvas.draw_point(pt).unwrap();
            if last_point != None{
                canvas.draw_line(last_point.unwrap(), pt).unwrap();
            }
            last_point = Some(pt);
            n += step;
            canvas.present();
            //Quit if the user presses escape
            for event in evp.poll_iter(){
                match event{
                    Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
                        return;
                    }
                    _ => {}
                }
            }
            //Sleep to match the desired IPS
            std::thread::sleep(std::time::Duration::from_millis(IPS));
        }
    }

    'display: loop {
        for event in evp.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'display
                },
                _ => {}
            }
        }
    }
}