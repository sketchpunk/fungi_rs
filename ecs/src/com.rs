#![allow(dead_code)]

use std::slice::{ Iter, IterMut };
use std::fmt;
use std::fmt::Debug;

use std::cell::{ RefCell, RefMut, Ref };
use std::collections::HashMap;
use std::any::{ TypeId, Any };


//LOOK, MORE DEREF EXAMPLES https://stackoverflow.com/questions/29401626/how-do-i-return-a-reference-to-something-inside-a-refcell-without-breaking-encap

/*
Try this out, get interal Reference of RefCell
use std::ops::Deref;

impl Foo {
    fn iter(&self) -> impl Deref<Target = HashMap<i32, i32>> + '_ {
        self.map.borrow()
    }
}
 */

//Example of how to create passing Guard for Ref<T>
//https://www.oipapio.com/question-622831
//https://play.rust-lang.org/?gist=fd69c4c3ba5e19fa10d4&version=stable

/*
	- ICom is the interface Trait that all Components Need to implement. Is also
	as an internal definition of what Storage type is the component is going to be used.
	The reason why we do this is when we have a generic function that is <T:ICom>, we can
	call the function like so something<Pos>. Within the function we can then access the 
	storage type as T::Storage, this allows us to basicly pass Two types in One to a generic
	function that uses ICom. As long as the Storage type implements the Default trait, it makes
	it super easy to create our storage object in a generic function, like so:
	let store : T::Storage = Default::default();	// Create IStorage Trait Object
	Box::new( Com::<T>::new( store ) )				// Save it into our Boxed Wrapper.
				

	- IStorage is an interface for all Storage Objects. We save the type storage
	as a trait object, so IStorage needs to contain all the functionality we will
	ever need for any storage the system will use.
	
	- IAnyCom is an interface we need as a starting point for our Type Map, when
	we define the struct for our manager we can only use Traits, not Generic Traits.
	The inteface's sole purpose is to help cast back to our Storage Wrapper Object Com<T>
	and to call the remove function of our IStorage Object.

	- Com<T> is our wrapper for our IStorage. Its whole purpose is so We can use ICom and RefCell
	together in a way that Rust allows. This object is stored as an ICom Trait object, by using
	ICom.as_any, we can cast is back to our Com<T>. From there we can use its inner RefCell to
	borrow our IStorage object where aur components are finally saved to.
*/



//##############################################################
	// All components need to implement this, This is so we can easily handle creating storage types.
	pub trait ICom: Debug + Sized + 'static{
		type Storage: IStorage<Self> + Default + Debug;
	}

	// Main Trait that all storage types needs to implement. Should contain all the methods you'd ever
	// need to manage the data.
	pub trait IStorage<T:ICom>{
		fn new() -> Self;

		fn get( &self, idx: usize ) -> &T;
		fn get_mut( &mut self, idx: usize ) ->&mut T;
		fn get_entities( &self ) -> Vec<usize>;

		fn insert( &mut self, idx: usize, data: T );
		fn remove( &mut self, idx: usize );

		fn iter_entity( &self ) -> Iter<usize>; //TODO Maybe Not use it, causing mut issues in some senarios
		fn iter( &self ) -> Iter<T>;
		fn iter_mut( &mut self ) -> IterMut<T>;

		fn debug( &self );
	}

	// Trait type for saving the Component Storage Wrapper
	trait IAnyCom{
		fn as_any( &self ) -> &dyn Any;
		fn rm( &self, idx: usize );
	}


//##############################################################
	// Component Storage Wrapper. Handles internal mutability, deleting entity component and helps in casting back to IStorage trait
	struct Com<T:ICom>{ 
		inner : RefCell< T::Storage >,
	}
	
	impl<T:ICom> Com<T>{
		fn new( inner: T::Storage ) -> Self{
			Com{ inner: RefCell::new( inner ) }
		}
	}

	impl<T:ICom> IAnyCom for Com<T>{
		fn as_any( &self ) -> &dyn Any{ self }

		// Call IStorage.remove() from inner
		fn rm( &self, idx: usize ){ 
			self.inner.borrow_mut().remove( idx );
		}
	}


