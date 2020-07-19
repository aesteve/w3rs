use crate::blocks::action::UnitCommand;
use crate::blocks::command::{GameComponent, ParsedAction, Position, SelectedComponent};
use crate::building::{Building, Upgrade};
use crate::environment::Environment;
use crate::item::Item;
use crate::spell::{HeroSpell, Spell};
use crate::unit::{Hero, Inventory, Unit};
use nom::lib::std::collections::HashMap;
use nom::lib::std::fmt::Formatter;
use std::fmt::{Debug, Display};

pub(crate) fn from_parsed_action(
    selection: &[SelectedComponent],
    action: &ParsedAction,
    components: &HashMap<u32, GameComponent>,
    inventories: &mut HashMap<Hero, Inventory>,
    shop_buyer: Option<&Hero>,
) -> Option<Action> {
    match action {
        ParsedAction::UnitBuildingAbilityNoParams(ability) => {
            if building_selected(selection) {
                match &ability.item {
                    GameComponent::Unit(unit) => Some(Action::TrainUnit(unit.clone())),
                    GameComponent::Hero(hero) => {
                        inventories.insert(hero.clone(), Inventory::default());
                        Some(Action::TrainHero(hero.clone()))
                    }
                    GameComponent::Upgrade(upgrade) => Some(Action::TrainUpgrade(upgrade.clone())),
                    GameComponent::Item(item) => {
                        if let Some(hero) = shop_buyer {
                            Some(Action::BuyItem {
                                item: item.clone(),
                                buyer: hero.clone(),
                            })
                        } else {
                            println!(
                                "WARN :Could not find the hero who bought the item. {:?}",
                                ability
                            );
                            None
                        }
                    }
                    GameComponent::Building(building) => {
                        Some(Action::UpgradeBuilding(building.clone()))
                    }
                    _ => {
                        println!("TODO(no params && building selected) {:?}", ability);
                        None
                    }
                }
            } else if unit_or_hero_selected(selection) {
                match &ability.item {
                    GameComponent::UsedSpell(spell) => Some(Action::UsedSpell {
                        spell: spell.clone(),
                        target: None,
                        position: None,
                    }),
                    GameComponent::TrainedSpell(spell) => Some(Action::TrainSpell(spell.clone())),
                    GameComponent::Action(cmd) => match cmd {
                        UnitCommand::UseItem(slot) => {
                            use_item_from_inventory(*slot, selection, inventories, ability)
                        }
                        UnitCommand::Stop | UnitCommand::Hold => None,
                        _ => {
                            println!("TODO (action): {:?}", ability);
                            None
                        }
                    },
                    _ => {
                        println!(
                            "TODO!(noparams): ability={:?}, selection={:?}",
                            ability, selection
                        );
                        None
                    }
                }
            } else {
                None
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
                    UnitCommand::UseItem(slot) => {
                        use_item_from_inventory(*slot, selection, inventories, ability)
                    }
                    UnitCommand::SwapItem(slot) => {
                        let item1 = components.get(&ability.object_1);
                        let item2 = components.get(&ability.object_2);
                        println!(
                            "TODO(swap) Swapping item with slot {}.  ability: {:?} | {:?} | {:?}",
                            slot, ability, item1, item2
                        );
                        None
                    }
                    UnitCommand::ChangeShopBuyer => {
                        if let Some(GameComponent::Hero(hero)) = components
                            .get(&ability.object_1)
                            .or_else(|| components.get(&ability.object_2))
                        {
                            Some(Action::ChangeShopBuyer(hero.clone()))
                        } else {
                            println!(
                                "WARN: Could not find hero targeted by shop to buy items from {:?}",
                                ability
                            );
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
            GameComponent::Item(item) => {
                if let Some(GameComponent::Hero(hero)) = first_unit_from_selection(selection) {
                    Some(Action::ConsumeItem {
                        item: item.clone(),
                        by: hero,
                        at: Some(ability.target_position.clone()),
                        on: None,
                    })
                } else {
                    None
                }
            }
            GameComponent::UsedSpell(spell) => Some(Action::UsedSpell {
                spell: spell.clone(),
                target: None,
                position: Some(ability.target_position.clone()),
            }),
            _ => {
                println!("TODO(UnitBuildingAbilityTargetPosition) {:?}", ability);
                None
            }
        },
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

fn use_item_from_inventory<T: Debug>(
    slot: u8,
    selection: &[SelectedComponent],
    inventories: &mut HashMap<Hero, Inventory>,
    ability: T,
) -> Option<Action> {
    // find item in inventory
    if let Some(GameComponent::Hero(hero)) = first_unit_from_selection(selection) {
        let maybe_item = inventories
            .get_mut(&hero)
            .and_then(|inventory| inventory.use_slot(slot));
        if let Some(item) = maybe_item {
            Some(Action::ConsumeItem {
                item,
                by: hero,
                at: None,
                on: None,
            })
        } else {
            println!("WARN: Could not find item in inventory for hero: {:?} in slot {:?}. This is a WIP, please report a bug", hero, slot);
            None
        }
    } else {
        println!("BUG: looks like an item used by something that's not an hero, this is definitely a parser bug. (some unit selection has been forgotten). Ability: {:?}. Selection: {:?}", ability, selection);
        None
    }
}

fn first_unit_from_selection(selection: &[SelectedComponent]) -> Option<GameComponent> {
    selection
        .get(0)
        .and_then(|s| s.kind.as_ref())
        .map(GameComponent::clone)
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
    ChangeShopBuyer(Hero),
    BuyItem {
        item: Item,
        buyer: Hero,
    },
    ConsumeItem {
        item: Item,
        by: Hero,
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
            Action::BuyItem { item, buyer } => write!(f, "{:?} bought item {:?}", buyer, item),
            Action::ConsumeItem { item, by, at, on } => {
                write!(f, "consumed item {:?} with {:?}", item, by)?;
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
