#[allow(unused)]
//==============================
//Consts-----------------------
//==============================
#[allow(unused)]
pub const     PI: f32 = 3.14169265358;
#[allow(unused)]
pub const 	 PI2: f32 = 6.28318530718;
#[allow(unused)]
pub const HALFPI: f32 = 1.57079632679;
#[allow(unused, non_upper_case_globals)]
pub const RadToD: f32 = 57.2957795131;
#[allow(unused, non_upper_case_globals)]
pub const DToRad: f32 = 0.01745329251;

//==============================
//Generic Math Structs----------
//==============================

//Matrix struct---------

#[allow(unused)]
pub struct Matrix {
	data: Vec<f32>,
	width: u32,
	height: u32
}

#[allow(unused)]
impl Matrix {
	
	pub fn new(width: u32, height: u32, data: Vec<f32>) -> Self {
		Matrix{data: data, width: width, height: height}
	}
	
	pub fn empty(width: u32, height: u32) -> Self {
		let mut data = Vec::new();
		for _i in 0..width*height { data.push(0.0); }
		Matrix{data: data, width: width, height: height}
	}	
	
	pub fn identity4f() -> Self {
		let data = vec![1.0, 0.0, 0.0, 0.0,
						0.0, 1.0, 0.0, 0.0,
						0.0, 0.0, 1.0, 0.0,
						0.0, 0.0, 0.0, 1.0];
		Matrix{data: data, width: 4, height: 4}		
	}
	
	pub fn projection4f(near_plane: f32, far_plane: f32, fov: f32, aspect_ratio: f32) -> Self {
		let x_scale = 1.0 / tand(fov / 2.0);
		let y_scale = x_scale * aspect_ratio;
		let frustum_length = far_plane - near_plane;
		let data = vec![x_scale, 0.0, 0.0, 0.0,
						0.0, y_scale, 0.0, 0.0,
						0.0, 0.0, (-(far_plane + near_plane) / frustum_length), -1.0,
						0.0, 0.0, (-(2.0 * near_plane * far_plane) / frustum_length), 0.0];
		Matrix{data: data, width: 4, height: 4}	
	}
	
	pub fn translation(translation: &Vec3) -> Self{
		let data = vec![1.0, 0.0, 0.0, 0.0,
						0.0, 1.0, 0.0, 0.0,
						0.0, 0.0, 1.0, 0.0,
						translation.x, -translation.y, translation.z, 1.0];
		Matrix{data: data, width: 4, height: 4}	
	}
	
	pub fn scale(scale: &Vec3) -> Self{
		let data = vec![scale.x, 0.0, 0.0, 0.0,
						0.0, scale.y, 0.0, 0.0,
						0.0, 0.0, scale.z, 0.0,
						0.0, 0.0, 0.0, 1.0];
		Matrix{data: data, width: 4, height: 4}	
	}
	
	pub fn create_transformation_matrix(location: &Vec3, rx: f32, ry: f32, rz: f32, scale: f32) -> Option<Matrix> {
		let mut matrix = Matrix::translation(location);
		matrix = Matrix::rotation_y(ry).multiply(&matrix).unwrap();
		matrix = matrix.multiply(&Matrix::rotation_x(rx)).unwrap();
		matrix = Matrix::rotation_z(rz).multiply(&matrix).unwrap();
		matrix = matrix.multiply(&Matrix::scale(&Vec3::new(scale, scale, scale))).unwrap();
		Some(matrix)
	}
	
	pub fn create_camera_matrix(location: &Vec3, rx: f32, ry: f32) -> Option<Matrix> {
		let mut matrix = Matrix::identity4f();
		matrix = Matrix::rotation_x(rx).multiply(&matrix).unwrap();
		matrix = matrix.multiply(&Matrix::rotation_y(ry)).unwrap();
		matrix = Matrix::translation(&Vec3::new(location.x, -location.y, location.z)).multiply(&matrix).unwrap();
		matrix = matrix.multiply(&Matrix::identity4f()).unwrap();
		Some(matrix)
	}
	
	pub fn rotation_x(angle: f32) -> Self {
		let cos = cosd(angle);
		let sin = sind(angle);
		let data = vec![1.0, 0.0, 0.0, 0.0,
						0.0, cos, sin, 0.0,
						0.0, -sin, cos, 0.0,
						0.0, 0.0, 0.0, 1.0];
		Matrix{data: data, width: 4, height: 4}		
	}
	
