#![allow(dead_code)]


//##############################################################################
// TYPES
pub type Mat4 = [f32; 16];


//##############################################################################
// MAIN
pub trait Mat4Trait{
	////////////////////////////////////////////////////////////
	// Static Functions
	////////////////////////////////////////////////////////////  
		fn new( ) -> Mat4{		[ 0.0, 0.0, 0.0, 0.0,  0.0, 0.0, 0.0, 0.0,  0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ] }
		fn identity() -> Mat4{	[ 1.0, 0.0, 0.0, 0.0,  0.0, 1.0, 0.0, 0.0,  0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0 ] }

		fn from_proj( fovy: f32, aspect: f32, near: f32, far: f32 ) -> Mat4{
			let f	= 1.0 / (fovy / 2.0).tan();
			let nf	= 1.0 / (near - far);
			[	f / aspect, 0.0, 0.0, 0.0,
				0.0, f, 0.0, 0.0,
				0.0, 0.0, (far + near) * nf, -1.0, 
				0.0, 0.0, (2.0 * far * near) * nf, 0.0 ]
		}

		/*
		fn from_quat( q: &[f32; 4] ) -> Mat4 {
			let x2 = q[0] + q[0];
			let y2 = q[1] + q[1];
			let z2 = q[2] + q[2];
			let xx = q[0] * x2;
			let xy = q[0] * y2;
			let xz = q[0] * z2;
			let yy = q[1] * y2;
			let yz = q[1] * z2;
			let zz = q[2] * z2;
			let wx = q[3] * x2;
			let wy = q[3] * y2;
			let wz = q[3] * z2;
			[	1.0 - (yy + zz),	xy + wz,			xz - wy,			0.0,
				xy - wz,			1.0 - (xx + zz),	yz + wx,			0.0,
				xz + wy,			yz - wx,			1.0 - (xx + yy),	0.0,
				0.0,				0.0,				0.0,				1.0 ]
		}
		*/

	////////////////////////////////////////////////////////////
	// Instance Getters Setters
	////////////////////////////////////////////////////////////
		fn proj_per( &mut self, fovy: f32, aspect: f32, near: f32, far: f32 ) -> &mut Self;
		fn proj_ortho( &mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32 ) -> &mut Self;

		fn from_quat_tran_scale( &mut self, q: &[f32;4], t: &[f32;3], s: &[f32;3] ) -> &mut Self;
		fn from_quat_tran( &mut self, q: &[f32;4], t: &[f32;3] ) -> &mut Self;
		fn from_quat( &mut self, q: &[f32;4] ) -> &mut Self;

		fn from_invert( &mut self, m: &Mat4 ) -> &mut Mat4;
		fn from_mul( &mut self, a: &Mat4, b: &Mat4) -> &mut Mat4;

		//fn invert_out<'a>( &self, out: &'a mut Mat4 ) -> &'a mut Mat4;

	////////////////////////////////////////////////////////////
	// Instance Data Manipulation
	////////////////////////////////////////////////////////////
		//fn add( &mut self, v: &Vec3 ) -> &mut Vec3;
		//fn add_from( &mut self, a: &Vec3, b: &Vec3 ) -> &mut Vec3;
}


impl Mat4Trait for Mat4{
	////////////////////////////////////////////////////////////
	// Getters - Setters
	////////////////////////////////////////////////////////////
		fn proj_per( &mut self, fovy: f32, aspect: f32, near: f32, far: f32 ) -> &mut Self{
			let f	= 1.0 / (fovy / 2.0).tan();
			let nf	= 1.0 / (near - far);
			self[0]		= f / aspect;
			self[1]		= 0.0;
			self[2]		= 0.0;
			self[3]		= 0.0;
			self[4]		= 0.0;
			self[5]		= f;
			self[6]		= 0.0;
			self[7]		= 0.0;
			self[8]		= 0.0;
			self[9]		= 0.0;
			self[10]	= (far + near) * nf;
			self[11]	= -1.0;
			self[12]	= 0.0;
			self[13]	= 0.0;
			self[14]	= (2.0 * far * near) * nf;
			self[15]	= 0.0;
			self
		}

