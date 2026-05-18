#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRoutingRule {
    /// Protocol schemes to match for the Backend Routing Rule. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "acceptedProtocols")]
    pub r#accepted_protocols: Vec<String>,
    /// `Enable` or `Disable` use of this Backend Routing Rule. Permitted values are `true` or `false`. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// A `forwarding_configuration` block as defined below.
    #[builder(into)]
    #[serde(rename = "forwardingConfiguration")]
    pub r#forwarding_configuration: Option<Box<super::super::types::frontdoor::FrontdoorRoutingRuleForwardingConfiguration>>,
    /// The names of the `frontend_endpoint` blocks within this resource to associate with this `routing_rule`.
    #[builder(into)]
    #[serde(rename = "frontendEndpoints")]
    pub r#frontend_endpoints: Vec<String>,
    /// The ID of the FrontDoor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the name of the Routing Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The route patterns for the Backend Routing Rule.
    #[builder(into)]
    #[serde(rename = "patternsToMatches")]
    pub r#patterns_to_matches: Vec<String>,
    /// A `redirect_configuration` block as defined below.
    #[builder(into)]
    #[serde(rename = "redirectConfiguration")]
    pub r#redirect_configuration: Option<Box<super::super::types::frontdoor::FrontdoorRoutingRuleRedirectConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRoutingRule {
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
                    "accepted_protocols",
                    &self.r#accepted_protocols,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "forwarding_configuration",
                    &self.r#forwarding_configuration,
                ),
                to_pulumi_object_field(
                    "frontend_endpoints",
                    &self.r#frontend_endpoints,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "patterns_to_matches",
                    &self.r#patterns_to_matches,
                ),
                to_pulumi_object_field(
                    "redirect_configuration",
                    &self.r#redirect_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRoutingRule {
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
                    r#accepted_protocols: {
                        let field_value = match fields_map.get("accepted_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'accepted_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#forwarding_configuration: {
                        let field_value = match fields_map.get("forwarding_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarding_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_endpoints: {
                        let field_value = match fields_map.get("frontend_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontend_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patterns_to_matches: {
                        let field_value = match fields_map.get("patterns_to_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'patterns_to_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_configuration: {
                        let field_value = match fields_map.get("redirect_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
