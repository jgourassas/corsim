use fltk::*;
use fltk::{button::*, frame::*, valuator::*, window::*};
use glu_sys::*;
use std::cell::RefCell;
//use std::fmt;
//use std::iter;
use std::rc::Rc;
extern crate csv;

extern crate nalgebra as na;
use na::Point3;

extern crate ncollide3d; // If you need 3D.
                         //use ncollide3d::shape::ConvexHull

//use ncollide3d::procedural;
//use ncollide3d::procedural::path::{ArrowheadCap, PolylinePath, PolylinePattern, StrokePattern};
//use ncollide3d::shape::{Ball, ConvexHull, Cuboid, Cylinder, Polyline, ShapeHandle};
use ncollide3d::shape::{ Polyline};


/********************************* */
const MARGIN_TOP: i32 = 50;
const MARGIN_BOTTOM: i32 = 100;
const MARGIN_RIGHT: i32 = 220;
const MARGIN_LEFT: i32 = 10;
const SIZE_UNIT: f32 = 2.5;

mod simcor_data_functions;
use simcor_data_functions::{get_segments_names_92, get_midpoint_92, get_segment_points_92, 
    get_midpoint_color_92, get_diameter};


//const SIZE_UNIT: f32 = 2.0;
/*
4:3 aspect ratio resolutions: 640×480, 800×600, 960×720, 1024×768, 1280×960, 1400×1050,
1440×1080 , 1600×1200, 1856×1392, 1920×1440, and 2048×1536.

16:10 aspect ratio resolutions: – 1280×800, 1440×900, 1680×1050, 1920×1200 and 2560×1600.
16:9 aspect ratio resolutions: 1024×576, 1152×648, 1280×720, 1366×768, 1600×900, 1920×1080, 2560×1440 and 3840×2160.

*/

//const FLTK_WINDOW_WIDTH: i32 = 2048 - MARGIN_LEFT - MARGIN_RIGHT;
//const FLTK_WINDOW_HEIGHT: i32 = 1536 - MARGIN_TOP - MARGIN_BOTTOM;
const FLTK_WINDOW_WIDTH: i32 = 1600 - MARGIN_LEFT - MARGIN_RIGHT;
const FLTK_WINDOW_HEIGHT: i32 = 1200 - MARGIN_TOP - MARGIN_BOTTOM;

const GL_WINDOW_WIDTH: i32 = FLTK_WINDOW_WIDTH - 100;
const GL_WINDOW_HEIGHT: i32 = FLTK_WINDOW_HEIGHT - 100;
const OUTER_RADIOUS: f64 = 0.85;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Raolao,
    Crca,
    AnteriorPosterior,
}

pub fn main() {
    //get_data_92();
    //get_branch_data_92();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut fltk_wind = Window::new(
        100,
        100,
        FLTK_WINDOW_WIDTH,
        FLTK_WINDOW_HEIGHT,
        "CORONARY SIM. By John Gkourasas ",
    );

    //let mut frame = Frame::new(0, 0, FLTK_WINDOW_WIDTH,  FLTK_WINDOW_HEIGHT, "FRAME");
    /*
       let mut widget_rao_lao = Roller::new(
           FLTK_WINDOW_WIDTH - 60,
           FLTK_WINDOW_HEIGHT - 240,
           30,
           200,
           "RAO-LAO",
       );
    */
    let mut widget_rao_lao = NiceSlider::new(
        FLTK_WINDOW_WIDTH - 80,
        FLTK_WINDOW_HEIGHT - 275,
        30,
        240,
        "↑Lao/Rao↓",
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
    widget_rao_lao.set_color(Color::from_rgb(203, 213, 232)); //
    widget_rao_lao.set_frame(FrameType::RoundUpBox);
    widget_rao_lao.set_bounds(-90.0, 90.0);
    //widget_rao_lao.set_bounds(90.0, -90.0);
    widget_rao_lao.set_step(1.0, 1);
    widget_rao_lao.set_label_size(22);
    widget_rao_lao.set_trigger(CallbackTrigger::Changed);
    let widget_rao_lao_c = widget_rao_lao.clone();

    let mut frame_rao_lao =
        Frame::new(FLTK_WINDOW_WIDTH - 60, FLTK_WINDOW_HEIGHT - 210, 70, 40, "");
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
    let mut widget_cr_ca = HorNiceSlider::new(
        FLTK_WINDOW_WIDTH - 380,
        FLTK_WINDOW_HEIGHT - 60,
        240,
        30,
        "←Cra/Cau→",
    );
    widget_cr_ca.set_color(Color::from_rgb(203, 213, 232));

    widget_cr_ca.set_frame(FrameType::RoundUpBox);

    widget_cr_ca.set_bounds(90.0, -90.0);
    widget_cr_ca.set_step(1.0, 1);
    widget_cr_ca.set_label_size(22);

    let widget_cr_ca_c = widget_cr_ca.clone();

    let mut frame_cr_ca = Frame::new(FLTK_WINDOW_WIDTH - 300, FLTK_WINDOW_HEIGHT - 95, 70, 40, "");
    frame_cr_ca.set_color(Color::from_rgb(39, 206, 201)); //green
    frame_cr_ca.set_label_size(22);

    let mut but_quit = Button::new(
        FLTK_WINDOW_WIDTH - 80,
        FLTK_WINDOW_HEIGHT - 380,
        70,
        40,
        "Quit➤",
    );
    but_quit.set_color(Color::from_rgb(183, 19, 19)); //red
    but_quit.set_callback(Box::new(move || cb_quit()));

    let mut but_ap_view = Button::new(
        FLTK_WINDOW_WIDTH - 480,
        FLTK_WINDOW_HEIGHT - 60,
        70,
        40,
        "P-A View",
    );
    but_ap_view.set_color(Color::from_rgb(179, 205, 227)); //blue light

    let mut gl_wind =
        window::GlWindow::new(10, 10, GL_WINDOW_WIDTH, GL_WINDOW_HEIGHT, "GL WINDOW!");
    gl_wind.make_resizable(true);

    let rao_lao = Rc::from(RefCell::from(0.0));
    let rao_lao_c = rao_lao.clone();

    let cr_ca = Rc::from(RefCell::from(0.0));
    let cr_ca_c = cr_ca.clone();

    gl_wind.draw(Box::new(move || {
        draw_scene(&rao_lao_c.borrow(), &cr_ca_c.borrow())
    }));
    gl_wind.end();
    gl_wind.show();

    fltk_wind.make_resizable(true);
    fltk_wind.end();
    fltk_wind.show();

    let (s, r) = app::channel::<(Message, f32)>();

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

            _ => println!(""),
        }
    } //while
} //main

