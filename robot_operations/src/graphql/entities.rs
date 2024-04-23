use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use models::{
    robot_action,
    sea_orm_active_enums::{ActionType, Status},
};

/// Combines autoproc integration, autoproc program, autoproc and autoproc scaling
#[derive(Debug, Clone, SimpleObject)]
#[graphql(name = "RobotAction", unresolvable)]
pub struct RobotAction {
    /// An opaque unique identifier for the robot actions
    pub robot_action_id: u32,
    /// An opaque unique identifier for the data collection
    #[graphql(skip)]
    pub blsession_id: u32,
    /// An opaque unique identifier for the auto processing program
    pub blsample_id: Option<u32>,
    /// Refined X position of the beam
    pub action_type: Option<RobotActionType>,
    /// Refined Y position of the beam
    pub start_timestamp: DateTime<Utc>,
    /// Name of the processing programs
    pub end_timestamp: DateTime<Utc>,
    /// Processing program status
    pub status: Option<RobotActionStatus>,
    /// Processing program message
    pub message: Option<String>,
    /// An opaque unique identifier for the  processing processing job
    pub container_location: Option<i16>,
    /// An opaque unique identifier for the auto processing
    pub dewar_location: Option<i16>,
    /// Space group of the processing job
    pub sample_barcode: Option<String>,
    /// Refined cell a in the auto processing job
    pub xtal_snapshot_before: Option<String>,
    /// Refined cell b in the auto processing job
    pub xtal_snapshot_after: Option<String>,
}

#[derive(Enum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum RobotActionType {
    Load,
    Unload,
    Dispose,
    Store,
    Wash,
    Anneal,
    Mosaic,
}

#[derive(Enum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum RobotActionStatus {
    Success,
    Error,
    Critical,
    Warning,
    Epicsfail,
    Commandnotsent,
}

impl From<Option<ActionType>> for RobotActionType {
    fn from(value: Option<ActionType>) -> Self {
        match value {
            Some(ActionType::Load) => RobotActionType::Load,
            Some(ActionType::Unload) => RobotActionType::Unload,
            Some(ActionType::Dispose) => RobotActionType::Dispose,
            Some(ActionType::Store) => RobotActionType::Store,
            Some(ActionType::Wash) => RobotActionType::Wash,
            Some(ActionType::Anneal) => RobotActionType::Anneal,
            Some(ActionType::Mosaic) => RobotActionType::Mosaic,
            None => None.into(),
        }
    }
}

impl From<Option<Status>> for RobotActionStatus {
    fn from(value: Option<Status>) -> Self {
        match value {
            Some(Status::Success) => RobotActionStatus::Success,
            Some(Status::Error) => RobotActionStatus::Error,
            Some(Status::Critical) => RobotActionStatus::Critical,
            Some(Status::Warning) => RobotActionStatus::Warning,
            Some(Status::Epicsfail) => RobotActionStatus::Epicsfail,
            Some(Status::Commandnotsent) => RobotActionStatus::Commandnotsent,
            None => None.into(),
        }
    }
}

impl From<robot_action::Model> for RobotAction {
    fn from(value: robot_action::Model) -> Self {
        Self {
            robot_action_id: value.robot_action_id,
            blsession_id: value.blsession_id,
            blsample_id: value.blsample_id,
            action_type: Some(RobotActionType::from(value.action_type)),
            start_timestamp: value.start_timestamp,
            end_timestamp: value.end_timestamp,
            status: Some(RobotActionStatus::from(value.status)),
            message: value.message,
            container_location: value.container_location,
            dewar_location: value.dewar_location,
            sample_barcode: value.sample_barcode,
            xtal_snapshot_after: value.xtal_snapshot_after,
            xtal_snapshot_before: value.xtal_snapshot_before,
        }
    }
}

/// Sessions subgraph extension
#[derive(SimpleObject)]
#[graphql(name = "Session", complex)]
pub struct Session {
    /// An opaque unique identifier for the sessions
    pub id: i32,
}
