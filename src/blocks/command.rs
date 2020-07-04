use crate::utils::zero_terminated;
use nom::{
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    IResult,
};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct CommandData {
    pub player: u8,
    length: u16,
    pub actions: Vec<Action>,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Pause,
    Resume,
    SetSpeed(u8),
    IncreaseGameSpeed,
    DecreaseGameSpeed,
    Save(String),
    SaveFinished,
    UnitBuildingAbilityNoParams(UnitBuildingAbilityActionNoParams),
    UnitBuildingAbilityTargetPosition(UnitBuildingAbilityActionTargetPosition),
    UnitBuildingAbilityTargetPositionTargetObjectId(
        UnitBuildingAbilityActionTargetPositionTargetObjectId,
    ),
    GiveItem(GiveItemToUnitAction),
    UnitBuildingAbilityTwoTargetPositions(UnitBuildingAbilityActionTwoTargetPositions),
    ChangeSelection(ChangeSelectionAction),
    AssignGroupHotkey(AssignGroupHotkeyAction),
    SelectGroupHotkey(u8),
    SelectSubgroup(SelectSubgroupAction),
    PreSubselection,
    Unknown, // 1B, skipped
    SelectGroundItem(SelectGroundItemAction),
    CancelHeroRevival(CancelHeroRevivalAction),
    RemoveUnitFromBuildingQueue(RemoveUnitFromBuildingQueueAction),
    ChangeAllyOptions(ChangeAllyOptionsAction),
    TransferResources(TransferResourcesAction),
    MapTriggerChat(String),
    EscapedPressed,
    ScenarioTrigger,
    ChooseHeroSkillSubmenu,
    EnterBuildingSubmenu,
    MinimapSignal(Position),
    ContinueGame,
    W3MMD(W3MMDAction),
    Data([u8; 16]),
}

