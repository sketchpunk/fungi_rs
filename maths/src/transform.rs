#![allow(dead_code)]
use super::{ vec3::*, quat::* };

//##############################################################################
#[derive(Debug)]
pub struct Transform{ 
	pub pos: Vec3, 
	pub rot: Quat, 
	pub scl: Vec3,
}


//##############################################################################
impl Transform{
	pub fn new() -> Self{
		Transform{ 
			pos	: Vec3::new(), 
			rot	: Quat::new(), 
			scl	: Vec3::init( 1.0, 1.0, 1.0 ),
		}
	}

	pub fn from_pos( v: &Vec3 ) -> Self {
		Transform{ 
			pos	: v.clone(), 
			rot	: Quat::new(), 
			scl	: Vec3::init( 1.0, 1.0, 1.0 ),
		}
	}

	pub fn from_pos_rot( v: &Vec3, r: &Quat ) -> Self {
		Transform{ 
			pos	: v.clone(), 
			rot	: r.clone(), 
			scl	: Vec3::init( 1.0, 1.0, 1.0 ),
		}
	}


	////////////////////////////////////////////////////////////
	// Operations
	//////////////////////////////////////////////////////////// 
		pub fn add_t( &mut self, t: &Transform ) -> &Self { self.add( &t.pos, &t.rot, &t.scl ); self }

		pub fn add( &mut self, pos: &Vec3, rot: &Quat, scl: &Vec3 ) -> &Self{ 
			//POSITION - parent.position + ( parent.rotation * ( parent.scale * child.position ) )
			let mut v = Vec3::new();
			self.pos.add( &v.mul_from( &self.scl, pos ).transform_quat( &self.rot ) );
			self.rot.mul( rot );
			self.scl.mul( scl );
			self
		}

		pub fn add_rp( &mut self, pos: &Vec3, rot: &Quat ) -> &Self { 
			let mut v = Vec3::new();
			self.pos.add( &v.mul_from( &self.scl, pos ).transform_quat( &self.rot ) );
			self.rot.mul( rot );
			self
		}

		pub fn transform_vec3<'a>( &self, v: &'a mut Vec3 ) -> &'a mut Vec3{
			//GLSL - vecQuatRotation(model.rotation, a_position.xyz * model.scale) + model.position;
			v.mul( &self.scl ).transform_quat( &self.rot ).add( &self.pos )
		}

		pub fn invert_out<'a>( &self, out: &'a mut Transform ) -> &'a mut Transform{
			// Invert Rotation
			self.rot.invert_out( &mut out.rot );

			// Invert Scale
			out.scl[0] = if self.scl[0] != 0.0 { 1.0 / self.scl[0] }else{ 0.0 };
			out.scl[1] = if self.scl[1] != 0.0 { 1.0 / self.scl[1] }else{ 0.0 };
			out.scl[2] = if self.scl[2] != 0.0 { 1.0 / self.scl[2] }else{ 0.0 };

			// Invert Position : rotInv * ( invScl * invPos )
			self.pos
				.invert_out( &mut out.pos )
				.mul( &out.scl )
				.transform_quat( &out.rot );

			out
		}


	////////////////////////////////////////////////////////////
	// Methods
	//////////////////////////////////////////////////////////// 
		
		pub fn reset( &mut self ) -> &mut Self{
			self.pos.set( 0.0, 0.0, 0.0 );
			self.scl.set( 1.0, 1.0, 1.0 );
			self.rot.reset();
			self
		}

}

//##############################################################################
impl Default for Transform{
	fn default() -> Self{ Transform::new() }
}
