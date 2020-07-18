#[derive(Debug, Clone, PartialEq)]
pub enum UnitCommand {
    RightClick,
    Stop,
    Cancel,
    Rally,
    Attack,
    AttackGround,
    Move,
    Patrol,
    Hold,
    DropItem,
    SwapItem(u8),
    UseItem(u8),
}

impl UnitCommand {
    pub(crate) fn from_bin(binary: [u8; 2]) -> Option<UnitCommand> {
        match binary {
            [3, 0] => Some(UnitCommand::RightClick),
            [4, 0] => Some(UnitCommand::Stop),
            [8, 0] => Some(UnitCommand::Cancel),
            [12, 0] => Some(UnitCommand::Rally),
            [15, 0] => Some(UnitCommand::Attack),
            [16, 0] => Some(UnitCommand::AttackGround),
            [18, 0] => Some(UnitCommand::Move),
            [22, 0] => Some(UnitCommand::Patrol),
            [25, 0] => Some(UnitCommand::Hold),
            [33, 0] => Some(UnitCommand::DropItem),
            [34, 0] => Some(UnitCommand::SwapItem(7)),
            [35, 0] => Some(UnitCommand::SwapItem(8)),
            [36, 0] => Some(UnitCommand::SwapItem(4)),
            [37, 0] => Some(UnitCommand::SwapItem(5)),
            [38, 0] => Some(UnitCommand::SwapItem(1)),
            [39, 0] => Some(UnitCommand::SwapItem(2)),
            [40, 0] => Some(UnitCommand::UseItem(7)),
            [41, 0] => Some(UnitCommand::UseItem(8)),
            [42, 0] => Some(UnitCommand::UseItem(4)),
            [43, 0] => Some(UnitCommand::UseItem(5)),
            [44, 0] => Some(UnitCommand::UseItem(1)),
            [45, 0] => Some(UnitCommand::UseItem(2)),
            _ => None,
        }
    }
}
