use std::slice::{ Iter, IterMut };
use super::com::{ ICom, IStorage };


/////////////////////////////////////////////////////////////////
//
////////////////////////////////////////////////////////////////
	
	#[derive(Debug, Default)]
	pub struct DenseVec<T>{
		data	: Vec<T>,
		d2e		: Vec<usize>,
		e2d		: Vec<usize>,
	}

	impl<T:ICom> IStorage<T> for DenseVec<T>{
		fn new() -> Self{ DenseVec{ data: Vec::new(), e2d: Vec::new(), d2e: Vec::new() } }

		fn get( &self, idx: usize ) -> &T { &self.data[ self.e2d[ idx ] ] }
		fn get_mut( &mut self, idx: usize) -> &mut T{ &mut self.data[ self.e2d[ idx ] ]	}
		
		fn get_entities( &self ) -> Vec<usize>{ self.d2e.clone() }

		fn insert( &mut self, idx: usize, v: T ){
			let e_len = self.e2d.len();
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~			
			// Check if needing to replace a component instead of inserting one.
			if idx < e_len {
				let d_idx = self.e2d[ idx ];			// What Data is Entity Index Pointing to
				if d_idx < self.data.len() {
					let e_idx = self.d2e[ d_idx ];		// What Entity is the Data pointing to.

					if e_idx == idx {
						self.data[ d_idx ] = v;
						return;	
					}
				}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// If Entity Link isn't large enough, increase its size.
			if e_len <= idx {
				let size = idx + 1 - self.e2d.len();
				self.e2d.reserve( size );
				unsafe{ self.e2d.set_len( idx + 1 ); }
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//*self.e2d.get_unchecked_mut(idx) = self.data.len();
			self.e2d[ idx ] = self.data.len();	// Save Next Data Index to Entity ID
			self.d2e.push( idx );
			self.data.push( v );
		}

		fn remove( &mut self, idx: usize ){
			let di = self.e2d[ idx ];			// Data Index linked to Entity Index
			let ei = *self.d2e.last().unwrap();	// Get Entity ID of the last Data Item
			self.e2d[ ei ] = di;				// Update The Entity to link to a new Data Index.
			self.d2e.swap_remove( di );			// Item gets swapped with the last item in vector.
			self.data.swap_remove( di );
		}

		fn iter_entity( &self )		-> Iter<usize>{ self.d2e.iter() }
		fn iter( &self )			-> Iter<T>{ self.data.iter() }
		fn iter_mut( &mut self )	-> IterMut<T>{ self.data.iter_mut() }

		fn debug( &self ){
			println!( "Data: {:?}", self.data );
			println!( "e2d: {:?}", self.e2d );
			println!( "d2e: {:?}", self.d2e );
		}
	}


/////////////////////////////////////////////////////////////////
//
////////////////////////////////////////////////////////////////

//https://github.com/slide-rs/specs/blob/master/src/storage/storages.rs#L183
	
	#[derive(Debug, Default)]
	pub struct SparseVec<T>{
		data	: Vec<T>,
	}

	/*
	impl<T:ICom> IStorage<T> for SparseVec<T>{
		fn new() -> Self{ SparseVec{ data: Vec::new() } }

		fn get( &self, idx: usize ) -> &T { &self.data[ idx ] }
		fn get_mut( &mut self, idx: usize) -> &mut T{ &mut self.data[ idx ]	}
		
		#[allow(unused_variables)]
		fn insert( &mut self, idx: usize, v: T ){
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// If Entity Link isn't large enough, increase its size.
			//if self.data.len() <= idx{
			//	let size = idx + 1 - self.data.len();
			//	self.data.reserve( size );
			//	unsafe{ self.data.set_len( idx + 1 ); }
			//}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// *self.e2d.get_unchecked_mut(idx) = self.data.len();
			//self.data[ idx ] = v;	// Save Next Data Index to Entity ID
			//ptr::write(self.0.get_unchecked_mut(id), v);
		}
		#[allow(unused_variables)]
		fn remove( &mut self, idx: usize ){
			//ptr::read(self.data[idx])
		}

		fn iter_entity( &self ) -> Iter<usize>{  }
		fn iter( &self )		-> Iter<T>{ self.data.iter() }
		fn iter_mut( &mut self )-> IterMut<T>{ self.data.iter_mut() }

		fn debug( &self ){
			println!( "Data: {:?}", self.data );
		}
	}
	*/
