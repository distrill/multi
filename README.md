# multiplayer
connect to the server and move around the map.

server maintains and broadcasts game state, clients subscribe to game state
updates and send user input to server for processing.    


### messages
`Init` - sent from client to server when a client wants to add the
logged in player to the server's player list      

`Update` - sent from client to server when a player provides input to the
client - the server processes this message and incororates into the world state         

`Tick` - sent from server to client with the current state of the world. this
happens in the game loop, hopefully about 60 times per second

### server
* listen with a websocket for player updates from clients:
	 * Connected - add connection endpoint to `clients`
	 * Disconnected - remove connection endpoint from `clients` and player information from `players`
	 * Message:
	 	* Init - load player information into `players`
		* Update - parse and process player commands from clients
* game loop, broadcast game state to all connections with Tick messages

### client
* initialize:
	* "login" - add player to or load player from database
	* connect to socket
		* Connected - noop
		* Disconnected - shutdown
		* Message:
			* Tick - update game state with server data
	* emit an Init message
	* 
* game loop
	* send Update messages on user input
	* draw map and entities
