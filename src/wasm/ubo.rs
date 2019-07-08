use web_sys::{ WebGl2RenderingContext as GL };

use std::collections::HashMap;
use std::ops::{ Deref, DerefMut };

use super::{ glctx, Buffer };

// [[ NOTES ]]
// http://www.geeks3d.com/20140704/gpu-buffers-introduction-to-opengl-3-1-uniform-buffers-objects/

//####################################################################################

#[derive(Debug, Clone)]
pub enum DataType{
	Float,
	Int,
	Bool,
	Mat2x4,
	Mat4,
	Mat3,
	Vec2,
	Vec3,
	Vec4,
}


//####################################################################################

pub struct UboBuilder{ items: Vec<Item> }

#[derive(Debug)]
struct Item{
	name 		: String,
	data_type 	: DataType,
	offset		: u32,
	block_size	: u32,
	data_size	: u32,
	ary_len		: u32,
}


impl UboBuilder{
	pub fn new() -> Self { UboBuilder{ items: Vec::new() } }

	// Add Variable Item
	pub fn add( &mut self, name: &str, data_type: DataType, ary_len: u32 ) -> &mut Self{
		self.items.push( Item{ 
			name		: name.to_string(),
			offset		: 0,
			block_size	: 0,
			data_size	: 0,
			data_type, 
			ary_len,
		});
		self
	}

	//Calcuate the Buffer Stride Information then setup ubo in gl
	pub fn build( &mut self, bind_pnt: u32 ) -> Result< Ubo, String >{
		let buf 		= Buffer::new_uniform()?;
		let byte_len 	= self.calc() as usize;

		let mut ubo = Ubo{
			buf,
			name 		: "test".to_string(),
			bind_point	: bind_pnt,
			byte_len	: byte_len,
			bytes 		: vec![ 0; byte_len ],
			items		: HashMap::new(),
		};

		for i in &self.items{
			ubo.items.insert( i.name.clone(), UboSpec{ 
				offset		: i.offset as usize,
				data_size	: i.data_size as usize,
				data_type	: i.data_type.clone(),
			} );
		}

		ubo.buf.bind();								// Bind it for work
		ubo.buf.set_empty_f32( byte_len as f64 );	// Allocate Space in empty buf
		ubo.buf.unbind();							// Unbind
		ubo.buf.set_bindpoint( bind_pnt );			// Save Buffer to Uniform Buffer Bind point

		Ok( ubo )
	}

	// Get the Alignment Size and Byte Size for specific Types
	fn get_size( &self, t: &DataType ) -> (u32, u32) { // ( Alignment, Size )
		match t{
			DataType::Float | DataType::Int | DataType::Bool
								=> (4,4),
			DataType::Mat2x4	=> (32,32),	// 16*2
			DataType::Mat4		=> (64,64),	// 16*4
			DataType::Mat3		=> (48,48),	// 16*3
			DataType::Vec2		=> (8,8),
			DataType::Vec3		=> (16,12),	// Special Case
			DataType::Vec4		=> (16,16),
		}
	}

