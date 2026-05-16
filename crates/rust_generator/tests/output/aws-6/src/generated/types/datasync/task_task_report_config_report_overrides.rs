#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskTaskReportConfigReportOverrides {
    /// Specifies the level of reporting for the files, objects, and directories that DataSync attempted to delete in your destination location. This only applies if you configure your task to delete data in the destination that isn't in the source. Valid values: `ERRORS_ONLY` and `SUCCESSES_AND_ERRORS`.
    #[builder(into)]
    #[serde(rename = "deletedOverride")]
    pub r#deleted_override: Option<String>,
    /// Specifies the level of reporting for the files, objects, and directories that DataSync attempted to skip during your transfer. Valid values: `ERRORS_ONLY` and `SUCCESSES_AND_ERRORS`.
    #[builder(into)]
    #[serde(rename = "skippedOverride")]
    pub r#skipped_override: Option<String>,
    /// Specifies the level of reporting for the files, objects, and directories that DataSync attempted to transfer. Valid values: `ERRORS_ONLY` and `SUCCESSES_AND_ERRORS`.
    #[builder(into)]
    #[serde(rename = "transferredOverride")]
    pub r#transferred_override: Option<String>,
    /// Specifies the level of reporting for the files, objects, and directories that DataSync attempted to verify at the end of your transfer. Valid values: `ERRORS_ONLY` and `SUCCESSES_AND_ERRORS`.
    /// 
    /// > **NOTE:** If any `report_overrides` are set to the same value as `task_report_config.report_level`, they will always be flagged as changed. Only set overrides to a value that differs from `task_report_config.report_level`.
    #[builder(into)]
    #[serde(rename = "verifiedOverride")]
    pub r#verified_override: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskTaskReportConfigReportOverrides {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("deleted_override".to_string(), self.r#deleted_override.to_pulumi_value().await);
            map.insert("skipped_override".to_string(), self.r#skipped_override.to_pulumi_value().await);
            map.insert("transferred_override".to_string(), self.r#transferred_override.to_pulumi_value().await);
            map.insert("verified_override".to_string(), self.r#verified_override.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskTaskReportConfigReportOverrides {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#deleted_override: {
                        let field_value = match fields_map.get("deleted_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'deleted_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#skipped_override: {
                        let field_value = match fields_map.get("skipped_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'skipped_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#transferred_override: {
                        let field_value = match fields_map.get("transferred_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'transferred_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#verified_override: {
                        let field_value = match fields_map.get("verified_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'verified_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
