#![allow(dead_code)]
use super::vec3::*;


//##############################################################################
pub type Quat = [f32; 4];


//##############################################################################

pub trait QuatTrait{
	////////////////////////////////////////////////////////////
	// Static Functions
	////////////////////////////////////////////////////////////  
		fn new( ) -> Quat{ [ 0.0, 0.0, 0.0, 1.0 ] }

		fn from_axis_angle( axis: &[f32; 3], rad: f32 ) -> Quat{
			let rad_h	= rad * 0.5;
			let s 		= rad_h.sin();
			[ axis[ 0 ] * s, axis[ 1 ] * s, axis[ 2 ] * s, rad_h.cos() ]
		}

		fn dot( a: &Quat, b: &Quat ) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

		fn from_mul( a: &Quat, b: &Quat ) -> Quat{
			let ax = a[0];
			let ay = a[1];
			let az = a[2];
			let aw = a[3];
			let bx = b[0];
			let by = b[1];
			let bz = b[2];
			let bw = b[3];
			[ 	ax * bw + aw * bx + ay * bz - az * by,
				ay * bw + aw * by + az * bx - ax * bz,
				az * bw + aw * bz + ax * by - ay * bx,
				aw * bw - ax * bx - ay * by - az * bz ]
		}

		// THIS ISN"T Polar, Its kinda broken
		// Need to convert PolarCoords to X,Y, Then take that position - target, then use
		// the results in a Look Rotation call.
		fn from_polar( x: f32, y: f32 ) -> Quat{
			let xx = x * 0.01745329251;	// Convert Deg to Rad
			let yy = y * 0.01745329251;
			let c1 = (xx*0.5).cos();
			let c2 = (yy*0.5).cos();
			let s1 = (xx*0.5).sin();
			let s2 = (yy*0.5).sin();
			[ s1 * c2, c1 * s2, -s1 * s2 , c1 * c2 ]
		}


	////////////////////////////////////////////////////////////
	// Instance Getters Setters
	////////////////////////////////////////////////////////////
		fn copy( &mut self, q: &Quat ) -> &mut Self;

		fn set_axis_angle( &mut self, axis: &[f32; 3], rad: f32 ) -> &mut Self;
		fn get_axis_angle( &mut self ) -> [f32; 4];
		fn get_axis_out<'a>( &mut self, v: &'a mut [f32; 3] ) -> &'a mut [f32; 3];

		fn axis_from( &mut self, x_axis: &Vec3, y_axis: &Vec3, z_axis: &Vec3 ) -> &mut Self;

		fn mat4_from( &mut self, m: &[f32; 16] ) -> &mut Self;

		fn euler_yxz_from( &mut self, x: f32, y: f32, z: f32) -> &mut Self;
		fn get_euler( &self ) -> [f32; 3];
	
		fn lerp_from( &mut self, a: &Quat, b: &Quat, t: f32, do_norm: bool) -> &mut Self;
		fn slerp_from( &mut self, a: &Quat, b: &Quat, t:f32, do_norm: bool) -> &mut Self;

		fn set_polar_rad( &mut self, x: f32, y: f32 ) -> &mut Self;


	////////////////////////////////////////////////////////////
	// Operations
	////////////////////////////////////////////////////////////
		fn mul( &mut self, q: &Quat ) -> &mut Self;
		fn pmul( &mut self, q: &Quat ) -> &mut Self;

		fn norm( &mut self ) -> &mut Self;

		fn invert( &mut self ) -> &mut Self;
		fn invert_out<'a>( &self, q: &'a mut Quat ) -> &'a mut Quat;

		fn look( &mut self, v_dir: &Vec3, v_up: &Vec3 ) -> &mut Self;
		fn rot_to( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Self;

		fn negate( &mut self ) -> &mut Self;
		fn conjugate( &mut self ) -> &mut Self;

		fn reset( &mut self ) -> &mut Self;


	////////////////////////////////////////////////////////////
	// Instance Data Manipulation
	////////////////////////////////////////////////////////////
	
		fn rot_x( &mut self, rad: f32 ) -> &mut Self;
		fn rot_y( &mut self, rad: f32 ) -> &mut Self;
		fn rot_z( &mut self, rad: f32 ) -> &mut Self;

}