#[derive(Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionNoParams {
    ability: u16,
    item: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTargetPosition {
    ability: u16,
    item: [u8; 4],
    target_position: Position,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTargetPositionTargetObjectId {
    ability: u16,
    item: [u8; 4],
    target_position: Position,
    object_1: u32,
    object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct GiveItemToUnitAction {
    ability: u16,
    item: [u8; 4],
    target_position: Position,
    object_1: u32,
    object_2: u32,
    item_object_1: u32,
    item_object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTwoTargetPositions {
    ability: u16,
    item_1: [u8; 4],
    target_position_1: Position,
    item_2: [u8; 4],
    target_position_2: Position,
}

#[derive(Debug, PartialEq)]
pub struct ChangeSelectionAction {
    select_mode: u8,
    selected_units: Vec<UnitSelection>,
}

#[derive(Debug, PartialEq)]
pub struct AssignGroupHotkeyAction {
    hotkey: u8,
    selected_units: Vec<UnitSelection>,
}

#[derive(Debug, PartialEq)]
pub struct UnitSelection {
    item_1: [u8; 4],
    item_2: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct SelectSubgroupAction {
    item: [u8; 4],
    object_1: u32,
    object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct SelectGroundItemAction {
    object_1: [u8; 4],
    object_2: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct CancelHeroRevivalAction {
    unit_1: [u8; 4],
    unit_2: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct RemoveUnitFromBuildingQueueAction {
    slot: u8,
    unit: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct ChangeAllyOptionsAction {
    player_slot: u8,
    option: [u8; 4],
}

#[derive(Debug, PartialEq)]
pub struct TransferResourcesAction {
    player_slot: u8,
    gold_amount: [u8; 4],   // TODO: real number
    lumber_amount: [u8; 4], // TODO: real number
}

#[derive(Debug, PartialEq)]
pub struct W3MMDAction {
    filename: String,
    mission_key: String,
    key: String,
    value: u32,
}

named!(
    pub(crate) parse_command<&[u8], CommandData>,
    do_parse!(
        player: le_u8
        >> length: le_u16
        >> actions: map_res!(take!(length), |b| parse_actions_vec(b))
        >> (CommandData {
            player,
            length,
            actions: actions.1
        })
    )
);

named!(
    parse_actions<&[u8], (Vec<Action>, &[u8])>,
    many_till!(parse_action, eof!())
);

pub(crate) fn parse_actions_vec(input: &[u8]) -> IResult<&[u8], Vec<Action>> {
    let (input, (res, _)) = parse_actions(input)?;
    Ok((input, res))
}

named!(
    pub(crate) parse_action<&[u8], Action>,
    switch!(take!(1),
        [1] => value!(Action::Pause) |
        [2] => value!(Action::Resume) |
        [3] => call!(set_speed_game) |
        [4] => value!(Action::IncreaseGameSpeed) |
        [5] => value!(Action::DecreaseGameSpeed) |
        [6] => call!(save_game) |
        [7] => call!(save_game_finished) |
        [16] => call!(unit_building_ability_no_params) |
        [17] => call!(unit_building_ability_target_position) |
        [18] => call!(unit_building_ability_target_position_target_object_id) |
        [19] => call!(give_item) |
        [20] => call!(unit_building_ability_two_target_positions) |
        [22] => call!(change_selection) |
        [23] => call!(assign_group_hotkey) |
        [24] => call!(select_group_hotkey) |
        [25] => call!(select_subgroup) |
        [26] => value!(Action::PreSubselection) |
        [27] => call!(unknown_9) |
        [28] => call!(select_ground_item) |
        [29] => call!(cancel_hero_revival) |
        [30] => call!(remove_unit_from_building_queue) |
        [31] => call!(remove_unit_from_building_queue) |
        [33] => call!(unknown_8) |
        [39] => call!(unknown_5) |
        [40] => call!(unknown_5) |
        [45] => call!(unknown_5) |
        [46] => call!(unknown_4) |
        [80] => call!(change_ally_options) |
        [81] => call!(transfer_resources) |
        [96] => call!(map_trigger_chat) |
        [97] => value!(Action::EscapedPressed) |
        [98] => call!(scenario_trigger) |
        [101] => value!(Action::ChooseHeroSkillSubmenu) |
        [102] => value!(Action::ChooseHeroSkillSubmenu) |
        [103] => value!(Action::EnterBuildingSubmenu) |
        [104] => call!(minimap_signal) |
        [105] => call!(continue_game) |
        [106] => call!(continue_game) |
        [107] => call!(w3mmd) |
        [117] => call!(unknown_1) |
        [119] => call!(unknown_13) |
        [120] => call!(unknown_20) |
        [123] => call!(data_action) |
        _ => call!(unknown)
    )
    // TODO: get missing from there: https://gist.github.com/ForNeVeR/48dfcf05626abb70b35b8646dd0d6e92
);

fn unknown_bytes(input: &[u8], len: usize) -> IResult<&[u8], Action> {
    do_parse!(input, ignore: take!(len) >> (Action::Unknown))
}

fn set_speed_game(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(input, speed: le_u8 >> (Action::SetSpeed(speed)))
}

fn save_game(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        name: zero_terminated >> (Action::Save(String::from_utf8_lossy(name).to_string()))
    )
}

fn save_game_finished(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(input, ignored: le_u16 >> (Action::SaveFinished))
}

fn unit_building_ability_no_params(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        ability: le_u16
        >> item: take!(4)
        >> ignored1: le_u32 // TODO
        >> ignored2: le_u32 // TODO
        >> (Action::UnitBuildingAbilityNoParams(UnitBuildingAbilityActionNoParams {
            ability,
            item: item[0..4].try_into().unwrap(),
        }))
    )
}

fn unit_building_ability_target_position(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        ability: le_u16
        >> item: take!(4)
        >> ignored1: le_u32 // TODO
        >> ignored2: le_u32 // TODO
        >> target_x: le_f32
        >> target_y: le_f32
        >> (Action::UnitBuildingAbilityTargetPosition(UnitBuildingAbilityActionTargetPosition {
            ability,
            item: item[0..4].try_into().unwrap(),
            target_position: Position { x: target_x, y: target_y }
        }))
    )
}

fn unit_building_ability_target_position_target_object_id(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        ability: le_u16
        >> item: take!(4)
        >> ignored1: le_u32 // TODO
        >> ignored2: le_u32 // TODO
        >> target_x: le_f32
        >> target_y: le_f32
        >> object_1: le_u32
        >> object_2: le_u32
        >> (Action::UnitBuildingAbilityTargetPositionTargetObjectId(UnitBuildingAbilityActionTargetPositionTargetObjectId {
            ability,
            item: item[..4].try_into().unwrap(),
            target_position: Position { x: target_x, y: target_y },
            object_1,
            object_2
        }))
    )
}

fn give_item(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        ability: le_u16
        >> item: take!(4)
        >> ignored1: le_u32 // TODO
        >> ignored2: le_u32 // TODO
        >> target_x: le_f32
        >> target_y: le_f32
        >> object_1: le_u32
        >> object_2: le_u32
        >> item_object_1: le_u32 // README: slot? => check
        >> item_object_2: le_u32 // README: slot? => check
        >> (Action::GiveItem(GiveItemToUnitAction {
            ability,
            item: item[0..4].try_into().unwrap(),
            target_position: Position { x: target_x, y: target_y },
            object_1,
            object_2,
            item_object_1,
            item_object_2
        }))
    )
}

fn unit_building_ability_two_target_positions(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        ability: le_u16
        >> item_1: take!(4)
        >> ignored1: le_u32 // TODO
        >> ignored2: le_u32 // TODO
        >> target_1_x: le_f32
        >> target_1_y: le_f32
        >> item_2: take!(4)
        >> ignored3: take!(9) // TODO?
        >> target_2_x: le_f32
        >> target_2_y: le_f32
        >> (Action::UnitBuildingAbilityTwoTargetPositions(UnitBuildingAbilityActionTwoTargetPositions {
            ability,
            item_1: item_1[0..4].try_into().unwrap(),
            target_position_1: Position { x: target_1_x, y: target_1_y },
            item_2: item_2[0..4].try_into().unwrap(),
            target_position_2: Position { x: target_2_x, y: target_2_y },
        }))
    )
}

fn change_selection(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        select_mode: le_u8
            >> selected_units: length_count!(le_u8, unit_selection)
            >> (Action::ChangeSelection(ChangeSelectionAction {
                select_mode,
                selected_units
            }))
    )
}

fn unit_selection(input: &[u8]) -> IResult<&[u8], UnitSelection> {
    do_parse!(
        input,
        item_1: take!(4)
            >> item_2: take!(4)
            >> (UnitSelection {
                item_1: item_1[0..4].try_into().unwrap(),
                item_2: item_2[0..4].try_into().unwrap()
            })
    )
}

fn assign_group_hotkey(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        hotkey: le_u8
            >> selected_units: length_count!(le_u8, unit_selection)
            >> (Action::AssignGroupHotkey(AssignGroupHotkeyAction {
                hotkey,
                selected_units
            }))
    )
}

