#![allow(dead_code)]

use std::rc::Rc;
use std::cell::{ Ref, RefMut, RefCell };

//##################################################
thread_local!{
	pub static WORLD: Rc<RefCell<World>> = Rc::new( RefCell::new( World::new() ) );
}

//##################################################################
// WORLD STATE
	pub struct World{
		pub	canvas_size	: [i32;2],
		pub main_camera	: usize,
		pub cache		: Cache,
		pub mouse 		: MouseState,
	}

	impl World{
		pub fn new() -> Self{
			World{
				main_camera : 0,			// TODO, Bad but don't want to use Option.
				canvas_size : [100,100],
				cache 		: Cache::new(),
				mouse 		: MouseState::new(),
			}
		}

		pub fn get() -> Rc<RefCell<World>> { WORLD.with( |w|{ w.clone() } ) }

		//////////////////////////////////////////////////////
		// Inserts
		//////////////////////////////////////////////////////
			pub fn insert_shader( sh: crate::wasm::Shader ) -> usize{
				let ws_r		= WORLD.with( |w|{ w.clone() } );
				let ws			= ws_r.borrow();
				let mut sh_c	= ws.cache.shader.borrow_mut();
				sh_c.insert( sh )
			}


		//////////////////////////////////////////////////////
		// Canvas Size
		//////////////////////////////////////////////////////
			pub fn set_size( w: i32, h: i32 ){
				let ws_r	= WORLD.with( |w|{ w.clone() } );
				let mut ws	= ws_r.borrow_mut();

				ws.canvas_size[ 0 ] = w;
				ws.canvas_size[ 1 ] = h;
			}

			pub fn get_size() -> [i32; 2]{
				let ws_r	= WORLD.with( |w|{ w.clone() } );
				let ws		= ws_r.borrow();
				ws.canvas_size.clone()
			}


		//////////////////////////////////////////////////////
		// Mouse State
		//////////////////////////////////////////////////////
			pub fn get_mouse() -> MouseState{
				let ws_r	= WORLD.with( |w|{ w.clone() } );
				let ws		= ws_r.borrow();
				ws.mouse.clone()
			}


		//////////////////////////////////////////////////////
		// Main Camera ID
		//////////////////////////////////////////////////////
			pub fn get_camera_id() -> usize{
				let ws_r	= WORLD.with( |w|{ w.clone() } );
				let ws		= ws_r.borrow();
				ws.main_camera	
			}

			pub fn set_camera_id( v: usize ){
				let ws_r	= WORLD.with( |w|{ w.clone() } );
				let mut ws	= ws_r.borrow_mut();
				ws.main_camera = v;
			}
	}

	fn test(){
		let ws_guard = World::get();
		let mut ws = ws_guard.borrow_mut();

		ws.canvas_size[0] = 1000;
	}

	fn test2(){
		let ws_guard = World::get();
		let ws = ws_guard.borrow();

		let mut vc = ws.cache.vao.borrow_mut();
		let vao = vc.get_mut( 0 );

		let b 			= vao.buffers.get_mut( "vertices" ).unwrap();
		b.elm_cnt = 0;
	}


//##################################################################
// MOUSE STATE
	#[derive(Debug,Clone)]
	pub struct MouseState{
		pub is_down		: bool, 
		pub is_shift	: bool, 
		pub is_ctrl		: bool,

		pub wheel_val	: i32, 

		pub idx			: i32, 
		pub idy			: i32,
		
		pub pos_ver		: u32,
		pub wh_ver		: u32,
	}

	impl MouseState{
		pub fn new() -> Self {
			MouseState{
				is_down		: false, 
				is_shift	: false, 
				is_ctrl		: false,

				wheel_val	: 0, 

				idx			: 0, 
				idy			: 0,
				
				pos_ver		: 0,
				wh_ver		: 0,
			}
		}
	}


//##################################################################
// CACHE
	use crate::wasm::{ Ubo, Vao, UboCache, VaoCache, ShaderCache };

	pub struct Cache{
		pub vao		: RefCell< VaoCache >,
		pub shader	: RefCell< ShaderCache >,
		pub ubo		: RefCell< UboCache >,
	}

	impl Cache{
		pub fn new() -> Self{
			Cache{
				vao		: RefCell::new( VaoCache::new() ),
				shader	: RefCell::new( ShaderCache::new() ),
				ubo		: RefCell::new( UboCache::new() ),
			}
		}


		//////////////////////////////////////////////
		// VAO
		//////////////////////////////////////////////
			pub fn insert_vao( &self, v: Vao ) -> usize {
				let mut vao = self.vao.borrow_mut();
				vao.insert( v )
			}

			pub fn get_vao_mut( &self, i: usize ) -> RefMut<Vao>{
				RefMut::map( self.vao.borrow_mut(), |b|{ b.get_mut(i) })
			}

		//////////////////////////////////////////////
		// UBO
		//////////////////////////////////////////////
			pub fn insert_ubo( &self, name: &str, v: Ubo ){
				let mut ubo = self.ubo.borrow_mut();
				ubo.insert( name.to_string(), v );
			}

		//////////////////////////////////////////////
		// SHADER
		//////////////////////////////////////////////
	}


/*
App.init();
App.render();
App.update_mouse();

How do I pass World State to Systems??

App Context lives in JS.

*/