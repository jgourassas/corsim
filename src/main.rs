use fltk::*;
use fltk::{app::*, button::*, frame::*, valuator::*, window::*};
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

use std::collections::HashMap;

/********************************* */
const MARGIN_TOP: i32 = 50;
const MARGIN_BOTTOM: i32 = 100;
const MARGIN_RIGHT: i32 = 220;
const MARGIN_LEFT: i32 = 10;
const SIZE_UNIT: f32 = 2.5;


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




/* ************************************************/

/*************************************************************/
fn get_segments_names_92() -> Vec<String> {
    let segment_names: Vec<String> = vec![
        "LM".to_string(),
        "L1".to_string(),
        "L2".to_string(),
        "L3".to_string(),
        "L4".to_string(),
        "C1".to_string(),
        "C2".to_string(),
        "C3".to_string(),
        "C4".to_string(),
        "R1".to_string(),
        "R2".to_string(),
        "R3".to_string(),
        "R4".to_string(),
        "S1".to_string(),
        "S2".to_string(),
        "S3".to_string(),
        "D1".to_string(),
        "D2".to_string(),
        "D3".to_string(),
        "MR".to_string(),
        // "OM".to_string(),
        //"OA".to_string(),
        //"OP".to_string(),
        "M1".to_string(),
        "M2".to_string(),
        "M3".to_string(),
        "CP".to_string(),
        "CI".to_string(),
        //"CD".to_string(),
        "RD".to_string(),
        "RI".to_string(),
        "RP".to_string(),
    ];
    segment_names
} //get segments_names
/***********************************************************************8 */
fn get_segment_points_92(segment_name: &str) -> Vec<&str> {
    let mut segments = HashMap::new();
    segments.insert(String::from("LM"), vec!["LMp", "LMm", "LMd"]);
    segments.insert(String::from("L1"), vec!["LMd", "L1p", "L1m", "L1d"]);
    segments.insert(String::from("L2"), vec!["L1d", "L2p", "L2m", "L2d"]);
    segments.insert(String::from("L3"), vec!["L2d", "L3p", "L3m", "L3d"]);
    segments.insert(String::from("L4"), vec!["L3d", "L4p", "L4m", "L4d"]);

    segments.insert(String::from("C1"), vec!["LMd", "C1p", "C1m", "C1d"]);
    segments.insert(String::from("C2"), vec!["C1d", "C2p", "C2m", "C2d"]);
    segments.insert(String::from("C3"), vec!["C2d", "C3p", "C3m", "C3d"]);
    segments.insert(String::from("C4"), vec!["C3d", "C4p", "C4m", "C4d"]);

    segments.insert(String::from("R1"), vec!["R1p", "R1m", "R1d"]);
    segments.insert(String::from("R2"), vec!["R1d", "R2p", "R2m", "R2d"]);
    segments.insert(String::from("R3"), vec!["R2d", "R3p", "R3m", "R3d"]);
    segments.insert(String::from("R4"), vec!["R3d", "R4p", "R4m", "R4d"]);
    segments.insert(String::from("S1"), vec!["L1d", "S1o", "S1p", "S1m"]);
    segments.insert(String::from("S2"), vec!["L2m", "S2o", "S2p", "S2m"]);
    segments.insert(String::from("S3"), vec!["L2d", "S3o", "S3p", "S3m"]);
    segments.insert(String::from("D1"), vec!["L1d", "D1o", "D1p", "D1m"]);
    segments.insert(String::from("D2"), vec!["L2m", "D2o", "D2p", "D2m"]);
    segments.insert(String::from("D3"), vec!["L2d", "D3o", "D3p", "D3m"]);
    segments.insert(String::from("MR"), vec!["LMd", "MRo", "MRp", "MRm"]); //Ramus Medianus
    segments.insert(String::from("OM"), vec!["C1d", "OMo", "OMp", "OMm"]); //Obtuse marginal
    segments.insert(String::from("OA"), vec!["OMp", "OAo", "OAp", "OAm"]); //Anterior branch Obtuse marginal
    segments.insert(String::from("OP"), vec!["OMm", "OPo", "OPp", "OPm"]); //POSTERIOR branch Obtuse marginal
    segments.insert(String::from("M1"), vec!["C1d", "M1o", "M1p", "M1m"]); //M1 MARGINAL FROM LCX
    segments.insert(String::from("M2"), vec!["C2m", "M2o", "M2p", "M2m"]); //M2 MARGINAL FROM LCX
    segments.insert(String::from("M3"), vec!["C3p", "M3o", "M3p", "M3m"]); //M3 MARGINAL FROM LCX
                                                                           /* CP = Proximal most inferior wall branch arises from junction of C3 and C4,
                                                                            present in small-right, balanced, and left-dominant anatomy
                                                                           */
    segments.insert(String::from("CP"), vec!["C3d", "CPo", "CPp", "CPm"]);
    /*
       CI Inferior Inferior wall branch arises from C4, present only in balanced and
                  left-dominant anatomy
    */
    segments.insert(String::from("CI"), vec!["C4d", "CIo", "CIp", "CIm"]);

    /*
    CD Posterior descending Distal most inferior wall branch arises from C4, present only in
                              left-dominant anatomy
    */
    segments.insert(String::from("CD"), vec!["C4d", "CDo", "CDp", "CDm"]);
    /*
    RD Posterior descending Proximal most inferior wall branch arises from junction of R3 and R4,
                              present in right, small-right, and balanced-dominant anatomy

    */
    segments.insert(String::from("RD"), vec!["R3d", "RDo", "RDp", "RDm"]);
    /*
    RI Inferior Inferior wall branch arises from R4, present only in right and
                  small-right-dominant anatomy

    */
    segments.insert(String::from("RI"), vec!["R4m", "RIo", "RIp", "RIm"]);
    /*
    RP Posterior Distal most inferior wall branch arises from R4, present only in
                   right-dominant anatomy
    */
    segments.insert(String::from("RP"), vec!["R4d", "RPo", "RPp", "RPm"]);

    let segment = segment_name.trim().to_string();
    let point_names = segments.get(&segment);

    if let Some(point_names) = point_names {
        point_names.to_vec()
    } else {
        vec!["R", "R", "R"]
    }
}
/***************************************************/
//The parameters to glColor3f have to be floating point values in the range [0.0, 1.0],
//compare to glColor3ub, where the parameters are integral values in the range [0, 255].
//['rgb(215,25,28)','rgb(253,174,97)','rgb(166,217,106)','rgb(26,150,65)']
fn get_midpoint_color_92(point_name: &str) -> Vec<u8> {
    let mut colors = HashMap::new();
    colors.insert(String::from("LMp"), vec![215, 25, 28]);
    colors.insert(String::from("LMm"), vec![215, 25, 28]);
    colors.insert(String::from("LMd"), vec![215, 25, 28]);

    colors.insert(String::from("L1p"), vec![166, 217, 106]);
    colors.insert(String::from("L1m"), vec![166, 217, 106]);
    colors.insert(String::from("L1d"), vec![166, 217, 106]);

    colors.insert(String::from("L2p"), vec![166, 217, 106]);
    colors.insert(String::from("L2m"), vec![166, 217, 106]);
    colors.insert(String::from("L2d"), vec![166, 217, 106]);

    colors.insert(String::from("L3p"), vec![166, 217, 106]);
    colors.insert(String::from("L3m"), vec![166, 217, 106]);
    colors.insert(String::from("L3d"), vec![166, 217, 106]);

    colors.insert(String::from("L4p"), vec![166, 217, 106]);
    colors.insert(String::from("L4m"), vec![166, 217, 106]);
    colors.insert(String::from("L4d"), vec![166, 217, 106]);

    colors.insert(String::from("C1p"), vec![255, 255, 191]);
    colors.insert(String::from("C1m"), vec![255, 255, 191]);
    colors.insert(String::from("C1d"), vec![255, 255, 191]);

    colors.insert(String::from("C2p"), vec![255, 255, 191]);
    colors.insert(String::from("C2m"), vec![255, 255, 191]);
    colors.insert(String::from("C2d"), vec![255, 255, 191]);

    colors.insert(String::from("C3p"), vec![255, 255, 191]);
    colors.insert(String::from("C3m"), vec![255, 255, 191]);
    colors.insert(String::from("C3d"), vec![255, 255, 191]);

    colors.insert(String::from("C4p"), vec![255, 255, 191]);
    colors.insert(String::from("C4m"), vec![255, 255, 191]);
    colors.insert(String::from("C4d"), vec![255, 255, 191]);

    colors.insert(String::from("R1p"), vec![253, 174, 97]);
    colors.insert(String::from("R1m"), vec![253, 174, 97]);
    colors.insert(String::from("R1d"), vec![253, 174, 97]);

    colors.insert(String::from("R2p"), vec![253, 174, 97]);
    colors.insert(String::from("R2m"), vec![253, 174, 97]);
    colors.insert(String::from("R2d"), vec![253, 174, 97]);

    colors.insert(String::from("R3p"), vec![253, 174, 97]);
    colors.insert(String::from("R3m"), vec![253, 174, 97]);
    colors.insert(String::from("R3d"), vec![253, 174, 97]);

    colors.insert(String::from("R4p"), vec![253, 174, 97]);
    colors.insert(String::from("R4m"), vec![253, 174, 97]);
    colors.insert(String::from("R4d"), vec![253, 174, 97]);

    colors.insert(String::from("S1p"), vec![166, 217, 106]);
    colors.insert(String::from("S1m"), vec![166, 217, 106]);
    colors.insert(String::from("S1d"), vec![166, 217, 106]);

    colors.insert(String::from("S2p"), vec![166, 217, 106]);
    colors.insert(String::from("S2m"), vec![166, 217, 106]);
    colors.insert(String::from("S2d"), vec![166, 217, 106]);

    colors.insert(String::from("S3p"), vec![166, 217, 106]);
    colors.insert(String::from("S3m"), vec![166, 217, 106]);
    colors.insert(String::from("S3d"), vec![166, 217, 106]);

    colors.insert(String::from("D1p"), vec![166, 217, 106]);
    colors.insert(String::from("D1m"), vec![166, 217, 106]);
    colors.insert(String::from("D1d"), vec![166, 217, 106]);

    colors.insert(String::from("D2p"), vec![166, 217, 106]);
    colors.insert(String::from("D2m"), vec![166, 217, 106]);
    colors.insert(String::from("D2d"), vec![215, 25, 28]);

    colors.insert(String::from("D3p"), vec![166, 217, 106]);
    colors.insert(String::from("D3m"), vec![166, 217, 106]);
    colors.insert(String::from("D3d"), vec![215, 25, 28]);

    colors.insert(String::from("MRp"), vec![203, 201, 226]);
    colors.insert(String::from("MRm"), vec![203, 201, 226]);
    colors.insert(String::from("MRd"), vec![203, 201, 226]);

    colors.insert(String::from("M1p"), vec![255, 255, 191]);
    colors.insert(String::from("M1m"), vec![255, 255, 191]);
    colors.insert(String::from("M1d"), vec![255, 255, 191]);

    colors.insert(String::from("M2p"), vec![255, 255, 191]);
    colors.insert(String::from("M2m"), vec![255, 255, 191]);
    colors.insert(String::from("M2d"), vec![255, 255, 191]);

    colors.insert(String::from("M3p"), vec![255, 255, 191]);
    colors.insert(String::from("M3m"), vec![255, 255, 191]);
    colors.insert(String::from("M3d"), vec![255, 255, 191]);

    colors.insert(String::from("CPp"), vec![171, 217, 233]);
    colors.insert(String::from("CPm"), vec![171, 217, 233]);
    colors.insert(String::from("CPd"), vec![171, 217, 233]);

    colors.insert(String::from("CIo"), vec![215, 25, 28]);
    colors.insert(String::from("CIp"), vec![215, 25, 28]);
    colors.insert(String::from("CIm"), vec![215, 25, 28]);

    colors.insert(String::from("RDo"), vec![253, 174, 97]);
    colors.insert(String::from("RDp"), vec![253, 174, 97]);
    colors.insert(String::from("RDm"), vec![253, 174, 97]);

    colors.insert(String::from("RIo"), vec![215, 25, 28]);
    colors.insert(String::from("RIp"), vec![253, 174, 97]);
    colors.insert(String::from("RIm"), vec![215, 25, 28]);

    colors.insert(String::from("RPo"), vec![253, 174, 97]);
    colors.insert(String::from("RPp"), vec![253, 174, 97]);
    colors.insert(String::from("RPm"), vec![253, 174, 97]);

    let point = point_name.trim().to_string();
    let color = colors.get(&point);

    if let Some(color) = color {
        color.to_vec()
    } else {
        vec![166, 97, 26]
    }
    /*
        let point = point_name.trim().to_string();
        let color_vec = colors.get(&point);

        if let Some(color_vec) = color_vec {
            color_vec.to_vec()
        } else {
            vec!(166.0, 97.0, 26.0)
           }
    */
}
/********************************************************** */
// Fom dodge 92  RCA Dominant
// midpoints_diameters
fn get_midpoint_92(point_name: &str) -> Vec<f32> {
    let mut points = HashMap::new();
    points.insert(
        String::from("LMp"),
        vec![
            0.29554435894139947,
            0.03656080302154424,
            0.036288284339950155,
        ],
    );
    points.insert(
        String::from("LMm"),
        vec![0.6876861675754944, 0.04882953162088771, 0.12125762560356244],
    );
    points.insert(
        String::from("LMd"),
        vec![1.0797899017924304, 0.0, 0.20988989491419943],
    );
    points.insert(
        String::from("L1p"),
        vec![1.5014476875323806, -0.08373752998871013, 0.5464822665696085],
    );
    points.insert(
        String::from("L1m"),
        vec![2.041516743331695, -0.20045820831961378, 1.0402047362964846],
    );
    points.insert(
        String::from("L1d"),
        vec![2.5617165610111714, -0.2614672282429745, 1.5392345986223264],
    );
    points.insert(
        String::from("L2p"),
        vec![3.0703120202865937, -0.6257378601609235, 2.486289650954789],
    );
    points.insert(
        String::from("L2m"),
        vec![3.601235617488854, -1.1019319613341243, 3.7291886490120087],
    );
    points.insert(
        String::from("L2d"),
        vec![3.911843539405599, -1.900416080697789, 4.830724463621986],
    );
    points.insert(
        String::from("L3p"),
        vec![4.010743000887889, -3.1258490279141897, 6.17600262593182],
    );
    points.insert(
        String::from("L3m"),
        vec![4.282642885589951, -5.352184568755369, 7.4177550685151825],
    );
    points.insert(
        String::from("L3d"),
        vec![4.655596252298606, -7.584893794301164, 7.748213320808538],
    );
    points.insert(
        String::from("L4p"),
        vec![4.490976801318241, -8.775945564131543, 7.187065244271911],
    );
    points.insert(
        String::from("L4m"),
        vec![4.21632120625497, -8.90557304662871, 6.492565301700468],
    );
    points.insert(
        String::from("L4d"),
        vec![3.924718218543799, -8.98332259111663, 5.81863404322389],
    );
    points.insert(
        String::from("C1p"),
        vec![1.3660708352759223, -0.291076367144863, 0.09552497839985384],
    );
    points.insert(
        String::from("C1m"),
        vec![
            1.6403761922803517,
            -0.7321259575364403,
            -0.11470627751419989,
        ],
    );
    points.insert(
        String::from("C1d"),
        vec![1.8445595049403363, -1.1330837648021193, -0.3920732261658757],
    );
    points.insert(
        String::from("C2p"),
        vec![2.085264097312316, -1.4160614910390705, -0.6375292134925724],
    );
    points.insert(
        String::from("C2m"),
        vec![2.2252074089401765, -1.8656265717713496, -1.085306169658326],
    );
    points.insert(
        String::from("C2d"),
        vec![2.3529516082286777, -2.361812504365826, -1.3584772444009892],
    );
    points.insert(
        String::from("C3p"),
        vec![2.3454517515525057, -2.8991378028648445, -1.7040704448875588],
    );
    points.insert(
        String::from("C3m"),
        vec![2.091484565436457, -3.652586018847763, -2.0914845654364567],
    );
    points.insert(
        String::from("C3d"),
        vec![1.9003220820963247, -4.476802891797226, -2.346702335681518],
    );
    points.insert(
        String::from("C4p"),
        vec![0.9058706050175953, -5.24687766467891, -2.0346186912683786],
    );
    points.insert(
        String::from("C4m"),
        vec![0.4646961839316486, -5.578559596035969, -1.8637945943999221],
    );
    points.insert(
        String::from("C4d"),
        vec![1.6005231360110088, -5.9290894869708195, -0.8510132470094607],
    );
    points.insert(
        String::from("R1p"),
        vec![-0.43833477874433646, -0.187303296707956, 0.1509307681868909],
    );
    points.insert(
        String::from("R1m"),
        vec![-1.439941817856379, -0.6914522932288603, 0.5817742580907276],
    );
    points.insert(
        String::from("R1d"),
        vec![-2.0569992919589706, -1.2999999999999998, 0.9158350904394823],
    );
    points.insert(
        String::from("R2p"),
        vec![-2.466796077701761, -2.139689329569447, 0.9469142959961353],
    );
    points.insert(
        String::from("R2m"),
        vec![-2.927352707778549, -3.418466197196013, 0.9511545525779821],
    );
    points.insert(
        String::from("R2d"),
        vec![-3.129703972901258, -4.587251448018354, 0.7225490949822015],
    );
    points.insert(
        String::from("R3p"),
        vec![-2.894778440348643, -5.474275075725346, 0.30425347423641175],
    );
    points.insert(
        String::from("R3m"),
        vec![-2.293205557573059, -5.974914729582092, -0.04002805190118708],
    );
    points.insert(
        String::from("R3d"),
        vec![0.5069981483899437, -6.407241864952337, 1.8921428491346153],
    );
    points.insert(
        String::from("R4p"),
        vec![1.3811897602375875, -6.24042259895549, -1.6460378588174798],
    );
    points.insert(
        String::from("R4m"),
        vec![-0.04401090588894932, -5.663981837384125, -2.521383109712917],
    );
    points.insert(
        String::from("R4d"),
        vec![0.5282419597121412, -5.1997575212616285, -3.3351884732474417],
    );

    /******************************BRANCHES*********************************************/
    points.insert(
        String::from("S1o"),
        vec![2.7335389671479917, -0.5162337346327618, 1.7751809615671865],
    );
    points.insert(
        String::from("S1p"),
        vec![2.7105161936648146, -1.081775307474126, 2.274393138404191],
    );
    points.insert(
        String::from("S1m"),
        vec![2.114099152481461, -2.2146637221132326, 3.0192464906715926],
    );
    points.insert(
        String::from("S2o"),
        vec![3.465339933810029, -1.0603496231705725, 3.588464549049291],
    );
    points.insert(
        String::from("S2p"),
        vec![3.240034927366695, -1.6377900701872212, 3.8613232648199793],
    );
    points.insert(
        String::from("S2m"),
        vec![2.6126822114121246, -2.811895797428755, 4.348233398355378],
    );
    points.insert(
        String::from("S3o"),
        vec![3.9262882559084624, -2.246420265754381, 5.210360498192356],
    );
    points.insert(
        String::from("S3p"),
        vec![3.603178471626892, -2.735117899424916, 5.341931764619418],
    );
    points.insert(
        String::from("S3m"),
        vec![3.1286606475627097, -3.7597779468433954, 5.418999201219959],
    );
    points.insert(
        String::from("D1o"),
        vec![2.771545498471191, -0.44535392307220945, 1.536292756315758],
    );
    points.insert(
        String::from("D1p"),
        vec![3.5792536949729974, -0.9447944282442331, 1.984012720569301],
    );
    points.insert(
        String::from("D1m"),
        vec![4.991568449151188, -2.422532996633497, 2.766871572310565],
    );
    points.insert(
        String::from("D2o"),
        vec![3.505322155780238, -1.0187672850070206, 3.2687657920376627],
    );
    points.insert(
        String::from("D2p"),
        vec![4.147831567340345, -1.626260399320295, 3.8679155113028947],
    );
    points.insert(
        String::from("D2m"),
        vec![5.308853618159379, -3.131872151683661, 4.614916043273639],
    );
    points.insert(
        String::from("D3o"),
        vec![4.1180648163315725, -2.0395121628746526, 4.737291663596232],
    );
    points.insert(
        String::from("D3p"),
        vec![4.532380623883003, -2.651922826635222, 5.213907479215658],
    );
    points.insert(
        String::from("D3m"),
        vec![5.516784590489722, -3.9453403211016966, 5.916027174787512],
    );
    points.insert(
        String::from("MRo"),
        vec![1.1772723146284882, -0.2083778132003164, 0.10299798141291751],
    );
    points.insert(
        String::from("MRp"),
        vec![2.5336929727108926, -0.9234543869793056, 0.13278522208482535],
    );
    points.insert(
        String::from("MRm"),
        vec![4.572078722347505, -3.1314802594361826, 0.806180836672952],
    );
    points.insert(
        String::from("OMp"),
        vec![2.342245650465862, -1.3999999999999997, -0.6276028305176377],
    );
    points.insert(
        String::from("OMm"),
        vec![2.8384734967392573, -1.8547174248162173, -0.8678084364604395],
    );
    points.insert(
        String::from("OMd"),
        vec![3.1720338774402985, -2.4099195343991395, -0.9697880789597789],
    );
    points.insert(
        String::from("OAo"),
        vec![3.3604513207724396, -2.6479861018690127, -1.027393072308413],
    );
    points.insert(
        String::from("OAp"),
        vec![3.79147090319087, -3.6145913083312418, -0.8059020188541839],
    );
    points.insert(
        String::from("OAm"),
        vec![4.632501989794268, -5.358438019581681, 0.48689557900405106],
    );
    points.insert(
        String::from("OPo"),
        vec![3.3604513207724396, -2.6479861018690127, -1.027393072308413],
    );
    points.insert(
        String::from("OPp"),
        vec![3.2738117750170477, -3.3417919643062426, -1.4575949125384446],
    );
    points.insert(
        String::from("OPm"),
        vec![3.407718886629475, -5.031403713297944, -1.6620555536805697],
    );
    points.insert(
        String::from("M1o"),
        vec![1.8445595049403363, -1.1330837648021193, -0.3920732261658757],
    );
    points.insert(
        String::from("M1p"),
        vec![3.0335387554850115, -1.6431504697506178, -0.5896602010724263],
    );
    points.insert(
        String::from("M1m"),
        vec![4.412188120211939, -3.4477042618236857, -0.07701503011380952],
    );
    points.insert(
        String::from("M2o"),
        vec![2.265654624361911, -2.2306006985677245, -1.204669932660672],
    );
    points.insert(
        String::from("M2p"),
        vec![2.841286757641696, -3.0007927842749935, -1.5107389670929507],
    );
    points.insert(
        String::from("M2m"),
        vec![3.570204718446285, -4.6628757687418245, -1.2293200696581095],
    );
    points.insert(
        String::from("M3o"),
        vec![2.36208075552619, -3.5471350270470285, -1.9820210907729772],
    );
    points.insert(
        String::from("M3p"),
        vec![2.5358317946220725, -4.176456994115626, -2.053476098153194],
    );
    points.insert(
        String::from("M3m"),
        vec![3.041291101967833, -5.512312625016769, -1.6170831633444278],
    );
    points.insert(
        String::from("CPo"),
        vec![1.275159295210584, -5.392764277795003, -2.3004482641381436],
    );
    points.insert(
        String::from("CPp"),
        vec![1.2312963651384998, -5.983281547440861, -2.2213174436019734],
    );
    points.insert(
        String::from("CPm"),
        vec![1.419210485854215, -7.185941174554807, -2.026842626520203],
    );
    points.insert(
        String::from("CPo"),
        vec![1.275159295210584, -5.392764277795003, -2.3004482641381436],
    );
    points.insert(
        String::from("CIo"),
        vec![2.2629359004682725, -5.983281547440861, -1.1530234318631762],
    );
    points.insert(
        String::from("CIp"),
        vec![2.235157267734093, -6.535062985480412, -1.138869511532953],
    );
    points.insert(
        String::from("CIm"),
        vec![2.0457124380835676, -7.6504380477042835, -1.1339569210992335],
    );
    points.insert(
        String::from("CDo"),
        vec![-1.6152216849229866, -6.440453362786736, -0.8953320000556038],
    );
    points.insert(
        String::from("CDp"),
        vec![1.9398856050425095, -7.11333654994356, 0.6303070415169999],
    );
    points.insert(
        String::from("CDm"),
        vec![-0.7806259699282441, -8.592888563177699, 1.1148494229641517],
    );
    points.insert(
        String::from("RDo"),
        vec![1.8921428491346153, -6.407241864952337, -0.506998148389944],
    );
    points.insert(
        String::from("RDp"),
        vec![1.5342990137617705, -6.820590453496647, -0.35422083795291515],
    );
    points.insert(
        String::from("RDm"),
        vec![1.8106738439157182, -7.3742475196975725, 0.31927065194544907],
    );
    points.insert(
        String::from("RIo"),
        vec![0.19951329116353397, -5.613341102386717, -2.853172990707883],
    );
    points.insert(
        String::from("RIp"),
        vec![0.5137082370002675, -6.3441545092565494, -2.9133841850874838],
    );
    points.insert(
        String::from("RIm"),
        vec![1.7133734552402526, -7.046597294707584, -2.2737233712784475],
    );
    points.insert(
        String::from("RPo"),
        vec![0.8136827674910369, -4.905612271448018, -4.186034949221846],
    );
    points.insert(
        String::from("RPp"),
        vec![1.0872765354568406, -5.752478501329071, -4.360827997891206],
    );
    points.insert(
        String::from("RPm"),
        vec![2.565918982444365, -6.632300580440334, -3.6645120802396716],
    );

    let point = point_name.trim().to_string();
    let point_vec = points.get(&point);

    if let Some(point_vec) = point_vec {
        point_vec.to_vec()
    } else {
        vec![0.0, 0.0, 0.0]
    }
}
fn get_diameter(point_name: &str) -> f32 {
    let mut diameters = HashMap::new();

    diameters.insert(String::from("LMp"), 4.6);
    diameters.insert(String::from("LMm"), 4.5);
    diameters.insert(String::from("LMd"), 4.5);

    diameters.insert(String::from("L1p"), 3.7);
    diameters.insert(String::from("L1m"), 3.6);
    diameters.insert(String::from("L1d"), 3.5);

    diameters.insert(String::from("L2p"), 2.9);
    diameters.insert(String::from("L2m"), 2.5);
    diameters.insert(String::from("L2d"), 2.3);

    diameters.insert(String::from("L3p"), 2.0);
    diameters.insert(String::from("L3m"), 1.7);
    diameters.insert(String::from("L3d"), 1.4);

    diameters.insert(String::from("L4p"), 1.4);
    diameters.insert(String::from("L4m"), 1.1);
    diameters.insert(String::from("L4d"), 0.9);

    diameters.insert(String::from("C1p"), 3.4);
    diameters.insert(String::from("C1m"), 3.4);
    diameters.insert(String::from("C1d"), 3.3);

    diameters.insert(String::from("C2p"), 2.8);
    diameters.insert(String::from("C2m"), 2.8);
    diameters.insert(String::from("C2d"), 2.7);

    diameters.insert(String::from("C3p"), 1.7);
    diameters.insert(String::from("C3m"), 1.6);
    diameters.insert(String::from("C3d"), 1.3);

    diameters.insert(String::from("C4p"), 1.2); // Is Empty in the Paper
    diameters.insert(String::from("C4m"), 1.1); // Is Empty in the Paper
    diameters.insert(String::from("C4d"), 1.0); // Is Empty in the Paper

    diameters.insert(String::from("R1p"), 4.0);
    diameters.insert(String::from("R1m"), 3.9);
    diameters.insert(String::from("R1d"), 3.8);

    diameters.insert(String::from("R2p"), 3.5);
    diameters.insert(String::from("R2m"), 3.4);
    diameters.insert(String::from("R2d"), 3.2);

    diameters.insert(String::from("R3p"), 3.2);
    diameters.insert(String::from("R3m"), 3.1);
    diameters.insert(String::from("R3d"), 3.1);

    diameters.insert(String::from("R4p"), 2.4);
    diameters.insert(String::from("R4m"), 2.2);
    diameters.insert(String::from("R4d"), 1.9);

    diameters.insert(String::from("S1o"), 1.8);
    diameters.insert(String::from("S1p"), 1.8);
    diameters.insert(String::from("S1m"), 1.1);
    diameters.insert(String::from("S2o"), 1.9);
    diameters.insert(String::from("S2p"), 1.7);
    diameters.insert(String::from("S2m"), 1.4);
    diameters.insert(String::from("S3o"), 1.4);
    diameters.insert(String::from("S3p"), 1.3);
    diameters.insert(String::from("S3m"), 1.2);
    diameters.insert(String::from("D1o"), 2.4);
    diameters.insert(String::from("D1p"), 2.3);
    diameters.insert(String::from("D1m"), 1.8);
    diameters.insert(String::from("D2o"), 2.6);
    diameters.insert(String::from("D2p"), 2.4);
    diameters.insert(String::from("D2m"), 1.9);
    diameters.insert(String::from("D3o"), 2.4);
    diameters.insert(String::from("D3p"), 2.3);
    diameters.insert(String::from("D3m"), 1.7);
    diameters.insert(String::from("MRo"), 2.7);
    diameters.insert(String::from("MRp"), 2.4);
    diameters.insert(String::from("MRm"), 2.0);
    diameters.insert(String::from("OMp"), 3.3);
    diameters.insert(String::from("OMm"), 3.0);
    diameters.insert(String::from("OMd"), 2.7);
    diameters.insert(String::from("OAo"), 2.5);
    diameters.insert(String::from("OAp"), 2.4);
    diameters.insert(String::from("OAm"), 2.0);
    diameters.insert(String::from("OPo"), 2.5);
    diameters.insert(String::from("OPp"), 2.3);
    diameters.insert(String::from("OPm"), 1.9);
    diameters.insert(String::from("M1o"), 2.6);
    diameters.insert(String::from("M1p"), 2.5);
    diameters.insert(String::from("M1m"), 2.0);
    diameters.insert(String::from("M2o"), 2.5);
    diameters.insert(String::from("M2p"), 2.3);
    diameters.insert(String::from("M2m"), 1.9);
    diameters.insert(String::from("M3o"), 2.8);
    diameters.insert(String::from("M3p"), 2.6);
    diameters.insert(String::from("M3m"), 2.3);
    diameters.insert(String::from("CPo"), 2.6);
    diameters.insert(String::from("CPp"), 2.1);
    diameters.insert(String::from("CPm"), 1.7);
    diameters.insert(String::from("CPo"), 2.6);
    diameters.insert(String::from("CIo"), 0.1);
    diameters.insert(String::from("CIp"), 0.1);
    diameters.insert(String::from("CIm"), 0.1);
    diameters.insert(String::from("CDo"), 3.2);
    diameters.insert(String::from("CDp"), 3.1);
    diameters.insert(String::from("CDm"), 2.5);
    diameters.insert(String::from("RDo"), 2.4);
    diameters.insert(String::from("RDp"), 2.3);
    diameters.insert(String::from("RDm"), 2.0);
    diameters.insert(String::from("RIo"), 2.1);
    diameters.insert(String::from("RIp"), 1.9);
    diameters.insert(String::from("RIm"), 1.5);
    diameters.insert(String::from("RPo"), 2.6);
    diameters.insert(String::from("RIp"), 2.4);
    diameters.insert(String::from("RIm"), 2.3);

    let point = point_name.trim().to_string();
    let diameter = diameters.get(&point);

    if let Some(diameter) = diameter {
        *diameter
    } else {
        0.0
    }

    // *diameter.unwrap()
} //get_diameter
/*
#[derive(Debug)]
pub struct Segment {
    name: String,
    pos: Vec<f64>,
    diameter: f32,
}
*/
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

