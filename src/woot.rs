//#![allow(unused_imports)]
#![allow(dead_code)]

use crate::App;
use crate::core::shader::{ UniformType };
use crate::maths::{ Math, mat4::*, quat::* };

//###############################################################

pub fn test_shaders( app: &mut App ){
	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// SHADER
	let v_src =  r#"#version 300 es
		in vec3 a_position;
		uniform mat4 u_rot;
		void main(void){
			gl_Position = u_rot * vec4(a_position, 1.0);
        }
	"#;

	let f_src = r#"#version 300 es
		precision mediump float;
		uniform vec3 u_color;
		out vec4 oFragColor;
		void main(void){ 
			//oFragColor = vec4(1.0, 0.0, 0.0, 1.0);
			oFragColor = vec4( u_color, 1.0);
		}
	"#;
	/**/

	let sh = app.shaders.from_src( "Tester", &v_src, &f_src, true ).unwrap();
	app.shaders
		.add_uniform( &sh, "u_color", UniformType::Vec3 )
		.add_uniform( &sh, "u_rot", UniformType::Mat4 )
		.uniform_af32( &sh, "u_color", &[ 1.0, 1.0, 0.0 ] );

	console_log!("Shader Count {}", app.shaders.count() );
}


pub fn test_mesh( app: &mut App ){
	//let vertices : Vec<f32> = [
	//	-0.7, -0.7, 0.0, 
	//	0.7, -0.7, 0.0, 
	//	0.0, 0.7, 0.0
	//].to_vec();

	let vertices : Vec<f32> = [
		-0.5, 0.5, 0.0,
		-0.5, -0.5, 0.0,
		0.5, -0.5, 0.0,

		0.5, -0.5, 0.0,
		0.5, 0.5, 0.0,
		-0.5, 0.5, 0.0,
	].to_vec();

	//self.gl.f32_buffer( &vertices, 0, 3 ).unwrap();
		
	app.vaos.standard( "triangle", 3, &vertices );
	app.vaos.bind( "triangle" );
}


pub fn test_render( app: &mut App ){
	app.angle += Math::PI * 0.005;
	//app.angle = 0.0;
	
	let mat	= Mat4::from_quat( &Quat::from_axis_angle( &[0.0,0.0,1.0], app.angle ) );
	let sh = app.shaders.get( "Tester" ).unwrap();

	app.shaders.uniform_af32( &sh, "u_rot", &mat );
	app.gl.clear().draw( 6 );
}