		fn proj_ortho( &mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32 ) -> &mut Self{
			let lr = 1.0 / (left - right);
			let bt = 1.0 / (bottom - top);
			let nf = 1.0 / (near - far);
			self[0]		= -2.0 * lr;
			self[1]		= 0.0;
			self[2]		= 0.0;
			self[3]		= 0.0;
			self[4]		= 0.0;
			self[5]		= -2.0 * bt;
			self[6]		= 0.0;
			self[7]		= 0.0;
			self[8]		= 0.0;
			self[9]		= 0.0;
			self[10]	= 2.0 * nf;
			self[11]	= 0.0;
			self[12]	= (left + right) * lr;
			self[13]	= (top + bottom) * bt;
			self[14]	= (far + near) * nf;
			self[15]	= 1.0;
			self
		}

		fn from_quat_tran_scale( &mut self, q: &[f32;4], t: &[f32;3], s: &[f32;3] ) -> &mut Self{
			let x2 = q[0] + q[0];
			let y2 = q[1] + q[1];
			let z2 = q[2] + q[2];
			let xx = q[0] * x2;
			let xy = q[0] * y2;
			let xz = q[0] * z2;
			let yy = q[1] * y2;
			let yz = q[1] * z2;
			let zz = q[2] * z2;
			let wx = q[3] * x2;
			let wy = q[3] * y2;
			let wz = q[3] * z2;
			self[0] = (1.0 - (yy + zz)) * s[0];
			self[1] = (xy + wz) * s[0];
			self[2] = (xz - wy) * s[0];
			self[3] = 0.0;
			self[4] = (xy - wz) * s[1];
			self[5] = (1.0 - (xx + zz)) * s[1];
			self[6] = (yz + wx) * s[1];
			self[7] = 0.0;
			self[8] = (xz + wy) * s[2];
			self[9] = (yz - wx) * s[2];
			self[10] = (1.0 - (xx + yy)) * s[2];
			self[11] = 0.0;
			self[12] = t[0];
			self[13] = t[1];
			self[14] = t[2];
			self[15] = 1.0;
			self
		}

		// https://github.com/toji/gl-matrix/blob/master/src/gl-matrix/mat4.js
		fn from_quat_tran( &mut self, q: &[f32;4], t: &[f32;3] ) -> &mut Self{
			let x2 = q[0] + q[0];
			let y2 = q[1] + q[1];
			let z2 = q[2] + q[2];
			let xx = q[0] * x2;
			let xy = q[0] * y2;
			let xz = q[0] * z2;
			let yy = q[1] * y2;
			let yz = q[1] * z2;
			let zz = q[2] * z2;
			let wx = q[3] * x2;
			let wy = q[3] * y2;
			let wz = q[3] * z2;
			self[0]		= 1.0 - (yy + zz);
			self[1]		= xy + wz;
			self[2]		= xz - wy;
			self[3]		= 0.0;
			self[4]		= xy - wz;
			self[5]		= 1.0 - (xx + zz);
			self[6]		= yz + wx;
			self[7]		= 0.0;
			self[8]		= xz + wy;
			self[9]		= yz - wx;
			self[10] 	= 1.0 - (xx + yy);
			self[11]	= 0.0;
			self[12]	= t[0];
			self[13]	= t[1];
			self[14]	= t[2];
			self[15]	= 1.0;
			self
		}

