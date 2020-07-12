#[derive(Debug, Clone, PartialEq)]
pub enum UnitAction {
    RightClick,
    Stop,
    Cancel,
    Rally,
    Attack,
    AttackGround,
    Move,
    Patrol,
    Hold,
    SwapItem(u8),
    UseItem(u8),
}

impl UnitAction {
    pub(crate) fn from_bin(binary: [u8; 2]) -> Option<UnitAction> {
        match binary {
            [3, 0] => Some(UnitAction::RightClick),
            [4, 0] => Some(UnitAction::Stop),
            [8, 0] => Some(UnitAction::Cancel),
            [15, 0] => Some(UnitAction::Attack),
            [16, 0] => Some(UnitAction::AttackGround),
            [18, 0] => Some(UnitAction::Move),
            [22, 0] => Some(UnitAction::Patrol),
            [25, 0] => Some(UnitAction::Hold),
            [34, 0] => Some(UnitAction::SwapItem(7)),
            [35, 0] => Some(UnitAction::SwapItem(8)),
            [36, 0] => Some(UnitAction::SwapItem(4)),
            [37, 0] => Some(UnitAction::SwapItem(5)),
            [38, 0] => Some(UnitAction::SwapItem(1)),
            [39, 0] => Some(UnitAction::SwapItem(2)),
            [40, 0] => Some(UnitAction::UseItem(7)),
            [41, 0] => Some(UnitAction::UseItem(8)),
            [42, 0] => Some(UnitAction::UseItem(4)),
            [43, 0] => Some(UnitAction::UseItem(5)),
            [44, 0] => Some(UnitAction::UseItem(1)),
            [45, 0] => Some(UnitAction::UseItem(2)),
            _ => None,
        }
    }
}
