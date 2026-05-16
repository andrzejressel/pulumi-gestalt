#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TrailEventSelector {
    /// Configuration block for data events. See details below.
    #[builder(into)]
    #[serde(rename = "dataResources")]
    pub r#data_resources: Option<Vec<super::super::types::cloudtrail::TrailEventSelectorDataResource>>,
    /// A set of event sources to exclude. Valid values include: `kms.amazonaws.com` and `rdsdata.amazonaws.com`. `include_management_events` must be set to`true` to allow this.
    #[builder(into)]
    #[serde(rename = "excludeManagementEventSources")]
    pub r#exclude_management_event_sources: Option<Vec<String>>,
    /// Whether to include management events for your trail. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeManagementEvents")]
    pub r#include_management_events: Option<bool>,
    /// Type of events to log. Valid values are `ReadOnly`, `WriteOnly`, `All`. Default value is `All`.
    #[builder(into)]
    #[serde(rename = "readWriteType")]
    pub r#read_write_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TrailEventSelector {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_resources".to_string(), self.r#data_resources.to_pulumi_value().await);
            map.insert("exclude_management_event_sources".to_string(), self.r#exclude_management_event_sources.to_pulumi_value().await);
            map.insert("include_management_events".to_string(), self.r#include_management_events.to_pulumi_value().await);
            map.insert("read_write_type".to_string(), self.r#read_write_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TrailEventSelector {
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
                    r#data_resources: {
                        let field_value = match fields_map.get("data_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cloudtrail::TrailEventSelectorDataResource>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclude_management_event_sources: {
                        let field_value = match fields_map.get("exclude_management_event_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_management_event_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#include_management_events: {
                        let field_value = match fields_map.get("include_management_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_management_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#read_write_type: {
                        let field_value = match fields_map.get("read_write_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_write_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
