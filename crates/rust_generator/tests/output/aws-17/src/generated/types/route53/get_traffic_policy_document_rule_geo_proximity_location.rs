#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficPolicyDocumentRuleGeoProximityLocation {
    /// Specify a value for `bias` if you want to route more traffic to an endpoint from nearby endpoints (positive values) or route less traffic to an endpoint (negative values).
    #[builder(into)]
    #[serde(rename = "bias")]
    pub r#bias: Option<String>,
    /// References to an endpoint.
    #[builder(into)]
    #[serde(rename = "endpointReference")]
    pub r#endpoint_reference: Option<String>,
    /// Indicates whether you want Amazon Route 53 to evaluate the health of the endpoint and route traffic only to healthy endpoints.
    #[builder(into)]
    #[serde(rename = "evaluateTargetHealth")]
    pub r#evaluate_target_health: Option<bool>,
    /// If you want to associate a health check with the endpoint or rule.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<String>,
    /// Represents the location south (negative) or north (positive) of the equator. Valid values are -90 degrees to 90 degrees.
    #[builder(into)]
    #[serde(rename = "latitude")]
    pub r#latitude: Option<String>,
    /// Represents the location west (negative) or east (positive) of the prime meridian. Valid values are -180 degrees to 180 degrees.
    #[builder(into)]
    #[serde(rename = "longitude")]
    pub r#longitude: Option<String>,
    /// If your endpoint is an AWS resource, specify the AWS Region that you created the resource in.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// References to a rule.
    #[builder(into)]
    #[serde(rename = "ruleReference")]
    pub r#rule_reference: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTrafficPolicyDocumentRuleGeoProximityLocation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bias".to_string(), self.r#bias.to_pulumi_value().await);
            map.insert("endpoint_reference".to_string(), self.r#endpoint_reference.to_pulumi_value().await);
            map.insert("evaluate_target_health".to_string(), self.r#evaluate_target_health.to_pulumi_value().await);
            map.insert("health_check".to_string(), self.r#health_check.to_pulumi_value().await);
            map.insert("latitude".to_string(), self.r#latitude.to_pulumi_value().await);
            map.insert("longitude".to_string(), self.r#longitude.to_pulumi_value().await);
            map.insert("region".to_string(), self.r#region.to_pulumi_value().await);
            map.insert("rule_reference".to_string(), self.r#rule_reference.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTrafficPolicyDocumentRuleGeoProximityLocation {
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
                    r#bias: {
                        let field_value = match fields_map.get("bias") {
                            Some(value) => value,
                            None => bail!("Missing field 'bias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#endpoint_reference: {
                        let field_value = match fields_map.get("endpoint_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#evaluate_target_health: {
                        let field_value = match fields_map.get("evaluate_target_health") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluate_target_health' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#health_check: {
                        let field_value = match fields_map.get("health_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#latitude: {
                        let field_value = match fields_map.get("latitude") {
                            Some(value) => value,
                            None => bail!("Missing field 'latitude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#longitude: {
                        let field_value = match fields_map.get("longitude") {
                            Some(value) => value,
                            None => bail!("Missing field 'longitude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rule_reference: {
                        let field_value = match fields_map.get("rule_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
