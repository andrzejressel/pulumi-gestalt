#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DevicePostureRuleInput {
    /// The number of active threats from SentinelOne.
    #[builder(into)]
    #[serde(rename = "activeThreats")]
    pub r#active_threats: Option<i32>,
    /// The UUID of a Cloudflare managed certificate.
    #[builder(into)]
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Option<String>,
    /// Specific volume(s) to check for encryption.
    #[builder(into)]
    #[serde(rename = "checkDisks")]
    pub r#check_disks: Option<Vec<String>>,
    /// Confirm the certificate was not imported from another device.
    #[builder(into)]
    #[serde(rename = "checkPrivateKey")]
    pub r#check_private_key: Option<bool>,
    /// The common name for a certificate.
    #[builder(into)]
    #[serde(rename = "cn")]
    pub r#cn: Option<String>,
    /// The workspace one or intune device compliance status. `compliant` and `noncompliant` are values supported by both providers. `unknown`, `conflict`, `error`, `ingraceperiod` values are only supported by intune. Available values: `compliant`, `noncompliant`, `unknown`, `conflict`, `error`, `ingraceperiod`.
    #[builder(into)]
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Option<String>,
    /// The workspace one or intune connection id.
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Option<String>,
    /// The count comparison operator for kolide. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into)]
    #[serde(rename = "countOperator")]
    pub r#count_operator: Option<String>,
    /// The domain that the client must join.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    /// The time a device last seen in Tanium. Must be in the format `1h` or `30m`. Valid units are `d`, `h` and `m`.
    #[builder(into)]
    #[serde(rename = "eidLastSeen")]
    pub r#eid_last_seen: Option<String>,
    /// True if the firewall must be enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Checks if the file should exist.
    #[builder(into)]
    #[serde(rename = "exists")]
    pub r#exists: Option<bool>,
    /// List of values indicating purposes for which the certificate public key can be used. Available values: `clientAuth`, `emailProtection`.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Option<Vec<String>>,
    /// The Teams List id. Required for `serial_number` and `unique_client_id` rule types.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// True if SentinelOne device is infected.
    #[builder(into)]
    #[serde(rename = "infected")]
    pub r#infected: Option<bool>,
    /// True if SentinelOne device is active.
    #[builder(into)]
    #[serde(rename = "isActive")]
    pub r#is_active: Option<bool>,
    /// The number of issues for kolide.
    #[builder(into)]
    #[serde(rename = "issueCount")]
    pub r#issue_count: Option<String>,
    /// The duration of time that the host was last seen from Crowdstrike. Must be in the format `1h` or `30m`. Valid units are `d`, `h` and `m`.
    #[builder(into)]
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Option<String>,
    /// List of operating system locations to check for a client certificate..
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<super::types::DevicePostureRuleInputLocation>>,
    /// The network status from SentinelOne. Available values: `connected`, `disconnected`, `disconnecting`, `connecting`.
    #[builder(into)]
    #[serde(rename = "networkStatus")]
    pub r#network_status: Option<String>,
    /// The current operational state of a SentinelOne Agent. Available values: `na`, `partially_disabled`, `auto_fully_disabled`, `fully_disabled`, `auto_partially_disabled`, `disabled_error`, `db_corruption`.
    #[builder(into)]
    #[serde(rename = "operationalState")]
    pub r#operational_state: Option<String>,
    /// The version comparison operator. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
    /// OS signal score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into)]
    #[serde(rename = "os")]
    pub r#os: Option<String>,
    /// The operating system excluding version information.
    #[builder(into)]
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Option<String>,
    /// The operating system version excluding OS name information or release name.
    #[builder(into)]
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Option<String>,
    /// Extra version value following the operating system semantic version.
    #[builder(into)]
    #[serde(rename = "osVersionExtra")]
    pub r#os_version_extra: Option<String>,
    /// Overall ZTA score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into)]
    #[serde(rename = "overall")]
    pub r#overall: Option<String>,
    /// The path to the file.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// True if all drives must be encrypted.
    #[builder(into)]
    #[serde(rename = "requireAll")]
    pub r#require_all: Option<bool>,
    /// The risk level from Tanium. Available values: `low`, `medium`, `high`, `critical`.
    #[builder(into)]
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Option<String>,
    /// Checks if the application should be running.
    #[builder(into)]
    #[serde(rename = "running")]
    pub r#running: Option<bool>,
    /// A value between 0-100 assigned to devices set by the 3rd party posture provider for custom device posture integrations.
    #[builder(into)]
    #[serde(rename = "score")]
    pub r#score: Option<i32>,
    /// Sensor signal score from Crowdstrike. Value must be between 1 and 100.
    #[builder(into)]
    #[serde(rename = "sensorConfig")]
    pub r#sensor_config: Option<String>,
    /// The sha256 hash of the file.
    #[builder(into)]
    #[serde(rename = "sha256")]
    pub r#sha_256: Option<String>,
    /// The host’s current online status from Crowdstrike. Available values: `online`, `offline`, `unknown`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// The thumbprint of the file certificate.
    #[builder(into)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Option<String>,
    /// The total score from Tanium.
    #[builder(into)]
    #[serde(rename = "totalScore")]
    pub r#total_score: Option<i32>,
    /// The operating system semantic version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// The version comparison operator for Crowdstrike. Available values: `>`, `>=`, `<`, `<=`, `==`.
    #[builder(into)]
    #[serde(rename = "versionOperator")]
    pub r#version_operator: Option<String>,
}
