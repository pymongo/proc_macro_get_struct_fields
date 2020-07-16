use print_struct_trait::PrintStruct;

#[derive(PrintStruct)]
struct Point {
    name: String,
    x: i32,
    y: i32,
}

fn main() {
    let point = Point {
        name: "origin".to_string(),
        x: 2,
        y: 3,
    };
    point.print();
}
