use num::range;

use super::vec2d::*;
use core::ops::*;

pub struct Polygon {
	points: Vec<Vec2d>,
}

impl Polygon {
	pub fn new() -> Self {
		Self {
			points: Vec::new()
		}
	}

	pub fn add_point(&mut self, x: f64, y: f64) {
		self.points.push(Vec2d::new(x, y));
	}

	pub fn get_size(&self) -> f64 {
		let n = self.points.len();
		// let mut index = 0;
		let mut result: f64 = 0.0;
		for i in range(1, n - 1) {
			result += cross_value(
				&(self.points[i] - self.points[0]),
				&(self.points[i + 1] - self.points[0]),
			)
		}
		// for i in self.points.iter() {
		// 	if index < (n-2) {
		// 		result += cross_value(
		// 			&(self.points[index + 1] - *i), 
		// 			&(self.points[index + 2] - *i)
		// 		);
		// 	}
		// 	else {
		// 		break;
		// 	}
		// 	index += 1;
		// }
		(result / 2.0).abs()
	}

	pub fn len(&self) -> usize {
		self.points.len()
	}

	pub fn print(&self) {
		print!("{} -> {} : ", self.points.len(), self.get_size());
		for i in self.points.iter() {
			print!("({}, {}), ", i.x, i.y);
		}
		println!("");
	}

	pub fn clear(&mut self) {
		self.points.clear();
	}
}


impl Index<i32> for Polygon {
	type Output = Vec2d;
	fn index(&self, index: i32) -> &Self::Output {
		if (index as usize) > self.points.len() {
			panic!("Index out of bound!");
		}
		&self.points[index as usize]
	}
}

impl IndexMut<i32> for Polygon {
	fn index_mut(&mut self, index: i32) -> &mut Self::Output {
		if (index as usize) > self.points.len() {
			panic!("Index out of bound!");
		}
		&mut self.points[index as usize]
	}
}