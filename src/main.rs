mod calendar;

fn main() {
    println!("Hello, stars!");

    let date = calendar::CalendarDate::new(-323, 32, 10, 12, 00, 00.0);

    println!("{:?}", date);
}