/*
fn get_data_92() {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/dodge_92.csv")
        .expect("Cannot read field");

    for result in reader.records() {
        let record = result.unwrap();
        let segment_name: String = record[0].trim().to_string();

        let rho_str: String = record[5].trim().to_string();
        let theta_str: String = record[6].trim().to_string();
        let phi_str: String = record[7].trim().to_string();

        let rho: f64 = rho_str.parse().unwrap();
        let theta: f64 = theta_str.parse().unwrap();
        let phi: f64 = phi_str.parse().unwrap();

        // println!("seg:{}, size:{},  point:{}", segment_name, size, point);
        angle2pos(segment_name, rho, theta, phi);
    } //records
      // println!("vessel_vec:{:?}", vessels_vec);
      //println!("vessels_data{:?}", vessels_data);
} //read data
*/

/*
//"SEGMENT_NAME","MEDIUM_LEN", "LONG_LEN", "RHO", "THETA",  "PHI"
fn get_branch_data_92() {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/branches_spacial_loc_diameters92.csv")
        .expect("Cannot read field");

    for result in reader.records() {
        let record = result.unwrap();
        let segment_name: String = record[0].trim().to_string();
        let medium_diameter: f32 = record[2].parse::<f32>().unwrap();

        let rho_str: String = record[3].trim().to_string();
        let theta_str: String = record[4].trim().to_string();
        let phi_str: String = record[5].trim().to_string();

        let rho: f64 = rho_str.parse().unwrap();
        let theta: f64 = theta_str.parse().unwrap();
        let phi: f64 = phi_str.parse().unwrap();

        //"R1".to_string(),
        // println!("{:?}.to_string();", segment_name);

        //println!("diameters.insert(String::from({:?}), {:?});", segment_name, medium_diameter);
        // println!("seg:{}, size:{},  point:{}", segment_name, size, point);
        angle2pos(segment_name, rho, theta, phi);
    } //records
      // println!("vessel_vec:{:?}", vessels_vec);
      //println!("vessels_data{:?}", vessels_data);
} //read data
  /******************************************** */