	pub fn rotation_y(angle: f32) -> Self {
		let cos = cosd(angle);
		let sin = sind(angle);
		let data = vec![cos, 0.0, -sin, 0.0,
						0.0, 1.0, 0.0, 0.0,
						sin, 0.0, cos, 0.0,
						0.0, 0.0, 0.0, 1.0];
		Matrix{data: data, width: 4, height: 4}		
	}
	
	pub fn rotation_z(angle: f32) -> Self {
		let cos = cosd(angle);
		let sin = sind(angle);
		let data = vec![cos, sin, 0.0, 0.0,
						-sin, cos, 0.0, 0.0,
						0.0, 0.0, 1.0, 0.0,
						0.0, 0.0, 0.0, 1.0];
		Matrix{data: data, width: 4, height: 4}		
	}
	
	pub fn get_width(&self) -> u32 {
		return self.width;
	}
	
	pub fn get_height(&self) -> u32 {
		return self.height;
	}
	
	pub fn get_data_ptr(&self) -> *const f32 {
		return self.data.as_ptr();
	}
	
	pub fn get_data(&self) -> &Vec<f32> {
		return &self.data;
	}
	
	pub fn multiply(&self, matrix_b: &Matrix) -> Result<Matrix, &'static str> {
		if self.get_width() == matrix_b.get_height() {
			let mut new_data = Vec::new();
			for i in 0..self.get_height() {
				for j in 0..matrix_b.get_width() {
					let mut r = 0.0;
					for k in 0..self.get_width() {
						r += self.get_data()[(k + j*self.get_width()) as usize]*matrix_b.get_data()[(i + k*matrix_b.get_width()) as usize];
					}
					new_data.push(r);	
				}
			}
			return Ok(Matrix{data: new_data, width: self.get_height(), height: matrix_b.get_width()});
		} else {
			return Err("Width of base matrix must be equal to height of matrix_b");
		}
	}
	
	pub fn to_string(&self) -> String {
		let mut message = String::new();
		let data = self.get_data();
		let mut longest_number_length = 0;
		for i in 0..data.len() {
			let len = data[i].to_string().len() as u32;
			if len > longest_number_length {
				longest_number_length = len;
			}  
		}
		for i in 0..data.len() {
			let mut spaces = String::new();
			let space_count = (longest_number_length - data[i].to_string().len() as u32);
			for _i in 0..space_count { spaces += " "; }
			message += (spaces + data[i].to_string().as_str()).as_str();
			if (i as u32 + 1) % self.get_width() == 0 {
				message += "\n";
			} else {
				message += "|";
			}
		}
		return message;
	}
	
}

//Vec3 struct------------------
#[allow(unused)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Vec3{x: x, y: y, z: z}
	}
	
	pub fn cross_product(v1: &Vec3, v2: &Vec3) -> Self {
		Vec3::new((v1.y*v2.z) - (v1.z*v2.y), -((v1.x*v2.z) - (v1.z*v2.x)), (v1.x*v2.y) - (v1.y*v2.x))
	}
	
	pub fn angler(v1: &Vec3, v2: &Vec3) -> f32 {
		arccos(((v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)) / (sqrt((v1.x*v1.x + v1.y*v1.y + v1.z*v1.z) as f64).unwrap()*sqrt((v2.x*v2.x + v2.y*v2.y + v2.z*v2.z) as f64).unwrap()) as f32)
	}
	
	pub fn find_normal(p1: &Vec3, p2: &Vec3, p3: &Vec3) -> Vec3 {
		let n =  &Vec3::cross_product(&(p2 - p1), &(p3 - p1));
		let a1 = Vec3::angler(&(p2 - p1), &(p3 - p1));
		let a2 = Vec3::angler(&(p3 - p2), &(p1 - p2));
		let a3 = Vec3::angler(&(p1 - p3), &(p2 - p3));
		Vec3::new((n * a1).x, (n * a2).y, (n * a3).z)
	} 
	
	pub fn clone(&self) -> Self {
		Self{x: self.x, y: self.y, z: self.z}
	}
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self,  vec: &Vec3) -> Vec3 {
    	Vec3::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, vec: &Vec3) -> Vec3 {
    	Vec3::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}

impl std::ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, val: f32) -> Vec3 {
    	Vec3::new(self.x * val, self.y * val, self.z * val)
    }
}

//================================
//Generic Math Functions---------
//================================

