4. calculate_area_perimeter()
fn calculate_area_perimeter(length: f64, width: f64) -> (f64, f64) {
    let area = length * width;
    let perimeter = 2.0 * (length + width);
    (area, perimeter)
}

fn main() {
    let (area, perimeter) = calculate_area_perimeter(10.0, 5.0);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}
