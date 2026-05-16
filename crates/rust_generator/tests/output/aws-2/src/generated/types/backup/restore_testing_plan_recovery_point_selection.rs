#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RestoreTestingPlanRecoveryPointSelection {
    /// Specifies the algorithm used for selecting recovery points. Valid values are "RANDOM_WITHIN_WINDOW" and "LATEST_WITHIN_WINDOW".
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: String,
    /// Specifies the backup vaults to exclude from the recovery point selection. Each value must be a valid AWS ARN for a backup vault or "*" to exclude all backup vaults.
    #[builder(into)]
    #[serde(rename = "excludeVaults")]
    pub r#exclude_vaults: Option<Vec<String>>,
    /// Specifies the backup vaults to include in the recovery point selection. Each value must be a valid AWS ARN for a backup vault or "*" to include all backup vaults.
    #[builder(into)]
    #[serde(rename = "includeVaults")]
    pub r#include_vaults: Vec<String>,
    /// Specifies the types of recovery points to include in the selection. Valid values are "CONTINUOUS" and "SNAPSHOT".
    #[builder(into)]
    #[serde(rename = "recoveryPointTypes")]
    pub r#recovery_point_types: Vec<String>,
    /// Specifies the number of days within which the recovery points should be selected. Must be a value between 1 and 365.
    #[builder(into)]
    #[serde(rename = "selectionWindowDays")]
    pub r#selection_window_days: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RestoreTestingPlanRecoveryPointSelection {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("algorithm".to_string(), self.r#algorithm.to_pulumi_value().await);
            map.insert("exclude_vaults".to_string(), self.r#exclude_vaults.to_pulumi_value().await);
            map.insert("include_vaults".to_string(), self.r#include_vaults.to_pulumi_value().await);
            map.insert("recovery_point_types".to_string(), self.r#recovery_point_types.to_pulumi_value().await);
            map.insert("selection_window_days".to_string(), self.r#selection_window_days.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RestoreTestingPlanRecoveryPointSelection {
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
                    r#algorithm: {
                        let field_value = match fields_map.get("algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclude_vaults: {
                        let field_value = match fields_map.get("exclude_vaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_vaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#include_vaults: {
                        let field_value = match fields_map.get("include_vaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_vaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#recovery_point_types: {
                        let field_value = match fields_map.get("recovery_point_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_point_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#selection_window_days: {
                        let field_value = match fields_map.get("selection_window_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'selection_window_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
