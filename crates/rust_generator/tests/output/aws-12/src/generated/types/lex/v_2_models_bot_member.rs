#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsBotMember {
    /// (Required) - Alias ID of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "aliasId")]
    pub r#alias_id: String,
    /// (Required) - Alias name of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "aliasName")]
    pub r#alias_name: String,
    /// (Required) - Unique ID of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Name of the bot. The bot name must be unique in the account that creates the bot. Type String. Length Constraints: Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// (Required) - Version of a bot that is a member of this network of bots.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
