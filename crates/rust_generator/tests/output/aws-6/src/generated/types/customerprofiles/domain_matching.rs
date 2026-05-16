#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainMatching {
    /// A block that specifies the configuration about the auto-merging process. Documented below.
    #[builder(into)]
    #[serde(rename = "autoMerging")]
    pub r#auto_merging: Option<Box<super::super::types::customerprofiles::DomainMatchingAutoMerging>>,
    /// The flag that enables the matching process of duplicate profiles.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A block that specifies the configuration for exporting Identity Resolution results. Documented below.
    #[builder(into)]
    #[serde(rename = "exportingConfig")]
    pub r#exporting_config: Option<Box<super::super::types::customerprofiles::DomainMatchingExportingConfig>>,
    /// A block that specifies the day and time when you want to start the Identity Resolution Job every week. Documented below.
    #[builder(into)]
    #[serde(rename = "jobSchedule")]
    pub r#job_schedule: Option<Box<super::super::types::customerprofiles::DomainMatchingJobSchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainMatching {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("auto_merging".to_string(), self.r#auto_merging.to_pulumi_value().await);
            map.insert("enabled".to_string(), self.r#enabled.to_pulumi_value().await);
            map.insert("exporting_config".to_string(), self.r#exporting_config.to_pulumi_value().await);
            map.insert("job_schedule".to_string(), self.r#job_schedule.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainMatching {
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
                    r#auto_merging: {
                        let field_value = match fields_map.get("auto_merging") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_merging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::customerprofiles::DomainMatchingAutoMerging>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exporting_config: {
                        let field_value = match fields_map.get("exporting_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'exporting_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::customerprofiles::DomainMatchingExportingConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#job_schedule: {
                        let field_value = match fields_map.get("job_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::customerprofiles::DomainMatchingJobSchedule>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
