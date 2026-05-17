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
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auto_merging",
                    &self.r#auto_merging,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "exporting_config",
                    &self.r#exporting_config,
                ),
                to_pulumi_object_field(
                    "job_schedule",
                    &self.r#job_schedule,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainMatching {
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
                    r#auto_merging: {
                        let field_value = match fields_map.get("auto_merging") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_merging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#exporting_config: {
                        let field_value = match fields_map.get("exporting_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'exporting_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_schedule: {
                        let field_value = match fields_map.get("job_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
