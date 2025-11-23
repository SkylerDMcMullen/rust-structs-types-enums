enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}


fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(4.0),
        Shape::Triangle(3.0, 2.0),
    ];

    fn area(shape: &Shape) -> f64 {
        match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(height, base) => 0.5 * base * height,
        }
    }

    match shapes
        .iter()
        .map(|s| (s, area(s)))
        // max_by() is an iterator adaptor that returns the maximum element 
        // according to a custom comparator. Result is Option<T>.
        // |a, b| is the comparator closure
        .max_by(|a, b| a.1.total_cmp(&b.1))
    {
        Some((shape, area)) => match shape {
            Shape::Circle(r) => println!("Largest shape: Circle(radius={r}) area: {area} sq. units"),
            Shape::Square(l) => println!("Largest shape: Square(length={l}) area: {area} sq. units"),
            Shape::Triangle(h, b) => println!("Largest shape: Triangle(height={h}, base={b}) area: {area} sq. units"),
        }, 
        None => println!("Error: no shapes provided.")
    }

}
