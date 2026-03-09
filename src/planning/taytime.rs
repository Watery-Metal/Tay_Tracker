// Annoying Date-time structure

pub struct TayTime {
    y: u16,
    m: Option<u8>,
    d: Option<u8>,
    h: Option<u8>,
}

impl TayTime {
    pub fn create(y: u16, m:Option<u8>, d: Option<u8>, h: Option<u8>) -> TayTime {
        TayTime { y, m, d, h }
    }
    pub fn stamp(&self) -> String {
        let casc = format!("{}", self.y);
        if let Some(m) = self.m {
            let ym = format!("{}-{}", casc, m);
            if let Some(d) = self.d{
                let md = format!("{}-{}", ym, d);
                if let Some(h) = self.h {
                    return format!("{}-{}", md, hourer(h))
                }
                return md
            }
            return ym
        }
        casc
    }
}

fn hourer(hour: u8) -> String {
    format!("{}:00", hour%24)
}