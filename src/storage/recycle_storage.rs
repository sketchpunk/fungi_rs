#![allow(dead_code)]

const RESIZE_BY		: usize = 5;
const STARTING_SIZE	: usize = 1;

pub enum Item<T>{
	Free	{ next: Option<usize> },
	Active	{ data: T },
}

impl<T> Item<T>{
	pub fn is_free( &self )		-> bool{ match self { Item::Free{ .. }		=> true, _ => false, } }
	pub fn is_active( &self )	-> bool{ match self { Item::Active{ .. }	=> true, _ => false, } }

	// Use in iter.map mostly to get quick access to data without pattern matching
	pub fn as_ref( &self )		-> &T { 
		match self { Item::Active{ ref data } => data, _ => panic!(), } 
	}

	pub fn as_mut( &mut self ) -> &mut T {
		match self {
			Item::Active{ ref mut data } => data, _ => panic!(),
		}
	}
}

pub struct RecycleStorage< T >{
	pub items		: Vec< Item<T> >,
	free_idx		: Option< usize >,
	free_cnt		: usize,
	resize_by		: usize,
}


impl<T> RecycleStorage< T >{
	///////////////////////////////////////////////////////////
	// Constructors
	///////////////////////////////////////////////////////////
		pub fn new() -> Self{ Self::with_size( STARTING_SIZE ) }

		pub fn with_size( size: usize ) -> Self{
			let mut s = RecycleStorage::< T >{
				items		: Vec::with_capacity( size ),
				free_idx	: None,
				free_cnt	: 0,
				resize_by	: RESIZE_BY,
			};

			// Preallocate each element as Free and setup Link List Pattern to determine
			// Next Available Free Index that data can be added.
			let last = size - 1;
			for i in 0..size{
				s.items.push( Item::Free{ next : if i == last { None }else{ Some(i + 1) } } );
			}

			s.free_cnt = size;
			s.free_idx = Some( 0 );
			s
		}


	///////////////////////////////////////////////////////////
	// Manage Content
	///////////////////////////////////////////////////////////
		pub fn insert( &mut self, com : T ) -> usize{		
			if self.free_cnt == 0 { self.expand(); }

			if let Some( i ) = self.free_idx {
				if let Item::Free{ next } = self.items[ i ] {
					self.free_idx	= next;
					self.free_cnt	-= 1;
					self.items[ i ]	= Item::Active{ data: com };
					return i;
				}
			}

			println!("RecycleStorage.ERR - Free Idx not available Or free item was already active."); // This Should never Happen...
			0
		}

		pub fn rm( &mut self, idx:usize ){
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Check if overrun
			if idx >= self.items.len() { println!("RecycleStorage.ERR - Can't remove item of an invalid index, {}", idx ); return }
			
			// Check if item is already free, without check can create an infinite look on the link list
			if self.items[ idx ].is_free() { println!("RecycleStorage.ERR - Can't remove item at index {} since its already free", idx ); return }

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Clear out data and update object.
			self.items[ idx ]	= Item::Free{ next: self.free_idx };
			self.free_idx		= Some( idx );
			self.free_cnt		+= 1;
		}

		pub fn get( &self, idx: usize) -> &T{	//TODO - This isn't very safe...
			match self.items[ idx ]{
				Item::Active{ ref data } => data,
				_ => panic!("Index out of bounds or index is not active"),
			}
		}

		pub fn get_mut( &mut self, idx: usize) -> &mut T{	//TODO - This isn't very safe...
			match self.items[ idx ]{
				Item::Active{ ref mut data } => data,
				_ => panic!("Index out of bounds or index is not active"),
			}
		}


	///////////////////////////////////////////////////////////
	// 
	///////////////////////////////////////////////////////////
		// Expand Storage for more Entities
		fn expand( &mut self ){
			let cap		= self.items.capacity();
			let last	= cap + self.resize_by - 1;

			self.items.reserve_exact( self.resize_by );
			for i in cap..cap + self.resize_by{
				self.items.push( Item::Free{ next : if i == last { None }else{ Some(i+1) } } );
			}

			self.free_cnt = self.resize_by;
			self.free_idx = Some( cap );
		}


	///////////////////////////////////////////////////////////
	// Iterators
	///////////////////////////////////////////////////////////

		pub fn iter_ref( &self ) -> impl Iterator<Item = &T> {
			self.items.iter().filter( |x| x.is_active() ).map( |x| x.as_ref() )
		}

		pub fn iter_mut( &mut self ) -> impl Iterator<Item = &mut T> {
			self.items.iter_mut().filter( |x| x.is_active() ).map( |x| x.as_mut() )
		}
}	
