extends Button


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
var timer;

func _on_sendscore_pressed():
	self.visible = false;
	get_node("/root/gameover/name").visible = false;
	get_node("/root/gameover/score").visible = false;
	
	SilentWolf.Scores.persist_score( get_node("/root/gameover/name").text, get_node("/root/gameover/score").text );


func _on_getscore_pressed():
	get_node("/root/gameover/highscore").text = "";
	yield(SilentWolf.Scores.get_high_scores(), "sw_scores_received")
	
	for score in SilentWolf.Scores.scores:
		var line : String = str(int(score.score)) +" \t";
		line += score.player_name  ;
		line += "\n\r";
		#Console.print( line );
		#print_debug(line);
		get_node("/root/gameover/highscore").text += line;
