use core::ops::*;

#[derive(Copy, Clone)]
pub struct Vec2d{
	pub x: f64,
	pub y: f64,
}

impl Vec2d {
	pub fn new(x_: f64, y_: f64) -> Self {
		Self {
			x: x_,
			y: y_,
		}
	}

	pub fn length(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
	pub fn sqr_length(&self) -> f64 {
		self.x * self.x + self.y * self.y
	}
}

impl Neg for Vec2d {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl Add for Vec2d {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Sub for Vec2d {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

pub fn det_2d(a11: f64, a12: f64, a21: f64, a22: f64) -> f64 {
	a11 * a22 - a12 * a21
}

pub fn cross_value(a: &Vec2d, b: &Vec2d) -> f64 {
	det_2d(a.x, a.y, b.x, b.y)
}