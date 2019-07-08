//#![allow(unused_macros)]
//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(non_camel_case_types)]

use std::ops::{ Deref, DerefMut };
use std::cell::RefCell;
use std::collections::HashMap;

use web_sys::{ WebGlShader, WebGlProgram, WebGlUniformLocation, WebGl2RenderingContext as GL };

use super::{ glctx };
use crate::storage::RecycleStorage;

/*
	let mut sh = Shader::from_src( "Test", &v_src, &f_src ).unwrap();

	sh.bind();
	sh	.add_uniform( "u_color", UniformType::Vec3 )
		.add_uniform( "u_view_mat", UniformType::Mat4 )
		.add_uniform( "u_proj_mat", UniformType::Mat4 )
		.f32_array( "u_color", &[ 1.0, 1.0, 0.0 ] )
		.unbind();

	console_log!("{:?}", sh );
 */

//####################################################################################
#[repr(u32)]
pub enum AttribLoc{
	Pos			= 0,
	Norm 		= 1,
	Uv 			= 2,
	Col			= 3,
	BoneIdx 	= 8,
	BoneWeight	= 9,
}


#[derive(Debug)]
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


#[derive(Debug)]
pub struct Shader{
	pub name		: String,
	pub prog		: WebGlProgram,
	pub uniforms	: HashMap< String, Uniform >,
}


#[derive(Debug)]
pub struct Uniform{
	pub loc			: WebGlUniformLocation,
	pub data_type	: UniformType,
}


//####################################################################################
impl Shader{
	/////////////////////////////////////////////////////////////////////
	// Constructors
	////////////////////////////////////////////////////////////////////
		pub fn new( name: &str, prog: WebGlProgram ) -> Self {
			Shader{ 
				prog, 
				name		: name.to_string(), 
				uniforms	: HashMap::new(),
			}
		}

		pub fn from_src( name: &str, v_src: &str, f_src: &str ) -> Option< Shader > {
			match build_shader( &v_src, &f_src ) {
				Err( .. )	=> None,
				Ok( p )		=> Some( Shader::new( name, p ) ),
			}
		}

		pub fn unbind_all() { glctx().use_program( None ); }


	/////////////////////////////////////////////////////////////////////
	// Methods
	/////////////////////////////////////////////////////////////////////
		pub fn bind( &self )-> &Self { glctx().use_program( Some(&self.prog) ); self }
		pub fn unbind( &self )-> &Self { glctx().use_program( None ); self }
		pub fn delete( &self ){ glctx().delete_program( Some(&self.prog) ); }


	/////////////////////////////////////////////////////////////////////
	// Modify Data
	/////////////////////////////////////////////////////////////////////
		pub fn add_uniform( &mut self, name: &str, utype: UniformType ) -> &mut Self {
			match glctx().get_uniform_location( &self.prog, name ) {
				None		=> console_log!( "Shader Uniform Not Found {}", name ),
				Some( loc )	=>{
					console_log!( "Found Attribute {}", name );
					self.uniforms.insert( name.to_string(), Uniform{ loc, data_type: utype });
				},
			}
			self
		}

		// Update Shader Uniform on the GPU
		pub fn set_f32_array( &self, name: &str, ary: &[f32] ) -> &Self {
			let gl = glctx();

			match self.uniforms.get( name ) {
				None		=> console_log!( "Passing Uniform Not Found {}", name ),
				Some( u )	=> match u.data_type {
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					UniformType::Vec1 => gl.uniform1fv_with_f32_array( Some( &u.loc ), ary ),
					UniformType::Vec2 => gl.uniform2fv_with_f32_array( Some( &u.loc ), ary ),

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					UniformType::Vec3 | UniformType::Rgb	=> gl.uniform3fv_with_f32_array( Some( &u.loc ), ary ),
					UniformType::Vec4 | UniformType::Rgba	=> gl.uniform4fv_with_f32_array( Some( &u.loc ), ary ),

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//UniformType::Mat2x4	=> gl.uniform_matrix2x4fv_with_f32_array( Some( &u.loc ), false, ary ), // Need Data to be Mut?? Why?
					UniformType::Mat3	=> gl.uniform_matrix3fv_with_f32_array( Some( &u.loc ), false, ary ),
					UniformType::Mat4	=> gl.uniform_matrix4fv_with_f32_array( Some( &u.loc ), false, ary ),

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_ => console_log!( "Unknown Uniform Type set_f32_array {}", name ),
				},
			}

			self
		}