fn select_group_hotkey(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        hotkey: le_u8
        >> ignored: le_u8 // TODO?
        >> (Action::SelectGroupHotkey(hotkey))
    )
}

fn select_subgroup(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        item: take!(4)
            >> object_1: le_u32
            >> object_2: le_u32
            >> (Action::SelectSubgroup(SelectSubgroupAction {
                item: item[0..4].try_into().unwrap(),
                object_1,
                object_2,
            }))
    )
}

fn select_ground_item(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        ignored: take!(1)
            >> object_1: take!(4)
            >> object_2: take!(4)
            >> (Action::SelectGroundItem(SelectGroundItemAction {
                object_1: object_1[0..4].try_into().unwrap(),
                object_2: object_2[0..4].try_into().unwrap(),
            }))
    )
}

fn cancel_hero_revival(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        unit_1: take!(4)
            >> unit_2: take!(4)
            >> (Action::CancelHeroRevival(CancelHeroRevivalAction {
                unit_1: unit_1[0..4].try_into().unwrap(),
                unit_2: unit_2[0..4].try_into().unwrap(),
            }))
    )
}

fn remove_unit_from_building_queue(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        slot: le_u8
            >> unit: take!(4)
            >> (Action::RemoveUnitFromBuildingQueue(RemoveUnitFromBuildingQueueAction {
                slot,
                unit: unit[0..4].try_into().unwrap(),
            }))
    )
}

fn change_ally_options(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        player_slot: le_u8
            >> option: take!(4)
            >> (Action::ChangeAllyOptions(ChangeAllyOptionsAction {
                player_slot,
                option: option[0..4].try_into().unwrap()
            }))
    )
}

fn transfer_resources(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        player_slot: le_u8
            >> gold_amount: take!(4)
            >> lumber_amount: take!(4)
            >> (Action::TransferResources(TransferResourcesAction {
                player_slot,
                gold_amount: gold_amount[0..4].try_into().unwrap(),
                lumber_amount: lumber_amount[0..4].try_into().unwrap()
            }))
    )
}

fn map_trigger_chat(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        ignored_1: take!(4)
            >> ignored_2: take!(4)
            >> msg: zero_terminated
            >> (Action::MapTriggerChat(String::from_utf8_lossy(msg).to_string()))
    )
}

fn scenario_trigger(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(input, ignored: take!(12) >> (Action::ScenarioTrigger))
}

fn minimap_signal(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        location_x: le_f32
            >> location_y: le_f32
            >> ignored: take!(4)
            >> (Action::MinimapSignal(Position {
                x: location_y,
                y: location_y
            }))
    )
}

fn continue_game(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(input, ignored: take!(16) >> (Action::ContinueGame))
}

fn w3mmd(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        filename: zero_terminated
            >> mission_key: zero_terminated
            >> key: zero_terminated
            >> value: le_u32
            >> (Action::W3MMD(W3MMDAction {
                filename: String::from_utf8_lossy(filename).to_string(),
                mission_key: String::from_utf8_lossy(mission_key).to_string(),
                key: String::from_utf8_lossy(key).to_string(),
                value,
            }))
    )
}

fn data_action(input: &[u8]) -> IResult<&[u8], Action> {
    do_parse!(
        input,
        data: take!(16) >> (Action::Data(data[0..16].try_into().unwrap()))
    )
}

fn unknown(input: &[u8]) -> IResult<&[u8], Action> {
    Ok((input, Action::Unknown))
}

fn unknown_1(input: &[u8]) -> IResult<&[u8], Action> {
    unknown_bytes(input, 1)
}

fn unknown_4(input: &[u8]) -> IResult<&[u8], Action> {
    unknown_bytes(input, 4)
}

fn unknown_5(input: &[u8]) -> IResult<&[u8], Action> {
    unknown_bytes(input, 5)
}

fn unknown_8(input: &[u8]) -> IResult<&[u8], Action> {
    unknown_bytes(input, 8)
}

fn unknown_9(input: &[u8]) -> IResult<&[u8], Action> {
    unknown_bytes(input, 9)
}

fn unknown_13(input: &[u8]) -> IResult<&[u8], Action> {
    unknown_bytes(input, 13)
}

fn unknown_20(input: &[u8]) -> IResult<&[u8], Action> {
    unknown_bytes(input, 13)
}