impl QuatTrait for Quat{
	////////////////////////////////////////////////////////////
	// Getters - Setters
	////////////////////////////////////////////////////////////
		fn copy( &mut self, q: &Quat ) -> &mut Self{
			self[0] = q[0];
			self[1] = q[1];
			self[2] = q[2];
			self[3] = q[3];
			self
		}

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

		fn get_axis_angle( &mut self ) -> [f32; 4]{
			if self[3] > 1.0 { self.norm(); }

			let angle	= 2.0 * self[3].acos();
			let s		= ( 1.0 - self[3] * self[3] ).sqrt();

			if s < 0.001 {
				[ 1.0 , 0.0 , 0.0, 0.0 ]
			}else{
				[ self[0] / s, self[1] / s, self[2] / s, angle ]
			}
		}

		fn get_axis_out<'a>( &mut self, v: &'a mut [f32; 3] ) -> &'a mut [f32; 3]{
			if self[3] > 1.0 { self.norm(); }
			let s = ( 1.0 - self[3] * self[3] ).sqrt();

			if s < 0.001 {
				v[0] = 1.0;
				v[1] = 0.0;
				v[2] = 0.0;
			}else{
				v[0] = self[0] / s;
				v[1] = self[1] / s;
				v[2] = self[2] / s;
			}
			v
		}

		fn axis_from( &mut self, x_axis: &Vec3, y_axis: &Vec3, z_axis: &Vec3 ) -> &mut Self{
			// Mat3 to Quaternion
			let m00 = x_axis[0]; 
			let m01 = x_axis[1]; 
			let m02 = x_axis[2];
			let m10 = y_axis[0];
			let m11 = y_axis[1];
			let m12 = y_axis[2];
			let m20 = z_axis[0];
			let m21 = z_axis[1];
			let m22 = z_axis[2];
			let t   = m00 + m11 + m22;
			let mut s;

			if t > 0.0 {
				s = ( t + 1.0 ).sqrt();
				self[3] = s * 0.5;
				s = 0.5 / s;
				self[0] = (m12 - m21) * s;
				self[1] = (m20 - m02) * s;
				self[2] = (m01 - m10) * s;
			}else if (m00 >= m11) && (m00 >= m22) {
				s = ( 1.0 + m00 - m11 - m22 ).sqrt();
				self[0] = 0.5 * s;
				s = 0.5 / s;

				self[1] = (m01 + m10) * s;
				self[2] = (m02 + m20) * s;
				self[3] = (m12 - m21) * s;
			}else if m11 > m22 {
				s = ( 1.0 + m11 - m00 - m22 ).sqrt();
				self[1] = 0.5 * s;
				s = 0.5 / s;
				self[0] = (m10 + m01) * s;
				self[2] = (m21 + m12) * s;
				self[3] = (m20 - m02) * s;
			}else{
				s = ( 1.0 + m22 - m00 - m11 ).sqrt();
				self[2] = 0.5 * s;
				s = 0.5 / s;
				self[0] = (m20 + m02) * s;
				self[1] = (m21 + m12) * s;
				self[3] = (m01 - m10) * s;
			}
			self
		}

		// https://github.com/toji/gl-matrix/blob/master/src/mat4.js
		// Algorithm taken from http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/index.htm
		fn mat4_from( &mut self, m: &[f32; 16] ) -> &mut Self{
			let trace = m[0] + m[5] + m[10];
			if trace > 0.0 {
				let s = ( trace + 1.0 ).sqrt() * 2.0;
				self[3] = 0.25 * s;
				self[0] = ( m[6] - m[9] ) / s;
				self[1] = ( m[8] - m[2] ) / s;
				self[2] = ( m[1] - m[4] ) / s;
			} else if (m[0] > m[5] ) && (m[0] > m[10]) {
				let s = ( 1.0 + m[0] - m[5] - m[10] ).sqrt() * 2.0;
				self[3] = ( m[6] - m[9] ) / s;
				self[0] = 0.25 * s;
				self[1] = ( m[1] + m[4] ) / s;
				self[2] = ( m[8] + m[2] ) / s;
			} else if m[5] > m[10] {
				let s = ( 1.0 + m[5] - m[0] - m[10] ).sqrt() * 2.0;
				self[3] = ( m[8] - m[2] ) / s;
				self[0] = ( m[1] + m[4] ) / s;
				self[1] = 0.25 * s;
				self[2] = ( m[6] + m[9] ) / s;
			} else {
				let s = ( 1.0 + m[10] - m[0] - m[5] ).sqrt() * 2.0;
				self[3] = ( m[1] - m[4] ) / s;
				self[0] = ( m[8] + m[2] ) / s;
				self[1] = ( m[6] + m[9] ) / s;
				self[2] = 0.25 * s;
			}
			self
		}

		//https://github.com/mrdoob/three.js/blob/dev/src/math/Quaternion.js
		fn euler_yxz_from( &mut self, x: f32, y: f32, z: f32) -> &mut Self{ //, order="YXZ" 
			let c1 = (x*0.5).cos();
			let c2 = (y*0.5).cos();
			let c3 = (z*0.5).cos();
			let s1 = (x*0.5).sin();
			let s2 = (y*0.5).sin();
			let s3 = (z*0.5).sin();

			self[0] = s1 * c2 * c3 + c1 * s2 * s3;
			self[1] = c1 * s2 * c3 - s1 * c2 * s3;
			self[2] = c1 * c2 * s3 - s1 * s2 * c3;
			self[3] = c1 * c2 * c3 + s1 * s2 * s3;
			self
			/*
				case 'XYZ':			
					out[0] = s1 * c2 * c3 + c1 * s2 * s3;
					out[1] = c1 * s2 * c3 - s1 * c2 * s3;
					out[2] = c1 * c2 * s3 + s1 * s2 * c3;
					out[3] = c1 * c2 * c3 - s1 * s2 * s3;
				case 'YXZ':
					out[0] = s1 * c2 * c3 + c1 * s2 * s3;
					out[1] = c1 * s2 * c3 - s1 * c2 * s3;
					out[2] = c1 * c2 * s3 - s1 * s2 * c3;
					out[3] = c1 * c2 * c3 + s1 * s2 * s3;
				case 'ZXY':
					out[0] = s1 * c2 * c3 - c1 * s2 * s3;
					out[1] = c1 * s2 * c3 + s1 * c2 * s3;
					out[2] = c1 * c2 * s3 + s1 * s2 * c3;
					out[3] = c1 * c2 * c3 - s1 * s2 * s3;
				case 'ZYX':
					out[0] = s1 * c2 * c3 - c1 * s2 * s3;
					out[1] = c1 * s2 * c3 + s1 * c2 * s3;
					out[2] = c1 * c2 * s3 - s1 * s2 * c3;
					out[3] = c1 * c2 * c3 + s1 * s2 * s3;
				case 'YZX':
					out[0] = s1 * c2 * c3 + c1 * s2 * s3;
					out[1] = c1 * s2 * c3 + s1 * c2 * s3;
					out[2] = c1 * c2 * s3 - s1 * s2 * c3;
					out[3] = c1 * c2 * c3 - s1 * s2 * s3;
				case 'XZY':
					out[0] = s1 * c2 * c3 - c1 * s2 * s3;
					out[1] = c1 * s2 * c3 - s1 * c2 * s3;
					out[2] = c1 * c2 * s3 + s1 * s2 * c3;
					out[3] = c1 * c2 * c3 + s1 * s2 * s3;
			*/
		}

		/**/
		//http://bediyap.com/programming/convert-quaternion-to-euler-rotations/
		//http://schteppe.github.io/cannon.js/docs/files/src_math_Quaternion.js.html
		fn get_euler( &self ) -> [f32; 3]{ //order="YZX"
			let x		= self[0];
			let y		= self[1];
			let z		= self[2];
			let w		= self[3];
			let test	= x * y + z * w;
			let mut pitch	= 0.0;
			let mut yaw		= 0.0;
			let mut roll	= 0.0;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// singularity at north pole
			if test > 0.499 {
				pitch	= 2.0 * x.atan2(w);
				yaw		= std::f32::consts::PI * 0.5;
				roll	= 0.0;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// singularity at south pole
			if test < -0.499 {
				pitch	= -2.0 * x.atan2(w);
				yaw		= -std::f32::consts::PI * 0.5;
				roll	= 0.0;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if pitch.is_nan() {
				let sqz	= z * z;
				roll	= ( 2.0*x*w - 2.0*y*z ).atan2( 1.0 - 2.0*x*x - 2.0*sqz ); // bank
				pitch	= ( 2.0*y*w - 2.0*x*z ).atan2( 1.0 - 2.0*y*y - 2.0*sqz ); // Heading
				yaw		= ( 2.0*test ).asin(); // attitude
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			[ roll, pitch, yaw ]
		}
		

		fn lerp_from( &mut self, a: &Quat, b: &Quat, t: f32, do_norm: bool) -> &mut Self{
			let tm1 = 1.0 - t;
			self[0]	= a[0] * tm1 + b[0] * t;
			self[1]	= a[1] * tm1 + b[1] * t;
			self[2]	= a[2] * tm1 + b[2] * t;
			self[3]	= a[3] * tm1 + b[3] * t;
			if do_norm { self.norm(); }
			self
		}


		fn slerp_from( &mut self, a: &Quat, b: &Quat, t:f32, do_norm: bool) -> &mut Self{
			// benchmarks: http://jsperf.com/quaternion-slerp-implementations
			// calc cosine
			let mut cosom = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
			let mut b_sign = 1.0;
			let scale0;
			let scale1;
			
			// adjust signs (if necessary)
			if cosom < 0.0  {
				cosom	= -cosom;
				b_sign	= -1.0;
			}

			// calculate coefficients
			if (1.0 - cosom) > 0.000001 {
				// standard case (slerp)
				let omega	= cosom.acos();
				let sinom	= omega.sin();
				scale0		= ((1.0 - t) * omega).sin() / sinom;
				scale1		= (t * omega).sin() / sinom;
			}else{
				// "from" and "to" quaternions are very close
				//  ... so we can do a linear interpolation
				scale0 = 1.0 - t;
				scale1 = t;
			}

			// calculate final values
			self[0] = scale0 * a[0] + scale1 * ( b[0] * b_sign );
			self[1] = scale0 * a[1] + scale1 * ( b[1] * b_sign );
			self[2] = scale0 * a[2] + scale1 * ( b[2] * b_sign );
			self[3] = scale0 * a[3] + scale1 * ( b[3] * b_sign );
			if do_norm { self.norm(); }
			self
		}

		// TODO, THIS IS WRONG... THIS IS JUST Eular YX rotation.
		fn set_polar_rad( &mut self, x: f32, y: f32 ) -> &mut Self{
			let c1 = (x*0.5).cos();
			let c2 = (y*0.5).cos();
			let s1 = (x*0.5).sin();
			let s2 = (y*0.5).sin();

			self[0] =  s1 * c2;
			self[1] =  c1 * s2;
			self[2] = -s1 * s2;
			self[3] =  c1 * c2;
			self
		}

	////////////////////////////////////////////////////////////
	// Operations
	////////////////////////////////////////////////////////////
		fn mul( &mut self, q: &Quat ) -> &mut Self{
			let ax = self[0];
			let ay = self[1];
			let az = self[2];
			let aw = self[3];
			let bx = q[0];
			let by = q[1];
			let bz = q[2];
			let bw = q[3];
			self[0] = ax * bw + aw * bx + ay * bz - az * by;
			self[1] = ay * bw + aw * by + az * bx - ax * bz;
			self[2] = az * bw + aw * bz + ax * by - ay * bx;
			self[3] = aw * bw - ax * bx - ay * by - az * bz;
			self
		}

		fn pmul( &mut self, q: &Quat ) -> &mut Self{
			let ax = q[0];
			let ay = q[1];
			let az = q[2];
			let aw = q[3];
			let bx = self[0];
			let by = self[1];
			let bz = self[2];
			let bw = self[3];
			self[0] = ax * bw + aw * bx + ay * bz - az * by;
			self[1] = ay * bw + aw * by + az * bx - ax * bz;
			self[2] = az * bw + aw * bz + ax * by - ay * bx;
			self[3] = aw * bw - ax * bx - ay * by - az * bz;
			self
		}

		fn norm( &mut self ) -> &mut Self{
			let mut len = self[0]*self[0] + self[1]*self[1] + self[2]*self[2] + self[3]*self[3];
			if len > 0.0 {
				len = 1.0 / len.sqrt();
				self[0] *= len;
				self[1] *= len;
				self[2] *= len;
				self[3] *= len;
			}
			self
		}

		fn invert( &mut self ) -> &mut Self{
			let mut dot	= self[0]*self[0] + self[1]*self[1] + self[2]*self[2] + self[3]*self[3];
			
			if dot == 0.0 { 
				self[0] = 0.0;
				self[1] = 0.0;
				self[2] = 0.0;
				self[3] = 0.0;
			}else{
				dot = 1.0 / dot;
				self[0]	*= -dot;
				self[1]	*= -dot;
				self[2]	*= -dot;
				self[3]	*=  dot;
			}
			self
		}

		fn invert_out<'a>( &self, q: &'a mut Quat ) -> &'a mut Quat{
			let mut dot	= self[0]*self[0] + self[1]*self[1] + self[2]*self[2] + self[3]*self[3];
			
			if dot == 0.0 { 
				q[0] = 0.0;
				q[1] = 0.0;
				q[2] = 0.0;
				q[3] = 0.0;
			}else{
				dot = 1.0 / dot;
				q[0] = -self[0] * dot;
				q[1] = -self[1] * dot;
				q[2] = -self[2] * dot;
				q[3] =  self[3] * dot;
			}
			q
		}

		fn negate( &mut self ) -> &mut Self{
			self[0] = -self[0];
			self[1] = -self[1];
			self[2] = -self[2];
			self[3] = -self[3];
			self
		}

		fn conjugate( &mut self ) -> &mut Self{
			self[0] = -self[0];
			self[1] = -self[1];
			self[2] = -self[2];
			self[3] =  self[3];
			self
		}

		fn reset( &mut self ) -> &mut Self{ self[0] = 0.0; self[1] = 0.0; self[2] = 0.0; self[3] = 1.0; self }


	////////////////////////////////////////////////////////////
	// Data Manipulation
	////////////////////////////////////////////////////////////
		fn rot_x( &mut self, rad: f32 ) -> &mut Self{
			let x 		= self[ 0 ];
			let y 		= self[ 1 ];
			let z 		= self[ 2 ];
			let w 		= self[ 3 ];
			let rad_h 	= rad * 0.5;
			let bx		= rad_h.sin();
			let bw		= rad_h.cos();

			self[0] = x * bw + w * bx;
			self[1] = y * bw + z * bx;
			self[2] = z * bw - y * bx;
			self[3] = w * bw - x * bx;
			self
		}

		fn rot_y( &mut self, rad: f32 ) -> &mut Self{
			let x 		= self[ 0 ];
			let y 		= self[ 1 ];
			let z 		= self[ 2 ];
			let w 		= self[ 3 ];
			let rad_h 	= rad * 0.5;
			let by		= rad_h.sin();
			let bw 		= rad_h.cos();

			self[0] = x * bw - z * by;
			self[1] = y * bw + w * by;
			self[2] = z * bw + x * by;
			self[3] = w * bw - y * by;
			self
		}

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

		// Ported to JS from C# example at https://pastebin.com/ubATCxJY
		// [[ Note ]] if Dir and Up are equal, a roll happends. Need to find a way to fix this.
		fn look( &mut self, v_dir: &Vec3, v_up: &Vec3 ) -> &mut Self{
			let mut z_axis = Vec3::from_copy( v_dir );						// Forward
			let mut x_axis = Vec3::from_cross( v_up, &z_axis.norm() );		// Left
			let mut y_axis = Vec3::from_cross( &z_axis, &x_axis.norm() );	// Up
			y_axis.norm();

			// Mat3 to Quaternion
			let m00 = x_axis[0]; 
			let m01 = x_axis[1]; 
			let m02 = x_axis[2];
			let m10 = y_axis[0];
			let m11 = y_axis[1];
			let m12 = y_axis[2];
			let m20 = z_axis[0];
			let m21 = z_axis[1];
			let m22 = z_axis[2];
			let t   = m00 + m11 + m22;
			let mut s;

			if t > 0.0 {
				s = ( t + 1.0 ).sqrt();
				self[3] = s * 0.5;
				s = 0.5 / s;
				self[0] = (m12 - m21) * s;
				self[1] = (m20 - m02) * s;
				self[2] = (m01 - m10) * s;
			}else if (m00 >= m11) && (m00 >= m22) {
				s = ( 1.0 + m00 - m11 - m22 ).sqrt();
				self[0] = 0.5 * s;
				s = 0.5 / s;

				self[1] = (m01 + m10) * s;
				self[2] = (m02 + m20) * s;
				self[3] = (m12 - m21) * s;
			}else if m11 > m22 {
				s = ( 1.0 + m11 - m00 - m22 ).sqrt();
				self[1] = 0.5 * s;
				s = 0.5 / s;
				self[0] = (m10 + m01) * s;
				self[2] = (m21 + m12) * s;
				self[3] = (m20 - m02) * s;
			}else{
				s = ( 1.0 + m22 - m00 - m11 ).sqrt();
				self[2] = 0.5 * s;
				s = 0.5 / s;
				self[0] = (m20 + m02) * s;
				self[1] = (m21 + m12) * s;
				self[3] = (m01 - m10) * s;
			}
			self
		}

		//Using unit vectors, Shortest rotation from Direction A to Direction B
		//http://glmatrix.net/docs/quat.js.html#line548
		//http://physicsforgames.blogspot.com/2010/03/quaternion-tricks.html
		fn rot_to( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Self{
			let dot = Vec3::dot( a, b );

		    if dot < -0.999999 {
		      let mut tmp = Vec3::from_cross( &VEC3_L, a );
		      if tmp.len() < 0.000001 { tmp.cross_from( &VEC3_UP, a ); }

		      self.set_axis_angle( &tmp.norm(), std::f32::consts::PI );
		    }else if dot > 0.999999 {
		      self[0] = 0.0;
		      self[1] = 0.0;
		      self[2] = 0.0;
		      self[3] = 1.0;
		    }else{
		      let v   = Vec3::from_cross( a, b );
		      self[0] = v[0];
		      self[1] = v[1];
		      self[2] = v[2];
		      self[3] = 1.0 + dot;
		      self.norm();
		    }
		    self
		}
}