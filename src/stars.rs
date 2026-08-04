use crate::constants::{RADIANS_PER_DEGREE, RADIANS_PER_MILLIARCSECOND};

// I have added comments as to what these values mean, cause I will forget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CatalogueStar {
    /// ICRS right ascension at `epoch_jd_tt`
    pub ra_rad: f64,
    /// ICRS declination at `epoch_jd_tt`
    pub dec_rad: f64,

    /// Proper motion in right ascension, already multiplied by cos(dec)
    /// this is the catalog's mu_alpha-star: true angular motion on the sky
    pub pm_ra_cosdec_rad_per_yr: f64,

    /// Proper motion in declination
    pub pm_dec_rad_per_yr: f64,

    /// Annual parallax
    pub parallax_rad: f64,

    /// Radial velocity
    pub rv_km_per_s: f64,

    pub epoch_jd_tt: f64,
}

impl CatalogueStar {
    pub const fn from_catalog(
        ra_deg: f64,
        dec_deg: f64,
        pm_ra_cosdec_mas_per_yr: f64,
        pm_dec_mas_per_yr: f64,
        parallax_mas: f64,
        rv_km_per_s: f64,
        epoch_jd_tt: f64,
    ) -> Self {
        Self {
            ra_rad: ra_deg * RADIANS_PER_DEGREE,
            dec_rad: dec_deg * RADIANS_PER_DEGREE,
            pm_ra_cosdec_rad_per_yr: pm_ra_cosdec_mas_per_yr * RADIANS_PER_MILLIARCSECOND,
            pm_dec_rad_per_yr: pm_dec_mas_per_yr * RADIANS_PER_MILLIARCSECOND,
            parallax_rad: parallax_mas * RADIANS_PER_MILLIARCSECOND,
            rv_km_per_s,
            epoch_jd_tt,
        }
    }
}