//##############################################################
	pub struct ComponentManager{
		map 	: HashMap< TypeId, Box<IAnyCom> >,
		bit 	: HashMap< TypeId, usize >,
		types	: Vec< TypeId >,
		cnt 	: usize,
	}


	impl Debug for ComponentManager{
		fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result{
			write!( f, "cm.cnt: {}\ncm.map.keys: {:?} \ncm.bit: {:?}\ncm.types:{:?}",self.cnt, self.map.keys(), self.bit, self.types )
		}
	}


	impl ComponentManager{
		pub fn new() -> Self{
			ComponentManager{ 
				map		: HashMap::new(), 
				bit		: HashMap::new(),
				types	: Vec::new(),
				cnt		: 0,
			}
		}

		///////////////////////////////////////////////////////////
		//
		///////////////////////////////////////////////////////////
			// Create a new store based on Type and Its Storage Type
			pub fn reg<T:ICom>( &mut self ) -> &mut Self{
				let t_id = TypeId::of::<T>();
				let store : T::Storage = Default::default();
				
				self.types.push( t_id.clone() );
				self.bit.insert( t_id.clone(), self.cnt );
				self.map.insert( t_id, Box::new(
					Com::<T>::new( store )
				));

				self.cnt += 1;
				self
			}


		///////////////////////////////////////////////////////////
		// Getters - Setters
		///////////////////////////////////////////////////////////
			// On Insert, the BIT Number for that component type will be returned.
			pub fn insert<T:ICom>( &self, idx:usize, com: T ) -> usize{
				let mut cs = self.borrow_mut::<T>();
				cs.insert( idx, com );

				self.bit.get( &TypeId::of::<T>() ).unwrap().clone()
			}

			// Get a Components for Reading
			pub fn get<T:ICom>( &self, idx: usize ) -> Ref<T> {
				Ref::map( self.borrow::<T>(), |cs| { cs.get(idx) })
			}

			// Get a Component for Writing
			pub fn get_mut<T:ICom>( &self, idx: usize ) -> RefMut<T> {
				RefMut::map( self.borrow_mut::<T>(), |cs| { cs.get_mut(idx) })
			}

			//pub fn get_bit<T:ICom>( &self ) -> usize{
			//	*self.bit.get( &TypeId::of::<T>() ).unwrap()
			//}


		///////////////////////////////////////////////////////////
		// 
		///////////////////////////////////////////////////////////
			pub fn rm_by_type( &self, idx: usize, t_id: &TypeId ){
				let a = self.map.get( t_id ).unwrap();
				a.rm( idx );
			}

			pub fn rm_by_bit( &self, idx: usize, bit: usize ){
				let a = self.map.get( &self.types[ bit ] ).unwrap();
				a.rm( idx );
			}


		///////////////////////////////////////////////////////////
		// Casting
		///////////////////////////////////////////////////////////
			pub fn borrow<T:ICom>( &self ) -> Ref<T::Storage>{
				self.map.get( &TypeId::of::<T>() ).unwrap() // Get back Com as IAnyCom Trait
					.as_any()								// Cast IAnyCom to Any
					.downcast_ref::< Com<T> >().unwrap()	// Cast Any to Com<T>
					.inner.borrow()							// Borrow IStorage (can be DenseVec, SparseVec, etc)
			}

			pub fn borrow_mut<T:ICom>( &self ) -> RefMut<T::Storage>{
				self.map.get( &TypeId::of::<T>() ).unwrap() // Get back Com as IAnyCom Trait
					.as_any()								// Cast IAnyCom to Any
					.downcast_ref::< Com<T> >().unwrap()	// Cast Any to Com<T>
					.inner.borrow_mut()						// Borrow IStorage (can be DenseVec, SparseVec, etc)
			}


		///////////////////////////////////////////////////////////
		// Helper Functions
		///////////////////////////////////////////////////////////
			pub fn debug<T:ICom>( &self ){
				let cs = self.borrow::<T>();

				println!("------------------\nDebug Component: cnt:{:?} - {:?}", self.cnt, TypeId::of::<T>() );
				println!("Bit Map : {:?}", self.bit );
				cs.debug();
			}
	}