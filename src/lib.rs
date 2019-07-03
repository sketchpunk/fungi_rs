#![allow(unused_imports)]

extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;

#[macro_use]
mod wasm;			use wasm::{ glctx, Buffer, Cache, Vao, VaoCache, Shader, ShaderCache, UniformType };
mod maths;			use maths::{ Math, mat4::*, quat::*, vec3::* };
mod ecs;			use ecs::{ Ecs, IStorage };
mod com;			use com::{ Node, Draw, Camera };
mod primitives;		use primitives::{ quad_verts };
mod storage;


//###############################################################
#[wasm_bindgen(start)]
pub fn run(){
	console_error_panic_hook::set_once();
	console_log!("WASM_START");
}


//###############################################################
#[wasm_bindgen]
#[allow(dead_code)]
pub struct App{
	cache 		: Cache,
	ecs			: Ecs,
	proj 		: Mat4,
	main_camera	: usize,
	mouse_state : MouseState,
}


impl Drop for App{
	fn drop(&mut self){ console_log!("App being dropped"); }
}


//###############################################################
#[wasm_bindgen]
impl App{
	#[wasm_bindgen(constructor)]
	pub fn new( canvas_name: &str ) -> App{
		if wasm::get_webgl_context( canvas_name ).is_err(){
			panic!("Error getting WebGL Context");
		}

		App{
			ecs			: Ecs::new(),
			cache 		: Cache::new(),
			main_camera : 0,
			proj 		: Mat4::from_proj( 0.78539816339, 1.0 , 0.1, 1000.0 ), //fovy, aspect_ratio, near, far
			mouse_state : MouseState::new(),
		}
	}

	pub fn init( &mut self ){
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// GL
		let gl = glctx();
		gl.clear_color( 0.9, 0.9, 0.9, 1.0 );


		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ECS Setup
		self.ecs.cm	// Components
			.reg::<com::dynamic_vert::DynamicVert>()
			.reg::<com::Node>()
			.reg::<com::Draw>()
			.reg::<com::Camera>();

		self.ecs.sm	// Systems
			.reg_tr( "CameraHnd", 100, CameraHandler::new() )
			.reg( "test",		100,	test_sys_fn )
			.reg( "dvert",		100,	com::dynamic_vert::dynamic_vert_sys_fn )
			.reg( "node",		900,	com::node::node_sys_fn )
			.reg( "camera",		901,	com::camera::camera_sys_fn )
			.reg( "draw",		1000,	com::draw::draw_sys_fn )
			.reg( "node_clean",	2000,	com::node::node_clean_fn );

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		use wasm::ubo::{ UboBuilder, DataType };
		let mut _bld = UboBuilder::new();
		let ubo = _bld.add( "A", DataType::Vec3, 0 ).add( "B", DataType::Float, 0 ).build();
		match ubo{
			Ok( mut u ) => {
				u.set_f32( &"B", 1.0 );
				console_log!("{:?}", u );
				console_log!("{:?}", (1.0 as f32).to_bits().to_le_bytes() );
			}, _=>(),
		}


		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//
		let vao_i = self.cache.insert_vao( Vao::standard( "floor", 4, &primitives::grid_floor_verts() ) );
		let sh_i : usize = create_shader( self );
		self.e_draw( &"Test", 0, vao_i, 1, sh_i );


		let v_cnt 	= 10;
		let vao		= Vao::standard_empty( "Points_V4", 4, v_cnt );
		let vi		= self.cache.insert_vao( vao );
		let mut dbuf = com::dynamic_vert::DynamicVert::new( vi, 4, v_cnt * 4 * 4, 0 );
		dbuf.add( 0.0, 0.5, 0.0, 1.0 );
		let n 		= Node::new();
		let d 	 	= Draw::new( vi, 0, sh_i );
		let _ei		= self.ecs.new_entity_c3( "Points", 0, n, d, dbuf );


		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		let canvas_size	= wasm::get_canvas_size();
		let ratio 		= (canvas_size[0] as f32) / (canvas_size[1] as f32);

		console_log!("Size {:?} {}", canvas_size, ratio );

		let mut v = Vec3::init( 0.0, 1.0, 3.0 );
		let mut n = Node::from_pos( &v );
		n.local.rot.look( v.norm(), &VEC3_UP );

		self.main_camera = self.ecs.new_entity_c2( "Main_Camera", 0,
			n, //Node::from_pos( &[0.0, 1.0, 2.0] ), 
			Camera::with_proj( 0.78539816339, ratio , 0.1, 1000.0 )
		);

		console_log!( "Camera Entity ID {}", self.main_camera );

		//self.ecs.mk_entity_2( &"Test", Node::new(), Draw::new( vao_i, 6, sh_i ) );
		//self.e_draw( &"Test", 0, vao_i, 6, sh_i );	// Causes problem, the VAO Mut borrow is tied to self mut in the e_draw
	}
	
	pub fn render( &mut self ){
		self.ecs.sm.run( self );
	}

