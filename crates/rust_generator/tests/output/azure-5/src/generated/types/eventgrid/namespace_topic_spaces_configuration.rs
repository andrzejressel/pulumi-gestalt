#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NamespaceTopicSpacesConfiguration {
    /// Specifies a list of alternative sources for the client authentication name from the client certificate. Possible values are `ClientCertificateDns`, `ClientCertificateEmail`, `ClientCertificateIp`, `ClientCertificateSubject` and `ClientCertificateUri`.
    #[builder(into)]
    #[serde(rename = "alternativeAuthenticationNameSources")]
    pub r#alternative_authentication_name_sources: Option<Vec<String>>,
    /// One or more `dynamic_routing_enrichment` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "dynamicRoutingEnrichments")]
    pub r#dynamic_routing_enrichments: Option<Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfigurationDynamicRoutingEnrichment>>,
    /// Specifies the maximum number of client sessions per authentication name. Valid values can be between `1` and `100`.
    #[builder(into)]
    #[serde(rename = "maximumClientSessionsPerAuthenticationName")]
    pub r#maximum_client_sessions_per_authentication_name: Option<i32>,
    /// Specifies the maximum session expiry interval allowed for all MQTT clients connecting to the Event Grid namespace. Valid values can be between `1` and `8`.
    #[builder(into)]
    #[serde(rename = "maximumSessionExpiryInHours")]
    pub r#maximum_session_expiry_in_hours: Option<i32>,
    /// Specifies the Event Grid topic resource ID to route messages to.
    #[builder(into)]
    #[serde(rename = "routeTopicId")]
    pub r#route_topic_id: Option<String>,
    /// One or more `static_routing_enrichment` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "staticRoutingEnrichments")]
    pub r#static_routing_enrichments: Option<Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfigurationStaticRoutingEnrichment>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NamespaceTopicSpacesConfiguration {
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
                    "alternative_authentication_name_sources",
                    &self.r#alternative_authentication_name_sources,
                ),
                to_pulumi_object_field(
                    "dynamic_routing_enrichments",
                    &self.r#dynamic_routing_enrichments,
                ),
                to_pulumi_object_field(
                    "maximum_client_sessions_per_authentication_name",
                    &self.r#maximum_client_sessions_per_authentication_name,
                ),
                to_pulumi_object_field(
                    "maximum_session_expiry_in_hours",
                    &self.r#maximum_session_expiry_in_hours,
                ),
                to_pulumi_object_field(
                    "route_topic_id",
                    &self.r#route_topic_id,
                ),
                to_pulumi_object_field(
                    "static_routing_enrichments",
                    &self.r#static_routing_enrichments,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NamespaceTopicSpacesConfiguration {
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
                    r#alternative_authentication_name_sources: {
                        let field_value = match fields_map.get("alternative_authentication_name_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'alternative_authentication_name_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_routing_enrichments: {
                        let field_value = match fields_map.get("dynamic_routing_enrichments") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_routing_enrichments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_client_sessions_per_authentication_name: {
                        let field_value = match fields_map.get("maximum_client_sessions_per_authentication_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_client_sessions_per_authentication_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_session_expiry_in_hours: {
                        let field_value = match fields_map.get("maximum_session_expiry_in_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_session_expiry_in_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_topic_id: {
                        let field_value = match fields_map.get("route_topic_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_topic_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_routing_enrichments: {
                        let field_value = match fields_map.get("static_routing_enrichments") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_routing_enrichments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
