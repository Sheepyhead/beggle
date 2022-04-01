# Mechanics
This section details the game mechanics that this game will consist of.

## Elevator pitch

You are an adventurer exploring a dungeon, and you have to kill enemies and avoid traps by launching you character, a ball, through the various rooms one at a time until you've cleared the room or died, gathering loot, in the form of powerful equipment, and experience on your way through the dungeon.

## General peggle-like concepts

- Shooting the ball is an abstraction for you character charging into a single room of the dungeon, killing monsters, dodging traps, opening chests, etc.
- When the ball hits a monster peg, that's a melee attack.
- The ball can shoot at pegs as a ranged attack/spell
- Pegs have a health value that is reduced by attacks and spells
- Two ideas for pegs:
  - A monster is always a specific constellation of pegs of varying size, health, and type, representing a more classic d&d like experience where you kill a couple monsters each encounter
    - Pros:
      - Easier to show art of the monster and differentiate between different types of monsters.
      - Makes boards easier to make since they just have to be generated from collections of basic constellations of pegs
      - Comboing different body parts is fun, discombobulate!
    - Cons:
      - Destroying a peg should feel meaningful, does taking the arm off of a monster feel meaningful enough?
      - Can be tedious to spend multiple shots killing basic mobs
  - A monster is usually a single peg of varying size, health, and type, representing a more ARPG experience where your goal is to be able to mow through many mobs as efficiently as possible
    - Pros:
      - Mowing down many mobs is satisfying and plays well with the peggle style of high combos and rising bonuses
      - I like ARPGs and there aren't enough out there :D
      - More freedom in peg placement
    - Cons:
      - More complex board configurations, meaning more room for error
      - Harder to differentiate between monsters and communicate the differences
- Traps/trap pegs are undecided. Should there be pegs you wouldn't want to hit? May be feelsbad
- Loot pegs are a good idea. Make em spawn when you kill a monster?
- How do you manage how many balls you can shoot per room? Health bar/energy cost per ball? Counterattack from still living monsters?
- Dying/running out of balls should yeet you out of the dungeon with a penalty in terms of xp. Probably not loot cause losing loot sucks
- Combos arise from hitting multiple pegs with the same ball, may have an effect on damage, effects, etc., based on what your build has. Should it have any default effects? Maybe a slight damage and speed boost a la rampage in Path of Exile? Could be handled by upping the bounciness of the ball every time the combo increases, or at breakpoints