		fn from_quat( &mut self, q: &[f32; 4] ) -> &mut Self {
			let x2 = q[0] + q[0];
			let y2 = q[1] + q[1];
			let z2 = q[2] + q[2];
			let xx = q[0] * x2;
			let xy = q[0] * y2;
			let xz = q[0] * z2;
			let yy = q[1] * y2;
			let yz = q[1] * z2;
			let zz = q[2] * z2;
			let wx = q[3] * x2;
			let wy = q[3] * y2;
			let wz = q[3] * z2;
			self[0]		= 1.0 - (yy + zz);
			self[1]		= xy + wz;
			self[2]		= xz - wy;
			self[3]		= 0.0;
			self[4]		= xy - wz;
			self[5]		= 1.0 - (xx + zz);
			self[6]		= yz + wx;
			self[7]		= 0.0;
			self[8]		= xz + wy;
			self[9]		= yz - wx;
			self[10] 	= 1.0 - (xx + yy);
			self[11]	= 0.0;
			self[12]	= 0.0;
			self[13]	= 0.0;
			self[14]	= 0.0;
			self[15]	= 1.0;
			self
		}

		fn from_invert( &mut self, m: &Mat4 ) -> &mut Mat4 {
			let b00 = m[0]	* m[5]	- m[1]	* m[4];
			let b01 = m[0]	* m[6]	- m[2]	* m[4];
			let b02 = m[0]	* m[7]	- m[3]	* m[4];
			let b03 = m[1]	* m[6]	- m[2]	* m[5];
			let b04 = m[1]	* m[7]	- m[3]	* m[5];
			let b05 = m[2]	* m[7]	- m[3]	* m[6];
			let b06 = m[8]	* m[13]	- m[9]	* m[12];
			let b07 = m[8]	* m[14]	- m[10]	* m[12];
			let b08 = m[8]	* m[15]	- m[11]	* m[12];
			let b09 = m[9]	* m[14]	- m[10]	* m[13];
			let b10 = m[9]	* m[15]	- m[11]	* m[13];
			let b11 = m[10]	* m[15]	- m[11]	* m[14];

			// Calculate the determinan
			let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
			if det != 0.0 {
				det		= 1.0 / det;
				self[0]	= (m[5]	* b11 - m[6] * b10 + m[7] * b09) * det;
				self[1]	= (m[2] * b10 - m[1] * b11 - m[3] * b09) * det;
				self[2]	= (m[13] * b05 - m[14] * b04 + m[15] * b03) * det;
				self[3]	= (m[10] * b04 - m[9] * b05 - m[11] * b03) * det;
				self[4]	= (m[6] * b08 - m[4] * b11 - m[7] * b07) * det;
				self[5]	= (m[0] * b11 - m[2] * b08 + m[3] * b07) * det;
				self[6]	= (m[14] * b02 - m[12] * b05 - m[15] * b01) * det;
				self[7]	= (m[8] * b05 - m[10] * b02 + m[11] * b01) * det;
				self[8]	= (m[4] * b10 - m[5] * b08 + m[7] * b06) * det;
				self[9]	= (m[1] * b08 - m[0] * b10 - m[3] * b06) * det;
				self[10] = (m[12] * b04 - m[13] * b02 + m[15] * b00) * det;
				self[11] = (m[9] * b02 - m[8] * b04 - m[11] * b00) * det;
				self[12] = (m[5] * b07 - m[4] * b09 - m[6] * b06) * det;
				self[13] = (m[0] * b09 - m[1] * b07 + m[2] * b06) * det;
				self[14] = (m[13] * b01 - m[12] * b03 - m[14] * b00) * det;
				self[15] = (m[8] * b03 - m[9] * b01 + m[10] * b00) * det;
			}
			self
		}

