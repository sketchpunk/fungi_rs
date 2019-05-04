
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;	// dyn_into::<>

use js_sys::WebAssembly;
use web_sys::{ HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlShader, WebGlProgram, WebGlBuffer, WebGlVertexArrayObject as VAO };

use crate::core::macros;


//####################################################################################
pub struct WebGL{
	pub ctx: GL,
}


//use std::marker::Sync;
//unsafe impl Sync for WebGL{}


impl WebGL{
	pub fn new( c_name: &str ) -> Result< WebGL, &'static str >{
		let win		= web_sys::window().expect( "no global 'window' exists" );
		let doc		= win.document().expect( "should have a document on window" );
		//let bod		= doc.body().expect( "document should have a body" );
		let elm		= doc.get_element_by_id( c_name ).expect( &format!("Can not find canvas with ID of {}", c_name ) );

		let canvas: HtmlCanvasElement = elm.dyn_into::<HtmlCanvasElement>()
			.expect("Unable to convert html element to canvas element");

		//match elm.dyn_into::<HtmlCanvasElement>(){
		//	Ok(e)	=> canvas = e,
		//	Err(_)	=> return Err( "Unable to convert html element to canvas element" ),
		//};
		
		//let ctx = canvas.get_context( "webgl2" )?
		//		.unwrap()
		//		.dyn_into::<WebGl2RenderingContext>()?;

		let ctx = canvas.get_context( "webgl2" )	// Returns Result< Option<>, JsValue ), so need to unwrap Option
					.expect( "Unable to get WebGL2.0 Context from canvas" )
					.unwrap()												// UnWrap Option
					.dyn_into::<GL>()					// Convert Object
					.expect( "Error converting webgl context to rust." );

		
		//let ctx = match canvas.get_context( "webgl2" ) { // get_context returns Result< Option<>, JsValue ) 
		//	Ok(e)	=> e.unwrap(),
		//	Err(_)	=> return Err( "Unable to get WebGL2.0 Context from canvas " ),
		//};

		//let ctx = ctx.dyn_into::<WebGl2RenderingContext>().expect( "Error converting webgl context to rust." );
		Ok( WebGL{ ctx: ctx } )

