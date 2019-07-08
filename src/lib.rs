#![allow(unused_imports)]

extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;

extern crate maths;		use maths::{ Math, mat4::*, quat::*, vec3::*, color::* };
extern crate ecs;		use ecs::{ Ecs, IStorage };

#[macro_use]
mod wasm;			use wasm::{ glctx, Buffer, Vao, VaoCache, Shader, ShaderCache, UniformType };
			
mod com;			use com::{ Node, Draw, Camera };
mod primitives;		use primitives::{ quad_verts };
mod storage;

mod world;			use world::{ World, Cache, MouseState };

mod temp;

//###############################################################
#[wasm_bindgen(start)]
pub fn run(){
	console_error_panic_hook::set_once();
	console_log!("WASM_START");
}


//###############################################################
#[wasm_bindgen]
#[allow(dead_code)]
pub struct App{
	ecs : Ecs,
}

impl Drop for App{
	fn drop(&mut self){ console_log!("App being dropped"); }
}


//###############################################################
#[wasm_bindgen]
impl App{
	#[wasm_bindgen(constructor)]
	pub fn new( canvas_name: &str ) -> App{
		if wasm::get_webgl_context( canvas_name ).is_err(){ panic!("Error getting WebGL Context"); }
		App{ ecs : Ecs::new(), }
	}

	pub fn init( &mut self ){
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// GL
		let gl = glctx();
		gl.clear_color( 0.9, 0.9, 0.9, 1.0 );

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		temp::setup_ecs( &mut self.ecs );
		temp::setup_camera( &self.ecs );

		let sh_i : usize = temp::create_shader();


		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		let ws_r = World::get();
		let ws = ws_r.borrow();

		use wasm::ubo::{ UboBuilder, DataType };
		let mut _bld = UboBuilder::new();
		let ubo = _bld.add( "A", DataType::Vec3, 0 ).add( "B", DataType::Float, 0 ).build( 0);
		match ubo{
			Ok( mut u ) => {
				//u.set_f32( &"B", 1.0 );
				//u.set_f32_ary( &"A", &[ 99999.0, 0.0, 1.0, 0.0 ] );
				u.set_f32_ary( &"A", &COLOR_GRAY_LITE.to_rgb_norm() );;
				u.update();
				console_log!("{:?}", u );
				ws.cache.insert_ubo( "test", u );
				
				console_log!("99999{:?}", (99999.0 as f32).to_bits().to_le_bytes() );
			}, _=>(),
		}


		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//
		let vao_i = ws.cache.insert_vao( Vao::standard( "floor", 4, &primitives::grid_floor_verts() ) );
		//let sh_i : usize = create_shader();
		self.e_draw( &"Test", 0, vao_i, 1, sh_i );


		let v_cnt 	= 10;
		let vao		= Vao::standard_empty( "Points_V4", 4, v_cnt );
		let vi		= ws.cache.insert_vao( vao );
		let mut dbuf = com::dynamic_vert::DynamicVert::new( vi, 4, v_cnt * 4 * 4, 0 );
		dbuf.add( 0.0, 0.5, 0.0, 1.0 );
		let n 		= Node::new();
		let d 	 	= Draw::new( vi, 0, sh_i );
		let _ei		= self.ecs.new_entity_c3( "Points", 0, n, d, dbuf );
	}
	
	pub fn render( &mut self ){
		self.ecs.sm.run( &self.ecs );
	}

	pub fn pass_mouse_state( &mut self, is_down: bool, is_shift: bool, is_ctrl: bool, wh_val: i32, idx: i32, idy: i32, p_ver: u32, w_ver: u32 ){
		let ws_r			= World::get();
		let mut ws			= ws_r.borrow_mut();
		ws.mouse.is_down	= is_down;
		ws.mouse.is_shift	= is_shift;
		ws.mouse.is_ctrl	= is_ctrl;
		ws.mouse.wheel_val	= wh_val;
		ws.mouse.idx		= idx;
		ws.mouse.idy		= idy;
		ws.mouse.pos_ver	= p_ver;
		ws.mouse.wh_ver		= w_ver;
	}

	pub fn e_draw( &mut self, name: &str, tag:u16, vi:usize, mode: u32, shi: usize ){
		self.ecs.new_entity_c2( name, tag, Node::new(), Draw::new( vi, mode, shi ) );
	}
}


//###############################################################
pub fn test_sys_fn( app: &App ){
	let es 		= app.ecs.em.borrow();
	let mut ns 	= app.ecs.cm.borrow_mut::<com::Node>();
	let idx_ary	= ns.get_entities();

	for i in idx_ary{
		let e = &es[ i ];
		if e.tag == 1 {
			let mut n = ns.get_mut( i );
			n.local.rot.rot_z( 0.015 );
			n.local.rot.rot_y( 0.01 );
			n.local.rot.rot_x( 0.02 );
			//n.local.pos[2] = -2.0;
			n.is_mod = true;	
		}
	}	

	/*
	for n in store.iter_mut(){
		n.local.rot.rot_z( 0.015 );
		n.local.rot.rot_y( 0.01 );
		n.local.rot.rot_x( 0.02 );
		n.local.pos[2] = -2.0;
		n.is_mod = true;
	}
	*/
}

