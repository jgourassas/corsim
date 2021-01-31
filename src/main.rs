//First install https://github.com/burtonageo/cargo-bundle then run cargo bundle --release [target]
//For your OS run bundle --release ( cargo bundle)
// for Debian find executable in corsim/target/debug/bundle/deb/corsim_1.0.0_amd64/data/usr/bin/corsim
// If cairo.h not found-> change pangocairo.h to include your cairo.h

#![allow(non_snake_case)]

use fltk::dialog::message;
use fltk::*;
use fltk::{button::*, frame::*, valuator::*, window::*};
/*
use fltk::{
    app::{App, AppScheme, channel},
    browser::Browser,
    button::{Button, CheckButton},
    dialog:: {FileChooser, FileChooserType, message},
    input::{IntInput},
    prelude::*,
    window::DoubleWindow,
};
*/

use glu_sys::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::{error::Error, fmt, panic, thread, time};

//use std::ptr;
//use std::mem;
//use std::os::raw::c_void;
//use std::ffi::CStr;

//use std::fmt;
//use std::iter;
extern crate csv;

extern crate nalgebra as na;
use na::Point3;
//use na::{Isometry3, Point3, RealField, Vector3};

//use na::RealField;
//use std::cmp::Ordering;
//extern crate iset;
//use iset::IntervalMap;
//use iset::IntervalSet;

extern crate ncollide3d; // If you need 3D.
use ncollide2d::procedural::circle;
//use ncollide3d::procedural::cylinder;

//use ncollide3d::shape::{Ball, Compound, ConvexHull, Cuboid, Cylinder, ShapeHandle, TriMesh};

//use ncollide3d::TriMesh;

use ncollide3d::shape::Polyline;

/******test *************************** */

/**************************************/
const MARGIN_TOP: i32 = 50;
const MARGIN_BOTTOM: i32 = 100;
const MARGIN_RIGHT: i32 = 220;
const MARGIN_LEFT: i32 = 10;
const SIZE_UNIT: f32 = 2.5;

const FLTK_WINDOW_WIDTH: i32 = 1650 - MARGIN_LEFT - MARGIN_RIGHT;
const FLTK_WINDOW_HEIGHT: i32 = 1200 - MARGIN_TOP - MARGIN_BOTTOM;

//const GL_WINDOW_WIDTH: i32 = FLTK_WINDOW_WIDTH - 100;
//const GL_WINDOW_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - 100;

const GL_WINDOW_WIDTH: i32 = FLTK_WINDOW_WIDTH - 250;
const GL_WINDOW_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - 100;

const FRAME_INFO_X: i32 = GL_WINDOW_WIDTH + 10;
const FRAME_INFO_Y: i32 = MARGIN_TOP;

const FRAME_INFO_WIDTH: i32 = FLTK_WINDOW_WIDTH - (GL_WINDOW_WIDTH + 12);
const FRAME_INFO_HEIGHT: i32 = GL_WINDOW_HEIGHT - 370;

const OUTER_RADIOUS: f64 = 0.85;

mod simcor_data_functions;

use simcor_data_functions::{
    get_diameter,
    get_midpoint_92,
    get_midpoint_color_92,
    get_optimal_views,
    get_segment_points_92,
    get_segments_names_92,
    optimal_angles,
    get_nomeclature, 
    //get_nomeclature_data, 
    // get_rca_segments_names_92, get_left_segments_names_92,
};

//const SIZE_UNIT: f32 = 2.0;
/*
4:3 aspect ratio resolutions: 640×480, 800×600, 960×720, 1024×768, 1280×960, 1400×1050,
1440×1080 , 1600×1200, 1856×1392, 1920×1440, and 2048×1536.

16:10 aspect ratio resolutions: – 1280×800, 1440×900, 1680×1050, 1920×1200 and 2560×1600.
16:9 aspect ratio resolutions: 1024×576, 1152×648, 1280×720, 1366×768, 1600×900, 1920×1080, 2560×1440 and 3840×2160.

*/

