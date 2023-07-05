# A multiplayer game using planetary dynamics

This is a project serves the purpose of
1. learning rust
2. using `serde` to (de-)serialize update messages between several clients
3. building a little game
4. making the game multiplayer playable over a network

## Idea

I am interested in planetary dynamics i.e. solving coupled differential equations of massive objects influencing each other. These dynamics can be very difficult which is the reason for me to restrict everything to 2 dimensions.

Ideally the two players will create massive objects and send them into the playing field. The process of creation will have to break the laws of physics, because we cannot simply create massive objects the size of planets out of thin air. So these masses will have to "fade in" at a certain point (which is impossible in the real world), but thereafter gravitational dynamics will work correctly. 

The game (in theory) will be interesting because each player has to anticipate the planetary motions and exploit them in order to win the game. 