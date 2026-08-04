mod common;

use apparent_rs::constants::{RADIANS_PER_DEGREE, RADIANS_PER_MILLIARCSECOND};
use apparent_rs::stars::CatalogStar;
use common::assert_close;

const TOLERANCE: f64 = 1e-9;

#[test]
fn test_vega_ra_round_trips_to_catalog_degrees() {
    let degrees = CatalogStar::VEGA.ra_rad / RADIANS_PER_DEGREE;

    assert_close(degrees, 279.23410825, TOLERANCE);
}

#[test]
fn test_vega_dec_round_trips_to_catalog_degrees() {
    let degrees = CatalogStar::VEGA.dec_rad / RADIANS_PER_DEGREE;

    assert_close(degrees, 38.78299326, TOLERANCE);
}

#[test]
fn test_vega_parallax_round_trips_to_mas() {
    let mas = CatalogStar::VEGA.parallax_rad / RADIANS_PER_MILLIARCSECOND;

    assert_close(mas, 130.23, TOLERANCE);
}

#[test]
fn test_vega_proper_motion_round_trips_to_mas_per_year() {
    let mas_per_year_dec = CatalogStar::VEGA.pm_dec_rad_per_yr / RADIANS_PER_MILLIARCSECOND;
    let mas_per_year_ra = CatalogStar::VEGA.pm_ra_cosdec_rad_per_yr / RADIANS_PER_MILLIARCSECOND;

    assert_close(mas_per_year_dec, 286.23, TOLERANCE);
    assert_close(mas_per_year_ra, 200.94, TOLERANCE);
}

#[test]
fn test_vega_radial_velocity_is_not_converted() {
    let km_per_s = CatalogStar::VEGA.rv_km_per_s;

    assert_eq!(km_per_s, -13.5);
}

#[test]
fn test_vega_epoch_is_not_converted() {
    let jd = CatalogStar::VEGA.epoch_jd_tt;

    assert_eq!(jd, 2448349.0625);
}
