class Input {
	var left_down: Bool;
	var up_down: Bool;
	var right_down: Bool;
	var down_down: Bool;

	public function new() {
		this.left_down = false;
		this.up_down = false;
		this.right_down = false;
		this.down_down = false;
	}
}

class Vector2 {
	var x: Float;
	var y: Float;

	public function new(x:Float,y:Float) {
		this.x = x;
		this.y = y;
	}
}

class GameObject {
	var name: String;
	var pos: Vector2;

	public function new(name: String) {
		this.name = name;
		this.pos = new Vector2(0,0);
	}	
}

class GameState {
	var game_objects: Array<GameObject>;
	var input: Input;

	public function new() {
		this.game_objects = [];
		this.input = new Input();
	} public static function new_() { return new GameState(); }

	public function add_new_game_object(name: String) {
		this.game_objects.push(new GameObject(name));
	}
}

