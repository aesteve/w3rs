## W3rs

Warcraft 3 (Reforged only) replay parser, written in Rust.

## What is this?

Basically, a "week-end experiment" trying to learn Rust beyond docs and tutorials through a real-life example.
Provides a library (not a crate yet, maybe in the future? if things go well) to parse Warcraft 3 (Reforged) replay files.
It's based on 3 amazing pieces of work that MUST be credited:
* The [w3g file format gist](https://gist.github.com/ForNeVeR/48dfcf05626abb70b35b8646dd0d6e92#file-w3g_format-txt-L437) which describes, in a very exhaustive way, the replay format. Can't thank the people behind it enough for their incredible work
* [w3gjs](https://github.com/PBug90/w3gjs/) an already working Typescript implementation of a replay parser: go have a look and use it, fantastic work.
* [nom](https://github.com/Geal/nom) a parser-combinator library I'm in the process of learning. Amazing library, easy-to-use, that I'm not leveraging the full potential (yet, hopefully).

## Should I use it?

At the moment, certainly not. It's a huge work in progress I'm only committing publicly in order to receive feedback or advices (and work in a distributed way).
Hopefully, it'll prove useful in the future. 
* At least for reading the source and have yet another example on how the `nom` library can be useful & powerful.
* For people who have already written w3g parsers in other languages, and who may want to give Rust a try on an example they know
* Best case scenario, the parser will contain advanced features like displaying "build-orders" in a human-readable fashion

If your goal is to actually deal with w3g files for Reforged, count APMs, etc. Please go with existing implementation like w3gjs for instance. They're working great :)

## Can I help?

Yes!

Just opening issues:
* pointing out wrong implementations, or bugs
* highlighting some Rust anti-patterns
* indicating that some stuff could be re-rewritten in a more efficient fashion

Or submitting PRs if you want to, that'd be much appreciated.

## Current status?

Most of the implementation HAVE TO be refactored. I went the quick `unwrap` way to get started and have something that shows stuff in the console.
Obviously, a lot of work has to be done to wrap these in proper `Result`s. I'll address this some day once I'm fed up with parsing binary blocks and want to 

For now, metadata are properly parsed & almost understood: showing teams, players, their in-game races (Orc, night-elf etc.) and color (blue, red, cyan, etc.) and the game outcome (winner), when possible, in the console.
Example of output metadata:
```
Warcraft 3 Reforged game. (Ladder, OneOnOne)
	Map: Maps/frozenthrone/community/(2)northernisles.w3x
	Team 1: [ Grubby#1278 (OR) ] ✅
	Team 2: [ KroLo#11461 (OR) ]

```
(with the appropriate player colors, here `Grubby#1278` in blue, and `KroLo#11461 (OR)` in red). 

The tick mark indicates the game winner.
 
Both players having picked Orc in the example (`(OR)`).


Actions are properly parsed without crashing and are in the process of being "analyzed" (what does this action mean? what did the player actually do). 

The main goal is to be able to display a human-readable list of actions.
Some don't require any additional work, like player chat messages. 
The others do, and for now there's a derived `Debug` implementation for actions, and a `Display`, here's a display output example of some (filtered) actions of a game:

```
Anayzed game:
Warcraft 3 Reforged game. (Custom, OneOnOne)
	Map: Maps/frozenthrone/(2)terenasstand_lv.w3x
	Team 1: [ Grubby#xxxx (OR) ] ✅
	Team 2: [ Happy#xxxx (UD) ]

[1s 601ms] Player 9: [Building(GreatHall)] trained Peon
[1s 741ms] Player 9: [Building(GreatHall)] trained Peon
[2s 753ms] Player 9: [Unit(Peon)] built AltarOfStorms at {x=4256,y=-5088}
[3s 371ms] Player 8: [Unit(Acolyte)] built Ziggurat at {x=-4448,y=4064}
[6s 112ms] Player 9: [All] gl hf
[6s 131ms] Player 9: [All] gl hf
[8s 66ms] Player 8: [Building(Necropolis)] trained Acolyte
[10s 161ms] Player 8: [All] hf
[13s 163ms] Player 9: [Unit(Peon)] built Burrow at {x=4576,y=-4576}
[22s 229ms] Player 9: [Building(GreatHall)] trained Peon
[31s 889ms] Player 8: [Unit(Acolyte)] built Crypt at {x=-3584,y=4160}
[38s 657ms] Player 9: [Building(GreatHall)] trained Peon
[40s 758ms] Player 8: [Building(Necropolis)] trained Acolyte
[47s 963ms] Player 9: [Unit(Peon)] built WarMill at {x=4448,y=-4128}
[54s 941ms] Player 9: [Building(GreatHall)] trained Peon
...
[6m 38s 710ms] Player 9: [Building(Beastiary)] trained Raider
[6m 41s 83ms] Player 8: [Hero(Lich)] learned FrostNova
[6m 43s 45ms] Player 8: [Building(Slaughterhouse)] trained ObsidianStatue
...
[11m 30s 627ms] Player 9: [Unit(Raider)] used Unit(Ensnare) on Unit(Destroyer)
[11m 32s 387ms] Player 9: [Unit(Berserker)] used Unit(Berserk)
[11m 35s 925ms] Player 9: [Hero(TaurenChieftain)] used Hero(WarStomp)
[11m 36s 187ms] Player 8: [Hero(DeathKnight)] learned UnholyAura
[11m 38s 507ms] Player 9: [Hero(FarSeer)] used Hero(FeralSpirit)
[11m 39s 791ms] Player 8: [Unit(ObsidianStatue)] used Unit(MorphToDestroyer)
[11m 40s 22ms] Player 9: [Hero(TaurenChieftain)] consumed item in inventory slot 4
[11m 40s 317ms] Player 8: [Unit(Destroyer)] used Unit(DevourMagic) at Position { x: -466.86584, y: -3953.7498 }
[11m 40s 970ms] Player 9: [Hero(TaurenChieftain)] consumed item in inventory slot 5
...
[14m 23s 116ms] Player 9: [Building(Beastiary)] trained Raider
[14m 25s 894ms] Player 8: [Unit(ObsidianStatue)] used Unit(EnableAutoEssenceOfBlight)
[14m 29s 303ms] Player 8: [Hero(Lich)] used Hero(FrostNova) on Unit(Raider)
[14m 29s 399ms] Player 9: [Unit(Raider)] used Unit(Ensnare) at Position { x: 1874.1353, y: 2990.959 }
[14m 31s 839ms] Player 9: [Hero(FarSeer)] consumed item in inventory slot 5
[14m 35s 967ms] Player 9: [Hero(TaurenChieftain)] used Hero(WarStomp)
[14m 36s 371ms] Player 8: [Hero(Lich)] used Hero(FrostArmor) on Hero(DeathKnight)
[14m 36s 795ms] Player 9: [Hero(FarSeer)] used Hero(ChainLightning) on Hero(DeathKnight)
[14m 38s 228ms] Player 9: [Unit(Raider)] used Unit(Ensnare) on Hero(DeathKnight)
[14m 38s 730ms] Player 8: [Hero(DeathKnight)] used Hero(DeathCoil) on Unit(DireWolf)
[14m 39s 810ms] Player 8: [Hero(DeathKnight)] used Hero(DeathCoil) on Unit(DireWolf)
[14m 40s 718ms] Player 9: [Unit(KodoBeast)] used Unit(Devour) at Position { x: 1549.1111, y: 3128.6643 }
[14m 44s 505ms] Player 9: [Unit(Berserker)] consumed item in inventory slot 7
[14m 44s 850ms] Player 9: [Unit(Berserker)] used Hero(WarStomp)
[14m 47s 895ms] Player 9: [Building(OrcBarracks)] trained Berserker
[14m 48s 87ms] Player 9: [Building(Beastiary)] trained Raider
[14m 48s 886ms] Player 9: [Unit(Berserker)] used Unit(Berserk)
[14m 50s 511ms] Player 8: [All] gg
```

## Testing & stuff

I'm testing on a tiny subset of Reforged replays fetched from w3gjs repository, but way more (> 110 files) on a set of non-public replay files (the `replays-ignore` folder which is NOT committed).

I'm being careful that this repository does not leak "private" replays. 

So there are chances tests are not working for you. I will work on a more proper set of UNIT tests at some point in the future. For instance: extracting subset of the private replays for, say, some very specific actions like "Book of retraining" etc.

Since it's a week-end experiment, I'm more interested in "fun stuff" at the moment, like understanding how replays work, how actions are encoded, what useful information (apart from APMs) can be extracted.

---

Thank you for reading, do not hesitate to ask questions or open PRs if you feel like it :)

Thanks again to all the people involved in the reverse engineering of w3g replays,  
 
