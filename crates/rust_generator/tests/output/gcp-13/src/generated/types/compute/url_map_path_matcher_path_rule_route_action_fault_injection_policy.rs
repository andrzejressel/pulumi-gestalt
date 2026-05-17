#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicy {
    /// The specification for how client requests are aborted as part of fault injection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "abort")]
    pub r#abort: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicyAbort>>,
    /// The specification for how client requests are delayed as part of fault injection, before being sent to a backend service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicyDelay>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicy {
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
                "abort".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#abort,
                )
                .await,
            );
            map.insert(
                "delay".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#delay,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicy {
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
                    r#abort: {
                        let field_value = match fields_map.get("abort") {
                            Some(value) => value,
                            None => bail!("Missing field 'abort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delay: {
                        let field_value = match fields_map.get("delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
