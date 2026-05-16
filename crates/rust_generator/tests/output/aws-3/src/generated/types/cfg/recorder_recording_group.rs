#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecorderRecordingGroup {
    /// Specifies whether AWS Config records configuration changes for every supported type of regional resource (which includes any new type that will become supported in the future). Conflicts with `resource_types`. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "allSupported")]
    pub r#all_supported: Option<bool>,
    /// An object that specifies how AWS Config excludes resource types from being recorded by the configuration recorder.To use this option, you must set the useOnly field of RecordingStrategy to `EXCLUSION_BY_RESOURCE_TYPES` Requires `all_supported = false`. Conflicts with `resource_types`.
    #[builder(into)]
    #[serde(rename = "exclusionByResourceTypes")]
    pub r#exclusion_by_resource_types: Option<Vec<super::super::types::cfg::RecorderRecordingGroupExclusionByResourceType>>,
    /// Specifies whether AWS Config includes all supported types of _global resources_ with the resources that it records. Requires `all_supported = true`. Conflicts with `resource_types`.
    #[builder(into)]
    #[serde(rename = "includeGlobalResourceTypes")]
    pub r#include_global_resource_types: Option<bool>,
    /// Recording Strategy. Detailed below.
    #[builder(into)]
    #[serde(rename = "recordingStrategies")]
    pub r#recording_strategies: Option<Vec<super::super::types::cfg::RecorderRecordingGroupRecordingStrategy>>,
    /// A list that specifies the types of AWS resources for which AWS Config records configuration changes (for example, `AWS::EC2::Instance` or `AWS::CloudTrail::Trail`). See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types. In order to use this attribute, `all_supported` must be set to false.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RecorderRecordingGroup {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("all_supported".to_string(), self.r#all_supported.to_pulumi_value().await);
            map.insert("exclusion_by_resource_types".to_string(), self.r#exclusion_by_resource_types.to_pulumi_value().await);
            map.insert("include_global_resource_types".to_string(), self.r#include_global_resource_types.to_pulumi_value().await);
            map.insert("recording_strategies".to_string(), self.r#recording_strategies.to_pulumi_value().await);
            map.insert("resource_types".to_string(), self.r#resource_types.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RecorderRecordingGroup {
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
                    r#all_supported: {
                        let field_value = match fields_map.get("all_supported") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_supported' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclusion_by_resource_types: {
                        let field_value = match fields_map.get("exclusion_by_resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_by_resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cfg::RecorderRecordingGroupExclusionByResourceType>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#include_global_resource_types: {
                        let field_value = match fields_map.get("include_global_resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_global_resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#recording_strategies: {
                        let field_value = match fields_map.get("recording_strategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'recording_strategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cfg::RecorderRecordingGroupRecordingStrategy>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_types: {
                        let field_value = match fields_map.get("resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