//const FLTK_WINDOW_WIDTH: i32 = 2048 - MARGIN_LEFT - MARGIN_RIGHT;
//const FLTK_WINDOW_HEIGHT: i32 = 1536 - MARGIN_TOP - MARGIN_BOTTOM;

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

    //let mut frame = Frame::new(0, 0, FLTK_WINDOW_WIDTH,  FLTK_WINDOW_HEIGHT, "FRAME");
    /*
       let mut widget_rao_lao = Roller::new(
           FLTK_WINDOW_WIDTH - 60,
           FLTK_WINDOW_HEIGHT - 240,
           30,

           "RAO-LAO",
       );
    */
    let mut widget_rao_lao = NiceSlider::new(
        FLTK_WINDOW_WIDTH - 80,
        FLTK_WINDOW_HEIGHT - 350,
        30,
        240,
        "⇑Lao/Rao⇩",
    );
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
    // widget_rao_lao.set_color(Color::from_rgb(203, 213, 232)); //
    widget_rao_lao.set_color(Color::from_rgb(158, 188, 218)); //
    widget_rao_lao.set_frame(FrameType::RoundUpBox);
    widget_rao_lao.set_bounds(-90.0, 90.0);
    // widget_rao_lao.set_bounds(-180.0, 180.0);
    widget_rao_lao.set_step(1.0, 1);
    widget_rao_lao.set_label_size(18);
    widget_rao_lao.set_trigger(CallbackTrigger::Changed);
    let widget_rao_lao_c = widget_rao_lao.clone();

    let mut frame_rao_lao = Frame::new(
        FLTK_WINDOW_WIDTH - 60,
        FLTK_WINDOW_HEIGHT - 250,
        70,
        40,
        "",
    );
    frame_rao_lao.set_color(Color::from_rgb(39, 45, 206)); //blue
    frame_rao_lao.set_label_size(22);

    /*
        let mut widget_cr_ca = Roller::new(
            FLTK_WINDOW_WIDTH - 60,
            FLTK_WINDOW_HEIGHT - 480,
            30,
            200,
            "Cr-Ca",
        );
    */
    /*
        let mut widget_cr_ca = Slider::new(
            FLTK_WINDOW_WIDTH - 60,
            FLTK_WINDOW_HEIGHT - 480,
            30,
            200,
            "Cr-Ca",
        );
    */
    /*
        let mut widget_cr_ca = HorNiceSlider::new(
            FLTK_WINDOW_WIDTH - 380,
            FLTK_WINDOW_HEIGHT - 60,
            240,
            30,
            "+ ←Cra/Cau→ -",
        );
    */
    let mut widget_cr_ca = NiceSlider::new(
        FLTK_WINDOW_WIDTH - 200,
        FLTK_WINDOW_HEIGHT - 350,
        30,
        240,
        "↑Cra/Cau↓",
    );
    //    widget_cr_ca.set_color(Color::from_rgb(203, 213, 232));
    widget_cr_ca.set_color(Color::from_rgb(136, 86, 167));
    widget_cr_ca.set_frame(FrameType::RoundUpBox);

    widget_cr_ca.set_bounds(90.0, -90.0);
    //widget_cr_ca.set_bounds(180.0, -180.0);
    widget_cr_ca.set_step(1.0, 1);
    widget_cr_ca.set_label_size(18);

    let widget_cr_ca_c = widget_cr_ca.clone();

    //  let mut frame_cr_ca = Frame::new(FLTK_WINDOW_WIDTH - 300, FLTK_WINDOW_HEIGHT - 95, 70, 40, "");
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

    /*
        let mut but_rca_only = Button::new(
            FLTK_WINDOW_WIDTH - 220,
            FLTK_WINDOW_HEIGHT - 450,
            70,
            40,
            "RCA Only",
        );

        but_rca_only.set_color(Color::from_rgb(255,255,204)); //red
        but_rca_only.set_label_size(14);

        let mut but_left_only = Button::new(
            FLTK_WINDOW_WIDTH - 100,
            FLTK_WINDOW_HEIGHT - 450,
            70,
            40,
            "Left Only",
        );
        but_left_only.set_color(Color::from_rgb(255,255,204));
        but_left_only.set_label_size(14);
    */

    //Frame -> FLTK BOX = X,Y W, H
    //  const FLTK_WINDOW_HEIGHT: i32 = 1200 - MARGIN_TOP - MARGIN_BOTTOM;
    let mut frame_info_view = Frame::new(
        FRAME_INFO_X,
        FRAME_INFO_Y,
        FRAME_INFO_WIDTH,
        FRAME_INFO_HEIGHT - 200,
        "Info View",
    );
    //  Frame::new(FLTK_WINDOW_WIDTH - 60, MARGIN_TOP - 150, FRAME_INFO_WIDTH , FRAME_INFO_HEIGHT, "Info View");
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
        //setup_gl();
        //draw_rca_scene(&rao_lao_c.borrow(), &cr_ca_c.borrow());
        //draw_left_scene(&rao_lao_c.borrow(), &cr_ca_c.borrow());
    }));

    gl_wind.end();
    gl_wind.show();

    fltk_wind.make_resizable(true);
    fltk_wind.end();
    fltk_wind.show();

    let (s, r) = app::channel::<(Message, f32)>();
    //let (s_event, r_event) =  app::channel::(Message, <(i32, i32)>)) ();
    // let (s_event, r_event) = app::channel::< ( i32, i32 ) >();
    let (s_event, r_event) = app::channel::<(i32, i32)>();

    gl_wind.handle(Box::new(move |ev| match ev {
        Event::Push => {
            s_event.send(app::event_coords());
            //println!("coords: {:?} ",   app::event_coords() );
            //present_segment_name(app::event_coords());
            true
        }
        // _ => false,
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

    /*
    while app.wait(){

        if let Some(msg) = r.recv{
       match msg{

       }//match msg

        }//if r_recv
 
        if let Some(msg) = r_event{
           match msg{

           }//match r_event

        }//if msg = r_event
        

    }//while
    */
    while app.wait().unwrap() {
        match r.recv() {
            Some((Message::Raolao, rao_angle)) => {
                *rao_lao.borrow_mut() = rao_angle;
                frame_rao_lao.set_label(&(rao_angle).to_string());

                gl_wind.redraw();
            }
            Some((Message::Crca, cr_angle)) => {
                *cr_ca.borrow_mut() = cr_angle;
                frame_cr_ca.set_label(&(cr_angle).to_string());
                gl_wind.redraw();
            }

            Some((Message::AnteriorPosterior, angle)) => {
                *rao_lao.borrow_mut() = angle;
                *cr_ca.borrow_mut() = angle;

                widget_cr_ca.set_value(0.0);
                frame_cr_ca.set_label(&(angle).to_string());

                widget_rao_lao.set_value(0.0);
                frame_rao_lao.set_label(&(angle).to_string());
                gl_wind.redraw();
            }
            //  None => (),
            _ => println!(""),
        } //match

        /**********************************/
        /************************************/
    } //while
} //main
  /****************************************** */

/*
In the past I've used glReadPixels( xf, yf, 1, 1, GL_DEPTH_COMPONENT, GL_FLOAT, &zf);
to read the value of the depth buffer at a point (xf, yf) in screen space.
You can then unproject this point back into world coordinates (multiply
by the inverse of the modelview and projection matrices.

This can be done with gluUnProject provided you're happy including the GLU library in your application.
This gives you a coordinate, you then need to search through your objects to find one that has a bounding box
that contains this coordinate. This object is the one selected.
*/
/*
fn present_segment_name(coords: (i32, i32)) {
    let mut x = coords.0;
    let mut y = coords.1;
    //GLbyte color[4];
    unsafe {

       // GLuint buff[64] = {0};

	    GLint hits, view[4];

        let id: i32;
        glPushMatrix();
        /*
            This choose the buffer where store the values for the selection data
        */

        glSelectBuffer(64, buff);
        /*
            This retrieves info about the viewport
        */
        glGetIntegerv(GL_VIEWPORT, view);
        /*
            Switching in selecton mode
        */
        glRenderMode(GL_SELECT);
/*
		Clearing the names' stack
		This stack contains all the info about the objects
	*/
      glInitNames();
    
      /*
		Now fill the stack with one element (or glLoadName will generate an error)
	*/
    glPushName(0);
    /*
		Now modify the viewing volume, restricting selection area around the cursor
	*/
	glMatrixMode(GL_PROJECTION);
	glPushMatrix();
		glLoadIdentity();

		/*
			restrict the draw to an area around the cursor
		*/
		gluPickMatrix(x, y, 1.0, 1.0, view);
		gluPerspective(60, 1.0, 0.0001, 1000.0);

		/*
			Draw the objects onto the screen
		*/
		glMatrixMode(GL_MODELVIEW);
		
		/*
			draw only the names in the stack, and fill the array
		*/
        //gl_draw();
        

		/*
			Do you remeber? We do pushMatrix in PROJECTION mode
		*/
		glMatrixMode(GL_PROJECTION);
	glPopMatrix();

	/*
		get number of objects drawed in that area
		and return to render mode
	*/
	let hits = glRenderMode(GL_RENDER);

	/*
		Print a list of the objects
	*/
//	list_hits(hits, buff);

	/*
		uncomment this to show the whole buffer
	* /
	gl_selall(hits, buff);
	*/

	glMatrixMode(GL_MODELVIEW);
      
      
      
      
      
      
      
      glPopMatrix();

        
          
    } //unsafe
} //present_segment_name
*/
/******************************************** */
fn cb_quit() {
    //let nomeclature = get_nomeclature("LAD");
    // println!("nomeclature:{:?} ", nomeclature);

    println!("Enjoy. Quiting. Tnx ");
    std::process::exit(0x0100);
} //cb_quit
  /***********************************************************/
