#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapDefaultRouteActionFaultInjectionPolicyDelay {
    /// Specifies the value of the fixed delay interval.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fixedDelay")]
    pub r#fixed_delay: Option<Box<super::super::types::compute::UrlMapDefaultRouteActionFaultInjectionPolicyDelayFixedDelay>>,
    /// The percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.
    /// The value must be between 0.0 and 100.0 inclusive.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapDefaultRouteActionFaultInjectionPolicyDelay {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "fixed_delay".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fixed_delay,
                )
                .await,
            );
            map.insert(
                "percentage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#percentage,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapDefaultRouteActionFaultInjectionPolicyDelay {
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
                    r#fixed_delay: {
                        let field_value = match fields_map.get("fixed_delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#percentage: {
                        let field_value = match fields_map.get("percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
