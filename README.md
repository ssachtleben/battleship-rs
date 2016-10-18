# battleship-rs
A simple implementation of the game Battleship written in rust.

See: [see wikipedia](https://secure.wikimedia.org/wikipedia/en/wiki/Battleship_game)

Rules
--------
* Each player starts with a fleet of 5 ships, of length 5, 4, 3, 3, and 2
* The ships placed horizontally or vertically on a 10x10 grid
* Players take turns to fire at positions on the grid
* Each hit revealing opponent’s ships are and are not located
* A ship is destroyed when every cell of a ship has been hit
* The winner is the first player to destroy their opponent’s fleet

Open Tasks
--------
* Add board for second player (hits of first player)
* Add computer player which tries to kill our fleet
* Allow to place ships (currently its random placed)
