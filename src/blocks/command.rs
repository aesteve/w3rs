use crate::blocks::action::UnitCommand;
use crate::building::{Building, Upgrade};
use crate::environment::Environment;
use crate::item::Item;
use crate::spell::{HeroSpell, Spell, UnitSpell};
use crate::unit::{Hero, Unit};
use crate::utils::zero_terminated_string;
use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::lib::std::fmt::Formatter;
use nom::multi::{count, many0};
use nom::{
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    IResult,
};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct CommandData {
    pub player: u8,
    length: u16,
    pub(crate) actions: Vec<ParsedAction>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParsedAction {
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

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct SelectedComponent {
    pub(crate) id_1: u32,
    pub(crate) id_2: u32,
    pub(crate) kind: Option<GameComponent>,
}

impl ParsedAction {
    #[allow(dead_code)]
    pub(crate) fn selection(
        &self,
        hotkey_selections: &mut HashMap<u8, Vec<SelectedComponent>>,
    ) -> Option<Vec<SelectedComponent>> {
        match self {
            ParsedAction::SelectSubgroup(action) => Some(
                [SelectedComponent {
                    id_1: action.object_1,
                    id_2: action.object_2,
                    kind: Some(action.item.clone()),
                }]
                .to_vec(),
            ),
            ParsedAction::ChangeSelection(action) => {
                if action.select_mode == SelectionMode::Remove {
                    Some(Vec::new())
                } else {
                    Some(units_as_selection(&action.selected_units))
                }
            }
            ParsedAction::SelectGroupHotkey(hotkey) => {
                hotkey_selections.get(hotkey).map(|sel| sel.to_vec())
            }
            ParsedAction::AssignGroupHotkey(assign_hotkey) => {
                hotkey_selections.insert(
                    assign_hotkey.hotkey,
                    units_as_selection(&assign_hotkey.selected_units),
                );
                hotkey_selections
                    .get(&assign_hotkey.hotkey)
                    .map(|sel| sel.to_vec())
            }
            _ => None,
        }
    }

    fn discard(&self) -> bool {
        matches!(self, ParsedAction::W3MMD(_)
            | ParsedAction::ContinueGame
            | ParsedAction::EscapedPressed
            | ParsedAction::ScenarioTrigger
            | ParsedAction::MapTriggerChat(_)
            | ParsedAction::SaveFinished
            | ParsedAction::PreSubselection
            | ParsedAction::Unknown)
    }

    #[allow(dead_code)]
    pub(crate) fn should_display(&self) -> bool {
        !self.discard()
            && !matches!(self, ParsedAction::ChangeSelection(_) | ParsedAction::SelectSubgroup(_))
    }
}

fn units_as_selection(units: &[UnitSelection]) -> Vec<SelectedComponent> {
    units
        .iter()
        .map(|sel| SelectedComponent {
            id_1: sel.object_1,
            id_2: sel.object_2,
            kind: None,
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameComponent {
    // FIXME: replace by a public struct
    Unit(Unit),
    Hero(Hero),
    Building(Building),
    Upgrade(Upgrade),
    TrainedSpell(HeroSpell),
    UsedSpell(Spell),
    Item(Item),
    Action(UnitCommand),
    Environment(Environment),
    UnknownStr(String),
    UnknownBin([u8; 2]),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},y={}}}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    /*
    Train,
    Buy,
    TrainHero,
    Order,
    Summon,
     */
    /// TODO: Understand this, this must be something like "do", "buy", "queue", "use item", etc.
    /// => try to understand by pattern matching replays, the doc isn't crystal clear (or it has changed)
    Unknown(u16),
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionNoParams {
    command: Command,
    pub(crate) item: GameComponent,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTargetPosition {
    pub(crate) command: Command,
    pub(crate) item: GameComponent,
    pub(crate) target_position: Position,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTargetPositionTargetObjectId {
    command: Command,
    pub(crate) item: GameComponent,
    pub(crate) target_position: Position,
    pub(crate) object_1: u32,
    pub(crate) object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct GiveItemToUnitAction {
    command: Command,
    pub(crate) item: GameComponent,
    pub(crate) target_position: Position,
    pub(crate) object_1: u32,
    pub(crate) object_2: u32,
    pub(crate) item_object_1: u32,
    pub(crate) item_object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct UnitBuildingAbilityActionTwoTargetPositions {
    command: Command,
    item_1: GameComponent,
    target_position_1: Position,
    pub(crate) item_2: GameComponent,
    pub(crate) target_position_2: Position,
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
    pub(crate) item: GameComponent,
    object_1: u32,
    object_2: u32,
}

#[derive(Debug, PartialEq)]
pub struct SelectGroundItemAction {
    object_1: GameComponent,
    object_2: GameComponent,
}

#[derive(Debug, PartialEq)]
pub struct CancelHeroRevivalAction {
    unit_1: GameComponent,
    unit_2: GameComponent,
}

#[derive(Debug, PartialEq)]
pub struct RemoveUnitFromBuildingQueueAction {
    slot: u8,
    unit: GameComponent,
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

fn parse_ability(input: &[u8]) -> IResult<&[u8], Command> {
    let (rest, command) = le_u16(input)?;
    Ok((rest, Command::Unknown(command)))
}

fn parse_selection_mode(input: &[u8]) -> IResult<&[u8], SelectionMode> {
    let (rest, selection) = le_u8(input)?;
    match selection {
        1 => Ok((rest, SelectionMode::Add)),
        _ => Ok((rest, SelectionMode::Remove)),
    }
}

fn parse_actions(input: &[u8]) -> IResult<&[u8], Vec<ParsedAction>> {
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

fn str_component(input: &str) -> GameComponent {
    Unit::from_str(input)
        .map(GameComponent::Unit) // most often first
        .or_else(|| Hero::from_str(input).map(GameComponent::Hero))
        .or_else(|| Building::from_str(input).map(GameComponent::Building))
        .or_else(|| HeroSpell::from_str(input).map(GameComponent::TrainedSpell))
        .or_else(|| Item::from_str(input).map(GameComponent::Item))
        .or_else(|| Upgrade::from_str(input).map(GameComponent::Upgrade))
        .or_else(|| Environment::from_str(input).map(GameComponent::Environment))
        .unwrap_or_else(|| GameComponent::UnknownStr(input.to_string()))
}

fn binary_component(input: [u8; 2]) -> GameComponent {
    HeroSpell::from_bin(input)
        .map(Spell::Hero)
        .or_else(|| UnitSpell::from_bin(input).map(Spell::Unit))
        .map(GameComponent::UsedSpell)
        .or_else(|| UnitCommand::from_bin(input).map(GameComponent::Action))
        .unwrap_or_else(|| GameComponent::UnknownBin(input))
}

fn game_component(input: &[u8]) -> IResult<&[u8], GameComponent> {
    let (rest, bytes) = take(4usize)(input)?;
    match bytes {
        [_, _, 13, 0] => Ok((rest, binary_component(bytes[0..2].try_into().unwrap()))),
        _ => {
            let unit = str_component(
                String::from_utf8_lossy(bytes)
                    .to_string()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .as_str(),
            );
            Ok((rest, unit))
        }
    }
}

pub(crate) fn parse_action(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, kind) = le_u8(input)?;
    match kind {
        1 => Ok((rest, ParsedAction::Pause)),
        2 => Ok((rest, ParsedAction::Resume)),
        3 => set_speed_game(rest),
        4 => Ok((rest, ParsedAction::IncreaseGameSpeed)),
        5 => Ok((rest, ParsedAction::DecreaseGameSpeed)),
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
        26 => Ok((rest, ParsedAction::PreSubselection)),
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
        97 => Ok((rest, ParsedAction::EscapedPressed)),
        98 => scenario_trigger(rest),
        101 => Ok((rest, ParsedAction::ChooseHeroSkillSubmenu)),
        102 => Ok((rest, ParsedAction::ChooseHeroSkillSubmenu)),
        103 => Ok((rest, ParsedAction::EnterBuildingSubmenu)),
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

fn unknown_bytes(input: &[u8], len: usize) -> IResult<&[u8], ParsedAction> {
    let (rest, _) = take(len)(input)?;
    Ok((rest, ParsedAction::Unknown))
}

fn set_speed_game(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, speed) = le_u8(input)?;
    Ok((rest, ParsedAction::SetSpeed(speed)))
}

fn save_game(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, name) = zero_terminated_string(input)?;
    Ok((rest, ParsedAction::Save(name)))
}

fn save_game_finished(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, _) = le_u16(input)?;
    Ok((rest, ParsedAction::SaveFinished))
}

fn unit_building_ability_no_params(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, ability) = parse_ability(input)?;
    let (rest, item) = game_component(rest)?;
    let (rest, _) = le_u32(rest)?;
    let (rest, _) = le_u32(rest)?;
    Ok((
        rest,
        ParsedAction::UnitBuildingAbilityNoParams(UnitBuildingAbilityActionNoParams {
            command: ability,
            item,
        }),
    ))
}

fn unit_building_ability_target_position(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, ability) = parse_ability(input)?;
    let (rest, item) = game_component(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position) = parse_position(rest)?;
    Ok((
        rest,
        ParsedAction::UnitBuildingAbilityTargetPosition(UnitBuildingAbilityActionTargetPosition {
            command: ability,
            item,
            target_position,
        }),
    ))
}

fn unit_building_ability_target_position_target_object_id(
    input: &[u8],
) -> IResult<&[u8], ParsedAction> {
    let (rest, ability) = parse_ability(input)?;
    let (rest, item) = game_component(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position) = parse_position(rest)?;
    let (rest, object_1) = le_u32(rest)?;
    let (rest, object_2) = le_u32(rest)?;
    Ok((
        rest,
        ParsedAction::UnitBuildingAbilityTargetPositionTargetObjectId(
            UnitBuildingAbilityActionTargetPositionTargetObjectId {
                command: ability,
                item,
                target_position,
                object_1,
                object_2,
            },
        ),
    ))
}

fn give_item(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, ability) = parse_ability(input)?;
    let (rest, item) = game_component(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position) = parse_position(rest)?;
    let (rest, object_1) = le_u32(rest)?;
    let (rest, object_2) = le_u32(rest)?;
    let (rest, item_object_1) = le_u32(rest)?;
    let (rest, item_object_2) = le_u32(rest)?;
    Ok((
        rest,
        ParsedAction::GiveItem(GiveItemToUnitAction {
            command: ability,
            item,
            target_position,
            object_1,
            object_2,
            item_object_1,
            item_object_2,
        }),
    ))
}

fn unit_building_ability_two_target_positions(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, ability) = parse_ability(input)?;
    let (rest, item_1) = game_component(rest)?;
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, _) = le_u32(rest)?; // TODO
    let (rest, target_position_1) = parse_position(rest)?;
    let (rest, item_2) = game_component(rest)?;
    let (rest, _) = take(9usize)(rest)?; // TODO?
    let (rest, target_position_2) = parse_position(rest)?;
    Ok((
        rest,
        ParsedAction::UnitBuildingAbilityTwoTargetPositions(
            UnitBuildingAbilityActionTwoTargetPositions {
                command: ability,
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

fn change_selection(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, select_mode) = parse_selection_mode(input)?;
    let (rest, nb_selection) = le_u16(rest)?;
    let (rest, selected_units) = count(unit_selection, nb_selection as usize)(rest)?;
    Ok((
        rest,
        ParsedAction::ChangeSelection(ChangeSelectionAction {
            select_mode,
            selected_units,
        }),
    ))
}

fn assign_group_hotkey(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, hotkey) = le_u8(input)?;
    let (rest, nb_selection) = le_u16(rest)?;
    let (rest, selected_units) = count(unit_selection, nb_selection as usize)(rest)?;
    Ok((
        rest,
        ParsedAction::AssignGroupHotkey(AssignGroupHotkeyAction {
            hotkey,
            selected_units,
        }),
    ))
}

fn select_group_hotkey(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, hotkey) = le_u8(input)?;
    let (rest, _) = le_u8(rest)?; // TODO?
    Ok((rest, ParsedAction::SelectGroupHotkey(hotkey)))
}

fn select_subgroup(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, item) = game_component(input)?;
    let (rest, object_1) = le_u32(rest)?;
    let (rest, object_2) = le_u32(rest)?;
    Ok((
        rest,
        ParsedAction::SelectSubgroup(SelectSubgroupAction {
            item,
            object_1,
            object_2,
        }),
    ))
}

fn select_ground_item(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, _) = take(1usize)(input)?;
    let (rest, object_1) = game_component(rest)?;
    let (rest, object_2) = game_component(rest)?;
    Ok((
        rest,
        ParsedAction::SelectGroundItem(SelectGroundItemAction { object_1, object_2 }),
    ))
}

fn cancel_hero_revival(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, unit_1) = game_component(input)?;
    let (rest, unit_2) = game_component(rest)?;
    Ok((
        rest,
        ParsedAction::CancelHeroRevival(CancelHeroRevivalAction { unit_1, unit_2 }),
    ))
}

fn remove_unit_from_building_queue(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, slot) = le_u8(input)?;
    let (rest, unit) = game_component(rest)?;
    Ok((
        rest,
        ParsedAction::RemoveUnitFromBuildingQueue(RemoveUnitFromBuildingQueueAction { slot, unit }),
    ))
}

