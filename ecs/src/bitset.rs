#![allow(dead_code)]

const BITBLOCK_SIZE		: usize	= 16;
const BITBLOCK_SIZE_I	: f32	= 1.0 / 16.0;
type BitBlock = u16;

fn calc_block_size( n: f32 ) -> usize { ( n * BITBLOCK_SIZE_I ).ceil() as usize }
fn calc_block( n: f32 ) -> usize { ( n * BITBLOCK_SIZE_I ).floor() as usize }
fn calc_block_bit( n: usize ) -> (usize,usize){ 
	let i = ( n as f32 * BITBLOCK_SIZE_I ).floor() as usize;
	( i, if i == 0 { 1 << n }else{ 1 << n - i * BITBLOCK_SIZE } )
}

#[derive(Debug)]
pub struct BitSet{
	data: Vec< BitBlock >,
}


impl BitSet{
	//////////////////////////////////////////////////////////////////
	// Constructors
	//////////////////////////////////////////////////////////////////
		pub fn new() -> Self{ BitSet::with_size( BITBLOCK_SIZE ) }

		pub fn with_size( size: usize ) -> Self {
			let b_size	= calc_block_size( size as f32 );
			let mut bs	= BitSet{ data: Vec::with_capacity( b_size ) };

			for _i in 0..b_size{ bs.data.push( 0 ); }

			bs
		}

	//////////////////////////////////////////////////////////////////
	// Methods
	//////////////////////////////////////////////////////////////////
		pub fn on( &mut self, bit: usize ) -> &mut Self{
			let b_idx = calc_block( bit as f32 );
			if b_idx >= self.data.len() { self.expand( b_idx ); }

			self.data[ b_idx ] |= 1 << (bit % BITBLOCK_SIZE);
			self
		}

		pub fn off( &mut self, bit: usize )-> &mut Self{
			let b_idx = calc_block( bit as f32 );
			if b_idx >= self.data.len() { self.expand( b_idx ); }

			self.data[ b_idx ] &= !(1 << (bit % BITBLOCK_SIZE)); //Flip it to make a Masks 1011
			self
		}

		pub fn clear( &mut self ) -> &mut Self {
			for i in 0..self.data.len(){ self.data[i] = 0; }
			self 
		}

		pub fn is_on( &self, bit: usize ) -> bool{
			let bb = calc_block_bit( bit );
			if bb.0 >= self.data.len() { return false; }
			if (self.data[ bb.0 ] as usize) & bb.1 == bb.1 { true }else{ false }
		}

		pub fn is_mask( &self, b: &BitSet ) -> bool {
			let imax = b.data.len();
			if self.data.len() < imax { return false; }	// If the bitset has less data then a mask, automatic false
			for i in 0..imax{ 
				if self.data[i] & b.data[i] != b.data[i] { return false; } 
			}
			true
		}

		pub fn to_on_vec( &self ) -> Vec<usize>{
			let mut v = Vec::new();
			for i in self.iter( true ){ v.push( i.0 ); }
			v
		}
	
	//////////////////////////////////////////////////////////////////
	// Private Helper Functions
	//////////////////////////////////////////////////////////////////
		fn expand( &mut self, idx: usize ){
			let cap		= self.data.capacity();
			let resize	= idx - (cap - 1);
			self.data.reserve_exact( resize );

			for _i in 0..resize{ self.data.push( 0 ); }
		}

	//////////////////////////////////////////////////////////////////
	// 
	//////////////////////////////////////////////////////////////////
		pub fn iter<'a>( &'a self, on_only: bool ) -> BitSetIter<'a>{
			BitSetIter{ inner: self, block: 0, bit: 0, on_only }
		}
}


use std::iter::Iterator;

pub struct BitSetIter<'a>{
	inner	: &'a BitSet,
	block	: usize,
	bit		: usize,
	on_only	: bool,
}

impl<'a> Iterator for BitSetIter<'a>{
	//type Item = &'a (usize, bool);
	type Item = (usize, bool);
	fn next( &mut self ) -> Option<Self::Item>{
		loop{
			if self.bit >= BITBLOCK_SIZE{
				self.block	+= 1;
				self.bit	= 0;
			}

			if self.block >= self.inner.data.len() { return None; }

			let bit		= 1 << self.bit;
			let state	= if self.inner.data[ self.block ] & bit == bit { true }else{ false };
			
			// if requested for on only values, loop back if false.
			if self.on_only && !state {
				self.bit += 1;
				continue;
			}

			let bitnum 	= self.block * BITBLOCK_SIZE + self.bit;
			//println!("Look at {} - {}", self.block, self.bit );

			self.bit 	+= 1;
			return Some( (bitnum, state) );
		}
	}
}


/*
impl<'a> Iterator for BitSetIter<'a>{
	type: Item = &'a T;
// bitMax BITBLOCK_SIZE
// blockMax self.inner.data.len();
	fn next( &mut self )-> Option<Self::Item>{
		if self.pos >= self.inner.0.len(){
			None
		}else{
			self.pos += 1;
			self.inner.0.get( self.pos - 1 )
		}
	}
}
*/