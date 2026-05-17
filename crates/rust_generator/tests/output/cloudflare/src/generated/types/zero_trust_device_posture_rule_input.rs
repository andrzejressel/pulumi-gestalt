#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustDevicePostureRuleInput {
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
    pub r#locations: Option<Vec<super::types::ZeroTrustDevicePostureRuleInputLocation>>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustDevicePostureRuleInput {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "active_threats".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_threats,
                )
                .await,
            );
            map.insert(
                "certificate_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_id,
                )
                .await,
            );
            map.insert(
                "check_disks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#check_disks,
                )
                .await,
            );
            map.insert(
                "check_private_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#check_private_key,
                )
                .await,
            );
            map.insert(
                "cn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cn,
                )
                .await,
            );
            map.insert(
                "compliance_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compliance_status,
                )
                .await,
            );
            map.insert(
                "connection_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_id,
                )
                .await,
            );
            map.insert(
                "count_operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count_operator,
                )
                .await,
            );
            map.insert(
                "domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain,
                )
                .await,
            );
            map.insert(
                "eid_last_seen".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eid_last_seen,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "exists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exists,
                )
                .await,
            );
            map.insert(
                "extended_key_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extended_key_usages,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "infected".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#infected,
                )
                .await,
            );
            map.insert(
                "is_active".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_active,
                )
                .await,
            );
            map.insert(
                "issue_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issue_count,
                )
                .await,
            );
            map.insert(
                "last_seen".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_seen,
                )
                .await,
            );
            map.insert(
                "locations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#locations,
                )
                .await,
            );
            map.insert(
                "network_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_status,
                )
                .await,
            );
            map.insert(
                "operational_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operational_state,
                )
                .await,
            );
            map.insert(
                "operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operator,
                )
                .await,
            );
            map.insert(
                "os".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os,
                )
                .await,
            );
            map.insert(
                "os_distro_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_distro_name,
                )
                .await,
            );
            map.insert(
                "os_distro_revision".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_distro_revision,
                )
                .await,
            );
            map.insert(
                "os_version_extra".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_version_extra,
                )
                .await,
            );
            map.insert(
                "overall".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#overall,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "require_all".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_all,
                )
                .await,
            );
            map.insert(
                "risk_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#risk_level,
                )
                .await,
            );
            map.insert(
                "running".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#running,
                )
                .await,
            );
            map.insert(
                "score".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#score,
                )
                .await,
            );
            map.insert(
                "sensor_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sensor_config,
                )
                .await,
            );
            map.insert(
                "sha_256".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sha_256,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "thumbprint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#thumbprint,
                )
                .await,
            );
            map.insert(
                "total_score".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#total_score,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
                )
                .await,
            );
            map.insert(
                "version_operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version_operator,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustDevicePostureRuleInput {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#active_threats: {
                        let field_value = match fields_map.get("active_threats") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_threats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_id: {
                        let field_value = match fields_map.get("certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#check_disks: {
                        let field_value = match fields_map.get("check_disks") {
                            Some(value) => value,
                            None => bail!("Missing field 'check_disks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#check_private_key: {
                        let field_value = match fields_map.get("check_private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'check_private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cn: {
                        let field_value = match fields_map.get("cn") {
                            Some(value) => value,
                            None => bail!("Missing field 'cn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_status: {
                        let field_value = match fields_map.get("compliance_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_id: {
                        let field_value = match fields_map.get("connection_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#count_operator: {
                        let field_value = match fields_map.get("count_operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'count_operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain: {
                        let field_value = match fields_map.get("domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eid_last_seen: {
                        let field_value = match fields_map.get("eid_last_seen") {
                            Some(value) => value,
                            None => bail!("Missing field 'eid_last_seen' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exists: {
                        let field_value = match fields_map.get("exists") {
                            Some(value) => value,
                            None => bail!("Missing field 'exists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extended_key_usages: {
                        let field_value = match fields_map.get("extended_key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'extended_key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#infected: {
                        let field_value = match fields_map.get("infected") {
                            Some(value) => value,
                            None => bail!("Missing field 'infected' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_active: {
                        let field_value = match fields_map.get("is_active") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_active' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issue_count: {
                        let field_value = match fields_map.get("issue_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'issue_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_seen: {
                        let field_value = match fields_map.get("last_seen") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_seen' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#locations: {
                        let field_value = match fields_map.get("locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_status: {
                        let field_value = match fields_map.get("network_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operational_state: {
                        let field_value = match fields_map.get("operational_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'operational_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os: {
                        let field_value = match fields_map.get("os") {
                            Some(value) => value,
                            None => bail!("Missing field 'os' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_distro_name: {
                        let field_value = match fields_map.get("os_distro_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_distro_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_distro_revision: {
                        let field_value = match fields_map.get("os_distro_revision") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_distro_revision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_version_extra: {
                        let field_value = match fields_map.get("os_version_extra") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_version_extra' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overall: {
                        let field_value = match fields_map.get("overall") {
                            Some(value) => value,
                            None => bail!("Missing field 'overall' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_all: {
                        let field_value = match fields_map.get("require_all") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_all' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#risk_level: {
                        let field_value = match fields_map.get("risk_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'risk_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#running: {
                        let field_value = match fields_map.get("running") {
                            Some(value) => value,
                            None => bail!("Missing field 'running' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#score: {
                        let field_value = match fields_map.get("score") {
                            Some(value) => value,
                            None => bail!("Missing field 'score' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sensor_config: {
                        let field_value = match fields_map.get("sensor_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'sensor_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sha_256: {
                        let field_value = match fields_map.get("sha_256") {
                            Some(value) => value,
                            None => bail!("Missing field 'sha_256' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thumbprint: {
                        let field_value = match fields_map.get("thumbprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'thumbprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_score: {
                        let field_value = match fields_map.get("total_score") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_score' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_operator: {
                        let field_value = match fields_map.get("version_operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
