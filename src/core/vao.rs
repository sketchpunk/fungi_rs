#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use web_sys::{ WebGlBuffer, WebGlVertexArrayObject };

use crate::core::{ WebGL }; //, macros


//####################################################################################
pub struct Vao{
	pub id 			: u32,
	pub ctx			: WebGlVertexArrayObject,
	pub elm_cnt		: i32,
	pub is_indexed	: bool,
	pub buffers		: RefCell< HashMap< String, Buffer > >,
}

//#[repr(u32)]
//pub enum BDataType{
//	Float = 5126,
//}

pub struct Buffer{
	ctx			: WebGlBuffer,
	elm_cnt		: i32,
}

pub struct VaoManager{
	cache	: HashMap< String, Rc< RefCell< Vao > > >,
	bound	: Option< u32 >,
	id_cnt	: u32,
	gl		: Rc< WebGL >,
}


//####################################################################################
impl Vao{
	pub fn new( gl: &WebGL ) -> Option< Vao >{
		let obj = gl.ctx.create_vertex_array();
		match obj {
			None => None,
			Some( ctx ) => Some( Vao{ 
				ctx, 
				id			: 0,
				elm_cnt		: 0, 
				is_indexed	: false,
				buffers 	: RefCell::new( HashMap::new() ),
			}),
		}
	}

	pub fn add_f32_buf( &self, gl: &WebGL, name: &str, data: &Vec<f32>, attr_loc: u32, comp_len: i32, stride: i32, offset: i32, is_static: bool, is_instance: bool ) -> bool{
		match gl.f32_buffer_vec( data, attr_loc, comp_len, stride, offset, is_static, is_instance ) { 
			None		=> return false, 
			Some( buf )	=>{
				let mut b = self.buffers.borrow_mut();
				b.insert( name.to_string(), Buffer{ 
					ctx			: buf, 
					elm_cnt		: data.len() as i32 / comp_len,
				});
			}
		}
		true
	}

	pub fn set_elm_cnt_by_name( &mut self, name: &str ){
		match self.buffers.borrow().get( name ) {
			None 		=> console_log!("set_elm_cnt : buffer not found : {} ", name ),
			Some( buf )	=> self.elm_cnt = buf.elm_cnt,
		}
	}
}


//####################################################################################
impl VaoManager{
	pub fn new( gl: Rc<WebGL> ) -> Self{
		VaoManager{
			cache	: HashMap::new(),
			bound	: None,
			id_cnt	: 0,
			gl,
		}
	}

	/////////////////////////////////////////////////////////////////////
	// BINDING
	/////////////////////////////////////////////////////////////////////
		pub fn unbind( &mut self ) -> &Self{
			self.gl.unbind_vao();
			self.bound = None;
			self
		}

		pub fn bind( &mut self, name: &str ) -> &Self{
			match self.cache.get( name ){
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// If Vao not found in cache
				None =>{
					console_log!( "VaoManager - Vao not found for binding: {}", name );
					return self
				},

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				Some( v ) => {
					let vao = v.borrow();

					match &self.bound {
						Some( b_id ) if *b_id == vao.id =>{
							console_log!( "VaoManager - Already Bound: {}", name );
							return self
						} _ => ()
					}

					console_log!( "VaoManager - Binding {}", name );
					self.gl.bind_vao( &vao.ctx );

					self.bound = Some( vao.id.clone() );
				}
			}

			self
		}


	/////////////////////////////////////////////////////////////////////
	// 
	/////////////////////////////////////////////////////////////////////
		pub fn keep( &mut self, name: &str, mut v: Vao ){
			self.id_cnt	+= 1;
			v.id 		= self.id_cnt;
			self.cache.insert( name.to_string(), Rc::new( RefCell::new( v ) ) );
		}


	/////////////////////////////////////////////////////////////////////
	// CREATE VAOs
	/////////////////////////////////////////////////////////////////////

		pub fn standard( &mut self, name: &str, comp_len: i32, verts: &Vec<f32> ){
			// First Unbind Any Existing VAOs
			if self.bound.is_some() { self.unbind(); }

			// Create Object if Possible
		   	let mut v;
		   	match Vao::new( &self.gl ) {
		   		None		=>{ console_log!( "Problem Creating VAO Object" ); return }
		   		Some( o )	=> v = o,
		   	}

		   	// Bind and Add Verts
		   	self.gl.bind_vao( &v.ctx );
		   	v.add_f32_buf( &self.gl, "vertices", &verts, 0, comp_len, 0, 0, true, false );
		   	v.set_elm_cnt_by_name( "vertices" );	// Set the Element Count for Drawing

		   	self.gl.unbind_vao();					// Need to Unbind VAO before any buffers, or Screw up VAO configuration
		   	self.gl.unbind_ary_buf();				// Now Safe unbind array buffer

		   	self.keep( name, v );					// Give to manager to have ownership
		}
}



	/*
	pub fn create( &mut self, gl: &WebGL, name: &str, do_bind: bool ) -> Option< Rc< RefCell< Vao > > >{
		self.id_cnt += 1;

		let mut vao = match Vao::new( gl ) { Some( v ) => v, None => return None };
		vao.id = self.id_cnt;

		if do_bind { 
			gl.bind_vao( &vao.ctx );
			self.bound = Some( vao.id.clone() );
		}

		let rc_vao	= Rc::new( RefCell::new( vao ) );
		let rtn 	= Some( Rc::clone( &rc_vao ) );
		self.cache.insert( name.to_string(), rc_vao );

		rtn
	}
	*/
	    
	    /* Icky Way to Create and Append Vao
	    let rc_vao = match self.vaos.create( &self.gl, "triangle", true ) { 
	    	Some( v ) => v, 
	    	None => { console_log!( "Problem creating vao" ); return }
	    };

	    {	// Limit the Mutable Borrow
		    let mut vao = rc_vao.borrow_mut();
		    vao.add_f32_buf( &self.gl, "vertices", &vertices, 0, 3, 0, 0, true, false );
		    vao.set_elm_cnt_by_name( "vertices" );
	    }
	    self.vaos.unbind( &self.gl );
		self.gl.unbind_ary_buf();

	    self.vaos.bind(  &self.gl, "triangle" );
		*/

		/* Nicer Way to Create and Save Vaos 

	   	let mut v;
	   	match Vao::new( &self.gl ) {
	   		None		=>{ console_log!( "Problem Creating VAO Object" ); return }
	   		Some( o )	=> v = o,
	   	}

	   	self.vaos.unbind( &self.gl );	// Really Need to Unbind Globally so the App things nothing is bound
	   	self.gl.bind_vao( &v.ctx );		// Bind Directly Temporarily 
	   	v.add_f32_buf( &self.gl, "vertices", &vertices, 0, 3, 0, 0, true, false );
	   	v.set_elm_cnt_by_name( "vertices" );

	   	self.gl.unbind_vao();			// Need to Unbind VAO before any buffers, of Screw up VAO configuration
	   	self.gl.unbind_ary_buf();		// Now Safe unbind array buffer

	   	self.vaos.keep( "triangle", v );
	   	self.vaos.bind( &self.gl, "triangle" );
	   	console_log!( "bind VAO" );
	   	*/