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
pub fn draw_sys_fn( app: &App ){
	use web_sys::{ WebGl2RenderingContext as GL };

	use super::node::Node;

	let c_vao		= app.cache.vao.borrow();
	let c_shader	= app.cache.shader.borrow();
	let gl 			= glctx();

	let ds 			= app.ecs.cm.borrow::<Draw>();
	let ns			= app.ecs.cm.borrow::<Node>();
	let camera		= app.ecs.cm.get::<super::Camera>( app.main_camera );

	//console_log!("{:?}", &camera.pv_mat );
	//console_log!("{:?}", ns.get( app.main_camera ) );

	gl.clear( GL::COLOR_BUFFER_BIT );
	for i in ds.iter_entity() {
		let n = ns.get( *i );
		let d = ds.get( *i );

		let sh = &c_shader.get( d.shader ); //&c_shader.0.items[ d.shader ].as_ref();
		sh.bind();
		sh 	.f32_array( "u_model_mat", &n.matrix )
			.f32_array( "u_projview_mat", &camera.pv_mat );
			//.f32_array( "u_projview_mat", &app.proj );

		let v = &c_vao.get( d.vao ); //&c_vao.0.items[ d.vao ].as_ref();
		v.bind();
		gl.draw_arrays( d.mode, 0, v.elm_cnt );
	}
}