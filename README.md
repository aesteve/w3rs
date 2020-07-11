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
The others do, and for now there's only a derived `Debug` of actions, here's a debug output example of the last actions of a game:

```
[14m 27s 986ms] Player 1:
	SelectGroupHotkey(1)
	PreSubselection
	SelectSubgroup(SelectSubgroupAction { item: Str("edry"), object_1: 25843, object_2: 173511 })

[14m 28s 77ms] Player 1:
	UnitBuildingAbilityTargetPositionTargetObjectId(UnitBuildingAbilityActionTargetPositionTargetObjectId { ability: 0, item: Binary([3, 0, 13, 0]), target_position: Position { x: -3767.7666, y: -3365.984 }, object_1: 25770, object_2: 233844 })

[14m 28s 88ms] Player 1:
	SelectGroupHotkey(0)
	PreSubselection
	SelectSubgroup(SelectSubgroupAction { item: Str("Edem"), object_1: 27622, object_2: 42132 })

[14m 28s 159ms] Player 2: gg

[14m 28s 179ms] Player 1:
	SelectGroupHotkey(1)
	PreSubselection
	SelectSubgroup(SelectSubgroupAction { item: Str("edry"), object_1: 25843, object_2: 173511 })

[14m 28s 179ms] Player 2: gg

[14m 28s 220ms] Player 1:
	UnitBuildingAbilityTargetPositionTargetObjectId(UnitBuildingAbilityActionTargetPositionTargetObjectId { ability: 0, item: Binary([3, 0, 13, 0]), target_position: Position { x: -3734.205, y: -3364.9844 }, object_1: 25770, object_2: 233844 })

[14m 28s 331ms] Player 1:
	SelectGroupHotkey(0)
	PreSubselection
	SelectSubgroup(SelectSubgroupAction { item: Str("Edem"), object_1: 27622, object_2: 42132 })

[14m 28s 362ms] Player 1:
	UnitBuildingAbilityTargetPositionTargetObjectId(UnitBuildingAbilityActionTargetPositionTargetObjectId { ability: 24, item: Binary([3, 0, 13, 0]), target_position: Position { x: -3700.644, y: -3363.9854 }, object_1: 25770, object_2: 233844 })

[14m 28s 381ms] Player 1:
	SelectGroupHotkey(1)
	PreSubselection
	SelectSubgroup(SelectSubgroupAction { item: Str("edry"), object_1: 25843, object_2: 173511 })

[14m 28s 391ms] Player 2 left [1, 0, 0, 0]|[13, 0, 0, 0]

[14m 28s 391ms] Player 1 left [12, 0, 0, 0]|[7, 0, 0, 0]
```
Which would translate in an human-readable way as:
```
[14m 27s 986ms] Player 1 has selected the second control group. Within this group the first unit is a Dryad
[14m 28s 77ms] With this group selected, Player 1 chose to attack at Position { x: -3767.7666, y: -3365.984 } on map
[14m 28s 88ms] Player 1 has selected the second control group. Within this group the first unit is the Demon Hunter hero
[14m 28s 159ms] Player 2 said 'gg'
[14m 28s 179ms] Player 1 has selected the second control group. Within this group the first unit is a Dryad
[14m 28s 179ms] Player 2 said 'gg' (that seems to be a bug in Reforged games, some chat messages are recorded twice)
 [...]
[14m 28s 391ms] Player 2 left, losing the game
[14m 28s 391ms] Player 1 left, won the game
```

## Testing & stuff

I'm testing on a tiny subset of Reforged replays fetched from w3gjs repository, but way more (> 110 files) on a set of non-public replay files (the `replays-ignore` folder which is NOT committed).

I'm being careful that this repository does not leak "private" replays. 

So there are chances tests are not working for you. I will work on a more proper set of UNIT tests at some point in the future. For instance: extracting subset of the private replays for, say, some very specific actions like "Book of retraining" etc.

Since it's a week-end experiment, I'm more interested in "fun stuff" at the moment, like understanding how replays work, how actions are encoded, what useful information (apart from APMs) can be extracted.

---

Thank you for reading, do not hesitate to ask questions or open PRs if you feel like it :)

Thanks again to all the people involved in the reverse engineering of w3g replays,  
 
