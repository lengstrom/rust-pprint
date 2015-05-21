// struct Printer {
// 	indent:1, // # of tabs added for each additional recursive level
// 	width:80,
// 	depth:Option::None
// 	// stream:Option::None // Object is a stream
// }

#![allow(dead_code)]

extern crate serde;

struct PPrinter {
	indent:u16,
	width:u16,
	depth:Option<u16>,
	stream:Option<bool>
}

impl PPrinter {
	fn width(&mut self, width : u16) -> &mut PPrinter {
		self.width = width;
		return self;
	}

	fn indent(&mut self, indent : u16) -> &mut PPrinter {
		self.indent = indent;
		return self;
	}

	fn depth(&mut self, depth : Option<u16>) -> &mut PPrinter {
		self.depth = depth;
		return self;
	}

	fn stream(&mut self, stream : Option<bool>) -> &mut PPrinter {
		self.stream = stream;
		return self;
	}
}

macro_rules! PPrint {
	( $( $x:expr ),* ) => {
		{
			
		}
	};
}


#[test]
fn it_works() {
}
