#![allow(dead_code)]

use crate::App;
use crate::ecs::{ ICom, IStorage, DenseVec, BitSet, System };
use crate::maths::{ Math, mat4::*, vec3::*, quat::* };


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
pub fn camera_sys_fn( ecs: &crate::ecs::Ecs ){
	use super::node::Node;

	let ns		= ecs.cm.borrow::<Node>();
	let mut cs	= ecs.cm.borrow_mut::<Camera>();
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


//###############################################################
#[allow(dead_code)]
pub struct CameraHandler{
	wh_ver		: u32,
	pos_ver		: u32,
	is_down		: bool,

	init_pos 	: Vec3,
}

impl CameraHandler{
	pub fn new() -> Self {
		CameraHandler{ wh_ver:0, pos_ver:0, is_down:false, 
			init_pos: Vec3::new(),
		}
	}
}

impl System for CameraHandler{
	fn run( &mut self, ecs: &crate::ecs::Ecs ){
		use super::Node;
		let ws_r	= crate::World::get();
		let ws		= ws_r.borrow();

		
		//let mut store = app.ecs.cm.borrow_mut::<Node>();
		//let mut n = store.get_mut( app.main_camera );
		let do_wheel	= if self.wh_ver	!= ws.mouse.wh_ver		{ true }else{ false };
		let do_pos		= if self.pos_ver	!= ws.mouse.pos_ver 	{ true }else{ false };

		if !do_wheel && !do_pos { return; }

		let mut n = ecs.cm.get_mut::<Node>( ws.main_camera );

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		if do_wheel {
			// Get Forward Direction of Camera and scale it.
			let mut v = Vec3::from_copy( &VEC3_FWD );
			v.transform_quat( &n.local.rot ).scale( ws.mouse.wheel_val as f32 * 0.5 );

			// Move Camera Forward
			n.local.pos.add( &v );
			n.is_mod = true;

			self.wh_ver = ws.mouse.wh_ver;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		if do_pos {
			self.pos_ver = ws.mouse.pos_ver;

			if self.is_down != ws.mouse.is_down {
				// Down State
				if !self.is_down { self.init_pos.copy( &n.local.pos ); }
				self.is_down = ws.mouse.is_down;
			}

			if !self.is_down { return; }

			let mut polar = Math::cartesian_to_polar( &self.init_pos );
			polar[ 0 ] += Math::to_rad(0.2) * ws.mouse.idx as f32;
			polar[ 1 ] += Math::to_rad(0.2) * ws.mouse.idy as f32;
			polar[ 1 ] = polar[ 1 ].max( -Math::PI_H_MIN ).min( Math::PI_H_MIN ); // TODO Fix with clamp when rust makes it stable.

			let p = Math::polar_to_cartesian( polar[0], polar[1], n.local.pos.len() );
			n.local.pos.copy( &p );
			n.local.rot.look( &p, &VEC3_UP );
			n.is_mod = true;
			//console_log!(" idx {} ", mouse.idx );
		}
	}
}