/***************************************************************** */
/*
  fn draw_rca_scene(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    //setup_gl();
    let segment_names = get_rca_segments_names_92();
    let mut i = 0;
    while i < segment_names.len() {
        draw_segment_92(&segment_names[i], rotate_rao_lao, rotate_cr_ca);
        i = i + 1;
    } //while
    draw_machine(rotate_rao_lao, rotate_cr_ca);
    set_marker(rotate_rao_lao, rotate_cr_ca);
    draw_frame();
    /**************************************** */
} //draw_scene
/***************************************************************** */

fn draw_left_scene(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    //setup_gl();
    let segment_names = get_left_segments_names_92();
    let mut i = 0;
    while i < segment_names.len() {
        draw_segment_92(&segment_names[i], rotate_rao_lao, rotate_cr_ca);
        i = i + 1;
    } //while
    draw_machine(rotate_rao_lao, rotate_cr_ca);
    set_marker(rotate_rao_lao, rotate_cr_ca);
    draw_frame();
    /**************************************** */
} //draw_scene
*/
/***************************************************************** */
fn draw_scene(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    // setup_gl();
    let segment_names = get_segments_names_92();
    let mut i = 0;
    while i < segment_names.len() {
        draw_segment_92(&segment_names[i], rotate_rao_lao, rotate_cr_ca);
        i = i + 1;
    } //while
    draw_machine(rotate_rao_lao, rotate_cr_ca);
    set_marker(rotate_rao_lao, rotate_cr_ca);
    draw_frame();
    /**************************************** */
} //draw_scene

