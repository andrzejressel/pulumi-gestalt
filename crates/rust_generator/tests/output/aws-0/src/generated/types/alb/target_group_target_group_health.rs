#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetGroupTargetGroupHealth {
    /// Block to configure DNS Failover requirements. See DNS Failover below for details on attributes.
    #[builder(into)]
    #[serde(rename = "dnsFailover")]
    pub r#dns_failover: Option<Box<super::super::types::alb::TargetGroupTargetGroupHealthDnsFailover>>,
    /// Block to configure Unhealthy State Routing requirements. See Unhealthy State Routing below for details on attributes.
    #[builder(into)]
    #[serde(rename = "unhealthyStateRouting")]
    pub r#unhealthy_state_routing: Option<Box<super::super::types::alb::TargetGroupTargetGroupHealthUnhealthyStateRouting>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TargetGroupTargetGroupHealth {
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
                "dns_failover".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_failover,
                )
                .await,
            );
            map.insert(
                "unhealthy_state_routing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unhealthy_state_routing,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TargetGroupTargetGroupHealth {
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
                    r#dns_failover: {
                        let field_value = match fields_map.get("dns_failover") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_failover' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unhealthy_state_routing: {
                        let field_value = match fields_map.get("unhealthy_state_routing") {
                            Some(value) => value,
                            None => bail!("Missing field 'unhealthy_state_routing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
