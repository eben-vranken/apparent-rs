use apparent_rs::direction::Direction;
use apparent_rs::equatorial::Equatorial;
use apparent_rs::frames::Icrs;
use apparent_rs::stars::CatalogStar;
use apparent_rs::timescales::JdTt;

fn main() {
    println!("Hello, stars!");

    let vega_now: Direction<Icrs> = CatalogStar::VEGA.position_at(&JdTt::now().unwrap());
    let vega_equatorial = Equatorial::from_direction(vega_now);
    println!("{vega_equatorial}");
}
