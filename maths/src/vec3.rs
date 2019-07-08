#![allow(dead_code)]

//##############################################################################
// CONSTANTS
pub const VEC3_ZERO	: Vec3 = [  0.0,  0.0,  0.0 ];
pub const VEC3_FWD	: Vec3 = [  0.0,  0.0,  1.0 ];
pub const VEC3_BACK	: Vec3 = [  0.0,  0.0, -1.0 ];
pub const VEC3_UP	: Vec3 = [  0.0,  1.0,  0.0 ];
pub const VEC3_DOWN	: Vec3 = [  0.0, -1.0,  0.0 ];
pub const VEC3_L	: Vec3 = [  1.0,  0.0,  0.0 ];
pub const VEC3_R	: Vec3 = [ -1.0,  0.0,  0.0 ];


//##############################################################################
// TYPES
pub type Vec3 = [f32; 3];


//##############################################################################
// MAIN
pub trait Vec3Trait{
	////////////////////////////////////////////////////////////
	// Static Functions
	////////////////////////////////////////////////////////////  
		fn new( ) -> Vec3{ [0.0; 3] }
		
		fn init( x: f32, y:f32, z:f32 ) -> Vec3{ [ x, y, z ] }
		
		fn from_copy( v: &Vec3 ) -> Vec3 { [ v[0], v[1], v[2] ] }

		fn dot( a: &Vec3, b: &Vec3 ) -> f32{ a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

		fn len_from( a: &Vec3, b: &Vec3 ) -> f32{
			let x = a[0] - b[0];
			let y = a[1] - b[1];
			let z = a[2] - b[2];
			( x*x + y*y + z*z ).sqrt()
		}

		fn len_sqr_from( a: &Vec3, b: &Vec3 ) -> f32{
			let x = a[0] - b[0];
			let y = a[1] - b[1];
			let z = a[2] - b[2];
			( x*x + y*y + z*z )
		}

		fn from_cross( a: &Vec3, b: &Vec3 ) -> Vec3{
			[	a[1] * b[2] - a[2] * b[1], 
				a[2] * b[0] - a[0] * b[2], 
				a[0] * b[1] - a[1] * b[0] ]
		}

		fn angle( a: &Vec3, b:&Vec3 ) -> f32{
			let mut theta = Vec3::dot( a, b ) / ( a.len_sqr() * b.len_sqr() ).sqrt();
			if theta < -1.0 { theta = -1.0; }		// clamp( -1, 1 )
			else if theta > 1.0 { theta = 1.0; }

			theta.acos()
		}


	////////////////////////////////////////////////////////////
	// Instance Getters Setters
	////////////////////////////////////////////////////////////
		fn x( &mut self, v: f32) -> &mut Vec3;
		fn y( &mut self, v: f32) -> &mut Vec3;
		fn z( &mut self, v: f32) -> &mut Vec3;

		fn xy( &mut self, x: f32, y: f32 ) -> &mut Vec3;
		fn xz( &mut self, x: f32, z: f32 ) -> &mut Vec3;
		fn yz( &mut self, y: f32, z: f32 ) -> &mut Vec3;

		fn set( &mut self, x: f32, y:f32, z: f32 ) -> &mut Vec3;
		fn copy( &mut self, v: &Vec3 ) -> &mut Vec3;

		fn len( &self ) -> f32;
		fn len_sqr( &self ) -> f32;
		fn set_len( &mut self, len: f32 ) -> &mut Self;


	////////////////////////////////////////////////////////////
	// Operations
	////////////////////////////////////////////////////////////
		fn add( &mut self, v: &Vec3 ) -> &mut Vec3;
		fn add_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3;

		fn sub( &mut self, v: &Vec3 ) -> &mut Vec3;
		fn sub_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3;

		fn scale( &mut self, v: f32 ) -> &mut Vec3;

		fn mul( &mut self, v: &Vec3 ) -> &mut Self;
		fn mul_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Self;

		fn div( &mut self, v: &Vec3 ) -> &mut Self;

		fn cross_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Self;

		fn abs( &mut self )-> &mut Self;
		fn floor( &mut self )-> &mut Self;
		fn invert( &mut self )-> &mut Self;
		fn invert_out<'a>( &self, out: &'a mut Vec3 ) -> &'a mut Vec3;