fn setup_gl() {
    unsafe {
        glClearColor(0.0, 0.0, 0.0, 0.0);
        // glClearColor(64.0/255.0, 64.0/255.0, 64.0/255.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        //  glOrtho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
        // glFrustum(-1.0, 1.0, -1.0, 1.0, 1.5, 100.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        // Disable backface culling to render both sides of polygons
        glEnable(GL_DEPTH_TEST);
        glEnable(GL_LINE_SMOOTH);
        glHint(GL_LINE_SMOOTH_HINT, GL_NICEST);
        //glEnable(GL_POINT_SMOOTH);

        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glEnable(GL_MULTISAMPLE);
    } //usafe
}
/************************************************** */
fn draw_segment_92(
    segment_name: &str,
    rotate_rao_lao: &f32,
    rotate_cr_ca: &f32,
) {
    let point_names = get_segment_points_92(segment_name);

    let mut points = vec![];
    let mut diameters_vec = vec![];
    let mut midpoint_names_vec = vec![];

    let mut i = 0;
    unsafe {
        glPushMatrix();

        glLoadIdentity();
        glTranslatef(-0.2, 0.2, 0.0);

        glRotatef(*rotate_cr_ca, 1.0, 0.0, 0.0); //x
        glRotatef(*rotate_rao_lao, 0.0, 1.0, 0.0); //y axis

        //glRotatef(0.0, 0.0, 0.0, 1.0); //z axis
        glScalef(0.1, 0.1, 0.1);
        while i < point_names.len() {
            let midpoint = get_midpoint_92(&point_names[i]);
            let color_vec: Vec<u8> = get_midpoint_color_92(point_names[i]);

            let diameter: f32 = get_diameter(point_names[i]);
            diameters_vec.push(diameter);

            let x = midpoint[0];
            let y = midpoint[1];
            let z = midpoint[2];

            points.push(Point3::new(x, y, z));

            midpoint_names_vec.push(point_names[i]);
            // draw_as_bezier_segment(&points, &color_vec, &diameters_vec, point_names[i]);

            draw_as_polyline_segment(
                &points,
                &color_vec,
                &diameters_vec,
                &midpoint_names_vec,
            );

            i = i + 1;
        } //while
        draw_aortic_ring();

        glPopMatrix();
    } //unsafe
} //draw_segments
  /******************************************************* */