fn cb_quit() {
    println!("Enjoy. Quiting. Tnx ");
    std::process::exit(0x0100);
} //cb_quit
  /***********************************************************/

fn draw_scene(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    setup_gl();
    let segment_names = get_segments_names_92();
    let mut i = 0;
    while i < segment_names.len() {
        draw_segment_92(&segment_names[i], rotate_rao_lao, rotate_cr_ca);
        i = i + 1;
    } //while
    draw_machine(rotate_rao_lao, rotate_cr_ca);

    //draw_axis(rotate_rao_lao, rotate_cr_ca);
    //draw_floor();

    // draw_coronaries_88(rotate_rao_lao, rotate_cr_ca);
    /********************************* */
    //calc_midpoint_distance("L_OSTIUM - LMm",
    //    1.6,
    //    0.7,
    //    -43.0,
    //    80.0,
    //    24.0,
    //    4.0);
    /**************************************** */
} //draw_scene

fn setup_gl() {
    unsafe {
        glClearColor(0.0, 0.0, 0.0, 0.0);
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
        //glEnable(GL_POINT_SMOOTH);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glEnable(GL_BLEND);

        /*
                let mat_ambient = vec![1.0, 1.0, 1.0, 1.0 ];
                let  mat_ambient  = vec![ 1.0, 1.0, 1.0, 1.0 ];
                let  mat_diffuse = vec![ 1.0, 0.2, 1.0, 1.0 ];
                let mat_specular = vec![  1.0, 1.0, 1.0, 1.0 ];
              //  let  mat_shininess: f32  = 50.0;

                let  light0_position = vec![ 1.0, 0.1, 1.0, 0.0] ;
                let  light1_position = vec![ -1.0, 0.1, 1.0, 0.0 ];

                let  lmodel_ambient = vec![ 0.3, 0.3, 0.3, 1.0 ];

                glMaterialfv(GL_FRONT, GL_DIFFUSE, mat_diffuse.as_ptr() );
                glMaterialfv(GL_FRONT, GL_SPECULAR, mat_specular.as_ptr());
               // glMaterialfv(GL_FRONT, GL_SHININESS, mat_shininess.as_ptr() );
                glLightfv(GL_LIGHT0, GL_POSITION, light0_position.as_ptr() );
                glLightfv(GL_LIGHT1, GL_POSITION, light1_position.as_ptr());
                glLightModelfv(GL_LIGHT_MODEL_AMBIENT, lmodel_ambient.as_ptr());

                glEnable(GL_LIGHTING);
                glEnable(GL_LIGHT0);
                glEnable(GL_LIGHT1);
                glDepthFunc(GL_LESS);
                glEnable(GL_DEPTH_TEST);
                glEnable(GL_AUTO_NORMAL);
        */
    } //usafe
}

 