		fn norm( &mut self ) -> &mut Vec3;
		fn norm_out<'a>( &self, out: &'a mut Vec3) -> &'a mut Vec3;

		fn near_zero( &mut self )-> &mut Self;


	////////////////////////////////////////////////////////////
	// Data Transform
	////////////////////////////////////////////////////////////

		fn transform_quat( &mut self, q: &[f32; 4] ) -> &mut Self;

		fn transform_mat4( &mut self, m: &[f32; 16] ) -> &mut Self;

		fn rotate( &mut self, rad: f32, axis: i8 ) -> &mut Self;

		fn lerp_from( &mut self, a: &Vec3, b: &Vec3, t: f32 ) -> &mut Self;

}


impl Vec3Trait for Vec3{
	////////////////////////////////////////////////////////////
	// Getters - Setters
	////////////////////////////////////////////////////////////		
		fn x( &mut self, v: f32) -> &mut Vec3{ self[0] = v; self }
		fn y( &mut self, v: f32) -> &mut Vec3{ self[1] = v; self }
		fn z( &mut self, v: f32) -> &mut Vec3{ self[2] = v; self }

		fn xy( &mut self, x: f32, y: f32 ) -> &mut Vec3{ self[0] = x; self[1] = y; self }
		fn xz( &mut self, x: f32, z: f32 ) -> &mut Vec3{ self[0] = x; self[2] = z; self }
		fn yz( &mut self, y: f32, z: f32 ) -> &mut Vec3{ self[1] = y; self[2] = z; self }

		fn set( &mut self, x: f32, y:f32, z: f32 ) -> &mut Vec3{
			self[0] = x;
			self[1] = y;
			self[2] = z;
			self
		}

		fn copy( &mut self, v: &Vec3 ) -> &mut Vec3 {
			self[0] = v[0];
			self[1] = v[1];
			self[2] = v[2];
			self
		}

		fn len( &self ) -> f32{ ( self[0]*self[0] + self[1]*self[1] + self[2]*self[2] ).sqrt() }
		fn len_sqr( &self ) -> f32{ self[0]*self[0] + self[1]*self[1] + self[2]*self[2] }

		fn set_len( &mut self, len: f32 ) -> &mut Self{
			let mag = 1.0 / ( self[0]*self[0] + self[1]*self[1] + self[2]*self[2] ).sqrt() * len;
			self[0]	*= mag;
			self[1]	*= mag;
			self[2]	*= mag;
			self
		}

	////////////////////////////////////////////////////////////
	// Operations
	////////////////////////////////////////////////////////////
		fn add( &mut self, v: &Vec3 ) -> &mut Vec3 {
			self[0] += v[0];
			self[1] += v[1];
			self[2] += v[2];
			self
		}

		fn add_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3{
			self[0] = a[0] + b[0];
			self[1] = a[1] + b[1];
			self[2] = a[2] + b[2];
			self   
		}

		fn sub( &mut self, v: &Vec3 ) -> &mut Vec3 {
			self[0] -= v[0];
			self[1] -= v[1];
			self[2] -= v[2];
			self
		}

		fn sub_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3{
			self[0] = a[0] - b[0];
			self[1] = a[1] - b[1];
			self[2] = a[2] - b[2];
			self   
		}

		fn scale( &mut self, s: f32 ) -> &mut Vec3 {
			self[0] *= s;
			self[1] *= s;
			self[2] *= s;
			self
		}

		fn mul( &mut self, v: &Vec3 ) -> &mut Self{
			self[0] *= v[0];
			self[1] *= v[1];
			self[2] *= v[2];
			self
		}

		fn mul_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Self{
			self[0] = a[0] * b[0];
			self[1] = a[1] * b[1];
			self[2] = a[2] * b[2];
			self
		}

		fn div( &mut self, v: &Vec3 ) -> &mut Self{
			self[0] = if v[0] != 0.0 { self[0] / v[0] }else { 0.0 };
			self[1] = if v[1] != 0.0 { self[1] / v[1] }else { 0.0 };
			self[2] = if v[2] != 0.0 { self[2] / v[2] }else { 0.0 };
			self
		}

		fn cross_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Self{
			self[0] = a[1] * b[2] - a[2] * b[1]; 
			self[0] = a[2] * b[0] - a[0] * b[2];
			self[0] = a[0] * b[1] - a[1] * b[0];
			self
		}

		fn abs( &mut self )-> &mut Self{
			self[0] = self[0].abs();
			self[1] = self[1].abs();
			self[2] = self[2].abs();
			self
		}

		fn floor( &mut self )-> &mut Self{
			self[0] = self[0].floor();
			self[1] = self[1].floor();
			self[2] = self[2].floor();
			self
		}

