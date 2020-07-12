use crate::utils::zero_terminated_string;
use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::multi::{count, many0};
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
pub(crate) enum ItemOrUnit {
    Str(String),
    Binary([u8; 4]),
}

#[derive(Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionNoParams {
    ability: u16,
    item: ItemOrUnit,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTargetPosition {
    ability: u16,
    item: ItemOrUnit,
    target_position: Position,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTargetPositionTargetObjectId {
    ability: u16,
    item: ItemOrUnit,
    target_position: Position,
    object_1: u32,
    object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct GiveItemToUnitAction {
    ability: u16,
    item: ItemOrUnit,
    target_position: Position,
    object_1: u32,
    object_2: u32,
    item_object_1: u32,
    item_object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTwoTargetPositions {
    ability: u16,
    item_1: ItemOrUnit,
    target_position_1: Position,
    item_2: ItemOrUnit,
    target_position_2: Position,
}

#[derive(Debug, PartialEq)]
pub struct ChangeSelectionAction {
    select_mode: SelectionMode,
    selected_units: Vec<UnitSelection>,
}

#[derive(Debug, PartialEq)]
pub struct AssignGroupHotkeyAction {
    hotkey: u8,
    selected_units: Vec<UnitSelection>,
}

#[derive(Debug, PartialEq)]
pub struct UnitSelection {
    object_1: u32,
    object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct SelectSubgroupAction {
    item: ItemOrUnit,
    object_1: u32,
    object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct SelectGroundItemAction {
    object_1: ItemOrUnit,
    object_2: ItemOrUnit,
}

#[derive(Debug, PartialEq)]
pub struct CancelHeroRevivalAction {
    unit_1: ItemOrUnit,
    unit_2: ItemOrUnit,
}

#[derive(Debug, PartialEq)]
pub struct RemoveUnitFromBuildingQueueAction {
    slot: u8,
    unit: ItemOrUnit,
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

#[derive(Debug, PartialEq)]
pub enum SelectionMode {
    Add,
    Remove,
}

fn parse_selection_mode(input: &[u8]) -> IResult<&[u8], SelectionMode> {
    let (rest, selection) = le_u8(input)?;
    match selection {
        1 => Ok((rest, SelectionMode::Add)),
        _ => Ok((rest, SelectionMode::Remove)),
    }
}

fn parse_actions(input: &[u8]) -> IResult<&[u8], Vec<Action>> {
    many0(parse_action)(input)
}

pub(crate) fn parse_command(input: &[u8]) -> IResult<&[u8], CommandData> {
    let (rest, player) = le_u8(input)?;
    let (rest, length) = le_u16(rest)?;
    let (rest, actions) = map_res(take(length as usize), parse_actions)(rest)?;
    Ok((
        rest,
        CommandData {
            player,
            length,
            actions: actions.1,
        },
    ))
}

fn item_or_unit(input: &[u8]) -> IResult<&[u8], ItemOrUnit> {
    let (rest, bytes) = take(4usize)(input)?;
    match bytes {
        [_, _, 13, 0] => Ok((rest, ItemOrUnit::Binary(bytes[0..4].try_into().unwrap()))),
        _ => Ok((
            rest,
            ItemOrUnit::Str(
                String::from_utf8_lossy(bytes)
                    .to_string()
                    .chars()
                    .rev()
                    .collect(),
            ),
        )),
    }
}

pub(crate) fn parse_action(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, kind) = le_u8(input)?;
    match kind {
        1 => Ok((rest, Action::Pause)),
        2 => Ok((rest, Action::Resume)),
        3 => set_speed_game(rest),
        4 => Ok((rest, Action::IncreaseGameSpeed)),
        5 => Ok((rest, Action::DecreaseGameSpeed)),
        6 => save_game(rest),
        7 => save_game_finished(rest),
        16 => unit_building_ability_no_params(rest),
        17 => unit_building_ability_target_position(rest),
        18 => unit_building_ability_target_position_target_object_id(rest),
        19 => give_item(rest),
        20 => unit_building_ability_two_target_positions(rest),
        22 => change_selection(rest),
        23 => assign_group_hotkey(rest),
        24 => select_group_hotkey(rest),
        25 => select_subgroup(rest),
        26 => Ok((rest, Action::PreSubselection)),
        27 => unknown_9(rest),
        28 => select_ground_item(rest),
        29 => cancel_hero_revival(rest),
        30 => remove_unit_from_building_queue(rest),
        31 => remove_unit_from_building_queue(rest),
        33 => unknown_8(rest),
        39 => unknown_5(rest),
        40 => unknown_5(rest),
        45 => unknown_5(rest),
        46 => unknown_4(rest),
        80 => change_ally_options(rest),
        81 => transfer_resources(rest),
        96 => map_trigger_chat(rest),
        97 => Ok((rest, Action::EscapedPressed)),
        98 => scenario_trigger(rest),
        101 => Ok((rest, Action::ChooseHeroSkillSubmenu)),
        102 => Ok((rest, Action::ChooseHeroSkillSubmenu)),
        103 => Ok((rest, Action::EnterBuildingSubmenu)),
        104 => minimap_signal(rest),
        105 => continue_game(rest),
        106 => continue_game(rest),
        107 => w3mmd(rest),
        117 => unknown_1(rest),
        119 => unknown_13(rest),
        120 => unknown_20(rest),
        123 => data_action(rest),
        _ => unknown(rest),
    }
    // TODO: get missing from there: https://gist.github.com/ForNeVeR/48dfcf05626abb70b35b8646dd0d6e92
}

fn parse_position(input: &[u8]) -> IResult<&[u8], Position> {
    let (rest, x) = le_f32(input)?;
    let (rest, y) = le_f32(rest)?;
    Ok((rest, Position { x, y }))
}

fn unknown_bytes(input: &[u8], len: usize) -> IResult<&[u8], Action> {
    let (rest, _) = take(len)(input)?;
    Ok((rest, Action::Unknown))
}

fn set_speed_game(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, speed) = le_u8(input)?;
    Ok((rest, Action::SetSpeed(speed)))
}

fn save_game(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, name) = zero_terminated_string(input)?;
    Ok((rest, Action::Save(name)))
}

fn save_game_finished(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, _) = le_u16(input)?;
    Ok((rest, Action::SaveFinished))
}

fn unit_building_ability_no_params(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, ability) = le_u16(input)?;
    let (rest, item) = item_or_unit(rest)?;
    let (rest, _) = le_u32(rest)?;
    let (rest, _) = le_u32(rest)?;
    Ok((
        rest,
        Action::UnitBuildingAbilityNoParams(UnitBuildingAbilityActionNoParams { ability, item }),
    ))
}

fn unit_building_ability_target_position(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, ability) = le_u16(input)?;
    let (rest, item) = item_or_unit(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position) = parse_position(rest)?;
    Ok((
        rest,
        Action::UnitBuildingAbilityTargetPosition(UnitBuildingAbilityActionTargetPosition {
            ability,
            item,
            target_position,
        }),
    ))
}

