#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLoadBalancerPoolsPoolLoadShedding {
    /// Percent of traffic to shed 0 - 100.
    #[builder(into)]
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Option<f64>,
    /// Method of shedding traffic. Available values: `""`, `hash`, `random`
    #[builder(into)]
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Option<String>,
    /// Percent of session traffic to shed 0 - 100.
    #[builder(into)]
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Option<f64>,
    /// Method of shedding traffic. Available values: `""`, `hash`
    #[builder(into)]
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLoadBalancerPoolsPoolLoadShedding {
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
                "default_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_percent,
                )
                .await,
            );
            map.insert(
                "default_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_policy,
                )
                .await,
            );
            map.insert(
                "session_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_percent,
                )
                .await,
            );
            map.insert(
                "session_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_policy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLoadBalancerPoolsPoolLoadShedding {
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
                    r#default_percent: {
                        let field_value = match fields_map.get("default_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_policy: {
                        let field_value = match fields_map.get("default_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_percent: {
                        let field_value = match fields_map.get("session_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_policy: {
                        let field_value = match fields_map.get("session_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