/***********************************************************/
/*
fn draw_as_bezier_segment(points: &[Point3<f32>], color: &Vec<u8>, diameters: &Vec<f32>, midpoint_name: &str) {
    let polyline = Polyline::new(points.to_vec(), None);
    let bezier = procedural::bezier_curve(&polyline.points(), points.len());

    unsafe {
 /****************************************************** */

 glPushMatrix();
 glColor3ub(color[0], color[1], color[2]);
 glPointSize(diameters[0] * SIZE_UNIT * 0.7 );
 glLineWidth(diameters[0] * SIZE_UNIT * 0.7);
 let mut j: usize = 0;
 /*
 glBegin(GL_POINTS);
 while j <  bezier.len()   {
    glVertex3fv(&bezier[j][0]);
    j = j + 1;

 }
glEnd();
*/

glBegin(GL_LINE_STRIP);
 while j <  bezier.len()   {
    glVertex3fv(&bezier[j][0]);
    j = j + 1;

 }
glEnd();


glPopMatrix();
        }//unsafe



} //draw_segment
*/

fn draw_as_polyline_segment(
    points: &[Point3<f32>],
    color: &Vec<u8>,
    diameters: &Vec<f32>,
    midpoint_names: &Vec<&str>,
) {
    let polyline = Polyline::new(points.to_vec(), None);

    unsafe {
        /****************************************** */

        /*************START DRAW POLYLINE****************************************** */
        //let qobj = gluNewQuadric();
        glPushMatrix(); //start 1
                        /************START TEST LIGHT****************************** */
        /*
            //Red rubber
            let  mat_ambient =  [0.05,0.0,0.0,1.0 ];
            let  mat_diffuse =  [0.5,0.4,0.4,1.0 ];
            let  mat_specular = [0.7,0.04,0.04,1.0];
            let  light_position = [1.0,1.0,1.0,0.0];

            glMaterialfv(GL_FRONT,GL_AMBIENT, mat_ambient.as_ptr());
            glMaterialfv(GL_FRONT,GL_DIFFUSE, mat_diffuse.as_ptr());
            glMaterialfv(GL_FRONT,GL_SPECULAR,mat_specular.as_ptr());
            glLightfv(GL_LIGHT0, GL_POSITION,light_position.as_ptr());

            glMaterialf(GL_FRONT,GL_SHININESS, 10.0);

            glColorMaterial(GL_FRONT,GL_DIFFUSE);
            glEnable(GL_COLOR_MATERIAL);
            glEnable(GL_LIGHTING);
            glEnable(GL_LIGHT0);
            glEnable(GL_NORMALIZE);
           // glFrontFace(GL_CW);
        */

        /************END START LIGHT**********************************/
        glColor3ub(color[0], color[1], color[2]);
        let mut j = 0;

        while j < polyline.points().len() {
            glPushMatrix(); //start 2
            glLineWidth(diameters[j] * SIZE_UNIT * 0.7);
            /******************************/

            /************************************* */
            //  glBegin(GL_POINTS);
            glBegin(GL_LINE_STRIP);
            glVertex3f(
                polyline.points()[j][0],
                polyline.points()[j][1],
                polyline.points()[j][2],
            );
            j = j + 1;
        }

        glEnd();
        //glFlush();

        glPopMatrix(); //end 2

        //  glDisable(GL_LIGHTING);
        glPopMatrix(); //end 1

        /*************END DRAW POLYLINE****************************************** */

        /***************START DRAW POINTS***************************** */
        glPushMatrix(); //STARTT 1

        glColor3ub(color[0], color[1], color[2]);

        let mut k = 0;

        while k < polyline.points().len() {
            glPushMatrix(); //START 2

            glPointSize(diameters[k] * SIZE_UNIT * 0.5);
            glBegin(GL_POINTS);
            glVertex3f(
                polyline.points()[k][0],
                polyline.points()[k][1],
                polyline.points()[k][2],
            );

            k = k + 1;
        }

        glEnd();

        //glRasterPos2f(-3.0, -2.0);
        //glFlush();

        glPopMatrix(); //END 2

        glPopMatrix(); //END 1

        /***************END DRAW POINTS***************************** */
    } //unsafe
} //draw_segment
  /////
  /***********************************************************/
/*
fn draw_as_polyline_segment_1(
    points: &[Point3<f32>],
    color: &Vec<u8>,
    diameters: &Vec<f32>,
    _midpoint_name: &str,
) {
    let polyline = Polyline::new(points.to_vec(), None);
    //println!("Polyline: {:?}   ", polyline);

    unsafe {
        glPushMatrix();

        glColor3ub(color[0], color[1], color[2]);
        let mut j = 0;

        glLineWidth(diameters[0] * SIZE_UNIT * 0.7);
        glBegin(GL_LINE_STRIP);
        while j < polyline.points().len() {
            glVertex3f(
                polyline.points()[j][0],
                polyline.points()[j][1],
                polyline.points()[j][2],
            );

            j = j + 1;
        }
        glEnd();

        glPopMatrix();

        glPushMatrix();
        glPointSize(diameters[0] * SIZE_UNIT * 0.65);
        let mut j = 0;

        glBegin(GL_POINTS);
        while j < polyline.points().len() {
            glVertex3f(
                polyline.points()[j][0],
                polyline.points()[j][1],
                polyline.points()[j][2],
            );

            j = j + 1;
        }
        glEnd();

        glPopMatrix();
    } //unsafe
} //draw_segment
*/

