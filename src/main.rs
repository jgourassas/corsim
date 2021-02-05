//First install https://github.com/burtonageo/cargo-bundle then run cargo bundle --release [target]
//For your OS run bundle --release ( cargo bundle)
// for Debian find executable in corsim/target/debug/bundle/deb/corsim_1.0.0_amd64/data/usr/bin/corsim
// If cairo.h not found-> change pangocairo.h to include your cairo.h

#![allow(non_snake_case)]

use fltk::dialog::message;
use fltk::*;
use fltk::{button::*, frame::*, window::*};


use std::cell::RefCell;
use std::rc::Rc;
use std::{error::Error, fmt, panic, thread, time};

extern crate csv;


mod actions;

use actions::onclick::show_segment_name;

mod controls;
use controls::slider::make_slider;

mod graphics;

use graphics::draw::{
    draw_scene,
    //draw_segment_92,
    //draw_as_polyline_segment,
    setup_gl,
};

use crate::graphics::data_gets::get_optimal_views;

const MARGIN_TOP: i32 = 50;
const MARGIN_BOTTOM: i32 = 100;
const MARGIN_RIGHT: i32 = 220;
const MARGIN_LEFT: i32 = 10;
//const SIZE_UNIT: f32 = 2.5;

const FLTK_WINDOW_WIDTH: i32 = 1650 - MARGIN_LEFT - MARGIN_RIGHT;
const FLTK_WINDOW_HEIGHT: i32 = 1200 - MARGIN_TOP - MARGIN_BOTTOM;


const GL_WINDOW_WIDTH: i32 = FLTK_WINDOW_WIDTH - 250;
const GL_WINDOW_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - 100;

const FRAME_INFO_X: i32 = GL_WINDOW_WIDTH + 10;
const FRAME_INFO_Y: i32 = MARGIN_TOP;

const FRAME_INFO_WIDTH: i32 = FLTK_WINDOW_WIDTH - (GL_WINDOW_WIDTH + 12);
const FRAME_INFO_HEIGHT: i32 = GL_WINDOW_HEIGHT - 370;

const WIDGET_RAO_LAO_X: i32 =  FLTK_WINDOW_WIDTH - 80;
const WIDGET_RAO_LAO_Y: i32  = FLTK_WINDOW_HEIGHT - 350;
const WIDGET_RAO_LAO_BOUNDS: [i32;2] = [-90, 90];
const WIDGET_RAO_LAO_TITLE: &str =  "⇑Lao/Rao⇩";

const WIDGET_CR_CA_X: i32 =  FLTK_WINDOW_WIDTH - 200;
const WIDGET_CR_CA_Y: i32 = FLTK_WINDOW_HEIGHT - 350;
const WIDGET_CR_CA_BOUNDS: [i32;2] = [90, -90];
const WIDGET_CR_CA_TITLE: &str =  "↑Cra/Cau↓";


#[derive(Debug, Clone, Copy)]
pub enum Message {
    Raolao,
    Crca,
    AnteriorPosterior,
    RcaOnly,
    LeftOnly,
}

pub enum EventMessage {
    PushEvent,
}

