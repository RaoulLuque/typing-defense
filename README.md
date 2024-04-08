# Typing-Defense

![Gameplay Screenshot](https://github.com/RaoulLuque/typing-defense/assets/125205120/f9eb1adf-09fe-4834-af12-667ec57cf1c6)

Is a tower defense game in which one has to type in order to defeat monsters that try to breach the castle. The game is written using the [bevy game engine](https://bevyengine.org/) in the [Rust Programming Language](https://www.rust-lang.org/). It is more of a fun project in order to get an insight into game development, the bevy game engine, WASM and further understanding Rust.

## Gameplay
The game is played using the keyboard. Enemies (in form of animals) approach a castle at the center of the screen and the player's task is to type the words above those animals in order to prevent them from reaching said castle.

<img src="https://github.com/RaoulLuque/typing-defense/assets/125205120/ab336772-190d-4e91-947a-857b3feab181" width=35% height=35%>

## Starting the game
To start the game check the bevy setup [page](https://bevyengine.org/learn/quick-start/getting-started/setup/) for dependencies needed for compiling bevy projects locally. Then it's just running

``` cargo run ``` <br>

In the project directory. Also the game can be played on the [webpage](https://raoulluque.github.io/typing-defense/).

## Score
The score is increased by ``` current wpm * ((streak counter / 50) + 1) * ((round number / 10) + 1) * difficulty multiplier ``` every time a word is finished. Here the difficulty multiplier is 1 for easy, 2 for medium and 3 for hard. Decimals are just rounded down since the score is an integer number.

## Credits
The main framework that's used is of course [Bevy](https://bevyengine.org/) which is written in the [Rust Programming Language](https://www.rust-lang.org/). The assets are from asset packs provided by [Pixel Frog on Itch.io](https://pixelfrog-assets.itch.io/). Specifically the [Tiny Swords](https://pixelfrog-assets.itch.io/tiny-swords) and [Pixel Adventures](https://pixelfrog-assets.itch.io/pixel-adventure-1) asset packs.