/***********************************************************/

fn draw_machine(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    unsafe {
        glPushMatrix();
        glEnable(GL_BLEND);
        glDepthMask(GL_FALSE as u8);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        draw_arm(rotate_rao_lao, rotate_cr_ca);

        glDepthMask(GL_TRUE as u8);
        glDisable(GL_BLEND);
        glPopMatrix();
    } //unsafe
} //draw_machine

fn draw_arm(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    //let outer_radius = 0.85;
    let start_angle = 0.0;
    let end_angle = -180.0;
    unsafe {
        let quadric = gluNewQuadric();
        glPushMatrix();
        gluQuadricDrawStyle(quadric, GLU_LINE);
        gluQuadricNormals(quadric, GLU_SMOOTH);
        glTranslatef(-0.2, 0.0, 0.0);

        glRotatef(90.0, 0.0, 1.0, 0.0);

        glRotatef(*rotate_cr_ca, 0.0, 0.0, -1.0); //z
        glRotatef(*rotate_rao_lao, -1.0, 0.0, 0.0); //x axis

        //Semi Tranparent
        //glColor4f(252.0, 146.0, 114.0, 0.2);
        glColor4ub(224, 236, 244, 2);
        gluPartialDisk(
            quadric,
            OUTER_RADIOUS,
            OUTER_RADIOUS + 0.01,
            42,
            50,
            start_angle,
            end_angle,
        );

        /*************DRAW THE REST OF EQUIPMENT********* */
        draw_collimator();
        x_ray_beam();
        draw_digital_camera();
        //draw_cylinder(SLICES: i8, STACKS: i8, SCALE_X: f32, SCALE_Y: f32, SCALE_Z: f32)
        //glScalef(0.5, 1.0, 1.0);

        /*********************************************** */

        glPopMatrix();
    } //unsafe
} //draw_arm

/************************************************/

fn draw_frame() {
    unsafe {
        glPushMatrix();
        glLineWidth(10.0);
        glColor3ub(247, 247, 247);

        glBegin(GL_LINES);

        glVertex2f(-0.98, -0.98);
        glVertex2f(0.98, -0.98);

        glVertex2f(-0.98, -0.98);
        glVertex2f(-0.98, 0.98);

        glVertex2f(-0.98, 0.98);
        glVertex2f(0.98, 0.98);

        glVertex2f(0.98, 0.98);
        glVertex2f(0.98, -0.98);

        glEnd();

        glPopMatrix();
    }
} //draw_frame

/**********************************************************/
fn x_ray_beam() {
    let mut i: usize = 0;

    unsafe {
        glPushMatrix();
        glTranslatef(0.2, 0.0, 0.0);
        glColor4f(252.0, 146.0, 114.0, 0.7);
        glLineWidth(0.2);
        glBegin(GL_LINES);
        while i <= 8 {
            glVertex3f(-0.24, -OUTER_RADIOUS as f32, 0.0);
            glVertex3f(-0.24 + i as f32 * 0.01, 0.80, 0.0);
            i += 1;
        }
        glEnd();

        glPopMatrix();
    } //unsafe
} //x_ray_beam