		fn from_mul( &mut self, a: &Mat4, b: &Mat4) -> &mut Mat4 { 
			let a00 = a[0];
			let a01 = a[1];
			let a02 = a[2];
			let a03 = a[3];
			let a10 = a[4];
			let a11 = a[5]; 
			let a12 = a[6];
			let a13 = a[7];
			let a20 = a[8];
			let a21 = a[9]; 
			let a22 = a[10]; 
			let a23 = a[11];
			let a30 = a[12]; 
			let a31 = a[13]; 
			let a32 = a[14]; 
			let a33 = a[15];

			// Cache only the current line of the second matrix
			let mut b0 = b[0];
			let mut b1 = b[1];
			let mut b2 = b[2];
			let mut b3 = b[3];
			self[0] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
			self[1] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
			self[2] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
			self[3] = b0*a03 + b1*a13 + b2*a23 + b3*a33;

			b0 = b[4]; 
			b1 = b[5]; 
			b2 = b[6]; 
			b3 = b[7];
			self[4] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
			self[5] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
			self[6] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
			self[7] = b0*a03 + b1*a13 + b2*a23 + b3*a33;

			b0 = b[8]; 
			b1 = b[9]; 
			b2 = b[10]; 
			b3 = b[11];
			self[8] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
			self[9] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
			self[10] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
			self[11] = b0*a03 + b1*a13 + b2*a23 + b3*a33;

			b0 = b[12]; 
			b1 = b[13]; 
			b2 = b[14]; 
			b3 = b[15];
			self[12] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
			self[13] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
			self[14] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
			self[15] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
			self
		}


		/*
		fn invert_out<'a>( &self, out: &'a mut Mat4 ) -> &'a mut Mat4 {
			let b00 = self[0] * self[5] - self[1] * self[4];
			let b01 = self[0] * self[6] - self[2] * self[4];
			let b02 = self[0] * self[7] - self[3] * self[4];
			let b03 = self[1] * self[6] - self[2] * self[5];
			let b04 = self[1] * self[7] - self[3] * self[5];
			let b05 = self[2] * self[7] - self[3] * self[6];
			let b06 = self[8] * self[13] - self[9] * self[12];
			let b07 = self[8] * self[14] - self[10] * self[12];
			let b08 = self[8] * self[15] - self[11] * self[12];
			let b09 = self[9] * self[14] - self[10] * self[13];
			let b10 = self[9] * self[15] - self[11] * self[13];
			let b11 = self[10] * self[15] - self[11] * self[14];

			// Calculate the determinant
			let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
			if det != 0.0 {
				det		= 1.0 / det;
				out[0]	= (self[5] * b11 - self[6] * b10 + self[7] * b09) * det;
				out[1]	= (self[2] * b10 - self[1] * b11 - self[3] * b09) * det;
				out[2]	= (self[13] * b05 - self[14] * b04 + self[15] * b03) * det;
				out[3]	= (self[10] * b04 - self[9] * b05 - self[11] * b03) * det;
				out[4]	= (self[6] * b08 - self[4] * b11 - self[7] * b07) * det;
				out[5]	= (self[0] * b11 - self[2] * b08 + self[3] * b07) * det;
				out[6]	= (self[14] * b02 - self[12] * b05 - self[15] * b01) * det;
				out[7]	= (self[8] * b05 - self[10] * b02 + self[11] * b01) * det;
				out[8]	= (self[4] * b10 - self[5] * b08 + self[7] * b06) * det;
				out[9]	= (self[1] * b08 - self[0] * b10 - self[3] * b06) * det;
				out[10]	= (self[12] * b04 - self[13] * b02 + self[15] * b00) * det;
				out[11]	= (self[9] * b02 - self[8] * b04 - self[11] * b00) * det;
				out[12]	= (self[5] * b07 - self[4] * b09 - self[6] * b06) * det;
				out[13]	= (self[0] * b09 - self[1] * b07 + self[2] * b06) * det;
				out[14]	= (self[13] * b01 - self[12] * b03 - self[14] * b00) * det;
				out[15]	= (self[8] * b03 - self[9] * b01 + self[10] * b00) * det;
			}
			out
		}
		*/
	////////////////////////////////////////////////////////////
	// Data Manipulation
	////////////////////////////////////////////////////////////

}


//##############################################################################