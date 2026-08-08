pub trait Frame {
    const NAME: &'static str;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Icrs;
impl Frame for Icrs {
    const NAME: &'static str = "ICRS";
}