	fn calc( &mut self ) -> u32 {
		let mut block_size	= 16;			// Data size in Bytes, UBO using layout std140 needs to build out the struct in blocks of 16 bytes.
		let mut offset		= 0;			// Offset in the buffer allocation
		let mut size		: (u32, u32);	// Data Size of the current type
		let mut ary_len		: u32;
		let mut itm			: &mut Item;

 		for i in 0..self.items.len(){			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// When dealing with arrays, Each element takes up 16 bytes regardless of type, but if the type
			// is a factor of 16, then that values times array length will work, in case of matrices
			ary_len	= self.items[i].ary_len;
			size 	= self.get_size( &self.items[i].data_type );
			if ary_len > 0{
				size.0 = if size.0 < 16 { ary_len * 16 }else{ size.0 * ary_len };	// Alignment
				size.1 = if size.1 < 16 { ary_len * 16 }else{ size.1 * ary_len };	// Size
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Check if there is enough block space, if not 
			// give previous item the remainder block space
			// If the block space is full and the size is equal too or greater, dont give back to previous
			if block_size >= size.0{
				block_size -= size.1;

			}else if block_size > 0 && i > 0 && !( block_size == 16 && size.1 >= 16 ){
				self.items[i-1].block_size	+= block_size;
				offset						+= block_size;
				block_size					= 16 - size.1;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Save data about the item
			itm = &mut self.items[i];
			itm.offset		= offset;
			itm.block_size	= size.1;
			itm.data_size	= size.1;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Cleanup
			if block_size <= 0 { block_size = 16; } //Reset
			offset += size.1;
 			//console_log!("{:?}", &self.items[i] );
 		}

 		//self.visualize();
 		offset
	}

	fn visualize( &mut self ){
		let mut sbuf	= String::new();
		let mut i		= 0;
		let mut tchunk	= 0;
		let mut chunk	: u32;
		let mut b_size 	= 0;

		for itm in &self.items{
			console_log!( "Item: {:?}", itm );
			chunk	= itm.block_size / 4;
			b_size	+= itm.block_size;

			for x in 0..chunk{
				if x == 0 || x == chunk-1 { 
					sbuf.push_str( &format!( "|.{}.", i ) );
				}else{
					sbuf.push_str( &"|..." );
				}

				tchunk += 1;
				if tchunk % 4 == 0 { sbuf.push_str( &"| ~ " ) }
			}
			i += 1;
		}

		if tchunk % 4 != 0{ sbuf.push_str( "|" ) };
		console_log!( "{}", sbuf );
		console_log!( "Total Chunks: {}, Byte Size: {} ", tchunk, b_size );
	}
}


//####################################################################################

#[derive(Debug)]
pub struct Ubo{
	buf			: Buffer,
	name		: String,
	bind_point	: u32,
	byte_len	: usize,
	bytes		: Vec<u8>,
	items		: HashMap<String, UboSpec>,
}

#[derive(Debug)]
pub struct UboSpec{
	offset		: usize,
	data_size	: usize,
	data_type	: DataType,
}

impl Ubo{
	/////////////////////////////////////////////////////////////////////
	// Methods
	/////////////////////////////////////////////////////////////////////
		pub fn bind( &self ){ self.buf.bind(); }
		pub fn unbind( &self ){ self.buf.unbind(); }
		pub fn delete( &self ){ self.buf.delete(); }

		// Pass the Binary array to the GPU Buffer.
		pub fn update( &mut self ){	// Update needs mut, for some reason the sub buffer needs a mut array.
			self.buf.bind();
			self.buf.set_sub_u8( self.bytes.as_mut_slice(), 0 );
			self.buf.unbind();
		}


	/////////////////////////////////////////////////////////////////////
	// Data Writing
	/////////////////////////////////////////////////////////////////////
		// Write One float to the Byte Array
		pub fn set_f32( &mut self, name: &str, v:f32 ) -> &mut Self{
			let offset	= self.items.get( name ).unwrap().offset;
			let ary		= v.to_bits().to_le_bytes(); // Little endian WebGL Needs it. 
			self.write( offset, &ary );
			self
		}

		// Write Float Array to the Binary Array
		pub fn set_f32_ary( &mut self, name: &str, ary: &[f32] ) -> &mut Self{
			let itm			= self.items.get( name ).unwrap(); //= self.items.get( name ).unwrap().offset;
			let data_size	= itm.data_size / 4; // Data_size in bytes, div by 4 to get float count

			if ary.len() > data_size {
				console_log!("Can not save F32 Array, its to big for ubo: {}", name );
				return self;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			let mut offset	= itm.offset; 
			let mut bary	: [u8; 4];
			for i in 0..ary.len(){
				bary = ary[ i ].to_bits().to_ne_bytes(); //If there is an issue, use le_bytes, Webgl uses Little Endian
				for j in 0..4{
					self.bytes[ offset ] = bary[ j ];
					offset += 1;
				}
			}
			self
		}

		// Helper function to quicky copy Bytes to the Array
		pub fn write( &mut self, offset: usize, ary: &[u8] ){
			for i in 0..ary.len(){ self.bytes[ offset+i ] = ary[i]; }
		}
}


//####################################################################################
// UBO CACHE

pub struct UboCache( pub HashMap< String, Ubo > );

impl UboCache{
	pub fn new() -> Self{ UboCache( HashMap::new() ) }
}

impl Deref for UboCache{
	type Target = HashMap< String, Ubo >;
	fn deref( &self ) -> &Self::Target{ &self.0 }
}

impl DerefMut for UboCache{
	fn deref_mut( &mut self ) -> &mut Self::Target{ &mut self.0 }
}