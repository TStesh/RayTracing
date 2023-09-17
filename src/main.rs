use std::rc::Rc;

use raytracing::hitrecord::HittableList;
use raytracing::sphere::Sphere;
use raytracing::vec3::Vec3;
use raytracing::render;
use raytracing::raster::interpolate;

fn main() {
    let start = std::time::Instant::now();

    // World
    let s1 = Sphere::from(Vec3::from(0., 0., -1.), 0.5);
    let s2 = Sphere::from(Vec3::from(0., -100.5, -1.), 100.);

    let mut world = HittableList::new(Rc::new(s1));
    world.add(Rc::new(s2));

    // Render
    // render("C:\\Users\\alexa\\Downloads\\output", true, &world);

    let xs = interpolate((-50, -200), (60, 240));
    println!("{:?}", xs);

    println!("duration: {:?}", start.elapsed());
}
