// struct Printer {
// 	indent:1, // # of tabs added for each additional recursive level
// 	width:80,
// 	depth:Option::None
// 	// stream:Option::None // Object is a stream
// }

extern crate serde;

struct PPrinter {
	indent:u16,
	width:u16,
	depth:Option
	stream:Option
}

impl PPrinter {
	fn width(&mut self, width : u16) -> &mut PPrinter {
		self.width = width
		self
	}

	fn indent(&mut self, indent : u16) -> &mut PPrinter {
		self.indent = indent
		self
	}

	fn depth(&mut self, depth : Option) -> &mut PPrinter {
		self.depth = depth
		self
	}

	fn stream(&mut self, stream : Option) -> &mut PPrinter {
		self.stream = stream
		self
	}
}

macro_rules! PPrint {
	() => ()
}


#[test]
fn it_works() {
}
