class SuperClass {
	private function priv_fun() {
		trace("super class priv_fun");
	}

	public function pub_fun() {
		trace("public function pub_fun");
	}
}

class MyModule extends SuperClass {

	var x: Bool;
	var y: Bool;
	
    static public function main() {
	   trace("static public function main");
	   var foo = MyModule.new_(false,true);
	   trace(foo);
	   return foo;
    }

	public function new(x:Bool,y:Bool) {
		this.x = x;
		this.y = y;
	}

	public static function new_(x:Bool,y:Bool) {
		return new MyModule(x,y);
	}

	public function do_something_else() {
		trace("public function do_something_else");
	}

	public function method_with_arg(foo: String) {
		trace("public function method_with_arg" + foo);
	}
}