fn change_ally_options(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, player_slot) = le_u8(input)?;
    let (rest, option) = take(4usize)(rest)?;
    Ok((
        rest,
        ParsedAction::ChangeAllyOptions(ChangeAllyOptionsAction {
            player_slot,
            option: option[0..4].try_into().unwrap(),
        }),
    ))
}

fn transfer_resources(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, player_slot) = le_u8(input)?;
    let (rest, gold_amount) = take(4usize)(rest)?;
    let (rest, lumber_amount) = take(4usize)(rest)?;
    Ok((
        rest,
        ParsedAction::TransferResources(TransferResourcesAction {
            player_slot,
            gold_amount: gold_amount[0..4].try_into().unwrap(),
            lumber_amount: lumber_amount[0..4].try_into().unwrap(),
        }),
    ))
}

fn map_trigger_chat(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, _) = take(4usize)(input)?;
    let (rest, _) = take(4usize)(rest)?;
    let (rest, msg) = zero_terminated_string(rest)?;
    Ok((rest, ParsedAction::MapTriggerChat(msg)))
}

fn scenario_trigger(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, _) = take(12usize)(input)?;
    Ok((rest, ParsedAction::ScenarioTrigger))
}

fn minimap_signal(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, location) = parse_position(input)?;
    let (rest, _) = take(4usize)(rest)?;
    Ok((rest, ParsedAction::MinimapSignal(location)))
}

