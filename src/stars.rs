use crate::constants::{
    DAYS_PER_JULIAN_YEAR, HIPPARCOS_EPOCH_JD, RADIANS_PER_DEGREE, RADIANS_PER_MILLIARCSECOND,
};
use crate::timescales::JdTt;
use crate::vec3::Vec3;

// I have added comments as to what these values mean, cause I will forget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CatalogStar {
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

    /// Hipparcos is J1991.25, Gaia DR3 is 2016.0
    pub epoch_jd_tt: f64,
}

impl CatalogStar {
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

    pub fn position_at(&self, jd_tt: &JdTt) -> Vec3 {
        let elapsed_years =
            ((jd_tt.day() - self.epoch_jd_tt) + jd_tt.fraction()) / DAYS_PER_JULIAN_YEAR;

        let catalog_position = Vec3::from_spherical(self.ra_rad, self.dec_rad);

        let (sin_ra, cos_ra) = self.ra_rad.sin_cos();
        let (sin_dec, cos_dec) = self.dec_rad.sin_cos();

        let east = Vec3::new(-sin_ra, cos_ra, 0.0);
        let north = Vec3::new(-sin_dec * cos_ra, -sin_dec * sin_ra, cos_dec);

        let motion_per_year = east * self.pm_ra_cosdec_rad_per_yr + north * self.pm_dec_rad_per_yr;

        (catalog_position + motion_per_year * elapsed_years).normalize()
    }

    /// Vega, alpha Lyrae, HIP 91262.
    ///
    /// Astrometry (RA, Dec, proper motions, parallax): Hipparcos re-reduction,
    /// van Leeuwen 2007, VizieR I/311/hip2. Position is at epoch J1991.25, NOT J2000
    /// https://vizier.cds.unistra.fr/viz-bin/VizieR?-source=I/311/hip2&HIP=91262
    /// Retrieved 4th of August 2026.
    ///
    /// Radial velocity: SIMBAD, from 2004A&A...420..183A.
    /// https://simbad.u-strasbg.fr/simbad/sim-id?Ident=HIP+91262
    /// Retrieved 4th of August 2026.
    pub const VEGA: CatalogStar = CatalogStar::from_catalog(
        279.23410825,
        38.78299326,
        200.94,
        286.23,
        130.23,
        -13.5,
        HIPPARCOS_EPOCH_JD,
    );
}
