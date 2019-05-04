#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]

//#![allow(non_camel_case_types)]

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use web_sys::{ WebGlProgram, WebGlUniformLocation, WebGl2RenderingContext as GL };

use crate::core::{ WebGL, macros };


//####################################################################################
#[repr(u8)]
pub enum UniformType{
	Float	= 0,
	Vec1	= 1,
	Vec2	= 2,
	Vec3	= 3,
	Vec4	= 4,
	Rgb		= 5,
	Rgba	= 6,
	Int 	= 7,
	Mat4	= 8,
	Mat3	= 9,
	Mat2x4	= 10,
}

pub struct Shader{
	pub id 			: u32,
	pub prog		: WebGlProgram,
	pub name		: String,
	pub uniforms	: RefCell< HashMap< String, Uniform > >,
}


pub struct Uniform{
	pub loc : WebGlUniformLocation,
	pub data_type : UniformType,
}


pub struct ShaderManager{
	cache	: HashMap< String, Rc<Shader> >,
	bound	: Option< u32 >,
	gl		: Rc< WebGL >,
	id_cnt	: u32,
}


//####################################################################################
impl Shader{
	pub fn new( id: u32, prog: WebGlProgram, name: String ) -> Self{
		Shader{ id, prog, name, uniforms : RefCell::new( HashMap::new() ) }
	}
}

impl Drop for Shader{
	fn drop(&mut self){ console_log!("Shader being dropped"); }
}



//####################################################################################

impl ShaderManager{
	pub fn new( gl: Rc< WebGL > ) -> ShaderManager{
		ShaderManager{ 
			cache	: HashMap::new(),
			bound	: None,
			id_cnt	: 0,
			gl,
		}
	}

	/////////////////////////////////////////////////////////////////////
	// Creating / Configure Shaders
	/////////////////////////////////////////////////////////////////////
		pub fn from_src( &mut self, name: &str, v_src: &str, f_src: &str, do_bind: bool ) -> Option< Rc<Shader> >{
			let prog = self.gl.new_shader( v_src, f_src ).unwrap();

			self.id_cnt += 1;

			if do_bind { 
				self.gl.ctx.use_program( Some( &prog ) );
				self.bound = Some( self.id_cnt.clone() );
			}

			let rc_sh = Rc::new( Shader::new( self.id_cnt, prog, name.to_string() ) );
			let clone = Some( Rc::clone( &rc_sh ) );

			self.cache.insert( name.to_string(), rc_sh );

			clone
		}

		pub fn add_uniform( &self, sh: &Shader, name: &str, utype: UniformType ) -> &Self{
			match self.gl.ctx.get_uniform_location( &sh.prog, name ) {
				Some( loc ) =>{
					console_log!( "Found it {}", name );
					let mut u = sh.uniforms.borrow_mut();
					u.insert( name.to_string(), Uniform{ loc, data_type: utype });
				},
				None => console_log!( "Shader Uniform Not Found {}", name ),
			}
			self
		}

	/////////////////////////////////////////////////////////////////////
	// Binding
	/////////////////////////////////////////////////////////////////////

		pub fn unbind( &mut self ) -> &Self{ 
			self.gl.ctx.use_program( None );
			self.bound = None;
			self
		}

		pub fn bind( &mut self, sh_name: &str ) -> &Self {
			match self.cache.get( sh_name ){
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// If Shader Not Found In Cache
				None		=> {
					console_log!( "ShaderManager - Shader Not Found for Binding: {} ", sh_name );
					return self
				},

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				Some( sh )	=> {
					// Is Shader that is found already bound?
					match &self.bound {
						Some( b_sh ) if *b_sh == sh.id => {
							console_log!( "ShaderManager - Already bound: {}", sh_name );
							return self
						} _ => ()
					}

					console_log!( "ShaderManager - Bind Shader: {}", sh_name );
					self.gl.ctx.use_program( Some( &sh.prog ) );

					self.bound = Some( sh.id.clone() );
				}
			}

			self
		}

	/////////////////////////////////////////////////////////////////////
	// 
	/////////////////////////////////////////////////////////////////////

		// Get Refence to an Existing Shader
		pub fn get( &self, sh_name :&str ) -> Option< Rc< Shader > >{
			match self.cache.get( sh_name ){
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				Some( sh )	=> Some( Rc::clone( sh ) ),
				None 		=> {
					console_log!( "ShaderManager - Get Shader Not Found: {} ", sh_name );
					None
				}
			}
		}

		// Remove Shader
		pub fn rm( &mut self, sh_name: &str ){
			match self.cache.get( sh_name ){
				Some( sh ) => {
					// Unbind if shader is currently in use
					match self.bound {
						Some( b_sh ) if b_sh == sh.id => {
							self.gl.ctx.use_program( None );
							self.bound = None;
						} _ => ()
					}

					// Delete Shader from GPU and Cache.
					self.gl.ctx.delete_program( Some( &sh.prog ) );
					self.cache.remove( sh_name );
				} _ => ()
			}
		}

		pub fn count( &self ) -> usize{ self.cache.len() }


	/////////////////////////////////////////////////////////////////////
	// Shader Usage
	/////////////////////////////////////////////////////////////////////

		// Update Shader Uniform on the GPU
		pub fn uniform_af32( &self, sh: &Shader, name: &str, ary: &[f32] ) -> &Self{
			match sh.uniforms.borrow().get( name ) {
				None		=> console_log!( "Passing Uniform Not Found {}", name ),
				Some( u )	=> match u.data_type {
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					UniformType::Vec1 => self.gl.ctx.uniform1fv_with_f32_array( Some( &u.loc ), ary ),
					UniformType::Vec2 => self.gl.ctx.uniform2fv_with_f32_array( Some( &u.loc ), ary ),

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					UniformType::Vec3 | UniformType::Rgb =>
						self.gl.ctx.uniform3fv_with_f32_array( Some( &u.loc ), ary ),

					UniformType::Vec4 | UniformType::Rgba =>
						self.gl.ctx.uniform4fv_with_f32_array( Some( &u.loc ), ary ),

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//UniformType::Mat2x4	=> gl.uniform_matrix2x4fv_with_f32_array( Some( &u.loc ), false, ary ), // Need Data to be Mut?? Why?
					UniformType::Mat3	=> self.gl.ctx.uniform_matrix3fv_with_f32_array( Some( &u.loc ), false, ary ),
					UniformType::Mat4	=> self.gl.ctx.uniform_matrix4fv_with_f32_array( Some( &u.loc ), false, ary ),

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_ => console_log!( "Unknown Uniform Type put_f32_array {}", name ),
				},
			}

			self
		}
}