fn continue_game(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, _) = take(16usize)(input)?;
    Ok((rest, ParsedAction::ContinueGame))
}

fn w3mmd(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, filename) = zero_terminated_string(input)?;
    let (rest, mission_key) = zero_terminated_string(rest)?;
    let (rest, key) = zero_terminated_string(rest)?;
    let (rest, value) = le_u32(rest)?;
    Ok((
        rest,
        ParsedAction::W3MMD(W3MMDAction {
            filename,
            mission_key,
            key,
            value,
        }),
    ))
}

fn data_action(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    let (rest, data) = take(16usize)(input)?;
    Ok((rest, ParsedAction::Data(data[0..16].try_into().unwrap())))
}

fn unknown(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    Ok((input, ParsedAction::Unknown))
}

fn unknown_1(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    unknown_bytes(input, 1)
}

fn unknown_4(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    unknown_bytes(input, 4)
}

fn unknown_5(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    unknown_bytes(input, 5)
}

fn unknown_8(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    unknown_bytes(input, 8)
}

fn unknown_9(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    unknown_bytes(input, 9)
}

fn unknown_13(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    unknown_bytes(input, 13)
}

fn unknown_20(input: &[u8]) -> IResult<&[u8], ParsedAction> {
    unknown_bytes(input, 13)
}
