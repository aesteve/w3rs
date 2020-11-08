use crate::blocks::action::UnitCommand;
use crate::blocks::command::{GameComponent, ParsedAction, Position, SelectedComponent};
use crate::building::{Building, Upgrade};
use crate::environment::Environment;
use crate::item::{Item, ItemOrSlot};
use crate::spell::{HeroSpell, Spell};
use crate::unit::{Hero, Unit};
use nom::lib::std::collections::HashMap;
use nom::lib::std::fmt::Formatter;
use std::fmt::{Debug, Display};

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
                    GameComponent::Item(item) => Some(Action::BuyItem(item.clone())),
                    GameComponent::UsedSpell(spell) => Some(Action::UsedSpell {
                        spell: spell.clone(),
                        target: None,
                        position: None,
                    }),
                    GameComponent::Building(building) => {
                        Some(Action::UpgradeBuilding(building.clone()))
                    }
                    GameComponent::Action(UnitCommand::Cancel) => {
                        // ignore
                        None
                    }
                    _ => None,
                }
            } else {
                match &ability.item {
                    GameComponent::Item(item) => Some(Action::BuyItem(item.clone())),
                    GameComponent::UsedSpell(spell) => Some(Action::UsedSpell {
                        spell: spell.clone(),
                        target: None,
                        position: None,
                    }),
                    GameComponent::TrainedSpell(spell) => Some(Action::TrainSpell(spell.clone())),
                    GameComponent::Action(cmd) => match cmd {
                        UnitCommand::UseItem(slot) => Some(Action::UseItem {
                            item_or_slot: ItemOrSlot::Slot(*slot),
                            at: None,
                            on: None,
                        }),
                        UnitCommand::Stop | UnitCommand::Hold => None,
                        _ => {
                            println!("TODO (action): {:?}", ability);
                            None
                        }
                    },
                    _ => None,
                }
            }
        }
        ParsedAction::UnitBuildingAbilityTargetPositionTargetObjectId(ability) => {
            match &ability.item {
                GameComponent::UsedSpell(spell) => {
                    let target = components
                        .get(&ability.object_1)
                        .or_else(|| components.get(&ability.object_2))
                        .map(GameComponent::clone);
                    Some(Action::UsedSpell {
                        spell: spell.clone(),
                        target,
                        position: Some(ability.target_position.clone()),
                    })
                }
                GameComponent::Action(cmd) => match cmd {
                    UnitCommand::RightClick => match first_unit_from_selection(selection) {
                        Some(GameComponent::Building(_)) => {
                            Some(Action::SetRallyPoint(ability.target_position.clone()))
                        }
                        _ => {
                            match components
                                .get(&ability.object_1)
                                .or_else(|| components.get(&ability.object_2))
                            {
                                Some(target) => Some(Action::RightClick {
                                    at: ability.target_position.clone(),
                                    target: target.clone(),
                                }),
                                None => Some(Action::Move(ability.target_position.clone())),
                            }
                        }
                    },
                    UnitCommand::Move => Some(Action::Move(ability.target_position.clone())),
                    UnitCommand::Attack => Some(Action::Attack {
                        at: Some(ability.target_position.clone()),
                        target: components
                            .get(&ability.object_1)
                            .or_else(|| components.get(&ability.object_2))
                            .map(GameComponent::clone),
                    }),
                    UnitCommand::UseItem(slot) => Some(Action::UseItem {
                        item_or_slot: ItemOrSlot::Slot(*slot),
                        at: Some(ability.target_position.clone()),
                        on: components
                            .get(&ability.object_1)
                            .or_else(|| components.get(&ability.object_2))
                            .map(GameComponent::clone),
                    }),
                    UnitCommand::SwapItem(_) => None,
                    UnitCommand::ChangeShopBuyer => {
                        if let Some(GameComponent::Hero(hero)) = components
                            .get(&ability.object_1)
                            .or_else(|| components.get(&ability.object_2))
                        {
                            Some(Action::ChangeShopBuyer(hero.clone()))
                        } else {
                            None
                        }
                    }
                    _ => {
                        println!(
                            "TODO(targetpositionobjectid)2 ability: {:?}, selection: {:?}",
                            ability, selection
                        );
                        None
                    }
                },
                _ => {
                    println!(
                        "TODO(targetpositionobjectid)3 ability: {:?}, selection: {:?}",
                        ability, selection
                    );
                    None
                }
            }
        }
        ParsedAction::UnitBuildingAbilityTwoTargetPositions(ability) => match &ability.item_2 {
            GameComponent::Environment(env) => Some(Action::GatherResources {
                units: units_from_selection(selection),
                resource: env.clone(),
                at: ability.target_position_2.clone(),
            }),
            _ => None,
        },
        ParsedAction::UnitBuildingAbilityTargetPosition(ability) => match &ability.item {
            GameComponent::Building(building) => Some(Action::Build {
                building: building.clone(),
                position: ability.target_position.clone(),
            }),
            GameComponent::Item(item) => Some(Action::UseItem {
                item_or_slot: ItemOrSlot::Item(item.clone()),
                at: Some(ability.target_position.clone()),
                on: None,
            }),
            GameComponent::UsedSpell(spell) => Some(Action::UsedSpell {
                spell: spell.clone(),
                target: None,
                position: Some(ability.target_position.clone()),
            }),
            GameComponent::Action(action) => match action {
                UnitCommand::UseItem(slot) => Some(Action::UseItem {
                    item_or_slot: ItemOrSlot::Slot(*slot),
                    at: Some(ability.target_position.clone()),
                    on: None,
                }),
                _ => {
                    println!(
                        "TODO(UnitBuildingAbilityTargetPosition + Command) {:?}",
                        ability
                    );
                    None
                }
            },
            _ => {
                println!("TODO(UnitBuildingAbilityTargetPosition) {:?}", ability);
                None
            }
        },
        ParsedAction::GiveItem(action) => {
            if let Some(GameComponent::Item(item)) = components
                .get(&action.item_object_1)
                .or_else(|| components.get(&action.item_object_2))
            {
                Some(Action::DropItem(item.clone()))
            } else {
                None
            }
        }
        ParsedAction::ChangeSelection(_)
        | ParsedAction::AssignGroupHotkey(_)
        | ParsedAction::PreSubselection
        | ParsedAction::SelectSubgroup(_)
        | ParsedAction::EnterBuildingSubmenu
        | ParsedAction::ChooseHeroSkillSubmenu
        | ParsedAction::Data(_)
        | ParsedAction::EscapedPressed
        | ParsedAction::SelectGroupHotkey(_) => None,
        _ => {
            println!("Unhandled parsed action: {:?}", action);
            None
        } // TODO
    }
}

