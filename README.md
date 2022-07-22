# Chakana (working title)

A video game inspired by elements Quechua/Inca culture.

## General ideas for game elements

- [Pilgrimage of Qoyllur rit'i](https://pumadventuresperu.com/pilgrimage-to-qoyllur-riti-in-cusco-peru/)
- construction of the chakana: starts as a solid diamond, 'find/construct' the 12 points of the chakana (as abilities or something). [good write up on the cultural symbolism](https://eaglecondoralliance.com/2021/02/13/andean-wisdom-the-chakana/#:~:text=Its%20shape%20is%20that%20of,pyramid%20lying%20on%20the%20ground.)
- Concepts/words from Quechua that inspire: Apu (mountain deity), picchu (mountain), machu (old), chasquis (inca messengers), quipus (gifts/messages), Pachamama (earth mother), Ayni (reciprocity, community)
- shapeshifting between the condor, puma, and snake
  - condor: flies / glides, affected by wind maybe, land on tree tops, midweight, heavens, guidance
  - puma: fast on the ground, cut trees, heavy, present life, power/strength
  - snake: slithers, can wrap around and jump from trees, light, underworld, wisdom/intelligence

## Minimum Viable Game (living description)

1. Character is dropped on a single screen platforming level at the bottom of a mountain.
2. Able to move and swap between different forms, each with different movement options and abilities
3. Levels consist of landscape elements that react to the players movement and position differently based on their current form.
4. The game requires use of multiple forms for each level (or at least heavily encourages).
5. Some forms or abilities may only be gained after clearing some levels.
6. Reaching the right edge of the screen shifts the player to the next level.
7. The final level is the top of the mountain, finishing this level finishes the game.

## TODO

Not an exhaustive list; to serve as a development guide / design doc

- [ ] player
  - [ ] switch player physics on ground vs off ground
  - [ ] base form
    - [ ] movement
      - [x] walk to the sides
      - [x] jump
      - [x] momentum and velocity so its smoother
        - [ ] cap velocity at max
- [x] object collision system
- [ ] environment
  - [x] solid unmoving flat ground

## Credits

- Placeholder sprites:

  - [rogue from Calciumtrice](https://opengameart.org/content/animated-rogue)
  - [minimal mountainous platforms/trees](https://opengameart.org/content/minimal-2d-platformer-art-with-customisable-tress)

- Lot's of game organization inspiration from Herbert Wolverson's book [Hands-On Rust](https://hands-on-rust.com/about/)