	pub fn pass_mouse_state( &mut self,
			is_down: bool, is_shift: bool, is_ctrl: bool,
			wh_val: i32, idx: i32, idy: i32, 
			p_ver: u32, w_ver: u32 ){
		self.mouse_state.is_down	= is_down;
		self.mouse_state.is_shift	= is_shift;
		self.mouse_state.is_ctrl	= is_ctrl;
		self.mouse_state.wheel_val	= wh_val;
		self.mouse_state.idx		= idx;
		self.mouse_state.idy		= idy;
		self.mouse_state.pos_ver	= p_ver;
		self.mouse_state.wh_ver		= w_ver;
	}

	pub fn e_draw( &mut self, name: &str, tag:u16, vi:usize, mode: u32, shi: usize ){
		self.ecs.new_entity_c2( name, tag, Node::new(), Draw::new( vi, mode, shi ) );
	}
}


//###############################################################
pub fn test_sys_fn( app: &App ){
	let es 		= app.ecs.em.borrow();
	let mut ns 	= app.ecs.cm.borrow_mut::<com::Node>();
	let idx_ary	= ns.get_entities();

	for i in idx_ary{
		let e = &es[ i ];
		if e.tag == 1 {
			let mut n = ns.get_mut( i );
			n.local.rot.rot_z( 0.015 );
			n.local.rot.rot_y( 0.01 );
			n.local.rot.rot_x( 0.02 );
			//n.local.pos[2] = -2.0;
			n.is_mod = true;	
		}
	}	

	/*
	for n in store.iter_mut(){
		n.local.rot.rot_z( 0.015 );
		n.local.rot.rot_y( 0.01 );
		n.local.rot.rot_x( 0.02 );
		n.local.pos[2] = -2.0;
		n.is_mod = true;
	}
	*/
}


pub fn create_shader( app: &App ) -> usize{
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
		
		uniform vec3 u_color;
		in vec3 o_color;
		
		out vec4 oFragColor;

		void main(void){ 
			//oFragColor = vec4(1.0, 0.0, 0.0, 1.0);
			oFragColor = vec4( u_color, 1.0);
			oFragColor = vec4( o_color, 1.0);
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
		
		.f32_array( "u_color", &[ 1.0, 1.0, 0.0 ] )
		.f32_array( "u_colorAry", &[ 0.8, 0.8, 0.8, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0 ] )
		.unbind();

	app.cache.shader.borrow_mut().insert( sh )
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MouseState{
	is_down		: bool, 
	is_shift	: bool, 
	is_ctrl		: bool,

	wheel_val	: i32, 

	idx			: i32, 
	idy			: i32,
	
	pos_ver		: u32,
	wh_ver		: u32,
}

impl MouseState{
	pub fn new() -> Self {
		MouseState{
			is_down		: false, 
			is_shift	: false, 
			is_ctrl		: false,

			wheel_val	: 0, 

			idx			: 0, 
			idy			: 0,
			
			pos_ver		: 0,
			wh_ver		: 0,
		}
	}
}

#[allow(dead_code)]
struct CameraHandler{
	wh_ver	: u32,
	pos_ver	: u32,
	is_down	: bool,

	init_pos : Vec3,
}

impl CameraHandler{
	pub fn new() -> Self {
		CameraHandler{ wh_ver:0, pos_ver:0, is_down:false, 
			init_pos: Vec3::new(),
		}
	}
}

impl ecs::System for CameraHandler{
	fn run( &mut self, app: &App ){
		//let mut store = app.ecs.cm.borrow_mut::<Node>();
		//let mut n = store.get_mut( app.main_camera );
		let do_wheel	= if self.wh_ver != app.mouse_state.wh_ver { true }else{ false };
		let do_pos		= if self.pos_ver != app.mouse_state.pos_ver { true }else{ false };

		if !do_wheel && !do_pos { return; }

		let mut n = app.ecs.cm.get_mut::<Node>( app.main_camera );

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		if do_wheel {
			// Get Forward Direction of Camera and scale it.
			let mut v = Vec3::from_copy( &VEC3_FWD );
			v.transform_quat( &n.local.rot ).scale( app.mouse_state.wheel_val as f32 * 0.5 );

			// Move Camera Forward
			n.local.pos.add( &v );
			n.is_mod = true;

			self.wh_ver = app.mouse_state.wh_ver;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		if do_pos {
			self.pos_ver = app.mouse_state.pos_ver;

			if self.is_down != app.mouse_state.is_down {
				// Down State
				if !self.is_down { self.init_pos.copy( &n.local.pos ); }
				self.is_down = app.mouse_state.is_down;
			}

			if !self.is_down { return; }

			let mut polar = Math::cartesian_to_polar( &self.init_pos );
			polar[ 0 ] += Math::to_rad(0.2) * app.mouse_state.idx as f32;
			polar[ 1 ] += Math::to_rad(0.2) * app.mouse_state.idy as f32;
			polar[ 1 ] = polar[ 1 ].max( -Math::PI_H_MIN ).min( Math::PI_H_MIN ); // TODO Fix with clamp when rust makes it stable.

			let p = Math::polar_to_cartesian( polar[0], polar[1], n.local.pos.len() );
			n.local.pos.copy( &p );
			n.local.rot.look( &p, &VEC3_UP );
			n.is_mod = true;
			//console_log!(" idx {} ", app.mouse_state.idx );
		}
	}
}