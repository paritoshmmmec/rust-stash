#[derive(Debug)]
pub struct Rectangle {
    width: f32,
    length: f32,
}
#[allow(dead_code)]
enum RectangleTypes {
    Square(f32),
    Rectangle(f32, f32)
}

#[allow(dead_code)]
fn calculate(rect: RectangleTypes) -> f32 {
  match rect {
      RectangleTypes::Rectangle(width, length) => {
        return width * length;
      },
      RectangleTypes::Square(width) => {
        return width * width;
      }
  }
}
#[allow(dead_code)]
impl Rectangle {
    pub fn calculate_area(&self) -> f32 {
        return self.width * self.length;
    }
}
#[allow(dead_code)]
pub fn sub_main() {
    let rect = Rectangle {
        width: 10.0,
        length: 20.0,
    };

    let area = rect.calculate_area();

    println!("area of rectangle is {:?}", area);

    let rect = RectangleTypes::Rectangle(10.0, 30.0);
    let square = RectangleTypes::Square(10.0);

    let area = calculate(rect);
    println!("area of rectangle is {:?}", area);

    let area = calculate(square);
    println!("area of rectangle is {:?}", area);
    
}