fn unit_building_ability_target_position_target_object_id(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, ability) = le_u16(input)?;
    let (rest, item) = item_or_unit(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position) = parse_position(rest)?;
    let (rest, object_1) = le_u32(rest)?;
    let (rest, object_2) = le_u32(rest)?;
    Ok((
        rest,
        Action::UnitBuildingAbilityTargetPositionTargetObjectId(
            UnitBuildingAbilityActionTargetPositionTargetObjectId {
                ability,
                item,
                target_position,
                object_1,
                object_2,
            },
        ),
    ))
}

fn give_item(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, ability) = le_u16(input)?;
    let (rest, item) = item_or_unit(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position) = parse_position(rest)?;
    let (rest, object_1) = le_u32(rest)?;
    let (rest, object_2) = le_u32(rest)?;
    let (rest, item_object_1) = le_u32(rest)?;
    let (rest, item_object_2) = le_u32(rest)?;
    Ok((
        rest,
        Action::GiveItem(GiveItemToUnitAction {
            ability,
            item,
            target_position,
            object_1,
            object_2,
            item_object_1,
            item_object_2,
        }),
    ))
}

fn unit_building_ability_two_target_positions(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, ability) = le_u16(input)?;
    let (rest, item_1) = item_or_unit(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position_1) = parse_position(rest)?;
    let (rest, item_2) = item_or_unit(rest)?;
    let (rest, _) = take(9usize)(rest)?; // TODO?
    let (rest, target_position_2) = parse_position(rest)?;
    Ok((
        rest,
        Action::UnitBuildingAbilityTwoTargetPositions(
            UnitBuildingAbilityActionTwoTargetPositions {
                ability,
                item_1,
                target_position_1,
                item_2,
                target_position_2,
            },
        ),
    ))
}