fn draw_digital_camera() {
    unsafe {
        glPushMatrix();

        glTranslatef(0.0, OUTER_RADIOUS as f32, 0.0);

        glScalef(0.15, 0.05, 0.1);

        glBegin(GL_POLYGON);
        // Multi-colored side - FRONT
        glColor4f(224.0, 224.0, 22.0, 0.6);

        glVertex3f(-0.5, -0.5, -0.5); // P1
        glVertex3f(-0.5, 0.5, -0.5); // P2
        glVertex3f(0.5, 0.5, -0.5); // P3
        glVertex3f(0.5, -0.5, -0.5); // P4

        // White side - BACK
        glBegin(GL_POLYGON);
        //glColor4ub(1, 133, 113, 5); //dark
        glColor4f(153.0, 153.0, 153.0, 0.3);
        glVertex3f(0.5, -0.5, 0.5);
        glVertex3f(0.5, 0.5, 0.5);
        glVertex3f(-0.5, 0.5, 0.5);
        glVertex3f(-0.5, -0.5, 0.5);
        glEnd();

        // Purple side - RIGHT
        glBegin(GL_POLYGON);
        //glColor4ub(166, 97, 26, 5); //green dark
        glColor4f(153.0, 153.0, 153.0, 0.3);
        glVertex3f(0.5, -0.5, -0.5);
        glVertex3f(0.5, 0.5, -0.5);
        glVertex3f(0.5, 0.5, 0.5);
        glVertex3f(0.5, -0.5, 0.5);
        glEnd();

        // Green side - LEFT
        glBegin(GL_POLYGON);
        //glColor4ub(223, 194, 125, 5);
        glColor4f(153.0, 153.0, 153.0, 0.3);

        glVertex3f(-0.5, -0.5, 0.5);
        glVertex3f(-0.5, 0.5, 0.5);
        glVertex3f(-0.5, 0.5, -0.5);
        glVertex3f(-0.5, -0.5, -0.5);
        glEnd();

        // Blue side - TOP
        glBegin(GL_POLYGON);
        //glColor4ub(203, 201, 22, 5);
        glColor4f(153.0, 153.0, 153.0, 0.3);
        glVertex3f(0.5, 0.5, 0.5);
        glVertex3f(0.5, 0.5, -0.5);
        glVertex3f(-0.5, 0.5, -0.5);
        glVertex3f(-0.5, 0.5, 0.5);
        glEnd();

        // Red side - BOTTOM
        glBegin(GL_POLYGON);
        // glColor4ub(106, 81, 163, 5);

        glColor4f(153.0, 153.0, 153.0, 0.2);

        glVertex3f(0.5, -0.5, -0.5);
        glVertex3f(0.5, -0.5, 0.5);
        glVertex3f(-0.5, -0.5, 0.5);
        glVertex3f(-0.5, -0.5, -0.5);
        glEnd();

        glFlush();
        glPopMatrix();
        //glLoadIdentity();
    } //unsafe
} //draw_digital_cammer
  /**************************************************** */
fn draw_collimator() {
    let radius = SIZE_UNIT;
    let start = -(OUTER_RADIOUS + 0.1);
    unsafe {
        glPushMatrix();
        glDisable(GL_CULL_FACE);
        glColor4f(107.0, 174.0, 214.0, 0.5); //Cyan blue

        //Trick
        glTranslatef(0.0, start as f32, 0.0);
        glRotatef(-90.0, 1.0, 0.0, 0.0);
        glScalef(0.016, 0.016, 0.016);

        let quadric = gluNewQuadric();
        //gluCylinder(	GLUquadric* quad,GLdouble base,GLdouble top,GLdouble height,GLint slices,GLint stacks);
        gluCylinder(quadric, radius as f64, radius as f64 * 0.5, 10.0, 15, 20);
        //glFlush();
        glEnable(GL_CULL_FACE);

        glPopMatrix();
    } //unsafe
} //draw_collimator

/****************************************************/

fn set_marker(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    let angles = optimal_angles();

    for (point_name, angle_vec) in angles.iter() {
        if rotate_rao_lao > &angle_vec[0]
            && rotate_rao_lao < &angle_vec[1]
            && rotate_cr_ca > &angle_vec[2]
            && rotate_cr_ca < &angle_vec[3]
        {
            let point = get_midpoint_92(&point_name);
            draw_marker(point, rotate_rao_lao, rotate_cr_ca);
        } //if
    } //for

    show_rao_lao_lights(rotate_rao_lao);
    show_cr_ca_lights(rotate_cr_ca);

    draw_spine(rotate_rao_lao);
} //set marker

/**************************************************** */

fn show_rao_lao_lights(rao_lao: &f32) {
    let angles = optimal_angles();

    for (_point_name, angle_vec) in angles.iter() {
        if rao_lao > &angle_vec[0] && rao_lao < &angle_vec[1] {
            draw_rao_lao_lights()
        } //if
    } //for
}
/************************************************* */
fn show_cr_ca_lights(cr_ca: &f32) {
    let angles = optimal_angles();

    for (_point_name, angle_vec) in angles.iter() {
        if cr_ca > &angle_vec[2] && cr_ca < &angle_vec[3] {
            draw_cr_ca_lights();
        } //if
    } //for
}
/*************************************** */
fn draw_rao_lao_lights() {
    unsafe {
        glPushMatrix();
        glEnable(GL_POINT_SMOOTH);
        //glColor3ub(255,255,191); //light yellow
        glColor3ub(158, 188, 218); //
        glPointSize(10.0 * SIZE_UNIT * 1.0);
        glBegin(GL_POINTS);
        glVertex3f(0.9, -0.9, 0.0);
        glEnd();
        glPopMatrix();
    } //unsafe
} //show_lights
  /*************************************************/

