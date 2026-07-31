mod calendar;

fn main() {
    println!("Hello, stars!");

    let date = calendar::CalendarDate::new(-323, 2, 2, 23, 59, 60.0);

    println!("{:?}", date);
}
