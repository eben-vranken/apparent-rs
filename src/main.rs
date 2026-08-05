use apparent_rs::equatorial::Equatorial;
use apparent_rs::stars::CatalogStar;
use apparent_rs::timescales::JdTt;

fn main() {
    println!("Hello, stars!");

    println!("{:?}", CatalogStar::VEGA);
    let vega_now = CatalogStar::VEGA.position_at(&JdTt::new(2451545.0, 0.0));
    let vega_equatorial = Equatorial::from_vec3(vega_now);
    println!(
        "Asc: {:?} Dec: {:?}",
        vega_equatorial.ra_deg(),
        vega_equatorial.dec_deg()
    );
}
