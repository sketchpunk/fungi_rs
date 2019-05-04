#![allow(dead_code)]

//##############################################################################
pub type Quat = [f32; 4];


//##############################################################################

pub trait QuatMethods{
	////////////////////////////////////////////////////////////
	// Static Functions
	////////////////////////////////////////////////////////////  
		fn new( ) -> Quat{ [ 0.0, 0.0, 0.0, 1.0 ] }

		fn from_axis_angle( axis: &[f32; 3], rad: f32 ) -> Quat{
			let rad_h	= rad * 0.5;
			let s 		= rad_h.sin();
			[ axis[ 0 ] * s, axis[ 1 ] * s, axis[ 2 ] * s, rad_h.cos() ]
		}



	////////////////////////////////////////////////////////////
	// Instance Getters Setters
	////////////////////////////////////////////////////////////
		//fn x( &mut self, v: f32) -> &mut Vec3;
		//fn y( &mut self, v: f32) -> &mut Vec3;
		//fn z( &mut self, v: f32) -> &mut Vec3;
		fn set_axis_angle( &mut self, axis: &[f32; 3], rad: f32 ) -> &mut Self;


	////////////////////////////////////////////////////////////
	// Instance Data Manipulation
	////////////////////////////////////////////////////////////
		//fn add( &mut self, v: &Vec3 ) -> &mut Vec3;
		//fn add_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3;
		fn rot_z( &mut self, rad: f32 ) -> &mut Self;
}


impl QuatMethods for Quat{
	////////////////////////////////////////////////////////////
	// Getters - Setters
	////////////////////////////////////////////////////////////
		
		// NOTE : Axis must be normalized
		fn set_axis_angle( &mut self, axis: &[f32; 3], rad: f32 ) -> &mut Self{
			let rad_h	= rad * 0.5;
			let s 		= rad_h.sin();

			self[ 0 ] = axis[ 0 ] * s;
			self[ 1 ] = axis[ 1 ] * s;
			self[ 2 ] = axis[ 2 ] * s;
			self[ 3 ] = rad_h.cos();

			self
		}


	////////////////////////////////////////////////////////////
	// Data Manipulation
	////////////////////////////////////////////////////////////
		fn rot_z( &mut self, rad: f32 ) -> &mut Self{
			let x 		= self[ 0 ];
			let y 		= self[ 1 ];
			let z 		= self[ 2 ];
			let w 		= self[ 3 ];
			let rad_h 	= rad * 0.5;
			let bz		= rad_h.sin();
			let bw 		= rad_h.cos();

			self[0] = x * bw + y * bz;
			self[1] = y * bw - x * bz;
			self[2] = z * bw + w * bz;
			self[3] = w * bw - z * bz;

			self
		}
}