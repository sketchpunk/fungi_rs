#![allow(dead_code)]

use crate::maths::{ mat4::* };
use crate::App;
use crate::ecs::{ ICom, IStorage, DenseVec, BitSet };

//############################################################
#[derive(Debug, Default)]
pub struct Camera{
	pub view_mat	: Mat4,		// Inverse of World Transfrom Matrix
	pub	proj_mat 	: Mat4,		// Camera Projection Matrix
	pub pv_mat		: Mat4,		// Projection & View together
	//pub proj_inv_mat	: Mat4,	// Used to Transform Rays from Sceen Space to World Space
}

//TODO, This type is very Rare, Prob Best to create a HashMap Storage.
impl ICom for Camera{ type Storage = DenseVec<Self>; }

impl Camera{
	pub fn new() -> Self{  
		Camera{
			view_mat	: Mat4::new(),
			proj_mat	: Mat4::new(),
			pv_mat		: Mat4::new(),
		}
	}

	pub fn with_proj( fovy: f32, aspect: f32, near: f32, far: f32 ) -> Self{
		Camera{
			proj_mat	: Mat4::from_proj( fovy, aspect, near, far ),
			view_mat	: Mat4::new(),
			pv_mat		: Mat4::new(),
		}	
	}
}

//###############################################################
#[allow(unused_variables)]
pub fn camera_sys_fn( app: &App ){
	use super::node::Node;

	let ns		= app.ecs.cm.borrow::<Node>();
	let mut cs	= app.ecs.cm.borrow_mut::<Camera>();
	let idx_ary	= cs.get_entities();

	for i in idx_ary {
		let n = ns.get( i );

		if n.is_mod {
			let c = cs.get_mut( i );
			c.view_mat.from_invert( &n.matrix );
			c.pv_mat.from_mul( &c.proj_mat, &c.view_mat );
		}
	}
}
