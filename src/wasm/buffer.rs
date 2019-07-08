#![allow(dead_code)]

use std::ops::{ Deref, DerefMut, Drop };
use web_sys::{ WebGlBuffer, WebGl2RenderingContext as GL };
use crate::storage::RecycleStorage;
use super::{ glctx, Util };

//####################################################################################
pub const BTYPE_ARRAY	: u32 = 34962;
pub const BTYPE_UNIFORM	: u32 = 35345;
pub const BTYPE_ELEMENT	: u32 = 34963;
pub const DTYPE_FLOAT	: u32 = 5126;
pub const DTYPE_UBYTE	: u32 = 5121;

#[derive(Debug)]
pub struct Buffer{
	pub ctx			: WebGlBuffer,
	pub elm_cnt		: i32,
	pub comp_len	: i32,
	pub buf_type	: u32,
	pub data_type 	: u32,
	pub stride 		: i32,
	pub offset		: i32,
}


/*
pub struct BufferCache( pub RecycleStorage< Buffer > );

//####################################################################################
impl BufferCache{
	pub fn new() -> Self{ BufferCache( RecycleStorage::new() ) }
}

impl Deref for BufferCache{
	type Target = RecycleStorage< Buffer >;
	fn deref( &self ) -> &Self::Target{ &self.0 }
}

impl DerefMut for BufferCache{
	fn deref_mut( &mut self ) -> &mut Self::Target{ &mut self.0 }
}
*/


//####################################################################################
impl Buffer{
	/////////////////////////////////////////////////////////////////////
	// Constructors
	/////////////////////////////////////////////////////////////////////
		pub fn new( buf_type: u32, data_type: u32, comp_len: i32 ) -> Result< Buffer, String >{
			//let ctx = match gl.create_buffer() { Some( b ) => b, None => return Err("Unable to create buffer object".to_string()) };
			let b = Buffer{
				ctx			: glctx().create_buffer().ok_or_else( || "Unable to create buffer object" )?,
				elm_cnt		: 0,
				stride		: 0,
				offset		: 0,
				comp_len,
				buf_type,	
				data_type,
			};

			Ok( b )
		}

		pub fn f32_array( comp_len: i32 ) -> Result< Buffer, String >{
			Buffer::new( BTYPE_ARRAY, DTYPE_FLOAT, comp_len )
		}

		// Create a buffer used as a UBO.
		pub fn new_uniform() -> Result< Buffer, String >{
			Buffer::new( BTYPE_UNIFORM, DTYPE_UBYTE, 1 )
		}

		// Fully setup buffer for shaders and pass data to it.
		pub fn f32_array_full( attr_loc: u32, comp_len: i32, data: &Vec<f32>, is_static: bool, unbind: bool ) -> Result< Buffer, String >{
			let b = Buffer::new( BTYPE_ARRAY, DTYPE_FLOAT, comp_len );
			if b.is_err() { return b; }

			let mut b = b.unwrap();
			b.bind();
			b.set_vec_f32( data, is_static );
			b.set_attrib( attr_loc, false );
			if unbind { b.unbind( ); }

			Ok( b )
		}

		// Create and Empty Float Buffer of a specific size.
		pub fn f32_empty( attr_loc:u32, comp_len: i32, byte_cnt: f64, unbind: bool ) -> Result< Buffer, String >{
			let b = Buffer::new( BTYPE_ARRAY, DTYPE_FLOAT, comp_len );
			if b.is_err() { return b; }

			let b = b.unwrap();
			b.bind();
			b.set_empty_f32( byte_cnt );
			b.set_attrib( attr_loc, false );
			if unbind { b.unbind(); }

			Ok( b )
		}


	/////////////////////////////////////////////////////////////////////
	// 
	/////////////////////////////////////////////////////////////////////
		pub fn unbind_array(){ glctx().bind_buffer( GL::ARRAY_BUFFER, None ); }

	/////////////////////////////////////////////////////////////////////
	// Methods
	/////////////////////////////////////////////////////////////////////
		pub fn bind( &self ){ glctx().bind_buffer( self.buf_type, Some( &self.ctx ) ); }
		pub fn unbind( &self ){ glctx().bind_buffer( self.buf_type, None ); }
		pub fn delete( &self ){ glctx().delete_buffer( Some( &self.ctx ) ); }


	/////////////////////////////////////////////////////////////////////
	// GL Calls related to setting and full data on buffers
	/////////////////////////////////////////////////////////////////////)
		// Set and fill buffer with float array
		pub fn set_vec_f32( &mut self, data: &Vec<f32>, is_static: bool ){
			let f32_ary	= Util::vec_to_f32_array( &data );
			self.elm_cnt = (data.len() as i32) / self.comp_len;
			glctx().buffer_data_with_array_buffer_view( self.buf_type, &f32_ary, if is_static { GL::STATIC_DRAW }else{ GL::DYNAMIC_DRAW } );
		}

		// Insert Float Array
		pub fn set_sub_f32( &mut self, data: &Vec<f32> ){
			let f32_ary	= Util::vec_to_f32_array( &data );
			self.elm_cnt = (data.len() as i32) / self.comp_len;
			glctx().buffer_sub_data_with_f64_and_array_buffer_view( self.buf_type, 0.0, &f32_ary );
		}

		// Send a Byte array up. Don't know why this function need the data to be mut?
		pub fn set_sub_u8( &self, data: &mut [u8], offset: i32 ){
			glctx().buffer_sub_data_with_i32_and_u8_array( self.buf_type, offset, data );
		}

		// Create an Empty Float Buffer with its capacity set.
		pub fn set_empty_f32( &self, byte_cnt: f64 ){
			glctx().buffer_data_with_f64( self.buf_type, byte_cnt, GL::DYNAMIC_DRAW );
		}

		// Mainly used in Vao, assign the shader attribute location the the buffer is assigned to.
		pub fn set_attrib( &self, attr_loc: u32, is_instance: bool ){
			glctx().enable_vertex_attrib_array( attr_loc );
			glctx().vertex_attrib_pointer_with_i32( attr_loc, self.comp_len, self.data_type, false, self.stride, self.offset );
			if is_instance { glctx().vertex_attrib_divisor( attr_loc, 1 ); }
		}

		// Mainly used in Ubo, Assign the bind point that the buffer is assigned too.
		pub fn set_bindpoint( &self, bp: u32 ){
			glctx().bind_buffer_base( self.buf_type, bp, Some( &self.ctx ) );
		}
}


impl Drop for Buffer{
	fn drop( &mut self ){
		self.delete();
		console_log!("Buffer Dropped");
	}
}