fn units_from_selection(selection: &[SelectedComponent]) -> Vec<Unit> {
    selection
        .iter()
        .filter_map(|comp| match &comp.kind {
            Some(GameComponent::Unit(u)) => Some(u.clone()),
            _ => None,
        })
        .collect()
}

fn first_unit_from_selection(selection: &[SelectedComponent]) -> Option<&GameComponent> {
    selection.get(0).and_then(|s| s.kind.as_ref())
}

fn building_selected(selection: &[SelectedComponent]) -> bool {
    selection.iter().any(|s| match &s.kind {
        None => false,
        Some(gc) => matches!(gc, GameComponent::Building(_)),
    })
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Attack {
        at: Option<Position>,
        target: Option<GameComponent>,
    },
    Move(Position),
    Build {
        building: Building,
        position: Position,
    },
    RightClick {
        at: Position,
        target: GameComponent,
    },
    TrainHero(Hero),
    TrainUnit(Unit),
    TrainUpgrade(Upgrade),
    UpgradeBuilding(Building),
    TrainSpell(HeroSpell),
    SetRallyPoint(Position),
    UsedSpell {
        spell: Spell,
        target: Option<GameComponent>,
        position: Option<Position>,
    },
    // Items
    ChangeShopBuyer(Hero),
    BuyItem(Item),
    DropItem(Item),
    UseItem {
        item_or_slot: ItemOrSlot,
        at: Option<Position>,
        on: Option<GameComponent>,
    },
    GatherResources {
        units: Vec<Unit>,
        resource: Environment,
        at: Position,
    },
    Command {
        kind: UnitCommand,
        at: Option<Position>,
        target: Option<GameComponent>,
    },
    Leave,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Build { building, position } => {
                write!(f, "built {:?} at {}", building, position)
            }
            Action::UsedSpell {
                spell,
                target,
                position,
            } => {
                write!(f, "used {:?}", spell)?;
                if let Some(t) = target {
                    write!(f, " on {:?}", t)
                } else if let Some(pos) = position {
                    write!(f, " at {:?}", pos)
                } else {
                    Ok(())
                }
            }
            Action::Leave => write!(f, "left"),
            Action::TrainUpgrade(upgrade) => write!(f, "trained {:?}", upgrade),
            Action::TrainHero(hero) => write!(f, "trained {:?}", hero),
            Action::TrainUnit(unit) => write!(f, "trained {:?}", unit),
            Action::TrainSpell(spell) => write!(f, "learned {:?}", spell),
            Action::SetRallyPoint(position) => write!(f, "set rally point at {}", position),
            Action::BuyItem(item) => write!(f, "bought item {:?}", item),
            Action::UseItem {
                item_or_slot,
                at,
                on,
            } => {
                write!(f, "consumed item ")?;
                match item_or_slot {
                    ItemOrSlot::Slot(slot) => write!(f, "in inventory slot {}", slot)?,
                    ItemOrSlot::Item(item) => write!(f, "{:?}", item)?,
                }
                if let Some(target) = on {
                    write!(f, " on {:?}", target)
                } else if let Some(pos) = at {
                    write!(f, " at {:?}", pos)
                } else {
                    Ok(())
                }
            }
            Action::GatherResources {
                units,
                resource,
                at,
            } => write!(
                f,
                "sent units {:?} to gather resource {:?} at {:?}",
                units, resource, at
            ),
            Action::Move(at) => write!(f, "sent to {}", at),
            Action::Attack { at, target } => {
                write!(f, "ordered to attack")?;
                if let Some(t) = target {
                    write!(f, " {:?}", t)?;
                }
                if let Some(pos) = at {
                    write!(f, " at {:?}", pos)?;
                }
                Ok(())
            }
            Action::UpgradeBuilding(building) => write!(f, " upgrade to {:?}", building),
            other => write!(f, "{:?}", other), // TODO
        }
    }
}
