mod common;

use apparent_rs::constants::{
    DAYS_PER_JULIAN_YEAR, HIPPARCOS_EPOCH_JD, J2000_EPOCH_JD, RADIANS_PER_ARCSECOND,
    RADIANS_PER_DEGREE, RADIANS_PER_MILLIARCSECOND,
};
use apparent_rs::stars::CatalogStar;
use apparent_rs::timescales::JdTt;
use apparent_rs::vec3::Vec3;
use common::{assert_close, assert_close_vector};

const TOLERANCE: f64 = 1e-9;
const TOLERANCE_RAD: f64 = 1e-15;
const TOLERANCE_ORACLE_MAS: f64 = 0.5;

const PM_MAS: f64 = 1.0;
const STEP_RAD: f64 = PM_MAS * RADIANS_PER_MILLIARCSECOND;
const ONE_YEAR_LATER: JdTt = JdTt::new(J2000_EPOCH_JD + DAYS_PER_JULIAN_YEAR, 0.0);
const J1900_JD: f64 = 2_415_020.0;

const VEGA_J2000_RA_DEG: f64 = 279.234734796882;
const VEGA_J2000_DEC_DEG: f64 = 38.783688967190;

fn synthetic_star(
    ra_deg: f64,
    dec_deg: f64,
    pm_ra_cosdec_mas: f64,
    pm_dec_mas: f64,
) -> CatalogStar {
    CatalogStar::from_catalog(
        ra_deg,
        dec_deg,
        pm_ra_cosdec_mas,
        pm_dec_mas,
        0.0,
        0.0,
        J2000_EPOCH_JD,
    )
}

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

#[test]
fn test_zero_elapsed_returns_catalog_position() {
    let position = CatalogStar::VEGA.position_at(&JdTt::new(HIPPARCOS_EPOCH_JD, 0.0));

    assert_close_vector(
        position,
        Vec3::from_spherical(CatalogStar::VEGA.ra_rad, CatalogStar::VEGA.dec_rad),
        TOLERANCE_RAD,
    );
}

#[test]
fn test_fraction_contributes_to_elapsed_time() {
    let split = CatalogStar::VEGA.position_at(&JdTt::new(J2000_EPOCH_JD + 365.0, 0.25));
    let whole = CatalogStar::VEGA.position_at(&JdTt::new(J2000_EPOCH_JD + 365.25, 0.0));

    assert_eq!(split, whole);
}

#[test]
fn test_motion_before_catalog_epoch_reverses() {
    let position = CatalogStar::VEGA.position_at(&JdTt::new(J1900_JD, 0.0));
    let (_, dec_rad) = position.to_spherical();

    assert!(dec_rad < CatalogStar::VEGA.dec_rad);

    let drop_arcsec = (CatalogStar::VEGA.dec_rad - dec_rad) / RADIANS_PER_ARCSECOND;

    assert_close(drop_arcsec, 26.12, 0.1);
}

#[test]
fn test_pure_ra_motion_moves_toward_east() {
    let position = synthetic_star(0.0, 0.0, PM_MAS, 0.0).position_at(&ONE_YEAR_LATER);

    assert_close(position.y, STEP_RAD, TOLERANCE_RAD);
    assert_close(position.x, 1.0, TOLERANCE_RAD);
    assert_eq!(position.z, 0.0);
}

#[test]
fn test_pure_dec_motion_moves_toward_north() {
    let position = synthetic_star(0.0, 0.0, 0.0, PM_MAS).position_at(&ONE_YEAR_LATER);

    assert_close(position.z, STEP_RAD, TOLERANCE_RAD);
    assert_close(position.x, 1.0, TOLERANCE_RAD);
    assert_eq!(position.y, 0.0);
}

#[test]
fn test_dec_motion_at_high_declination() {
    let position = synthetic_star(45.0, 60.0, 0.0, PM_MAS).position_at(&ONE_YEAR_LATER);
    let (ra_rad, dec_rad) = position.to_spherical();

    assert_close(dec_rad, 60.0 * RADIANS_PER_DEGREE + STEP_RAD, TOLERANCE_RAD);
    assert_close(ra_rad, 45.0 * RADIANS_PER_DEGREE, TOLERANCE_RAD);
}

#[test]
fn test_ra_motion_is_already_cos_dec_corrected() {
    let position = synthetic_star(45.0, 60.0, PM_MAS, 0.0).position_at(&ONE_YEAR_LATER);
    let (ra_rad, dec_rad) = position.to_spherical();

    assert_close(
        ra_rad,
        45.0 * RADIANS_PER_DEGREE + 2.0 * STEP_RAD,
        TOLERANCE_RAD,
    );
    assert_close(dec_rad, 60.0 * RADIANS_PER_DEGREE, TOLERANCE_RAD);
}

#[test]
fn test_vega_propagates_to_published_j2000_position() {
    let position = CatalogStar::VEGA.position_at(&JdTt::new(J2000_EPOCH_JD, 0.0));
    let (ra_rad, dec_rad) = position.to_spherical();

    let ra_error_mas =
        (ra_rad - VEGA_J2000_RA_DEG * RADIANS_PER_DEGREE) / RADIANS_PER_MILLIARCSECOND;
    let dec_error_mas =
        (dec_rad - VEGA_J2000_DEC_DEG * RADIANS_PER_DEGREE) / RADIANS_PER_MILLIARCSECOND;

    assert_close(ra_error_mas, 0.0, TOLERANCE_ORACLE_MAS);
    assert_close(dec_error_mas, 0.0, TOLERANCE_ORACLE_MAS);
}