#[allow(unused)]
pub fn arccos(x: f32) -> f32 {
	if x < -1.0 || x > 1.0 {
		panic!("Input outside of domain. x >= -1.0 & x <= 1.0. Input: {}", x);
	}
	if x < 0.0 {
		let tx = -x;		
		let result = PI + (HALFPI*(-2.0*sqrt((-tx*tx - 3.0*tx + 4.0) as f64).unwrap() as f32)) / (tx + 4.0);
		return result;
	} else if x == 0.0 {
		return HALFPI;
	} else {
		let tx = x;		
		let result = -(HALFPI*(-2.0*sqrt((-tx*tx - 3.0*tx + 4.0) as f64).unwrap() as f32)) / (tx + 4.0);
		return result;
	}
}

#[allow(unused)]
pub fn sind(x: f32) -> f32 {
	let mut m: f32 = 1.0;
	let mut tx = 0.0;
	tx = x % 360.0;
	if tx < 0.0 {
		tx += 360.0;
	}
	if tx > 180.0 {
		m = -1.0;
		tx -= 180.0;
	}
	tx = 4.0*tx*(180.0 - tx)/(40500.0 - tx*(180.0 - tx));
	return tx * m;
}

#[allow(unused)]
pub fn cosd(x: f32) -> f32 {
	return sind(90.0 - x);
}

#[allow(unused)]
pub fn tand(x: f32) -> f32 {
	return sind(x)/cosd(x);
}

#[allow(unused)]
pub fn sinr(x: f32) -> f32 {
	let mut m: f32 = 1.0;
	let mut tx = 0.0;
	tx = (x * RadToD) % 360.0;
	if tx < 0.0 {
		tx += 360.0;
	}
	if tx > 180.0 {
		m = -1.0;
		tx -= 180.0;
	}
	tx = 4.0*tx*(180.0 - tx)/(40500.0 - tx*(180.0 - tx));
	return tx * m;
}

#[allow(unused)]
pub fn cosr(x: f32) -> f32 {
	return sinr(HALFPI - x);
}

#[allow(unused)]
pub fn tanr(x: f32) -> f32 {
	return sinr(x)/cosr(x);
}

#[allow(unused)]
pub fn absf64(x: f64) -> f64 {
	if x >= 0.0 {
		return x;
	} else {
		return -x;
	}
} 

#[allow(unused)]
pub fn absf32(x: f32) -> f32 {
	if x > 0.0 {
		return x;
	} else {
		return -x;
	}
}

#[allow(unused)]
pub fn absi64(x: i64) -> i64 {
	if x > 0 {
		return x;
	} else {
		return -x;
	}
}

#[allow(unused)]
pub fn absi32(x: i32) -> i32 {
	if x > 0 {
		return x;
	} else {
		return -x;
	}
}

#[allow(unused)]
pub fn sqrt(x: f64) -> Result<f64, &'static str> {
	if x > 0.0 {
		return Ok(truncate(sqrt_find(x, sqrt_geuss(x, 1.0)), 7));
	} else if x == 0.0 {
		return Ok(0.0);
	} else {
		return Err("Negative number in square root!");
	}
}

#[allow(unused)]
fn sqrt_find(x: f64, g: f64) -> f64 {
	if sqrt_close_test(x/g, g) {
		return g;
	} else {
		return sqrt_find(x, sqrt_geuss(x, g));
	}
}

#[allow(unused)]
fn sqrt_close_test(a: f64, b: f64) -> bool {
   return absf64(a - b) < 0.0000001;
}

#[allow(unused)]
fn sqrt_geuss(x: f64, g: f64) -> f64 {
   return (g + (x/g)) / 2.0;
}

#[allow(unused)]
pub fn truncate(x: f64, digits: i32) -> f64 {
	let z = { let mut z = 1; for _i in 0..digits { z *= 10; } z } as f64;
	return floor64(x * z) / z;
}

#[allow(unused)]
pub fn floor32(x: f32) -> f32 {
	return x as i32 as f32;
}

#[allow(unused)]
pub fn floor64(x: f64) -> f64 {
	return x as i64 as f64;
}

#[allow(unused)]
pub fn clampi32(x: i32, min: i32, max: i32) -> i32 {
	if x <= min {
		return min;
	}
	if x >= max {
		return max;
	}
	return x;
}

#[allow(unused)]
pub fn clampi64(x: i64, min: i64, max: i64) -> i64 {
	if x <= min {
		return min;
	}
	if x >= max {
		return max;
	}
	return x;
}

#[allow(unused)]
pub fn clampf32(x: f32, min: f32, max: f32) -> f32 {
	if x <= min {
		return min;
	}
	if x >= max {
		return max;
	}
	return x;
}

#[allow(unused)]
pub fn clampf64(x: f64, min: f64, max: f64) -> f64 {
	if x <= min {
		return min;
	}
	if x >= max {
		return max;
	}
	return x;
}

