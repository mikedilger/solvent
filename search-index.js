var searchIndex = {};
searchIndex['solvent'] = {"items":[[0,"","solvent","Solvent is a dependency resolver library written in rust."],[3,"DepGraph","","This is the dependency graph."],[12,"dependencies","","List of dependencies.  Key is the element, values are the\nother elements that the key element depends upon.",0],[12,"satisfied","","Nodes already satisfied.  dependencies_of() will prune\ndependency searches at these nodes, and not output nodes\nregistered here.",0],[3,"DepGraphIterator","","This iterates through the dependencies of the DepGraph's target"],[4,"SolventError","",""],[13,"CycleDetected","","",1],[11,"clone","","",0],[11,"eq","","",1],[11,"ne","","",1],[11,"fmt","","",1],[11,"new","","Create an empty DepGraph.",0],[11,"register_dependency","","Add a dependency to a DepGraph.  The node does not need\nto pre-exist, nor do the dependency nodes.  But if the\nnode does pre-exist, the depends_on will be added to its\nexisting dependency list.",0],[11,"register_dependencies","","Add multiple dependencies of one node to a DepGraph.  The\nnode does not need to pre-exist, nor do the dependency elements.\nBut if the node does pre-exist, the depends_on will be added\nto its existing dependency list.",0],[11,"mark_as_satisfied","","This marks a node as satisfied.  Iterators will not output\nsuch nodes.",0],[11,"dependencies_of","","Get an iterator to iterate through the dependencies of\nthe target node.",0],[6,"Item","",""],[11,"next","","Get next dependency.  Returns None when finished.  If\nSome(Err(SolventError)) occurs, all subsequent calls will\nreturn None.",2]],"paths":[[3,"DepGraph"],[4,"SolventError"],[3,"DepGraphIterator"]]};
searchIndex['log'] = {"items":[[0,"","log","A lightweight logging facade."],[3,"LogRecord","","The \"payload\" of a log message."],[3,"LogLocation","","The location of a log message."],[12,"module_path","","The module path of the message.",0],[12,"file","","The source file containing the message.",0],[12,"line","","The line containing the message.",0],[3,"MaxLogLevelFilter","","A token providing read and write access to the global maximum log level\nfilter."],[3,"SetLoggerError","","The type returned by `set_logger` if `set_logger` has already been called."],[4,"LogLevel","","An enum representing the available verbosity levels of the logging framework"],[13,"Error","","The \"error\" level.",1],[13,"Warn","","The \"warn\" level.",1],[13,"Info","","The \"info\" level.",1],[13,"Debug","","The \"debug\" level.",1],[13,"Trace","","The \"trace\" level.",1],[4,"LogLevelFilter","","An enum representing the available verbosity level filters of the logging\nframework."],[13,"Off","","A level lower than all log levels.",2],[13,"Error","","Corresponds to the `Error` log level.",2],[13,"Warn","","Corresponds to the `Warn` log level.",2],[13,"Info","","Corresponds to the `Trace` log level.",2],[13,"Debug","","Corresponds to the `Debug` log level.",2],[13,"Trace","","Corresponds to the `Trace` log level.",2],[5,"max_log_level","","Returns the current maximum log level."],[5,"set_logger","","Sets the global logger."],[5,"enabled","","Determines if the current logger will ignore a log message at the specified\nlevel from the specified module."],[5,"log","","Logs a message."],[8,"Log","","A trait encapsulating the operations required of a logger"],[10,"enabled","","Determines if a log message sent at the specified level from the\nspecified module would be logged.",3],[10,"log","","Logs the `LogRecord`.",3],[11,"fmt","","",1],[11,"clone","","",1],[11,"eq","","",1],[11,"eq","","",1],[11,"partial_cmp","","",1],[11,"partial_cmp","","",1],[11,"cmp","","",1],[6,"Err","",""],[11,"from_str","","",1],[11,"fmt","","",1],[11,"max","","Returns the most verbose logging level.",1],[11,"to_log_level_filter","","Converts the `LogLevel` to the equivalent `LogLevelFilter`.",1],[11,"fmt","","",2],[11,"clone","","",2],[11,"eq","","",2],[11,"eq","","",2],[11,"partial_cmp","","",2],[11,"partial_cmp","","",2],[11,"cmp","","",2],[6,"Err","",""],[11,"from_str","","",2],[11,"fmt","","",2],[11,"max","","Returns the most verbose logging level filter.",2],[11,"to_log_level","","Converts `self` to the equivalent `LogLevel`.",2],[11,"new","","Creates a new `LogRecord`.",4],[11,"args","","The message body.",4],[11,"location","","The location of the log directive.",4],[11,"level","","The verbosity level of the message.",4],[11,"fmt","","",0],[11,"clone","","",0],[11,"fmt","","",5],[11,"get","","Gets the current maximum log level filter.",5],[11,"set","","Sets the maximum log level.",5],[11,"fmt","","",6],[11,"fmt","","",6],[14,"log!","","The standard logging macro."],[14,"error!","","Logs a message at the error level."],[14,"warn!","","Logs a message at the warn level."],[14,"info!","","Logs a message at the info level."],[14,"debug!","","Logs a message at the debug level."],[14,"trace!","","Logs a message at the trace level."],[14,"log_enabled!","","Determines if a message logged at the specified level in that module will\nbe logged."]],"paths":[[3,"LogLocation"],[4,"LogLevel"],[4,"LogLevelFilter"],[8,"Log"],[3,"LogRecord"],[3,"MaxLogLevelFilter"],[3,"SetLoggerError"]]};

