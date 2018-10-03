fn main() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.",
             area(rect1));
}

fn area(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}
