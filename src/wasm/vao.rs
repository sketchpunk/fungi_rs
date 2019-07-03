#![allow(dead_code)]
#![allow(unused_imports)]

use std::ops::{ Drop, Deref, DerefMut };
use std::cell::RefCell;
use std::collections::HashMap;
use web_sys::{ WebGlVertexArrayObject }; //WebGlBuffer, 
use super::{ glctx, Buffer, AttribLoc };
use crate::storage::RecycleStorage;


//####################################################################################

const BNAME_VERTS : &str = "vertices";

#[derive(Debug)]
pub struct Vao{
	pub ctx		: WebGlVertexArrayObject,
	pub name	: String,
	pub elm_cnt	: i32,
	pub is_idx	: bool,
	pub buffers	: HashMap< String, Buffer >,
}


pub struct VaoCache( pub RecycleStorage< Vao > );


//####################################################################################
impl VaoCache{
	pub fn new() -> Self{ VaoCache( RecycleStorage::new() ) }
}

impl Deref for VaoCache{
	type Target = RecycleStorage< Vao >;
	fn deref( &self ) -> &Self::Target{ &self.0 }
}

impl DerefMut for VaoCache{
	fn deref_mut( &mut self ) -> &mut Self::Target{ &mut self.0 }
}



//####################################################################################
impl Vao{
	/////////////////////////////////////////////////////////////////////
	// Constructors
	/////////////////////////////////////////////////////////////////////
		pub fn new( name: &str ) -> Option< Vao >{
			let obj = glctx().create_vertex_array();
			match obj {
				None => None,
				Some( ctx ) => Some( Vao{ 
					ctx,
					name 		: name.to_string(),
					elm_cnt		: 0, 
					is_idx		: false,
					buffers 	: HashMap::new(),
				}),
			}
		}

		pub fn standard( name: &str, vcomp_len: i32, verts: &Vec<f32> ) -> Self{
			let mut vao = Vao::new( name ).unwrap();
			vao.bind();
				vao.add_buf( BNAME_VERTS, true, Buffer::f32_array_full( AttribLoc::Pos as u32, vcomp_len, verts, true, false ).unwrap() );
			vao.unbind();
			Buffer::unbind_array();
			vao
		}

		pub fn standard_empty( name: &str, vcomp_len: i32, vert_cnt: usize ) -> Self{
			let mut vao = Vao::new( name ).expect("Error creating a vao for standard empty");
			vao.bind();

				let byte_cnt: f64 = vert_cnt as f64 * vcomp_len as f64 * 4.0;
				let b = Buffer::f32_empty( AttribLoc::Pos as u32, vcomp_len, byte_cnt, false ).unwrap();
				vao.add_buf( BNAME_VERTS, true, b );

			vao.unbind();
			Buffer::unbind_array();
			vao
		}	


	/////////////////////////////////////////////////////////////////////
	// Methods
	/////////////////////////////////////////////////////////////////////
		pub fn bind( &self )-> &Self { glctx().bind_vertex_array( Some(&self.ctx) ); self }
		pub fn unbind( &self )-> &Self { glctx().bind_vertex_array( None ); self }
		pub fn delete( &self ){ glctx().delete_vertex_array( Some(&self.ctx) ); }

	/////////////////////////////////////////////////////////////////////
	// Modify Data
	/////////////////////////////////////////////////////////////////////
		
		pub fn add_buf( &mut self, name: &str, use_elm_cnt: bool , buf: Buffer) -> &mut Self {
			if use_elm_cnt { self.elm_cnt = buf.elm_cnt; }
			self.buffers.insert( name.to_string(), buf );
			self
		}
}


impl Drop for Vao{
	fn drop( &mut self ){
		self.delete();
		console_log!("Dropped Vao {}", self.name );
	}
}