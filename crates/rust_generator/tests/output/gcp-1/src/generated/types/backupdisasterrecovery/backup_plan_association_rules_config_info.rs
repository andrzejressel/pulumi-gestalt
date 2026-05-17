#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanAssociationRulesConfigInfo {
    /// (Output)
    /// google.rpc.Status object to store the last backup error
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lastBackupErrors")]
    pub r#last_backup_errors: Option<Vec<super::super::types::backupdisasterrecovery::BackupPlanAssociationRulesConfigInfoLastBackupError>>,
    /// (Output)
    /// State of last backup taken.
    #[builder(into)]
    #[serde(rename = "lastBackupState")]
    pub r#last_backup_state: Option<String>,
    /// (Output)
    /// Backup Rule id fetched from backup plan.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPlanAssociationRulesConfigInfo {
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
                "last_backup_errors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_backup_errors,
                )
                .await,
            );
            map.insert(
                "last_backup_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_backup_state,
                )
                .await,
            );
            map.insert(
                "rule_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPlanAssociationRulesConfigInfo {
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
                    r#last_backup_errors: {
                        let field_value = match fields_map.get("last_backup_errors") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_backup_errors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_backup_state: {
                        let field_value = match fields_map.get("last_backup_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_backup_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_id: {
                        let field_value = match fields_map.get("rule_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
