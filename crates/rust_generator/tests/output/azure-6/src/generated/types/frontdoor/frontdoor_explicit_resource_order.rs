#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorExplicitResourceOrder {
    #[builder(into)]
    #[serde(rename = "backendPoolHealthProbeIds")]
    pub r#backend_pool_health_probe_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "backendPoolIds")]
    pub r#backend_pool_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "backendPoolLoadBalancingIds")]
    pub r#backend_pool_load_balancing_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "frontendEndpointIds")]
    pub r#frontend_endpoint_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "routingRuleIds")]
    pub r#routing_rule_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorExplicitResourceOrder {
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
                "backend_pool_health_probe_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backend_pool_health_probe_ids,
                )
                .await,
            );
            map.insert(
                "backend_pool_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backend_pool_ids,
                )
                .await,
            );
            map.insert(
                "backend_pool_load_balancing_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backend_pool_load_balancing_ids,
                )
                .await,
            );
            map.insert(
                "frontend_endpoint_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frontend_endpoint_ids,
                )
                .await,
            );
            map.insert(
                "routing_rule_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#routing_rule_ids,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorExplicitResourceOrder {
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
                    r#backend_pool_health_probe_ids: {
                        let field_value = match fields_map.get("backend_pool_health_probe_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_pool_health_probe_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_pool_ids: {
                        let field_value = match fields_map.get("backend_pool_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_pool_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_pool_load_balancing_ids: {
                        let field_value = match fields_map.get("backend_pool_load_balancing_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_pool_load_balancing_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_endpoint_ids: {
                        let field_value = match fields_map.get("frontend_endpoint_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontend_endpoint_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routing_rule_ids: {
                        let field_value = match fields_map.get("routing_rule_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'routing_rule_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