pub fn main() {
    //get_data_92();
    // get_branch_data_92();
    //get_nomeclature_data();
    panic::set_hook(Box::new(|panic_info| {
        message(200, 200, &panic_info.to_string());
    }));

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut fltk_wind = Window::new(
        100,
        100,
        FLTK_WINDOW_WIDTH,
        FLTK_WINDOW_HEIGHT,
        "CORONARY Views. By John Gkourasas (old J. Gourassas)",
    );



    let mut  widget_rao_lao = make_slider(
        WIDGET_RAO_LAO_X,
        WIDGET_RAO_LAO_Y,
        30,
         240,
         WIDGET_RAO_LAO_TITLE,
         WIDGET_RAO_LAO_BOUNDS,
    );
    let widget_rao_lao_c = widget_rao_lao.clone();


    /*
    Philips XPER FD10C R7.0.4
    The projection angles for the Poly Diagnost C-arm in the head position (orientated parallel
    to the table) are: Rotation is 120 degrees LAO to 120 degrees RAO. Angulation is 45 degrees
    cranial to 45 degrees caudal.
    -------------
    FROM DICOM
    -------------
    Z Axis is from legs to head
    Y is perpendicular to the angio table Positive direction is from table to floor\
    X is from Right patient arm to his left -x -> +x
    -------------------------------------
    The Positioner Primary Angle is defined in the transaxial plane at the isocenter with zero degrees
    in the direction perpendicular to the patient's chest and + 90 degrees at the patient left hand side (LAO) a
    nd -90 at the patient right hand side (RAO). The valid range of Primary Positioner Angle is -180 to +180 degrees.
    The Positioner Secondary Angle is defined in the Sagittal Plane at the isocenter with zero degrees
    in the direction perpendicular  to the patient's chest. +90 degrees corresponds to the cranial direction.
    The Secondary Positioner Angle range is -90 to + 90 degrees.
    */

  
 
   

    let mut frame_rao_lao = Frame::new(
        FLTK_WINDOW_WIDTH - 60,
        FLTK_WINDOW_HEIGHT - 250,
        70,
        40,
        "",
    );
    frame_rao_lao.set_color(Color::from_rgb(39, 45, 206)); //blue
    frame_rao_lao.set_label_size(22);


    let mut widget_cr_ca = make_slider(
        WIDGET_CR_CA_X,
        WIDGET_CR_CA_Y,
        30,
        240,
        WIDGET_CR_CA_TITLE,
        WIDGET_CR_CA_BOUNDS,
    );
    
    let widget_cr_ca_c = widget_cr_ca.clone();


    let mut frame_cr_ca = Frame::new(
        FLTK_WINDOW_WIDTH - 255,
        FLTK_WINDOW_HEIGHT - 250,
        70,
        40,
        "",
    );

    frame_cr_ca.set_color(Color::from_rgb(39, 206, 201)); //green
    frame_cr_ca.set_label_size(22);

    let mut but_quit = Button::new(
        FLTK_WINDOW_WIDTH - 100,
        FLTK_WINDOW_HEIGHT - 400,
        70,
        40,
        "Quit➤",
    );
    but_quit.set_color(Color::from_rgb(251, 180, 174)); //red
    but_quit.set_label_size(18);
    // but_quit.set_color(Color::from_rgb(102,194,165)); //green
    but_quit.set_callback(Box::new(move || cb_quit()));

    let mut but_ap_view = Button::new(
        FLTK_WINDOW_WIDTH - 220,
        FLTK_WINDOW_HEIGHT - 400,
        70,
        40,
        "A-P View",
    );

          
    

    let mut frame_info_view = Frame::new(
        FRAME_INFO_X,
        FRAME_INFO_Y,
        FRAME_INFO_WIDTH,
        FRAME_INFO_HEIGHT - 200,
        "Info View",
    );

    frame_info_view.set_color(Color::from_rgb(222, 235, 247)); //blue
    frame_info_view.set_frame(FrameType::RFlatBox);
    frame_info_view.set_label_size(20);
    frame_info_view.set_label("Optimal Views");

    let mut table = table::Table::new(
        FRAME_INFO_X,
        FRAME_INFO_Y + 20,
        FRAME_INFO_WIDTH - 10,
        FRAME_INFO_HEIGHT - 195,
        "OPTIMAL VIEWS",
    );
    table.set_rows(14);
    table.set_row_header(true);
    table.set_row_resize(true);
    table.set_cols(3);
    table.set_col_header(true);
    table.set_col_width_all(240);
    table.set_row_height_all(25);

    table.set_col_resize(true);
    // table.selection_color(FL_YELLOW);
    table.end();
    // Called when the table is drawn then when it's redrawn due to events
    let table_c = table.clone();

    // Fl_Table calls this function to draw each visible cell in the table.
    // draw_cell(TableContext context, int ROW=0, int COL=0, int X=0, int Y=0, int W=0, int H=0)
    //We move cells to the heap by Box...

    table.draw_cell(Box::new(move |ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 16),
        table::TableContext::ColHeader => {
            let header = vec!["SITE", "Option A", "Option B"];
            let idx = col as usize;
            draw_header(&format!("{}", header[idx]), x, y, w, h);
        } // Column titles
        table::TableContext::RowHeader => {
            draw_header(&format!("{}", row + 1), x, y, w, h)
        } // Row titles
        table::TableContext::Cell => {
            let max_col = 3;

            let sarray = get_optimal_views();
            let idx = (row * max_col + col) as usize;

            draw_data(
                &format!("{}", sarray[idx]),
                x,
                y,
                w,
                h,
                table_c.is_selected(row, col),
            ); // Data in cells
        }

        _ => (),
    }));

    but_ap_view.set_color(Color::from_rgb(161, 215, 106)); //blue light

    let mut gl_wind = window::GlWindow::new(
        10,
        10,
        GL_WINDOW_WIDTH,
        GL_WINDOW_HEIGHT,
        "GL WINDOW!",
    );

    gl_wind.make_resizable(true);

    let rao_lao = Rc::from(RefCell::from(0.0));
    let rao_lao_c = rao_lao.clone();

    let cr_ca = Rc::from(RefCell::from(0.0));
    let cr_ca_c = cr_ca.clone();

    gl_wind.draw(Box::new(move || {
        setup_gl();
        draw_scene(&rao_lao_c.borrow(), &cr_ca_c.borrow())
    }));

    gl_wind.end();
    gl_wind.show();

    fltk_wind.make_resizable(true);
    fltk_wind.end();
    fltk_wind.show();

    let (s, r) = app::channel::<(Message, f32)>();
    let (s_event, r_event) = app::channel::<(i32, i32)>();

    gl_wind.handle(Box::new(move |ev| match ev {
        Event::Push => {
            // let msg = (EventMessage::PushEvent,  app::event_coords() );
            let msg = app::event_coords();
            s_event.send(msg);
            show_segment_name(app::event_coords());
            true
        }
        _ => false,
    }));

    widget_rao_lao.set_callback(Box::new(move || {
        let angle_rao_lao = widget_rao_lao_c.value() as f32;
        let msg = (Message::Raolao, angle_rao_lao);
        s.send(msg);
    }));

    widget_cr_ca.set_callback(Box::new(move || {
        let angle_cr_ca = widget_cr_ca_c.value() as f32;
        let msg = (Message::Crca, angle_cr_ca);
        s.send(msg)
    }));

    but_ap_view.set_callback(Box::new(move || {
        let msg = (Message::AnteriorPosterior, 0.0);
        s.send(msg)
    }));

    while app.wait().unwrap() {
        if let Some(msg) = r.recv() {
            //use Message::*;
            match msg {
                (Message::Raolao, rao_angle) => {
                    *rao_lao.borrow_mut() = rao_angle;
                    frame_rao_lao.set_label(&(rao_angle).to_string());

                    gl_wind.redraw();
                }
                (Message::Crca, cr_angle) => {
                    *cr_ca.borrow_mut() = cr_angle;
                    frame_cr_ca.set_label(&(cr_angle).to_string());
                    gl_wind.redraw();
                }

                (Message::AnteriorPosterior, angle) => {
                    *rao_lao.borrow_mut() = angle;
                    *cr_ca.borrow_mut() = angle;

                    widget_cr_ca.set_value(0.0);
                    frame_cr_ca.set_label(&(angle).to_string());

                    widget_rao_lao.set_value(0.0);
                    frame_rao_lao.set_label(&(angle).to_string());
                    gl_wind.redraw();
                }
                // None => (),
                //_ => continue,
                _ => println!(" Message End"),
            } //match msg
        } //if r_recv
    } //while

         
    
} //main
  /****************************************** */

fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::pop_clip();
}

fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xD3D3D3));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

/******************************************** */
fn cb_quit() {
    //let nomeclature = get_nomeclature("LAD");
    // println!("nomeclature:{:?} ", nomeclature);

    println!("Enjoy. Quiting. Tnx ");
    std::process::exit(0x0100);
} //cb_quit
  /***********************************************************/
