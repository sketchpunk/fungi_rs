use crate::App;
use crate::wasm::glctx;
use crate::ecs::{ ICom, IStorage, DenseVec };

//###############################################################
#[derive(Debug, Default)]
pub struct Draw{
	vao		: usize,
	shader 	: usize,
	mode	: u32,
}

impl ICom for Draw{ type Storage = DenseVec<Self>; }

impl Draw{
	pub fn new( vao: usize, mode: u32, shader: usize ) -> Self{
		Draw{ vao, mode, shader }
	}
}


//###############################################################
#[allow(unused_variables)]
pub fn draw_sys_fn( ecs: &crate::ecs::Ecs ){
	use web_sys::{ WebGl2RenderingContext as GL };
	use crate::World;

	let ws_r	= World::get();
	let ws		= ws_r.borrow();

	use super::node::Node;

	let c_vao		= ws.cache.vao.borrow();
	let c_shader	= ws.cache.shader.borrow();
	let gl 			= glctx();

	let ds 			= ecs.cm.borrow::<Draw>();
	let ns			= ecs.cm.borrow::<Node>();
	let camera		= ecs.cm.get::<super::Camera>( ws.main_camera );

	//console_log!("{:?}", &camera.pv_mat );
	//console_log!("{:?}", ns.get( app.main_camera ) );

	gl.clear( GL::COLOR_BUFFER_BIT );
	for i in ds.iter_entity() {
		let n = ns.get( *i );
		let d = ds.get( *i );

		let sh = &c_shader.get( d.shader ); //&c_shader.0.items[ d.shader ].as_ref();
		sh.bind();
		sh 	.set_f32_array( "u_model_mat", &n.matrix )
			.set_f32_array( "u_projview_mat", &camera.pv_mat );
			//.f32_array( "u_projview_mat", &app.proj );

		let v = &c_vao.get( d.vao ); //&c_vao.0.items[ d.vao ].as_ref();
		v.bind();
		gl.draw_arrays( d.mode, 0, v.elm_cnt );
	}
}