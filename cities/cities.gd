extends Spatial
signal restart;

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

func wipe_cmd():
	SilentWolf.Scores.wipe_leaderboard()

func ls_cmd(arg0, arg1):
	Console.print("ls output");
