
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct ServerMessage {
    pub request_id: Option<i32>,
    pub response: Option<String>,
    pub error_msg: Option<String>,
    pub token_refresh: Option<String>,
    pub meeting_update: Option<MeetingUpdate>,
}

impl std::fmt::Display for ServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServerMessage {{ request_id: {:?}, response: {:?}, error_msg: {:?}, token_refresh: {:?}, meeting_update: {:?} }}",
            self.request_id, self.response, self.error_msg, self.token_refresh, self.meeting_update
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct MeetingUpdate {
    pub meeting_permissions: Option<MeetingPermissions>,
    pub meeting_state: Option<MeetingState>,
}

impl std::fmt::Display for MeetingUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MeetingUpdate {{ meeting_permissions: {:?}, meeting_state: {:?} }}",
            self.meeting_permissions, self.meeting_state
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct MeetingPermissions {
    pub can_toggle_mute: bool,
    pub can_toggle_video: bool,
    pub can_toggle_hand: bool,
    pub can_toggle_blur: bool,
    pub can_leave: bool,
    pub can_react: bool,
    pub can_toggle_share_tray: bool,
    pub can_toggle_chat: bool,
    pub can_stop_sharing: bool,
    pub can_pair: bool,
}

impl MeetingPermissions {
    pub fn new() -> Self {
        Self {
            can_toggle_mute: false,
            can_toggle_video: false,
            can_toggle_hand: false,
            can_toggle_blur: false,
            can_leave: false,
            can_react: false,
            can_toggle_share_tray: false,
            can_toggle_chat: false,
            can_stop_sharing: false,
            can_pair: false,
        }
    }
}

impl Default for MeetingPermissions {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MeetingPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MeetingPermissions {{ can_toggle_mute: {}, can_toggle_video: {}, can_toggle_hand: {}, can_toggle_blur: {}, can_leave: {}, can_react: {}, can_toggle_share_tray: {}, can_toggle_chat: {}, can_stop_sharing: {}, can_pair: {} }}",
            self.can_toggle_mute, self.can_toggle_video, self.can_toggle_hand, self.can_toggle_blur, self.can_leave, self.can_react, self.can_toggle_share_tray, self.can_toggle_chat, self.can_stop_sharing, self.can_pair
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct MeetingState {
    pub is_muted: bool,
    pub is_hand_raised: bool,
    pub is_in_meeting: bool,
    pub is_recording_on: bool,
    pub is_background_blurred: bool,
    pub is_sharing: bool,
    pub has_unread_messages: bool,
    pub is_video_on: bool,
}

impl MeetingState {
    pub fn new() -> Self {
        Self {
            is_muted: false,
            is_hand_raised: false,
            is_in_meeting: false,
            is_recording_on: false,
            is_background_blurred: false,
            is_sharing: false,
            has_unread_messages: false,
            is_video_on: false,
        }
    }
}

impl Default for MeetingState {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MeetingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MeetingState {{ is_muted: {}, is_hand_raised: {}, is_in_meeting: {}, is_recording_on: {}, is_background_blurred: {}, is_sharing: {}, has_unread_messages: {}, is_video_on: {} }}",
            self.is_muted, self.is_hand_raised, self.is_in_meeting, self.is_recording_on, self.is_background_blurred, self.is_sharing, self.has_unread_messages, self.is_video_on
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct ClientMessageParameter {
    #[serde(rename = "type")]
    pub type_: ClientMessageParameterType,
}

impl ClientMessageParameter {
    pub fn new(type_: ClientMessageParameterType) -> Self {
        Self { type_ }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub enum ClientMessageParameterType {
    #[serde(rename = "applause")]
    ReactApplause = 0b0000_0111_0001_0000,
    #[serde(rename = "laugh")]
    ReactLaugh = 0b0000_0111_0001_0001,
    #[serde(rename = "like")]
    ReactLike = 0b0000_0111_0001_0010,
    #[serde(rename = "love")]
    ReactLove = 0b0000_0111_0001_0011,
    #[serde(rename = "wow")]
    ReactWow = 0b0000_0111_0001_0100,
    #[serde(rename = "chat")]
    ToggleUiChat = 0b0000_1001_0000_0001,
    #[serde(rename = "sharing-tray")]
    ToggleUiSharing = 0b0000_1001_0000_0010,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
#[serde(rename = "none")]
pub struct ClientMessage {
    pub action: MeetingAction,
    pub parameters: Option<ClientMessageParameter>,
    pub request_id: Option<i32>,
}

impl ClientMessage {
    pub fn new(action: MeetingAction, parameters: Option<ClientMessageParameter>) -> Self {
        Self { action, parameters, request_id: None }
    }
}

impl std::fmt::Display for ClientMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientMessage {{ action: {:?}, parameters: {:?}, request_id: {} }}",
            self.action, self.parameters, self.request_id.or(Some(0)).unwrap()
        )
    }
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
#[serde(rename = "none")]
pub enum MeetingAction {
    None = 0,
    #[serde(rename = "query-state")]
    QueryMeetingState = 0b0000_0001_0000_0000,
    #[serde(rename = "mute")]
    Mute = 0b0000_0010_0000_0000,
    #[serde(rename = "unmute")]
    Unmute = 0b0000_0010_0000_0001,
    #[serde(rename = "toggle-mute")]
    ToggleMute = 0b0000_0010_0000_0010,
    #[serde(rename = "hide-video")]
    HideVideo = 0b0000_0011_0000_0000,
    #[serde(rename = "show-video")]
    ShowVideo = 0b0000_0011_0000_0001,
    #[serde(rename = "toggle-video")]
    ToggleVideo = 0b0000_0011_0000_0010,
    #[serde(rename = "unblur-background")]
    UnblurBackground = 0b0000_0100_0000_0000,
    #[serde(rename = "blur-background")]
    BlurBackground = 0b0000_0100_0000_0001,
    #[serde(rename = "toggle-background-blur")]
    ToggleBlurBackground = 0b0000_0100_0000_0010,
    #[serde(rename = "lower-hand")]
    LowerHand = 0b0000_0101_0000_0000,
    #[serde(rename = "raise-hand")]
    RaiseHand = 0b0000_0101_0000_0001,
    #[serde(rename = "toggle-hand")]
    ToggleHand = 0b0000_0101_0000_0010,
    #[serde(rename = "leave-call")]
    LeaveCall = 0b0000_0111_0000_0000,
    #[serde(rename = "send-reaction")]
    React = 0b0000_1000_0000_0000,
    #[serde(rename = "toggle-ui")]
    ToggleUI = 0b0000_1001_0000_0000,
    #[serde(rename = "stop-sharing")]
    StopSharing = 0b0000_1010_0000_0000,
}