*/



/*
fn angle2pos(segment: String, radius_arg: f64, theta_arg: f64, phi_arg: f64) {
    let theta = std::f64::consts::PI * theta_arg / 180.0;
    let phi = std::f64::consts::PI * phi_arg / 180.0;
    let pos_x = radius_arg * (phi.cos() * theta.sin());
    let pos_y = radius_arg * phi.sin();
    let pos_z = radius_arg * (phi.cos() * theta.cos());

    /*
    let a_segment = Segment{
        name: segment.to_string(),
        pos: vec!(pos_x, pos_y, pos_z),
        diameter: get_diameter(&segment),
    };
    */

    /*
        println!("structure segm: {:?}", a_segment);
    */

    //   println!(
    //       "seg{:?}, glVertex3f({:?},{:?},{:?});",
    //       segment, pos_x, pos_y, pos_z
    //   );

    //print!("{:?},", segment);
    //println!("{:?}, {:?},{:?},{:?};", segment, pos_x, pos_y, pos_z);
    println!(
        "points.insert(String::from({:?}), vec![{:?}, {:?}, {:?}]);",
        segment, pos_x, pos_y, pos_z
    );

    //println!("{:?}, {:?},{:?},{:?};", segment, pos_x, pos_y, pos_z);
} //angle2pos
*/

