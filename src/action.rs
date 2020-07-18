use crate::blocks::action::UnitCommand;
use crate::blocks::command::{GameComponent, ParsedAction, Position, SelectedComponent};
use crate::building::{Building, Upgrade};
use crate::spell::{HeroSpell, Spell};
use crate::unit::{Hero, Unit};
use nom::lib::std::collections::HashMap;
use nom::lib::std::fmt::Formatter;
use std::fmt::Display;

pub(crate) fn from_parsed_action(
    selection: &[SelectedComponent],
    action: &ParsedAction,
    components: &HashMap<u32, GameComponent>,
) -> Option<Action> {
    match action {
        ParsedAction::UnitBuildingAbilityNoParams(ability) => {
            if building_selected(selection) {
                match &ability.item {
                    GameComponent::Unit(unit) => Some(Action::TrainUnit(unit.clone())),
                    GameComponent::Hero(hero) => Some(Action::TrainHero(hero.clone())),
                    GameComponent::Upgrade(upgrade) => Some(Action::TrainUpgrade(upgrade.clone())),
                    _ => None,
                }
            } else {
                None
            }
        }
        ParsedAction::UnitBuildingAbilityTargetPositionTargetObjectId(ability) => {
            if unit_or_hero_selected(selection) {
                match &ability.item {
                    GameComponent::UsedSpell(spell) => {
                        let target = components
                            .get(&ability.object_1)
                            .or_else(|| components.get(&ability.object_2))
                            .map(GameComponent::clone);
                        Some(Action::Spell {
                            spell: spell.clone(),
                            target,
                            position: ability.target_position.clone(),
                        })
                    }
                    _ => None, //TODO
                }
            } else {
                None
            }
        }
        _ => None, // TODO
    }
}

fn building_selected(selection: &[SelectedComponent]) -> bool {
    selection.iter().any(|s| match &s.kind {
        None => false,
        Some(gc) => match gc {
            GameComponent::Building(_) => true,
            _ => false,
        },
    })
}

fn unit_or_hero_selected(selection: &[SelectedComponent]) -> bool {
    selection.iter().any(|s| match &s.kind {
        None => false,
        Some(gc) => match gc {
            GameComponent::Unit(_) | GameComponent::Hero(_) => true,
            _ => false,
        },
    })
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Build {
        building: Building,
        position: Position,
    },
    TrainHero(Hero),
    TrainUnit(Unit),
    TrainUpgrade(Upgrade),
    TrainSpell(HeroSpell),
    SetRallyPoint {
        building: Building,
        position: Position,
    },
    Spell {
        spell: Spell,
        target: Option<GameComponent>,
        position: Position,
    },
    Command {
        kind: UnitCommand,
        at: Option<Position>,
    },
    Leave,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Build { building, position } => {
                write!(f, "built {:?} at {}", building, position)
            }
            Action::Spell {
                spell,
                target,
                position,
            } => {
                write!(f, "used {:?}", spell)?;
                if let Some(t) = target {
                    write!(f, " on {:?}", t)
                } else {
                    write!(f, " at {:?}", position)
                }
            }
            Action::Leave => write!(f, "left"),
            Action::TrainUpgrade(upgrade) => write!(f, "trained {:?}", upgrade),
            Action::TrainHero(hero) => write!(f, "trained {:?}", hero),
            Action::TrainUnit(unit) => write!(f, "trained {:?}", unit),
            Action::TrainSpell(spell) => writeln!(f, "learned {:?}", spell),
            Action::SetRallyPoint { building, position } => {
                writeln!(f, "set rally point for {:?} at {}", building, position)
            }
            _ => Ok(()), // TODO
        }
    }
}
