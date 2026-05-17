#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSourceBackupConfigInfoGcpBackupConfig {
    /// The name of the backup plan.
    #[builder(into)]
    #[serde(rename = "backupPlan")]
    pub r#backup_plan: String,
    /// The name of the backup plan association.
    #[builder(into)]
    #[serde(rename = "backupPlanAssociation")]
    pub r#backup_plan_association: String,
    /// The description of the backup plan.
    #[builder(into)]
    #[serde(rename = "backupPlanDescription")]
    pub r#backup_plan_description: String,
    /// The names of the backup plan rules which point to this backupvault
    #[builder(into)]
    #[serde(rename = "backupPlanRules")]
    pub r#backup_plan_rules: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSourceBackupConfigInfoGcpBackupConfig {
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
                "backup_plan".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_plan,
                )
                .await,
            );
            map.insert(
                "backup_plan_association".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_plan_association,
                )
                .await,
            );
            map.insert(
                "backup_plan_description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_plan_description,
                )
                .await,
            );
            map.insert(
                "backup_plan_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_plan_rules,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSourceBackupConfigInfoGcpBackupConfig {
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
                    r#backup_plan: {
                        let field_value = match fields_map.get("backup_plan") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_plan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_plan_association: {
                        let field_value = match fields_map.get("backup_plan_association") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_plan_association' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_plan_description: {
                        let field_value = match fields_map.get("backup_plan_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_plan_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_plan_rules: {
                        let field_value = match fields_map.get("backup_plan_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_plan_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
