#![allow(dead_code)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;	// dyn_into::<>
use web_sys::{ HtmlCanvasElement, WebGl2RenderingContext as GL };
use std::cell::{ RefCell, RefMut };
use std::rc::Rc;


use crate::World;


//##################################################
#[macro_use]
pub mod macros;
pub mod buffer;		pub use buffer::{ Buffer };
pub mod util;		pub use util::Util;
pub mod vao;		pub use vao::{ Vao, VaoCache };
pub mod shader;		pub use shader::{ Shader, UniformType, AttribLoc, ShaderCache };
pub mod ubo;		pub use ubo::{ Ubo, UboCache };


//##################################################
thread_local!{
	pub static GLOBAL_GL: RefCell< Option< Rc<GL>> > = RefCell::new( None );
}

#[allow(non_snake_case)]
pub fn glctx()-> Rc<GL> {
	GLOBAL_GL.with( |v|{ 
		match *v.borrow(){ //Deference Ref<>
			Some( ref d )	=> d.clone(),
			None 			=> panic!("Meh"),
		}
	})
}


//##################################################
pub fn get_webgl_context( c_name: &str ) -> Result< (), &'static str >{
	let win		= web_sys::window().expect( "Window Not Found" );
	let doc		= win.document().expect( "Window.Document Not Found" );
	let elm		= doc.get_element_by_id( c_name ).expect( &format!( "Canvas by the ID {}, Not Found", c_name ) );

	let canvas: HtmlCanvasElement = elm.dyn_into::<HtmlCanvasElement>()
		.expect( "Unable to convert html element to canvas element" );

	let ctx		= canvas.get_context( "webgl2" ) // Result< Option<>, Jsvalue >
					.expect( "Unable to get WebGL 2.0 context from canvas" )
					.unwrap() 			// For Option
					.dyn_into::< GL >()
					.expect( "Error converting WebGL context to rust." );

	let w = canvas.client_width();
	let h = canvas.client_height();

	console_log!( "set viewport {}, {}", w, h );
	ctx.viewport( 0, 0, w, h );
	World::set_size( w, h );

	GLOBAL_GL.with(|v|{  v.replace( Some( Rc::new(ctx) ) );  });	

	Ok(())
}