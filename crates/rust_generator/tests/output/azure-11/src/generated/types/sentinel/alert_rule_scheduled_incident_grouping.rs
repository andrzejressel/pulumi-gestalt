#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertRuleScheduledIncidentGrouping {
    /// A list of alert details to group by, only when the `entity_matching_method` is `Selected`. Possible values are `DisplayName` and `Severity`.
    #[builder(into)]
    #[serde(rename = "byAlertDetails")]
    pub r#by_alert_details: Option<Vec<String>>,
    /// A list of custom details keys to group by, only when the `entity_matching_method` is `Selected`. Only keys defined in the `custom_details` may be used.
    #[builder(into)]
    #[serde(rename = "byCustomDetails")]
    pub r#by_custom_details: Option<Vec<String>>,
    /// A list of entity types to group by, only when the `entity_matching_method` is `Selected`. Possible values are `Account`, `AzureResource`, `CloudApplication`, `DNS`, `File`, `FileHash`, `Host`, `IP`, `Mailbox`, `MailCluster`, `MailMessage`, `Malware`, `Process`, `RegistryKey`, `RegistryValue`, `SecurityGroup`, `SubmissionMail`, `URL`.
    #[builder(into)]
    #[serde(rename = "byEntities")]
    pub r#by_entities: Option<Vec<String>>,
    /// Enable grouping incidents created from alerts triggered by this Sentinel Scheduled Alert Rule. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The method used to group incidents. Possible values are `AnyAlert`, `Selected` and `AllEntities`. Defaults to `AnyAlert`.
    #[builder(into)]
    #[serde(rename = "entityMatchingMethod")]
    pub r#entity_matching_method: Option<String>,
    /// Limit the group to alerts created within the lookback duration (in ISO 8601 duration format). Defaults to `PT5M`.
    #[builder(into)]
    #[serde(rename = "lookbackDuration")]
    pub r#lookback_duration: Option<String>,
    /// Whether to re-open closed matching incidents? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "reopenClosedIncidents")]
    pub r#reopen_closed_incidents: Option<bool>,
}