		//match ctx.dyn_into::<WebGl2RenderingContext>() {
		//	Ok(e)	=> Ok( WebGL{ ctx: e } ),
		//	Err(_)	=> Err( "Error converting webgl context to rust." ),
		//}
	}

	pub fn set_clear( &self, r: f32, g:f32, b:f32, a:f32 ) -> &Self{ self.ctx.clear_color( r, g, b, a ); self }
	pub fn clear( &self ) -> &Self { self.ctx.clear(GL::COLOR_BUFFER_BIT); self }
    
	///////////////////////////////////////////////////////////////////////
	// BINDING
	///////////////////////////////////////////////////////////////////////

		pub fn bind_shader( &self, o: &WebGlProgram ){ self.ctx.use_program( Some( o ) ); }
		pub fn unbind_shader( &self ){ self.ctx.use_program( None ); }

		pub fn bind_ary_buf( &self, o: &WebGlBuffer ){ self.ctx.bind_buffer( GL::ARRAY_BUFFER, Some( o ) ); }
		pub fn unbind_ary_buf( &self ){ self.ctx.bind_buffer( GL::ARRAY_BUFFER, None ); }

		pub fn bind_vao( &self, o: &VAO ){ self.ctx.bind_vertex_array( Some( o ) ); }
		pub fn unbind_vao( &self ){ self.ctx.bind_vertex_array( None ); }


	///////////////////////////////////////////////////////////////////////
	// SHADERS
	///////////////////////////////////////////////////////////////////////

		pub fn new_shader( &self, v_src: &str, f_src: &str ) -> Result< WebGlProgram, String >{
			let v_sh = self.compile_shader( GL::VERTEX_SHADER, v_src ).expect( "Error compiling vertex shader" );
			console_log!("Vertex Shader Compiled");
			let f_sh = self.compile_shader( GL::FRAGMENT_SHADER, f_src ).expect( "Error compiling fragment shader" );
			console_log!("Fragment Shader Compiled");
			
			let prog = self.link_program( &v_sh, &f_sh );

			//TODO
			// Can delete the shaders since the program has been made.
			// gl.ctx.detachShader(prog,vShader); // TODO, detaching might cause issues on some browsers, Might only need to delete.
			// gl.ctx.detachShader(prog,fShader);
			// gl.ctx.deleteShader(fShader);
			// gl.ctx.deleteShader(vShader);

			prog
		}

		pub fn compile_shader( &self, sh_type: u32, src: &str ) -> Result< WebGlShader, String >{
			let sh = self.ctx.create_shader( sh_type ).ok_or_else( || "Unable to create shader object" )?;

			self.ctx.shader_source( &sh, src );
			self.ctx.compile_shader( &sh );

			if self.ctx.get_shader_parameter( &sh, GL::COMPILE_STATUS ).as_bool().unwrap_or( false ){
				Ok( sh )
			}else{
				let msg = match self.ctx.get_shader_info_log( &sh ){
					Some(str)	=> str,
					None		=> String::from("Unknown error creating shader"),
				};

				self.ctx.delete_shader( Some( &sh ) );
				Err( msg )
			}
		}

		pub fn link_program( &self, v_sh: &WebGlShader, f_sh: &WebGlShader ) -> Result< WebGlProgram, String >{
			let prog = self.ctx.create_program().ok_or_else( || "Unable to create shader program" )?;

			self.ctx.attach_shader( &prog, v_sh );
			self.ctx.attach_shader( &prog, f_sh );
			self.ctx.link_program( &prog );

			if self.ctx.get_program_parameter( &prog, GL::LINK_STATUS ).as_bool().unwrap_or( false ){			
				Ok( prog )
			}else{
				let msg = match self.ctx.get_program_info_log( &prog ){
					Some(str)	=> str,
					None		=> String::from("Unknown error linking shaders"),
				};
				
				self.ctx.delete_program( Some( &prog ) );
				
				Err( msg )
			}
		}


	///////////////////////////////////////////////////////////////////////
	// BUFFERS
	///////////////////////////////////////////////////////////////////////

		pub fn vec_to_f32_array( v: &Vec<f32> )-> js_sys::Float32Array{
			let mem_buf	= wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
	    	let loc		= v.as_ptr() as u32;

	   		js_sys::Float32Array::new_with_byte_offset_and_length( &mem_buf , loc, v.len() as u32 )
		}
		
		pub fn f32_buffer( &self, verts: &Vec<f32>, attr_loc: u32, comp_len: i32 ) -> Result< WebGlBuffer, String >{
			let buf = self.ctx.create_buffer().ok_or_else( || "Unable to create buffer object" )?;

			let f32_ary = WebGL::vec_to_f32_array( &verts );

			self.ctx.bind_buffer( GL::ARRAY_BUFFER, Some( &buf ) );

			self.ctx.buffer_data_with_array_buffer_view( GL::ARRAY_BUFFER, &f32_ary, GL::STATIC_DRAW );
	       	self.ctx.enable_vertex_attrib_array( attr_loc );
	       	self.ctx.vertex_attrib_pointer_with_i32( attr_loc, comp_len, GL::FLOAT, false, 0, 0 );

	       	console_log!( "Float Type {}", GL::FLOAT );

			Ok( buf )
		}

		pub fn f32_buffer_vec( &self, data: &Vec<f32>, attr_loc: u32, comp_len: i32, stride: i32, offset: i32, is_static: bool, is_instance: bool ) -> Option< WebGlBuffer >{
			let buf 	= match self.ctx.create_buffer() { Some( b ) => b, None => return None };
			let f32_ary	= WebGL::vec_to_f32_array( &data );

			self.ctx.bind_buffer( GL::ARRAY_BUFFER, Some( &buf ) );
			self.ctx.buffer_data_with_array_buffer_view( GL::ARRAY_BUFFER, &f32_ary, if is_static { GL::STATIC_DRAW }else{ GL::DYNAMIC_DRAW } );
	       	self.ctx.enable_vertex_attrib_array( attr_loc );
	       	self.ctx.vertex_attrib_pointer_with_i32( attr_loc, comp_len, GL::FLOAT, false, stride, offset );

	       	if is_instance == true { self.ctx.vertex_attrib_divisor( attr_loc, 1 ); }

			Some( buf )
		}


	///////////////////////////////////////////////////////////////////////
	//
	///////////////////////////////////////////////////////////////////////
		pub fn draw( &self, elm_cnt: i32 ){ self.ctx.draw_arrays( GL::TRIANGLES, 0, elm_cnt ); }
}



pub struct Buffer{
	pub ctx: WebGlBuffer,
}

/*
	static array( target, aryData, isStatic, dataType, attrLoc, compLen=3, stride=0, offset=0, isInstance=false ){
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Create and Bind New Buffer
		let id = gl.ctx.createBuffer();
		gl.ctx.bindBuffer( target, id );

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Depending on type, 
		switch( target ){
			case gl.ctx.ELEMENT_ARRAY_BUFFER:
				gl.ctx.bufferData( target, aryData, (isStatic)? gl.ctx.STATIC_DRAW : gl.ctx.DYNAMIC_DRAW );
				break;

			case gl.ctx.ARRAY_BUFFER:
				gl.ctx.bufferData( target, aryData, (isStatic)? gl.ctx.STATIC_DRAW : gl.ctx.DYNAMIC_DRAW );
				gl.ctx.enableVertexAttribArray( attrLoc );
				gl.ctx.vertexAttribPointer( attrLoc, compLen, dataType, false, stride, offset );
				break;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		if( isInstance == true ) gl.ctx.vertexAttribDivisor( attrLoc, 1 );

		return { id };
	}
 */