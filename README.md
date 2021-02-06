# rts-simple

The goal is develop really primitive real-time strategy game with lockstep networking. The gameplay is heavily inspired by Dune 2.

# Gameplay 

Map is fixed size grid of tiles: sand, rock and spice. All tiles are traversable by units on ground units, rock is only sutiable for building. Spice is main resource.

There are several kinds of units: underground (moves under sand), ground vechiles, buildings. 

Player starts with single building machine that can deploy buildings.

Buildings:
- Refinary -- place where harverster brings spice and converts it to money.
- Wind mill -- generates energy.
- Factory -- produces harversters, building machines and tanks.

Units: 
- Building machine -- builds buildings.
- Harverster -- collects spice tiles.
- Tank -- heavy attacking vechicle.
- Worm -- agressive neutral unit that eats everything moving on sand.
