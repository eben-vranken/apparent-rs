pub trait Frame {
    const NAME: &'static str;
}

pub struct Icrs;
impl Frame for Icrs {
    const NAME: &'static str = "ICRS";
}
