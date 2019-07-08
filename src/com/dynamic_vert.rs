#![allow(dead_code)]
use crate::App;
use crate::ecs::{ ICom, DenseVec, IStorage }; //,
use crate::wasm::{ glctx };

//############################################################
#[derive(Debug, Default)]
pub struct DynamicVert{
	vao_id		: usize,
	comp_len	: u8,
	capacity	: usize,	// In Bytes
	len 		: usize,	// In Bytes
	data 	 	: Vec<f32>,
	is_mod		: bool,
}

impl ICom for DynamicVert{ type Storage = DenseVec<Self>; }


//############################################################
impl DynamicVert{
	pub fn new( vao_id: usize, comp_len:u8, capacity: usize, len: usize ) -> Self{
		DynamicVert{
			vao_id,
		 	capacity, 
		 	comp_len, 
		 	len, 
		 	data	: Vec::with_capacity( capacity * comp_len as usize ),
		 	is_mod	: true,
		}
	}

	pub fn add( &mut self, x: f32, y:f32, z:f32, w:f32 ) -> &mut Self{
		self.data.push( x );
		self.data.push( y );
		self.data.push( z );
		if self.comp_len == 4 { self.data.push( w ); }

		self.is_mod = true;
		self
	}
}


//############################################################
pub fn dynamic_vert_sys_fn( ecs: &crate::ecs::Ecs ){
	let ws_r = crate::World::get();
	let ws = ws_r.borrow();

	let mut sm	= ecs.cm.borrow_mut::<DynamicVert>();
	let mut vc	= ws.cache.vao.borrow_mut();

	for dv in sm.iter_mut(){
		if dv.is_mod {
			let cur_size	= dv.data.len() * 4;
			let vao			= vc.get_mut( dv.vao_id );
			let b 			= vao.buffers.get_mut( "vertices" ).unwrap();

			b.bind();

			if cur_size <= dv.capacity {
				b.set_sub_f32( &dv.data );
			}else{
				dv.capacity = cur_size;
				b.set_vec_f32( &dv.data, false );
			}

			b.unbind();

			dv.len 		= cur_size;
			dv.is_mod	= false;

			// Total Bytes / 4 = Float Count / Comp_Len = Vert Count
			vao.elm_cnt	= dv.len as i32 / 4 / dv.comp_len as i32;
		}
	}
}

/*
	let v_cnt 	= 10;
	let vao		= Vao::standard_empty( "Points_V4", 4, v_cnt );
	let vi		= app.cache.insert_vao( vao );
	let dbuf	= DynamicVert::new( vi, 4, v_cnt * 4 * 4, 0 );
	let n 		= Node::new();
	let d 	 	= Draw::new( vi, 0, shi );
	let ei 		= ecs.new_entity_c3( "Points", 0, n, d, dbuf );
	self.ecs.new_entity_c3( name, tag, Node::new(), Draw::new( vi, mode, shi ) );
 */