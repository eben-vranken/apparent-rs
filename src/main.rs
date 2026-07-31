mod calendar;

fn main() {
    println!("Hello, stars!");

    let date = calendar::CalendarDate {
        year: -323,
        month: 6,
        day: 10,

        hour: 12,
        minute: 00,
        second: 00,
    };

    println!("{:?}", date);
}
