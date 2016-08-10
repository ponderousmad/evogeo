struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

fn main() {
    let origin = Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }; // origin: Point

    println!("The origin is at ({}, {}, {}, {})", origin.x, origin.y, origin.z, origin.w);
}