fn draw_cr_ca_lights() {
    unsafe {
        glPushMatrix();
        glEnable(GL_POINT_SMOOTH);
        //glColor3ub(255,255,191); //light yellow
        glColor3ub(136, 86, 167); //
        glPointSize(10.0 * SIZE_UNIT * 1.0);
        glBegin(GL_POINTS);
        glVertex3f(0.8, -0.9, 0.0);
        glEnd();
        glPopMatrix();
    } //unsafe
} //show_lights
  /*************************************************/

/******************************************************** */

fn draw_marker(center: Vec<f32>, rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    let mut j = 0;
    unsafe {
        //circle = diameter: &<P as EuclideanSpace>::Real,
        //nsubdivs: u32
        //-> Polyline<P>

        let polyline = circle(&0.8, 64);

        glPushMatrix();

        glTranslatef(-0.2, 0.2, 0.0);
        glRotatef(*rotate_cr_ca, 1.0, 0.0, 0.0); //x
        glRotatef(*rotate_rao_lao, 0.0, 1.0, 0.0); //y axis
        glScalef(0.1, 0.1, 0.1);
        /***********************************/
        glPushMatrix();
        glColor3f(1.0, 1.0, 1.0); //white
        glTranslatef(center[0], center[1], center[2]);
        glPointSize(1.0 * SIZE_UNIT * 0.6);
        glBegin(GL_POINTS);
        while j < polyline.coords().len() {
            glVertex3f(polyline.coords()[j][0], polyline.coords()[j][1], 0.0);
            j = j + 1;
        }
        glEnd();
        glPopMatrix();

        /**************************************/
        glFlush();
        glPopMatrix();
    } //usafe
} //draw_marker

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

/******************************************************** */
//glScalef(0.5, 1.0, 1.0);
fn draw_aortic_ring() {
    unsafe {
        let qobj = gluNewQuadric();

        glPushMatrix();
        gluQuadricNormals(qobj, GLU_SMOOTH);
        gluQuadricOrientation(qobj, GLU_OUTSIDE);
        gluQuadricTexture(qobj, GL_TRUE as u8);
        glLineWidth(1.4);

        glColor3ub(250, 159, 181);
        //Trick
        glTranslatef(-0.2, 0.1, 0.0);
        glRotatef(-10.0, 1.0, 0.0, 0.0);
        glRotatef(-50.0, 0.0, 1.0, 0.0);
        glScalef(1.1, 0.5, 1.1);

        gluDisk(qobj, 1.0, 1.1, 150, 120);

        glBegin(GL_LINE_STRIP);
        glVertex3f(-1.0, 0.2, 0.0);
        glVertex3f(0.01, 0.0, 0.0);

        glVertex3f(0.40, -1.0, 0.0);

        glVertex3f(0.01, 0.0, 0.0);
        glVertex3f(0.2, 1.0, 0.0);

        glEnd();

        glPopMatrix();

        gluDeleteQuadric(qobj);
    } //unsafe
} //render_triagle

fn draw_spine(rao_lao: &f32) {
    let incr = 0.05;
    let spine_left = *rao_lao * 0.003;
    //rao spine in th left
    //lao spine in the rigth

    unsafe {
        /********************************************** */
        glPushMatrix();

        glPointSize(12.0);
        glTranslatef(-spine_left, 0.0, 0.0);
        glColor3ub(43, 140, 190);

        glBegin(GL_POINTS);
        glVertex3f(-0.2, -0.7, 0.0);
        glVertex3f(-0.2, 0.7, 0.0);

        glVertex3f(-0.2, -0.7 - incr, 0.0);
        glVertex3f(-0.2, 0.7 - incr, 0.0);

        glVertex3f(-0.2, -0.7 + incr, 0.0);
        glVertex3f(-0.2, 0.7 + incr, 0.0);

        glEnd();

        glFlush();

        glPopMatrix();
        //glDisable(GL_BLEND); //restore blending options

        /****************************************** */
    } //unsafe
      /**************************************************** */

    test_implicit();

    /**********************************************************/
} //draw_spine

fn test_implicit() {
    unsafe {
        glPushMatrix();
        glPopMatrix();
    }
}
