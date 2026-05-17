#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OntapVolumeSnaplockConfiguration {
    /// Enables or disables the audit log volume for an FSx for ONTAP SnapLock volume. The default value is `false`.
    #[builder(into)]
    #[serde(rename = "auditLogVolume")]
    pub r#audit_log_volume: Option<bool>,
    /// The configuration object for setting the autocommit period of files in an FSx for ONTAP SnapLock volume. See `autocommit_period` Block for details.
    #[builder(into)]
    #[serde(rename = "autocommitPeriod")]
    pub r#autocommit_period: Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationAutocommitPeriod>>,
    /// Enables, disables, or permanently disables privileged delete on an FSx for ONTAP SnapLock Enterprise volume. Valid values: `DISABLED`, `ENABLED`, `PERMANENTLY_DISABLED`. The default value is `DISABLED`.
    #[builder(into)]
    #[serde(rename = "privilegedDelete")]
    pub r#privileged_delete: Option<String>,
    /// The retention period of an FSx for ONTAP SnapLock volume. See `retention_period` Block for details.
    #[builder(into)]
    #[serde(rename = "retentionPeriod")]
    pub r#retention_period: Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriod>>,
    /// Specifies the retention mode of an FSx for ONTAP SnapLock volume. After it is set, it can't be changed. Valid values: `COMPLIANCE`, `ENTERPRISE`.
    #[builder(into)]
    #[serde(rename = "snaplockType")]
    pub r#snaplock_type: String,
    /// Enables or disables volume-append mode on an FSx for ONTAP SnapLock volume. The default value is `false`.
    #[builder(into)]
    #[serde(rename = "volumeAppendModeEnabled")]
    pub r#volume_append_mode_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OntapVolumeSnaplockConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "audit_log_volume".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audit_log_volume,
                )
                .await,
            );
            map.insert(
                "autocommit_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autocommit_period,
                )
                .await,
            );
            map.insert(
                "privileged_delete".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#privileged_delete,
                )
                .await,
            );
            map.insert(
                "retention_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_period,
                )
                .await,
            );
            map.insert(
                "snaplock_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snaplock_type,
                )
                .await,
            );
            map.insert(
                "volume_append_mode_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volume_append_mode_enabled,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OntapVolumeSnaplockConfiguration {
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
                    r#audit_log_volume: {
                        let field_value = match fields_map.get("audit_log_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'audit_log_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autocommit_period: {
                        let field_value = match fields_map.get("autocommit_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'autocommit_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#privileged_delete: {
                        let field_value = match fields_map.get("privileged_delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'privileged_delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_period: {
                        let field_value = match fields_map.get("retention_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snaplock_type: {
                        let field_value = match fields_map.get("snaplock_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'snaplock_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_append_mode_enabled: {
                        let field_value = match fields_map.get("volume_append_mode_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_append_mode_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
