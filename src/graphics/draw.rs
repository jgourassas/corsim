use glu_sys::*;


extern crate nalgebra as na;
use na::Point3;

extern crate ncollide3d; // If you need 3D.
use ncollide2d::procedural::circle;
use ncollide3d::shape::Polyline;

pub use crate::graphics::data_gets::{
    get_diameter,
    get_midpoint_92,
    get_midpoint_color_92,
    get_nomeclature,
    //get_nomeclature_data,
    // get_rca_segments_names_92, get_left_segments_names_92,
    get_optimal_views,
    get_segment_points_92,
    get_segments_names_92,
    optimal_angles,
    
};
const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;


const SIZE_UNIT: f32 = 2.5;
const OUTER_RADIOUS: f64 = 0.85;
pub fn setup_gl(){
    unsafe{
        glClearColor(0.0, 0.0, 0.0, 0.0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();

        glEnable(GL_DEPTH_TEST);    // Enable depth buffering
        glDepthFunc(GL_LEQUAL);     // Useful for multipass shaders
    
        // Set polygon drawing mode for front and back of each triangle
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
        // glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
        // glPolygonMode(GL_FRONT_AND_BACK, GL_POINT);
    
        // Disable backface culling to render both sides of polygons
        // glDisable(GL_CULL_FACE);
    
        // The following commands should induce OpenGL to create round points and 
        //  antialias points and lines.  (This is implementation dependent unfortunately.)
        glEnable(GL_POINT_SMOOTH);
        glEnable(GL_LINE_SMOOTH);
        glHint(GL_POINT_SMOOTH_HINT, GL_NICEST);    // Make round points, not square points
        glHint(GL_LINE_SMOOTH_HINT, GL_NICEST);     // Antialias the lines
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

    }
    
}
pub fn setup_gl_old() {
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
/***************************************/

pub fn draw_scene(rotate_rao_lao: &f32, rotate_cr_ca: &f32) {
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

/***********************************/
pub fn draw_segment_92(
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

/*********************************/
pub fn draw_as_polyline_segment(
    points: &[Point3<f32>],
    color: &Vec<u8>,
    diameters: &Vec<f32>,
    midpoint_names: &Vec<&str>,
) {
    
    
 unsafe{
    

   let points_vec = points.to_vec();
  
     

   let mut i = 0;
   glPushMatrix(); //start 2
   glColor3ub(color[0], color[1], color[2]);

   while  i < points_vec.len() {
    glLineWidth(diameters[i] * SIZE_UNIT * 0.7);  

    glBegin(GL_LINE_STRIP);   
    glVertex3f(
          points_vec[i][0],
          points_vec[i][1],
          points_vec[i][2] );
    i = i+1;
   

}//while 

glEnd();
glPopMatrix();
/*************************************** */

let mut j = 0;
let mut vertices: Vec<f32> = Vec::new();

while  j < points_vec.len() {
    vertices.push(points_vec[j][0] );
    vertices.push(points_vec[j][1] );
    vertices.push(points_vec[j][2] );
      j = j+1;

}
 
println!("vertices: {:?} ", vertices);
    /*
    vertices: 
    [Point { coords: Matrix { data: [0.52824193, -5.1997576, -3.3351884] } }, 
    Point { coords: Matrix { data: [0.8136828, -4.9056125, -4.186035] } }, 
    Point { coords: Matrix { data: [1.0872766, -5.7524786, -4.360828] } }, 
    Point { coords: Matrix { data: [2.565919, -6.6323004, -3.6645122] } }] 
*/
/**************************************** */
   //let (mut VBO, mut VAO) = (0, 0);
   //gl::GenVertexArrays(1, &mut VAO);
   //gl::GenBuffers(1, &mut VBO);

 // let VAO: u32;
  //glReadBuffer(1, &VBO);
  

 
}//unsafe


}//fn
/**************************************/
pub fn draw_as_polyline_segment_old(
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

    

    /**********************************************************/
} //draw_spine
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


/************************************************** */

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

