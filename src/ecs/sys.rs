#![allow(dead_code)]

use std::fmt;
use std::cell::{ RefCell };
use super::Ecs;
use crate::App;

//#############################################################################
	// Store different types of System executors.
	enum Executor{
		Fn{ exe: SysFn },
		Tr{ exe: Box< RefCell< System > > },	// Need to store the trait in RefCell, because Need to borrow mut without needing the whole system manager be set as mut.
	}

	// System can just be a Function pointer, this is the signature
	type SysFn = fn( app: &App );

	// Or if keeping internal state, must implmenet the trait
	pub trait System{
		fn run( &mut self, app: &App );
	}


//#############################################################################
	// Manage a list of systems
	pub struct SystemManager{
		items: Vec< Item >,
	}

	// System Item
	struct Item{
		name		: String,
		priority	: u16,
		active		: bool,
		exe			: Executor,
	}

	impl Item{
		fn new( name: &str, priority: u16, exe: Executor ) -> Self { 
			Item{
				name 	: name.to_string(),
				active	: true,
				priority, 
				exe, 
			}
		}
	}


//#############################################################################
pub trait TSys: 'static + System{}			// Define a Trait as alias when Registering System Objects.
impl<T: 'static + System> TSys for T {}


impl SystemManager{

	///////////////////////////////////////////////////////////
	// Constructors
	///////////////////////////////////////////////////////////
		pub fn new() -> Self { 
			SystemManager{ items: Vec::new() }
		}

	///////////////////////////////////////////////////////////
	//
	///////////////////////////////////////////////////////////
		pub fn on( &mut self, name: &str ){		self.set_active( name, true ); }
		pub fn off( &mut self, name: &str ){	self.set_active( name, false ); }
		pub fn set_active( &mut self, name: &str, state: bool ){
			match self.items.iter_mut().find( |i| i.name.eq(name) ) {
				Some(i)=>{ i.active = state; }
				None =>{ eprintln!("Sys.set_state - Not Found : {}", name ); }
			}
		}


	///////////////////////////////////////////////////////////
	// Adding Systems
	///////////////////////////////////////////////////////////
		
		// Register a Function
		pub fn reg( &mut self, name: &str, priority: u16, f: SysFn ) -> &mut Self{
			self.save_sys( name, priority, Executor::Fn{ exe:f } );
			self
		}

		// Register a Trait
		pub fn reg_tr< T: TSys >( &mut self, name: &str, priority: u16, f: T ) -> &mut Self{
			self.save_sys( name, priority, Executor::Tr{ exe: Box::new( RefCell::new( f ) ) } );
			self
		}

		// Save the System in the order of it priority
		fn save_sys( &mut self, name: &str, priority: u16, exe: Executor ){
			let idx = self.find_priority_idx( priority );
			let itm = Item::new( name, priority, exe );

			match idx {
				None		=> self.items.push( itm ),
				Some( i )	=> self.items.insert( i, itm ),
			}
		}

		// Find the index of where to insert after based on Priority.
		fn find_priority_idx( &self, p: u16 ) -> Option<usize>{
			for (i, itm) in self.items.iter().enumerate() {
				if itm.priority <= p { continue; }
				return Some(i);
			}
			None
		}


	///////////////////////////////////////////////////////////
	// Running Systems
	///////////////////////////////////////////////////////////

		// Execute All the Systems
		pub fn run( &self, app: &App ) -> &Self {
			for i in self.items.iter(){
				if i.active {
					match &i.exe {
						Executor::Fn{ exe } => exe( app ),
						Executor::Tr{ exe } => exe.borrow_mut().run( app ),
					}
				}
			}
			self
		}
}

#[allow(unused_variables)]
impl fmt::Debug for SystemManager{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result{
		// https://doc.rust-lang.org/std/fmt/struct.Formatter.html
		// https://doc.rust-lang.org/std/fmt/struct.DebugStruct.html

		f.write_str( "System Manager" ).unwrap();
		for i in &self.items{
			/*
			f.write_fmt( format_args!("- Priority: {}\n- Active: {}\n", i.priority, i.active ) ).unwrap();
			match &i.exe {
				Executor::Fn{exe} => f.write_str( "- Exe: Fn\n" ).unwrap(),
				Executor::Tr{exe} => f.write_str( "- Exe: Tr\n" ).unwrap(),
			}
			*/
			let mut s = f.debug_struct("\nItem");
			s	.field( "Name", &i.name )
				.field( "Priority", &i.priority )
				.field( "Active", &i.active );

			match &i.exe {
				Executor::Fn{exe} =>{ s.field( "Exe", &"Fn" ); },
				Executor::Tr{exe} =>{ s.field( "Exe", &"Tr" ); },
			}
			s.finish().unwrap();
		}
		Ok(()) // write!( f, "{}", self.items.len() )
	}
}