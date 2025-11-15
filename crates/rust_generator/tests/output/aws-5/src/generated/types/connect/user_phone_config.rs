#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPhoneConfig {
    /// The After Call Work (ACW) timeout setting, in seconds. Minimum value of 0.
    #[builder(into)]
    #[serde(rename = "afterContactWorkTimeLimit")]
    pub r#after_contact_work_time_limit: Option<i32>,
    /// When Auto-Accept Call is enabled for an available agent, the agent connects to contacts automatically.
    #[builder(into)]
    #[serde(rename = "autoAccept")]
    pub r#auto_accept: Option<bool>,
    /// The phone number for the user's desk phone. Required if `phone_type` is set as `DESK_PHONE`.
    #[builder(into)]
    #[serde(rename = "deskPhoneNumber")]
    pub r#desk_phone_number: Option<String>,
    /// The phone type. Valid values are `DESK_PHONE` and `SOFT_PHONE`.
    #[builder(into)]
    #[serde(rename = "phoneType")]
    pub r#phone_type: String,
}
