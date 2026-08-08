use apparent_rs::direction::Direction;
use apparent_rs::frames::Icrs;
use apparent_rs::stars::CatalogStar;
use apparent_rs::timescales::JdTt;

fn main() {
    println!("Hello, stars!");

    let vega_now: apparent_rs::direction::Direction<Icrs> =
        CatalogStar::VEGA.position_at(&JdTt::now().unwrap());
    let vega_equatorial = Direction::as_vec3(vega_now);
    println!("{vega_equatorial:?}");
}
