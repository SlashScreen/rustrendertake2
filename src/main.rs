//raytraces an image

//Include
extern crate image;
extern crate piston_window;
extern crate bvh;

use piston_window::EventLoop;
use bvh::ray::Ray;
use bvh::nalgebra::{Point3,Vector3};


//Const
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
//STRUCTS
//Point - a holder for x,y,z.
#[allow(dead_code)]
struct Point
{
    x:f32,
    y:f32,
    z:f32
}
//Rotator - A holder for roll, pitch, and yaw.
#[allow(dead_code)]
struct Rotator
{
    pitch:f32,
    yaw:f32,
    roll:f32
}
//Triangle - A collection of 3 points.
#[allow(dead_code)]
struct Triangle 
{
    //Points
    v1:Point,
    v2:Point,
    v3:Point
}
impl Triangle{
    fn hits_ray (&self,ray:&bvh::ray::Ray) -> (bool,Point){
        //hits ray
        //arguments are self, bvh ray object, returns bool, Point object
        //vars
        let mut hit = false;
        let points = &self.to_points();
        //calc intersection
        let res = ray.intersects_triangle(&points.0,&points.1,&points.2);
        //if distance is not +INFINITY, it hit the triangle
        if res.distance.is_finite(){
            hit = true;
        }
        //Calculate position
        let space_pos = ray.origin + (res.distance * ray.direction);
        let pos = Point{x:space_pos.x,y:space_pos.y,z:space_pos.z};
        return (hit,pos);
    }
    fn to_points(&self) -> (Point3<f32>,Point3<f32>,Point3<f32>){
        //converts internal point types to point3
        //takes self as argument, returns tuple with 3 Point3
        return(Point3::new(self.v1.x,self.v1.y,self.v1.z),Point3::new(self.v2.x,self.v2.y,self.v2.z),Point3::new(self.v3.x,self.v3.y,self.v3.z));
    }
}
//Transform - A holder for translation  and rotation.
#[allow(dead_code)]
struct Transform 
{
    rotation:Rotator,
    translation:Point
}
//Mesh - A collection of triangles with a transform.
#[allow(dead_code)]
struct Mesh 
{
    tris:Vec<Triangle>, //Array full of Triangle objects
    transform:Transform,
}
impl Mesh
{
    fn transform_point(&self,p:&Point)->Point{
        let ox = p.x + self.transform.translation.x;
        let oy = p.y + self.transform.translation.y;
        let oz = p.z + self.transform.translation.z;
        return Point{x:ox,y:oy,z:oz};
    }
    fn transform_triangles(&self)->Vec<Triangle>{
        let mut out = Vec::new();
        for t in &self.tris{
            let n1 = self.transform_point(&t.v1);
            let n2 = self.transform_point(&t.v2);
            let n3 = self.transform_point(&t.v3);
            out.push(Triangle{v1:n1,v2:n2,v3:n3});
        }
        return out;
    }
}
//Camera - The camera object.
#[allow(dead_code)]
struct Camera 
{
    origin:Point3<f32>, //Point as point object.
    rotation:Rotator, //Rotation as rotator object.
    fov:u8, //FOV in degrees theta.
    perspective:bool //If perspective or ortho.
}
//FUNCTIONS
#[allow(dead_code)]
fn render_pixel(u:f32, v:f32,c:&Camera,meshes:&Vec<Mesh>) -> image::Rgba<u8> {
    //takes f32 u and v as arguments. Returns a color in RGBA.
    //creates a ray based on camera and UV position, and gets the color under that ray.
    let r = Ray::new(c.origin,Vector3::new(v,u,1.0));

    for mesh in meshes.iter(){
        for tri in mesh.tris.iter(){
            if tri.hits_ray(&r).0{
                return image::Rgba([(u*255.0) as u8,(v*255.0) as u8,0,255]); //if hit return UV for test.
            }
        }
    }
    return image::Rgba([0,0,0,255]); //if nothing hit return black.
}

fn main() {
    //buffer
    let mut frame_buffer = image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgba([0,0,0,255]));
    //camera
    let cam = Camera{origin:Point3::new(0.0,0.0,-1.0),rotation:Rotator{roll:0.0,pitch:0.0,yaw:0.0},fov:90,perspective:true};
    let plane = Mesh 
    {   
        tris : vec![
            Triangle
            {
                v1:Point{x:0.0,y:0.0,z:0.0},
                v2:Point{x:0.0,y:1.0,z:0.0},
                v3:Point{x:1.0,y:1.0,z:0.0}
            }
        ],
        transform:Transform{
            rotation:Rotator{roll:0.0,pitch:0.0,yaw:0.0},
            translation:Point{x:0.0,y:0.0,z:0.0}
        }
    };
    let mut world_meshes = Vec::new();
    world_meshes.push(plane);
    //Render loop
    for x in 0..WIDTH{
        let u = x as f32/WIDTH as f32;
        for y in 0..HEIGHT{
            let v = y as f32/HEIGHT as f32;
            frame_buffer.put_pixel(x, y, render_pixel(u, v,&cam,&world_meshes));
        }
    }
    //Window stuff
    let mut window: piston_window::PistonWindow =
    piston_window::WindowSettings::new("Raytracer", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|_e| { panic!("Could not create window!")});

    let tex = piston_window::Texture::from_image(
        &mut window.create_texture_context(),
        &frame_buffer,
        &piston_window::TextureSettings::new())
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0; 4], g);
            piston_window::image(&tex, c.transform, g)
        });
    }
}
