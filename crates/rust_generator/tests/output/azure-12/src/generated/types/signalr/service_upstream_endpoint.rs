#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceUpstreamEndpoint {
    /// The categories to match on, or `*` for all.
    #[builder(into)]
    #[serde(rename = "categoryPatterns")]
    pub r#category_patterns: Vec<String>,
    /// The events to match on, or `*` for all.
    #[builder(into)]
    #[serde(rename = "eventPatterns")]
    pub r#event_patterns: Vec<String>,
    /// The hubs to match on, or `*` for all.
    #[builder(into)]
    #[serde(rename = "hubPatterns")]
    pub r#hub_patterns: Vec<String>,
    /// The upstream URL Template. This can be a url or a template such as `http://host.com/{hub}/api/{category}/{event}`.
    #[builder(into)]
    #[serde(rename = "urlTemplate")]
    pub r#url_template: String,
    /// Specifies the Managed Identity IDs to be assigned to this signalR upstream setting by using resource uuid as both system assigned and user assigned identity is supported.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceUpstreamEndpoint {
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
                    "categoryPatterns",
                    &self.r#category_patterns,
                ),
                to_pulumi_object_field(
                    "eventPatterns",
                    &self.r#event_patterns,
                ),
                to_pulumi_object_field(
                    "hubPatterns",
                    &self.r#hub_patterns,
                ),
                to_pulumi_object_field(
                    "urlTemplate",
                    &self.r#url_template,
                ),
                to_pulumi_object_field(
                    "userAssignedIdentityId",
                    &self.r#user_assigned_identity_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceUpstreamEndpoint {
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
                    r#category_patterns: {
                        let field_value = match fields_map.get("categoryPatterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'categoryPatterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_patterns: {
                        let field_value = match fields_map.get("eventPatterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventPatterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hub_patterns: {
                        let field_value = match fields_map.get("hubPatterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'hubPatterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_template: {
                        let field_value = match fields_map.get("urlTemplate") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlTemplate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_assigned_identity_id: {
                        let field_value = match fields_map.get("userAssignedIdentityId") {
                            Some(value) => value,
                            None => bail!("Missing field 'userAssignedIdentityId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
