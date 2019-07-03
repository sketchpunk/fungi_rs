#![allow(dead_code)]

use crate::maths::{ Transform, vec3::*, mat4::*, quat::* };
use crate::App;
use crate::ecs::{ ICom, IStorage, DenseVec };

//############################################################
#[derive(Debug, Default)]
pub struct Node{
	pub local	: Transform,
	pub matrix	: Mat4,
	pub is_mod	: bool,
}

impl ICom for Node{ type Storage = DenseVec<Self>; }

impl Node{
	pub fn new() -> Self{  
		Node{
			local	: Transform::new(),
			matrix	: Mat4::new(),
			is_mod	: true,
		}
	}

	pub fn from_pos( v: &Vec3 ) -> Self{
		Node{
			local	: Transform::from_pos( v ),
			matrix	: Mat4::new(),
			is_mod	: true,
		}
	}

	pub fn from_pos_rot( v: &Vec3, r: &Quat ) -> Self{
		Node{
			local	: Transform::from_pos_rot( v, r ),
			matrix	: Mat4::new(),
			is_mod	: true,
		}
	}
}


//############################################################
pub fn node_sys_fn( app: &App ){
	let mut store = app.ecs.cm.borrow_mut::<Node>();
	for n in store.iter_mut(){
		if n.is_mod { n.matrix.from_quat_tran_scale( &n.local.rot, &n.local.pos, &n.local.scl ); }
	}
}

pub fn node_clean_fn( app: &App ){
	let mut store = app.ecs.cm.borrow_mut::<Node>();
	for n in store.iter_mut(){
		if n.is_mod { n.is_mod = false; }
	}
}