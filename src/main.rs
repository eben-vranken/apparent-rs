use apparent_rs::constants::PI;
use apparent_rs::constants::RADIANS_PER_DEGREE;
use apparent_rs::equatorial::Equatorial;
use apparent_rs::stars::CatalogStar;
use apparent_rs::timescales::JdTt;
use apparent_rs::vec3::Vec3;

fn main() {
    println!("Hello, stars!");

    println!("{:?}", CatalogStar::VEGA);
    let vega_now = CatalogStar::VEGA.position_at(&JdTt::new(2451545.0, 0.0));
    let (right_ascension, declination) = Vec3::to_spherical(vega_now);
    println!(
        "Asc: {:?} Dec: {:?}",
        right_ascension / RADIANS_PER_DEGREE,
        declination / RADIANS_PER_DEGREE
    );

    let res = Equatorial::new(3.9999 * PI, PI / 2.0).expect("Failed to parse equatorial");

    println!("{:?}", res.ra_hours());
}
