//#![allow(unused_imports)]

extern crate console_error_panic_hook;

use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[macro_use]
mod core;
mod maths;
mod woot;

use self::core::web_gl::WebGL;
use self::core::shader::{ ShaderManager };
use self::core::vao::{ VaoManager };


//###############################################################
#[wasm_bindgen(start)]
pub fn run(){
	console_error_panic_hook::set_once();
	console_log!("WASM_START");
}


//###############################################################
#[wasm_bindgen]
pub struct App{
	gl		: Rc< WebGL >,
	shaders	: ShaderManager,
	vaos	: VaoManager,
	angle : f32,
}


impl Drop for App{
	fn drop(&mut self){ console_log!("App being dropped"); }
}


//###############################################################

#[wasm_bindgen]
impl App{

	#[wasm_bindgen(constructor)]
	pub fn new( canvas_name: &str ) -> App{
		let gl		= core::get_webgl_context( canvas_name ).expect( "Error getting WebGL Context" );
		let rc_gl	= Rc::new( WebGL{ ctx : gl } );
		let sh_gl	= Rc::clone( &rc_gl );
		let vao_gl	= Rc::clone( &rc_gl );

		App{ 
			gl 		: rc_gl,
			shaders	: ShaderManager::new( sh_gl ),
			vaos	: VaoManager::new( vao_gl ),
			angle 	: 0.0,
		}
	}

	pub fn init( &mut self ){
		self.gl.set_clear( 0.0, 0.0, 0.0, 1.0 );

		woot::test_shaders( self );
		woot::test_mesh( self );
	}

	
	pub fn render( &mut self ){
		woot::test_render( self );
	}
}