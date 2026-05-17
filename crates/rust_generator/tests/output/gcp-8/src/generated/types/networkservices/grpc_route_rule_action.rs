#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GrpcRouteRuleAction {
    /// The destination to which traffic should be forwarded.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::networkservices::GrpcRouteRuleActionDestination>>,
    /// The specification for fault injection introduced into traffic to test the resiliency of clients to backend service failure.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "faultInjectionPolicy")]
    pub r#fault_injection_policy: Option<Box<super::super::types::networkservices::GrpcRouteRuleActionFaultInjectionPolicy>>,
    /// Specifies the retry policy associated with this route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Option<Box<super::super::types::networkservices::GrpcRouteRuleActionRetryPolicy>>,
    /// Specifies the timeout for selected route.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GrpcRouteRuleAction {
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
                "destinations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destinations,
                )
                .await,
            );
            map.insert(
                "fault_injection_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fault_injection_policy,
                )
                .await,
            );
            map.insert(
                "retry_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_policy,
                )
                .await,
            );
            map.insert(
                "timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GrpcRouteRuleAction {
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
                    r#destinations: {
                        let field_value = match fields_map.get("destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fault_injection_policy: {
                        let field_value = match fields_map.get("fault_injection_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'fault_injection_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_policy: {
                        let field_value = match fields_map.get("retry_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
