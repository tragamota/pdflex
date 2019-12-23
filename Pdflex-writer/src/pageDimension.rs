pub struct PageDimension {
    pub width: f32,
    pub heigth: f32,
    _secret: ()
}

impl PageDimension {
    pub const A0: PageDimension = PageDimension { width: 2384.0, heigth: 3370.0, _secret: () };
    pub const A1: PageDimension = PageDimension { width: 1684.0, heigth: 2384.0, _secret: () };
    pub const A2: PageDimension = PageDimension { width: 1190.0, heigth: 1684.0, _secret: () };
    pub const A3: PageDimension = PageDimension { width: 842.0, heigth: 1190.0, _secret: () };
    pub const A4: PageDimension = PageDimension { width: 595.0, heigth: 842.0, _secret: () };
    pub const A5: PageDimension = PageDimension { width: 420.0, heigth: 595.0, _secret: () };
    pub const LETTER: PageDimension = PageDimension { width: 612.0, heigth: 792.0, _secret: () };

    pub fn new(width: f32, heigth: f32) -> PageDimension {
        PageDimension { width: width, heigth: heigth, _secret: ()}
    }
}