fn draw_segment_92(segment_name: &str, rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
    let point_names = get_segment_points_92(segment_name);

    let mut points = vec![];

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
            let color: Vec<u8> = get_midpoint_color_92(point_names[i]);
            let diameter: f32 = get_diameter(point_names[i]);
            let x = midpoint[0];
            let y = midpoint[1];
            let z = midpoint[2];
            

            points.push(Point3::new(x, y, z));
            draw_segment(&points, color, diameter);
            i = i + 1;
        } //while
        glPopMatrix();
    } //unsafe
} //draw_segments

/***********************************************************/
//fn draw_segment(points: &[Point3<f32>], color: Vec<u8>, diameter: f32) {
    fn draw_segment(points: &[Point3<f32>], color: Vec<u8>, diameter: f32) {  
    // let convex = ConvexHull::try_from_points(&points).expect("Convex hull computation failed.");
    //println!("draw_segments convex: {:?}", convex.points().len() );
    //println!("draw_segments points: {:?}", points.to_vec());

    let polyline = Polyline::new(points.to_vec(), None);

    unsafe {
        /****************************************************** */

        glPushMatrix();
        glLineWidth(diameter * SIZE_UNIT * 0.9);
        glColor3ub(color[0], color[1], color[2]);
        
              
        glBegin(GL_LINE_STRIP);
        let mut j = 0;
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
        /*************************************************** */
    } //unsafe
} //draw_segment

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
        glColor4f(252.0, 146.0, 114.0, 0.2);
        
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

        /*********************************************** */

        glPopMatrix();
    } //unsafe
} //draw_arm

/*********************************************** */
fn x_ray_beam() {
    let mut i: usize = 0;

    unsafe {
        glPushMatrix();
        glTranslatef(0.2, 0.0, 0.0);
        glColor3ub(252, 146, 114);
        glLineWidth(0.2);
        glBegin(GL_LINES);
        while i <= 8 {
            glVertex3f(-0.22, -OUTER_RADIOUS as f32, 0.0);
            glVertex3f(-0.22 + i as f32 * 0.01, 0.90, 0.0);
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
        glColor3ub(128,205,193);//green light
        glVertex3f(-0.5, -0.5, -0.5); // P1
        glVertex3f(-0.5, 0.5, -0.5); // P2
        glVertex3f(0.5, 0.5, -0.5); // P3
        glVertex3f(0.5, -0.5, -0.5); // P4
        
        // White side - BACK
        glBegin(GL_POLYGON);
        glColor3ub(1,133,113);//green dark
        glVertex3f(0.5, -0.5, 0.5);
        glVertex3f(0.5, 0.5, 0.5);
        glVertex3f(-0.5, 0.5, 0.5);
        glVertex3f(-0.5, -0.5, 0.5);
        glEnd();

        // Purple side - RIGHT
        glBegin(GL_POLYGON);
        glColor3ub(166,97,26);//green dark
        glVertex3f(0.5, -0.5, -0.5);
        glVertex3f(0.5, 0.5, -0.5);
        glVertex3f(0.5, 0.5, 0.5);
        glVertex3f(0.5, -0.5, 0.5);
        glEnd();

        // Green side - LEFT
        glBegin(GL_POLYGON);
        glColor3ub(223,194,125);
        glVertex3f(-0.5, -0.5, 0.5);
        glVertex3f(-0.5, 0.5, 0.5);
        glVertex3f(-0.5, 0.5, -0.5);
        glVertex3f(-0.5, -0.5, -0.5);
        glEnd();

        // Blue side - TOP
        glBegin(GL_POLYGON);
        glColor3ub(203,201,22);
        glVertex3f(0.5, 0.5, 0.5);
        glVertex3f(0.5, 0.5, -0.5);
        glVertex3f(-0.5, 0.5, -0.5);
        glVertex3f(-0.5, 0.5, 0.5);
        glEnd();

        // Red side - BOTTOM
        glBegin(GL_POLYGON);
        glColor3ub(106,81,163);
        glVertex3f(0.5, -0.5, -0.5);
        glVertex3f(0.5, -0.5, 0.5);
        glVertex3f(-0.5, -0.5, 0.5);
        glVertex3f(-0.5, -0.5, -0.5);
        glEnd();

        
        glFlush();
        glPopMatrix();
    } //unsafe
} //draw_digital_cammer
  /**************************************************** */
fn draw_collimator() {
    let radius = SIZE_UNIT;
    let start = -(OUTER_RADIOUS + 0.1);
    unsafe {
        glPushMatrix();
        glColor3ub(107, 174, 214); //Cyan blue

        //Trick
        glTranslatef(0.0, start as f32, 0.0);
        glRotatef(-90.0, 1.0, 0.0, 0.0);
        glScalef(0.016, 0.016, 0.016);

        let quadric = gluNewQuadric();
        //gluCylinder(	GLUquadric* quad,GLdouble base,GLdouble top,GLdouble height,GLint slices,GLint stacks);
        gluCylinder(quadric, radius as f64, radius as f64 * 0.5, 10.0, 15, 20);
        glPopMatrix();
    } //unsafe
} //draw_collimator

/**************************************************** */

