use crate::action::Action;
use crate::item::ItemOrSlot;
use std::fmt::{Display, Formatter};

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
