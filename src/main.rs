use apparent_rs::equatorial::Equatorial;
use apparent_rs::stars::CatalogStar;
use apparent_rs::timescales::JdTt;

fn main() {
    println!("Hello, stars!");

    let vega_now: apparent_rs::vec3::Vec3 = CatalogStar::VEGA.position_at(&JdTt::now().unwrap());
    let vega_equatorial = Equatorial::from_vec3(vega_now);
    println!("{vega_equatorial}");
}
