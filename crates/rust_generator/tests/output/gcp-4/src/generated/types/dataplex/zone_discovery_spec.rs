#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneDiscoverySpec {
    /// Optional. Configuration for CSV data.
    #[builder(into)]
    #[serde(rename = "csvOptions")]
    pub r#csv_options: Option<Box<super::super::types::dataplex::ZoneDiscoverySpecCsvOptions>>,
    /// Required. Whether discovery is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Optional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[builder(into)]
    #[serde(rename = "excludePatterns")]
    pub r#exclude_patterns: Option<Vec<String>>,
    /// Optional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[builder(into)]
    #[serde(rename = "includePatterns")]
    pub r#include_patterns: Option<Vec<String>>,
    /// Optional. Configuration for Json data.
    #[builder(into)]
    #[serde(rename = "jsonOptions")]
    pub r#json_options: Option<Box<super::super::types::dataplex::ZoneDiscoverySpecJsonOptions>>,
    /// Optional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, "CRON_TZ=America/New_York 1 * * * *", or "TZ=America/New_York 1 * * * *".
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZoneDiscoverySpec {
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
                "csv_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#csv_options,
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
                "exclude_patterns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclude_patterns,
                )
                .await,
            );
            map.insert(
                "include_patterns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_patterns,
                )
                .await,
            );
            map.insert(
                "json_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#json_options,
                )
                .await,
            );
            map.insert(
                "schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedule,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZoneDiscoverySpec {
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
                    r#csv_options: {
                        let field_value = match fields_map.get("csv_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'csv_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#exclude_patterns: {
                        let field_value = match fields_map.get("exclude_patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_patterns: {
                        let field_value = match fields_map.get("include_patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_options: {
                        let field_value = match fields_map.get("json_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule: {
                        let field_value = match fields_map.get("schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
