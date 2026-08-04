// I have added comments as to what these values mean, cause I will forget.
pub struct CatalogueStar {
    /// ICRS right ascension at `epoch_jd_tt`
    pub ra_rad: f64,
    // ICRS declination at `epoch_jd_tt`
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
