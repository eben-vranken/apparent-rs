pub trait Frame {
    const NAME: &'static str;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Icrs;
impl Frame for Icrs {
    const NAME: &'static str = "ICRS";
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeanOfDate;
impl Frame for MeanOfDate {
    const NAME: &'static str = "mean of date";
}
