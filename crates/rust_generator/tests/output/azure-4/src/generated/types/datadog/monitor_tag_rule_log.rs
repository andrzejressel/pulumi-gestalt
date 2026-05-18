#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MonitorTagRuleLog {
    /// Whether AAD logs should be sent for the Monitor resource?
    #[builder(into)]
    #[serde(rename = "aadLogEnabled")]
    pub r#aad_log_enabled: Option<bool>,
    /// A `filter` block as defined below.
    /// 
    /// > **NOTE:** List of filtering tags to be used for capturing logs. This only takes effect if `resource_log_enabled` flag is enabled. If empty, all resources will be captured. If only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags.
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Option<Vec<super::super::types::datadog::MonitorTagRuleLogFilter>>,
    /// Whether Azure resource logs should be sent for the Monitor resource?
    #[builder(into)]
    #[serde(rename = "resourceLogEnabled")]
    pub r#resource_log_enabled: Option<bool>,
    /// Whether Azure subscription logs should be sent for the Monitor resource?
    #[builder(into)]
    #[serde(rename = "subscriptionLogEnabled")]
    pub r#subscription_log_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MonitorTagRuleLog {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "aad_log_enabled",
                    &self.r#aad_log_enabled,
                ),
                to_pulumi_object_field(
                    "filters",
                    &self.r#filters,
                ),
                to_pulumi_object_field(
                    "resource_log_enabled",
                    &self.r#resource_log_enabled,
                ),
                to_pulumi_object_field(
                    "subscription_log_enabled",
                    &self.r#subscription_log_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MonitorTagRuleLog {
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
                    r#aad_log_enabled: {
                        let field_value = match fields_map.get("aad_log_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'aad_log_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filters: {
                        let field_value = match fields_map.get("filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_log_enabled: {
                        let field_value = match fields_map.get("resource_log_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_log_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscription_log_enabled: {
                        let field_value = match fields_map.get("subscription_log_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscription_log_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
