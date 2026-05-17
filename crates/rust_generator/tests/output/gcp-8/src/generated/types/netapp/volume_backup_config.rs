#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeBackupConfig {
    /// Specify a single backup policy ID for scheduled backups. Format: `projects/{{projectId}}/locations/{{location}}/backupPolicies/{{backupPolicyName}}`
    #[builder(into)]
    #[serde(rename = "backupPolicies")]
    pub r#backup_policies: Option<Vec<String>>,
    /// ID of the backup vault to use. A backup vault is reqired to create manual or scheduled backups.
    /// Format: `projects/{{projectId}}/locations/{{location}}/backupVaults/{{backupVaultName}}`
    #[builder(into)]
    #[serde(rename = "backupVault")]
    pub r#backup_vault: Option<String>,
    /// When set to true, scheduled backup is enabled on the volume. Omit if no backup_policy is specified.
    #[builder(into)]
    #[serde(rename = "scheduledBackupEnabled")]
    pub r#scheduled_backup_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeBackupConfig {
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
                "backup_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_policies,
                )
                .await,
            );
            map.insert(
                "backup_vault".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_vault,
                )
                .await,
            );
            map.insert(
                "scheduled_backup_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scheduled_backup_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeBackupConfig {
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
                    r#backup_policies: {
                        let field_value = match fields_map.get("backup_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_vault: {
                        let field_value = match fields_map.get("backup_vault") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_vault' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheduled_backup_enabled: {
                        let field_value = match fields_map.get("scheduled_backup_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduled_backup_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
