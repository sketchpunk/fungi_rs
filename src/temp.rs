
#![allow(dead_code)]
#![allow(unused_variables)]

use ecs::Ecs;
use crate::World;
use crate::com;
use crate::com::{ Node, Camera };
use crate::wasm::{ Shader, UniformType };
use maths::{ vec3::*, quat::* };

//###############################################################################


//###############################################################################


pub fn setup_ecs( ecs: &mut Ecs ){
	ecs.cm	// Components
		.reg::<com::dynamic_vert::DynamicVert>()
		.reg::<com::Node>()
		.reg::<com::Draw>()
		.reg::<com::Camera>();

	ecs.sm	// Systems
		.reg_tr( "CameraHnd", 100, com::camera::CameraHandler::new() )
		//.reg( "test",		100,	test_sys_fn )
		.reg( "dvert",		100,	com::dynamic_vert::dynamic_vert_sys_fn )
		.reg( "node",		900,	com::node::node_sys_fn )
		.reg( "camera",		901,	com::camera::camera_sys_fn )
		.reg( "draw",		1000,	com::draw::draw_sys_fn )
		.reg( "node_clean",	2000,	com::node::node_clean_fn );
}


pub fn setup_camera( ecs: &Ecs ){
	let ws_r	= World::get();
	let mut ws	= ws_r.borrow_mut();

	let ratio  = (ws.canvas_size[0] as f32) / (ws.canvas_size[1] as f32);
	let mut v = Vec3::init( 0.0, 1.0, 3.0 );
	let mut n = Node::from_pos( &v );

	n.local.rot.look( v.norm(), &VEC3_UP );

	ws.main_camera = ecs.new_entity_c2( "Main_Camera", 0,
		n, //Node::from_pos( &[0.0, 1.0, 2.0] ), 
		Camera::with_proj( 0.78539816339, ratio , 0.1, 1000.0 )
	);

	console_log!( "Camera Entity ID {}", ws.main_camera );
}


pub fn setup_floor( ecs: &Ecs ){
	//let ws_r	= World::get();
	//let mut ws	= ws_r.borrow();

	//let vao_i = ws.cache.insert_vao( Vao::standard( "floor", 4, &primitives::grid_floor_verts() ) );
	//self.e_draw( &"Test", 0, vao_i, 1, sh_i );

	//ecs.new_entity_c2( name, tag, Node::new(), Draw::new( vi, mode, shi ) );
}


//###############################################################################
pub fn create_shader() -> usize{
	// SHADER
	let v_src =  r#"#version 300 es
		in vec4 a_position;

		uniform mat4 u_model_mat;
		uniform mat4 u_projview_mat;
		uniform vec3 u_colorAry[3];

		out vec3 o_color;

		//uniform mat4 u_view_mat;
		//uniform mat4 u_proj_mat;
		void main(void){
			gl_PointSize = 10.0;
			o_color = u_colorAry[ int(a_position.w) ];

			//gl_Position = u_proj_mat * u_view_mat * vec4(a_position, 1.0);
			gl_Position = u_projview_mat * u_model_mat * vec4(a_position.xyz, 1.0);
        }
	"#;

	let f_src = r#"#version 300 es
		precision mediump float;
		
		uniform UBOTest{
			vec3	c_rgb;
			float	f_value;
		};

		uniform vec3 u_color;
		in vec3 o_color;
		
		out vec4 oFragColor;

		void main(void){ 
			//oFragColor = vec4(1.0, 0.0, 0.0, 1.0);
			oFragColor = vec4( u_color, 1.0);
			oFragColor = vec4( o_color, 1.0);
			oFragColor = vec4( c_rgb, 1.0);
		}
	"#;


	let mut sh = Shader::from_src( "Test", &v_src, &f_src ).unwrap();
	sh.bind();
	sh	.add_uniform( "u_color", UniformType::Vec3 )
		//.add_uniform( "u_view_mat", UniformType::Mat4 )
		//.add_uniform( "u_proj_mat", UniformType::Mat4 )
		.add_uniform( "u_model_mat", UniformType::Mat4 )
		.add_uniform( "u_projview_mat", UniformType::Mat4 )
		.add_uniform( "u_colorAry", UniformType::Vec3 )
		.add_uniform_block( "UBOTest", 0 )
		
		.set_f32_array( "u_color", &[ 1.0, 1.0, 0.0 ] )
		.set_f32_array( "u_colorAry", &[ 0.8, 0.8, 0.8, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0 ] )
		.unbind();

	World::insert_shader( sh )
}
