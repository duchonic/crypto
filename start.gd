extends Spatial

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var ls_desc = "Text printed when using help for this command (help cmdname)"; #Optional
var ls_help = "Text printed when using 'help' command"; #Optional
var highscore_help = "get the current top10 highscore"
var highscore_desc = "get highscore"

var playername : String

# Called when the node enters the scene tree for the first time.
func _ready():
	Console.connect_node(self);
	

func _process(delta):
	pass
	#get_node("chari").rotate_y(delta)
	#get_node("box").rotate_x(delta)

func name_cmd(arg0):
	if arg0:
		Console.print(arg0)
		self.playername = arg0
	else:
		pass

func put_cmd():
	SilentWolf.Scores.persist_score(self.playername, randi()%256)
	Console.print("score uploaded");

func get_cmd(arg0, arg1):
	Console.print(arg0)
	Console.print(arg1)

	yield(SilentWolf.Scores.get_high_scores(), "sw_scores_received")
	for score in SilentWolf.Scores.scores:
		var line : String = score.player_name + " ";
		line +=  str(int(score.score));
		Console.print( line );

func ls_cmd(arg0, arg1):
	#arg0 and arg1 is an arguments provided after command,
	#there can be any number of arguments
	Console.print("ls output");


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_Button_pressed():
	print_debug("button pressed");
	SilentWolf.Scores.persist_score("player_name", randi()%256)


func _on_Button2_pressed():
	yield(SilentWolf.Scores.get_high_scores(), "sw_scores_received")
	for score in SilentWolf.Scores.scores:
		print_debug(score.player_name, str(int(score.score)))
	

func _on_Button3_pressed():
	SilentWolf.Scores.wipe_leaderboard()