fn unit_selection(input: &[u8]) -> IResult<&[u8], UnitSelection> {
    let (rest, object_1) = le_u32(input)?;
    let (rest, object_2) = le_u32(rest)?;
    Ok((rest, UnitSelection { object_1, object_2 }))
}

fn change_selection(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, select_mode) = parse_selection_mode(input)?;
    let (rest, nb_selection) = le_u16(rest)?;
    let (rest, selected_units) = count(unit_selection, nb_selection as usize)(rest)?;
    Ok((
        rest,
        Action::ChangeSelection(ChangeSelectionAction {
            select_mode,
            selected_units,
        }),
    ))
}

fn assign_group_hotkey(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, hotkey) = le_u8(input)?;
    let (rest, nb_selection) = le_u16(rest)?;
    let (rest, selected_units) = count(unit_selection, nb_selection as usize)(rest)?;
    Ok((
        rest,
        Action::AssignGroupHotkey(AssignGroupHotkeyAction {
            hotkey,
            selected_units,
        }),
    ))
}

fn select_group_hotkey(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, hotkey) = le_u8(input)?;
    let (rest, _) = le_u8(rest)?; // TODO?
    Ok((rest, Action::SelectGroupHotkey(hotkey)))
}

fn select_subgroup(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, item) = item_or_unit(input)?;
    let (rest, object_1) = le_u32(rest)?;
    let (rest, object_2) = le_u32(rest)?;
    Ok((
        rest,
        Action::SelectSubgroup(SelectSubgroupAction {
            item,
            object_1,
            object_2,
        }),
    ))
}

fn select_ground_item(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, _) = take(1usize)(input)?;
    let (rest, object_1) = item_or_unit(rest)?;
    let (rest, object_2) = item_or_unit(rest)?;
    Ok((
        rest,
        Action::SelectGroundItem(SelectGroundItemAction { object_1, object_2 }),
    ))
}

fn cancel_hero_revival(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, unit_1) = item_or_unit(input)?;
    let (rest, unit_2) = item_or_unit(rest)?;
    Ok((
        rest,
        Action::CancelHeroRevival(CancelHeroRevivalAction { unit_1, unit_2 }),
    ))
}

fn remove_unit_from_building_queue(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, slot) = le_u8(input)?;
    let (rest, unit) = item_or_unit(rest)?;
    Ok((
        rest,
        Action::RemoveUnitFromBuildingQueue(RemoveUnitFromBuildingQueueAction { slot, unit }),
    ))
}

fn change_ally_options(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, player_slot) = le_u8(input)?;
    let (rest, option) = take(4usize)(rest)?;
    Ok((
        rest,
        Action::ChangeAllyOptions(ChangeAllyOptionsAction {
            player_slot,
            option: option[0..4].try_into().unwrap(),
        }),
    ))
}

fn transfer_resources(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, player_slot) = le_u8(input)?;
    let (rest, gold_amount) = take(4usize)(rest)?;
    let (rest, lumber_amount) = take(4usize)(rest)?;
    Ok((
        rest,
        Action::TransferResources(TransferResourcesAction {
            player_slot,
            gold_amount: gold_amount[0..4].try_into().unwrap(),
            lumber_amount: lumber_amount[0..4].try_into().unwrap(),
        }),
    ))
}

fn map_trigger_chat(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, _) = take(4usize)(input)?;
    let (rest, _) = take(4usize)(rest)?;
    let (rest, msg) = zero_terminated_string(rest)?;
    Ok((rest, Action::MapTriggerChat(msg)))
}

fn scenario_trigger(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, _) = take(12usize)(input)?;
    Ok((rest, Action::ScenarioTrigger))
}

fn minimap_signal(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, location) = parse_position(input)?;
    let (rest, _) = take(4usize)(rest)?;
    Ok((rest, Action::MinimapSignal(location)))
}

fn continue_game(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, _) = take(16usize)(input)?;
    Ok((rest, Action::ContinueGame))
}

fn w3mmd(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, filename) = zero_terminated_string(input)?;
    let (rest, mission_key) = zero_terminated_string(rest)?;
    let (rest, key) = zero_terminated_string(rest)?;
    let (rest, value) = le_u32(rest)?;
    Ok((
        rest,
        Action::W3MMD(W3MMDAction {
            filename,
            mission_key,
            key,
            value,
        }),
    ))
}

fn data_action(input: &[u8]) -> IResult<&[u8], Action> {
    let (rest, data) = take(16usize)(input)?;
    Ok((rest, Action::Data(data[0..16].try_into().unwrap())))
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
