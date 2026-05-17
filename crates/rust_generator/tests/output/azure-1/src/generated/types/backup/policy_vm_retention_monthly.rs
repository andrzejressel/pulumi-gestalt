#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyVmRetentionMonthly {
    /// The number of monthly backups to keep. Must be between `1` and `9999`
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// The days of the month to retain backups of. Must be between `1` and `31`.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Option<Vec<i32>>,
    /// Including the last day of the month, default to `false`.
    /// 
    /// > **NOTE:**: Either `weekdays` and `weeks` or `days` and `include_last_days` must be specified.
    #[builder(into)]
    #[serde(rename = "includeLastDays")]
    pub r#include_last_days: Option<bool>,
    /// The weekday backups to retain . Must be one of `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Option<Vec<String>>,
    /// The weeks of the month to retain backups of. Must be one of `First`, `Second`, `Third`, `Fourth`, `Last`.
    #[builder(into)]
    #[serde(rename = "weeks")]
    pub r#weeks: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyVmRetentionMonthly {
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
                "count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count,
                )
                .await,
            );
            map.insert(
                "days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#days,
                )
                .await,
            );
            map.insert(
                "include_last_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_last_days,
                )
                .await,
            );
            map.insert(
                "weekdays".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weekdays,
                )
                .await,
            );
            map.insert(
                "weeks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weeks,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyVmRetentionMonthly {
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
                    r#count: {
                        let field_value = match fields_map.get("count") {
                            Some(value) => value,
                            None => bail!("Missing field 'count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days: {
                        let field_value = match fields_map.get("days") {
                            Some(value) => value,
                            None => bail!("Missing field 'days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_last_days: {
                        let field_value = match fields_map.get("include_last_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_last_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekdays: {
                        let field_value = match fields_map.get("weekdays") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekdays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weeks: {
                        let field_value = match fields_map.get("weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
