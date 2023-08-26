use genpdf::{PaperSize, Size};

pub struct A4Landscape;

impl From<A4Landscape> for Size {
    fn from(_: A4Landscape) -> Self {
        let a4 = PaperSize::A4;
        let size = Size::from(a4);
        Size {
            width: size.height,
            height: size.width,
        }
    }
}
