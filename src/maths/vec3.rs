#![allow(dead_code)]

//##############################################################################
// CONSTANTS
pub const VEC3_ZERO : Vec3 = [ 0.0, 0.0, 0.0 ];
pub const VEC3_FWD  : Vec3 = [ 0.0, 0.0, 1.0 ];
pub const VEC3_UP   : Vec3 = [ 0.0, 1.0, 0.0 ];
pub const VEC3_LFT  : Vec3 = [ 1.0, 0.0, 0.0 ];


//##############################################################################
// TYPES
pub type Vec3 = [f32; 3];


//##############################################################################
// MAIN
pub trait Vec3Methods{
	////////////////////////////////////////////////////////////
	// Static Functions
	////////////////////////////////////////////////////////////  
		fn new( ) -> Vec3{ [0.0; 3] }
		
		fn init( x: f32, y:f32, z:f32 ) -> Vec3{ [ x, y, z ] }
		
		fn from( v: &Vec3 ) -> Vec3 { [ v[0], v[1], v[2] ] }

		fn dot( a: &Vec3, b: &Vec3 ) -> f32{ a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }


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


	////////////////////////////////////////////////////////////
	// Instance Data Manipulation
	////////////////////////////////////////////////////////////
		fn add( &mut self, v: &Vec3 ) -> &mut Vec3;
		fn add_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3;

		fn sub( &mut self, v: &Vec3 ) -> &mut Vec3;
		fn sub_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3;

		fn scale( &mut self, v: f32 ) -> &mut Vec3;

		fn norm( &mut self ) -> &mut Vec3;
		fn norm_out<'a>( &self, out: &'a mut Vec3) -> &'a mut Vec3;
}


impl Vec3Methods for Vec3{
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


	////////////////////////////////////////////////////////////
	// Data Manipulation
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

		fn norm( &mut self ) -> &mut Vec3{
			let mut mag = ( self[0]*self[0] + self[1]*self[1] + self[2]*self[2] ).sqrt();
			mag 	= 1.0 / mag;
			self[0]	*= mag;
			self[1]	*= mag;
			self[2]	*= mag;
			self
		}

		fn norm_out<'a>( &self, out: &'a mut Vec3) -> &'a mut Vec3{
			let mut mag = ( self[0]*self[0] + self[1]*self[1] + self[2]*self[2] ).sqrt();
			mag 	= 1.0 / mag;
			out[0]	= self[0] * mag;
			out[1]	= self[1] * mag;
			out[2]	= self[2] * mag;
			out
		}
}


//##############################################################################