		pub fn add_uniform_block( &mut self, name: &str, bind_pnt: u32 ) -> &mut Self{
			let gl		= glctx();
			let b_idx	= gl.get_uniform_block_index( &self.prog, name );

			if b_idx > 1000 {
				console_log!("Ubo: {}, not found in shader: {}", name, self.name );
				return self;
			}
			console_log!("Add Uniform {}", b_idx );
			gl.uniform_block_binding( &self.prog, b_idx, bind_pnt );
			self
		}
}


impl Drop for Shader{
	fn drop(&mut self){ 
		self.delete();
		console_log!("Shader dropped : {} ", self.name );
		
	}
}


//####################################################################################

pub fn build_shader( v_src: &str, f_src: &str ) -> Result< WebGlProgram, String >{
	let gl = glctx();

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// Compile Vertex Shader
	let v_sh = compile_shader( &*gl, GL::VERTEX_SHADER, v_src );
	if v_sh.is_err() { return Err( v_sh.unwrap_err() ); }
	
	let v_sh = v_sh.unwrap();

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// Compile Fragment Shader - If fragment fails, delete vertex shader.
	let gl		= glctx();
	let f_sh	= compile_shader( &*gl, GL::FRAGMENT_SHADER, f_src );
	if f_sh.is_err() {
		gl.delete_shader( Some( &v_sh ) );
		return Err( f_sh.unwrap_err() );
	}
	
	let f_sh = f_sh.unwrap();

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// Link Shaders as a Single Program
	let prog = gl.create_program().ok_or_else( || "Unable to create shader program" )?;

	gl.attach_shader( &prog, &v_sh );
	gl.attach_shader( &prog, &f_sh );
	gl.link_program( &prog );

	if !gl.get_program_parameter( &prog, GL::LINK_STATUS ).as_bool().unwrap_or( false ){
		let msg = match gl.get_program_info_log( &prog ){
			Some(str)	=> str,
			None		=> String::from("Unknown error linking shaders"),
		};
		
		gl.detach_shader( &prog, &v_sh ); // TODO, detaching might cause issues on some browsers, Might only need to delete.
		gl.detach_shader( &prog, &f_sh );
		gl.delete_shader( Some(&v_sh) );
		gl.delete_shader( Some(&f_sh) );
		gl.delete_program( Some( &prog ) );
		return Err( msg );
	}

	// Can delete the shaders since the program has been made.
	gl.detach_shader( &prog, &v_sh ); // TODO, detaching might cause issues on some browsers, Might only need to delete.
	gl.detach_shader( &prog, &f_sh );
	gl.delete_shader( Some(&v_sh) );
	gl.delete_shader( Some(&f_sh) );

	Ok( prog )
}

pub fn compile_shader( gl: &GL, sh_type: u32, src: &str ) -> Result< WebGlShader, String >{
	//let gl = glctx();
	let sh = gl.create_shader( sh_type ).ok_or_else( || "Unable to create shader object" )?;

	gl.shader_source( &sh, src );
	gl.compile_shader( &sh );

	if gl.get_shader_parameter( &sh, GL::COMPILE_STATUS ).as_bool().unwrap_or( false ){
		Ok( sh )
	}else{
		let msg = match gl.get_shader_info_log( &sh ){
			Some(str)	=> str,
			None		=> String::from("Unknown error creating shader"),
		};

		gl.delete_shader( Some( &sh ) );
		console_log!( "COMPILE SHADER ERR - {} \n {}", msg, src );
		Err( msg )
	}
}


//####################################################################################

pub struct ShaderCache( pub RecycleStorage< Shader > );

impl ShaderCache{
	pub fn new() -> Self{ ShaderCache( RecycleStorage::new() ) }

	pub fn get_idx( &self, name: &str ) -> Option<usize>{
		use crate::storage::recycle_storage::Item;

		for i in 0..self.0.items.len(){
			match &self.0.items[ i ] {
				Item::Active{ data } => {
					if data.name.eq( name ) { return Some( i ); }
				}, _ => (),
			}
		}

		None
	}
}

impl Deref for ShaderCache{
	type Target = RecycleStorage< Shader >;
	fn deref( &self ) -> &Self::Target{ &self.0 }
}

impl DerefMut for ShaderCache{
	fn deref_mut( &mut self ) -> &mut Self::Target{ &mut self.0 }
}