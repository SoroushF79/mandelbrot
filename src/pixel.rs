extern crate num_traits;

use std::fmt::UpperHex;
use self::num_traits::{Unsigned, Bounded, Zero};

#[derive(Clone)]
pub struct Pixel<T: Unsigned + Bounded> {
    r: T,
    g: T,
    b: T,
    a: T
}

pub trait PixelMath<T: Unsigned + Bounded> {

    fn default() -> Self; 
    fn new(r: T, g: T, b: T) -> Self;
    fn new_rgba(r: T, g: T, b: T, a: T) -> Self;

    fn set_alpha(&mut self, a: T) -> &Self;
    fn set_r(&mut self, r: T) -> &Self;
    fn set_g(&mut self, g: T) -> &Self;
    fn set_b(&mut self, b: T) -> &Self;

    fn set_rgb(&mut self, r: T, g: T, b: T) -> &Self;
    fn set_rgba(&mut self, r: T, g: T, b: T, a: T) -> &Self;

    fn get_tuple(&self) -> (T, T, T, T);
    fn get_vector(&self) -> Vec<T>;
    fn get_slice(&self) -> [T; 4];

    fn to_hex(&self) -> String;
    fn to_hsv(&self) -> (T, T, T);
}

impl<T: Unsigned + Bounded + UpperHex + Zero + Copy> PixelMath<T> for Pixel<T> {
    
    fn default() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    fn new(r: T, g: T, b: T) -> Self {
        Self::new_rgba(r, g, b, T::max_value())
    }

    fn new_rgba(r: T, g: T, b: T, a: T) -> Self {
        Pixel{r, g, b, a}
    }

    fn set_alpha(&mut self, a: T) -> &Self {
        self.a = a;
        self
    }

    fn set_r(&mut self, r: T) -> &Self {
        self.r = r;
        self
    }

    fn set_g(&mut self, g: T) -> &Self {
        self.g = g;
        self
    }

    fn set_b(&mut self, b: T) -> &Self {
        self.b = b;
        self
    }


    fn set_rgb(&mut self, r: T, g: T, b: T) -> &Self {
        self.r = r;
        self.g = g;
        self.b = b;

        self
    }

    fn set_rgba(&mut self, r: T, g: T, b: T, a: T) -> &Self {
        self.set_rgb(r, g, b);
        self.a = a;

        self
    }


    fn get_tuple(&self) -> (T, T, T, T) {
        (self.r, self.g, self.b, self.a)
    }

    fn get_vector(&self) -> Vec<T> {
        vec![self.r, self.g, self.b, self.a]
    }

    fn get_slice(&self) -> [T; 4] {
        [self.r, self.g, self.b, self.a]
    }


    fn to_hex(&self) -> String {
        format!("{:#X}{:X}{:X}{:X}", self.r, self.g, self.b, self.a)
    }

    fn to_hsv(&self) -> (T, T, T) {
        unimplemented!();
    }
}


pub struct IntoPixel<'a, T: 'a + Unsigned + Bounded> {
    px: &'a Pixel<T>,
    remaining: u8,
}

impl<'a, T: Unsigned + Bounded> IntoPixel<'a, T> {
    pub fn new(px: &'a Pixel<T>) -> Self {
        IntoPixel { px: &px, remaining: 5 }
    }
}

impl<'a, T: Unsigned + Bounded + Copy> Iterator for IntoPixel<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining -= 1;

        match self.remaining {
            r @ 1 ... 4 => Some(
                match r {
                    4 => self.px.r,
                    3 => self.px.g,
                    2 => self.px.b,
                    _ => self.px.a,
                }),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use ::pixel::{Pixel, PixelMath, IntoPixel};

    #[test]
    fn pixel_iterator() {
        let px = Pixel::new(9u8, 234, 5);

        let iter = IntoPixel::new(&px);

        for i in iter {
            println!("{:?}", i);
        }

    }
}