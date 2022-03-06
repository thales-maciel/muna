=> "set key value"
=> match split request { Command, Error }
=> SetCmd(key: key, value: value)
=> match command
=> execute command
