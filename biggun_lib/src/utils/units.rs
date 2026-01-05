//! Contains all structs for in-world units

#[derive(Clone)]
pub struct Ounces(u32);

impl Ounces {
    const fn from_lbs_ozs(lbs: u32, ozs: u32) -> Ounces {
        Ounces(lbs * 16 + ozs)
    }

    fn lbs_ozs(self) -> (u32, u32) {
        (self.0 / 16, self.0 % 16)
    }

    fn lerp(&self, other: &Ounces, weight: f32) -> Ounces {
        Ounces(self.0 + ((other.0 - self.0) as f32 * weight).round() as u32)
    }
}

/// 1 Inch corresponds to 1 world unit
#[derive(Clone)]
pub struct Inches(u32);

impl Inches {
    const fn from_ft_ins(ft: u32, ins: u32) -> Inches {
        Inches(ft * 12 + ins)
    }

    fn ft_ins(&self) -> (u32, u32) {
        (self.0 / 12, self.0 % 12)
    }

    fn lerp(&self, other: &Inches, weight: f32) -> Inches {
        Inches(self.0 + ((other.0 - self.0) as f32 * weight).round() as u32)
    }
}