		fn invert( &mut self )-> &mut Self{
			self[0] = -self[0];
			self[1] = -self[1];
			self[2] = -self[2];
			self
		}

		fn invert_out<'a>( &self, out: &'a mut Vec3 ) -> &'a mut Vec3{
			out[0] = -self[0];
			out[1] = -self[1];
			out[2] = -self[2];
			out	
		}

		fn norm( &mut self ) -> &mut Vec3{
			let mag = 1.0 / ( self[0]*self[0] + self[1]*self[1] + self[2]*self[2] ).sqrt();
			self[0]	*= mag;
			self[1]	*= mag;
			self[2]	*= mag;
			self
		}

		fn norm_out<'a>( &self, out: &'a mut Vec3) -> &'a mut Vec3{
			let mag = 1.0 / ( self[0]*self[0] + self[1]*self[1] + self[2]*self[2] ).sqrt();
			out[0]	= self[0] * mag;
			out[1]	= self[1] * mag;
			out[2]	= self[2] * mag;
			out
		}

		//When values are very small, like less then 0.000001, just make it zero.
		fn near_zero( &mut self )-> &mut Self{
			if self[0].abs() <= 0.000001 { self[0] = 0.0; }
			if self[1].abs() <= 0.000001 { self[1] = 0.0; }
			if self[2].abs() <= 0.000001 { self[2] = 0.0; }
			self
		}


	////////////////////////////////////////////////////////////
	// Data Transform
	////////////////////////////////////////////////////////////

		// https://github.com/toji/gl-matrix/blob/master/src/vec3.js#L505
		fn transform_quat( &mut self, q: &[f32; 4] ) -> &mut Self{
			// var qvec = [qx, qy, qz];
			// var uv = vec3.cross([], qvec, v);
			let uvx = q[1] * self[2] - q[2] * self[1];
			let uvy = q[2] * self[0] - q[0] * self[2];
			let uvz = q[0] * self[1] - q[1] * self[0];

			// var uuv = vec3.cross([], qvec, uv);
			// vec3.scale(uuv, uuv, 2);
			let uuvx = ( q[1] * uvz - q[2] * uvy ) * 2.0;
			let uuvy = ( q[2] * uvx - q[0] * uvz ) * 2.0;
			let uuvz = ( q[0] * uvy - q[1] * uvx ) * 2.0;
			
			// vec3.scale(uv, uv, 2 * w);
			// return vec3.add(out, a, vec3.add(out, uv, uuv));
			let w2 = q[3] * 2.0;	
			self[0] += uvx * w2 + uuvx;
			self[1] += uvy * w2 + uuvy;
			self[2] += uvz * w2 + uuvz;
			self
		}

		fn transform_mat4( &mut self, m: &[f32; 16] ) -> &mut Self{
		    let x = self[0];
		    let y = self[1];
		    let z = self[2];
		    
		    let mut w = m[3] * x + m[7] * y + m[11] * z + m[15];
		    if w == 0.0 { w = 1.0; }

		    self[0] = (m[0] * x + m[4] * y + m[ 8] * z + m[12]) / w;
		    self[1] = (m[1] * x + m[5] * y + m[ 9] * z + m[13]) / w;
		    self[2] = (m[2] * x + m[6] * y + m[10] * z + m[14]) / w;
		    self
		}

		// https://www.siggraph.org/education/materials/HyperGraph/modeling/mod_tran/3drota.htm
		fn rotate( &mut self, rad: f32, axis: i8 ) -> &mut Self{
			let sin = rad.sin();
			let cos = rad.cos();
			let x 	= self[0];
			let y 	= self[1];
			let z 	= self[2];

			match axis{
				0 => { // X ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					self[1]	= y*cos - z*sin; // y
					self[2]	= y*sin + z*cos; // z
				},
				1 => { // Y ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					self[0]	= z*sin + x*cos; // x
					self[2]	= z*cos - x*sin; // z
				},
				2 => { // Z ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					self[0]	= x*cos - y*sin; // x
					self[1]	= x*sin + y*cos; // y
				}, _ => {},
			}
			self
		}

		fn lerp_from( &mut self, a: &Vec3, b: &Vec3, t: f32 ) -> &mut Self{
			let ti = 1.0 - t; // Linear Interpolation : (1 - t) * v0 + t * v1;
			self[0] = a[0] * ti + b[0] * t;
			self[1] = a[1] * ti + b[1] * t;
			self[2] = a[2] * ti + b[2] * t;
			self
		}

}


//##############################################################################