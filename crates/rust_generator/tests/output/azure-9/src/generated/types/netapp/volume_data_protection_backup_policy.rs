#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeDataProtectionBackupPolicy {
    /// Resource ID of the backup policy to apply to the volume.
    #[builder(into)]
    #[serde(rename = "backupPolicyId")]
    pub r#backup_policy_id: String,
    /// Resource ID of the backup backup vault to associate this volume to.
    #[builder(into)]
    #[serde(rename = "backupVaultId")]
    pub r#backup_vault_id: String,
    /// Enables the backup policy on the volume, defaults to `true`.
    /// 
    /// For more information on Azure NetApp Files Backup feature please see [Understand Azure NetApp Files backup](https://learn.microsoft.com/en-us/azure/azure-netapp-files/backup-introduction)
    #[builder(into)]
    #[serde(rename = "policyEnabled")]
    pub r#policy_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeDataProtectionBackupPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "backup_policy_id",
                    &self.r#backup_policy_id,
                ),
                to_pulumi_object_field(
                    "backup_vault_id",
                    &self.r#backup_vault_id,
                ),
                to_pulumi_object_field(
                    "policy_enabled",
                    &self.r#policy_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeDataProtectionBackupPolicy {
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
                    r#backup_policy_id: {
                        let field_value = match fields_map.get("backup_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_vault_id: {
                        let field_value = match fields_map.get("backup_vault_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_vault_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_enabled: {
                        let field_value = match fields_map.get("policy_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