searchIndex['regex'] = {"items":[[0,"","regex","This crate provides a native implementation of regular expressions that is\nheavily based on RE2 both in syntax and in implementation. Notably,\nbackreferences and arbitrary lookahead/lookbehind assertions are not\nprovided. In return, regular expression searching provided by this package\nhas excellent worst case performance. The specific syntax supported is\ndocumented further down."],[1,"Error","","Error corresponds to something that can go wrong while parsing\na regular expression."],[11,"pos","","The *approximate* character index of where the error occurred.",0],[11,"msg","","A message describing the error.",0],[1,"Captures","","Captures represents a group of captured strings for a single match."],[1,"SubCaptures","","An iterator over capture groups for a particular match of a regular\nexpression."],[1,"SubCapturesPos","","An iterator over capture group positions for a particular match of a\nregular expression."],[1,"FindCaptures","","An iterator that yields all non-overlapping capture groups matching a\nparticular regular expression."],[1,"FindMatches","","An iterator over all non-overlapping matches for a particular string."],[1,"NoExpand","","NoExpand indicates literal string replacement."],[1,"RegexSplits","","Yields all substrings delimited by a regular expression match."],[1,"RegexSplitsN","","Yields at most `N` substrings delimited by a regular expression match."],[2,"Regex","","A compiled regular expression"],[3,"quote","","Escapes all regular expression meta characters in `text`."],[3,"is_match","","Tests if the given regular expression matches somewhere in the text given."],[10,"fmt","","",0],[10,"fmt","","",0],[10,"clone","","",1],[10,"fmt","","Shows the original regular expression.",1],[10,"fmt","","Shows the original regular expression.",1],[10,"new","","Compiles a dynamic regular expression. Once compiled, it can be\nused repeatedly to search, split or replace text in a string.",1],[10,"is_match","","Returns true if and only if the regex matches the string given.",1],[10,"find","","Returns the start and end byte range of the leftmost-first match in\n`text`. If no match exists, then `None` is returned.",1],[10,"find_iter","","Returns an iterator for each successive non-overlapping match in\n`text`, returning the start and end byte indices with respect to\n`text`.",1],[10,"captures","","Returns the capture groups corresponding to the leftmost-first\nmatch in `text`. Capture group `0` always corresponds to the entire\nmatch. If no match is found, then `None` is returned.",1],[10,"captures_iter","","Returns an iterator over all the non-overlapping capture groups matched\nin `text`. This is operationally the same as `find_iter` (except it\nyields information about submatches).",1],[10,"split","","Returns an iterator of substrings of `text` delimited by a match\nof the regular expression.\nNamely, each element of the iterator corresponds to text that *isn't*\nmatched by the regular expression.",1],[10,"splitn","","Returns an iterator of at most `limit` substrings of `text` delimited\nby a match of the regular expression. (A `limit` of `0` will return no\nsubstrings.)\nNamely, each element of the iterator corresponds to text that *isn't*\nmatched by the regular expression.\nThe remainder of the string that is not split will be the last element\nin the iterator.",1],[10,"replace","","Replaces the leftmost-first match with the replacement provided.\nThe replacement can be a regular string (where `$N` and `$name` are\nexpanded to match capture groups) or a function that takes the matches'\n`Captures` and returns the replaced string.",1],[10,"replace_all","","Replaces all non-overlapping matches in `text` with the\nreplacement provided. This is the same as calling `replacen` with\n`limit` set to `0`.",1],[10,"replacen","","Replaces at most `limit` non-overlapping matches in `text` with the\nreplacement provided. If `limit` is 0, then all non-overlapping matches\nare replaced.",1],[10,"as_str","","Returns the original string of this regex.",1],[10,"reg_replace","","",2],[10,"next","","",3],[10,"next","","",4],[10,"pos","","Returns the start and end positions of the Nth capture group.\nReturns `None` if `i` is not a valid capture group or if the capture\ngroup did not match anything.\nThe positions returned are *always* byte indices with respect to the\noriginal string matched.",5],[10,"at","","Returns the matched string for the capture group `i`.  If `i` isn't\na valid capture group or didn't match anything, then `None` is\nreturned.",5],[10,"name","","Returns the matched string for the capture group named `name`.  If\n`name` isn't a valid capture group or didn't match anything, then\n`None` is returned.",5],[10,"iter","","Creates an iterator of all the capture groups in order of appearance\nin the regular expression.",5],[10,"iter_pos","","Creates an iterator of all the capture group positions in order of\nappearance in the regular expression. Positions are byte indices\nin terms of the original string matched.",5],[10,"expand","","Expands all instances of `$name` in `text` to the corresponding capture\ngroup `name`.",5],[10,"len","","Returns the number of captured groups.",5],[10,"is_empty","","Returns if there are no captured groups.",5],[10,"next","","",6],[10,"next","","",7],[10,"next","","",8],[10,"next","","",9],[6,"Replacer","","Replacer describes types that can be used to replace matches in a string."],[9,"reg_replace","","Returns a possibly owned string that is used to replace the match\ncorresponding to the `caps` capture group.",10]],"paths":[[1,"Error"],[2,"Regex"],[1,"NoExpand"],[1,"RegexSplits"],[1,"RegexSplitsN"],[1,"Captures"],[1,"SubCaptures"],[1,"SubCapturesPos"],[1,"FindCaptures"],[1,"FindMatches"],[6,"Replacer"]]};

initSearch(searchIndex);
