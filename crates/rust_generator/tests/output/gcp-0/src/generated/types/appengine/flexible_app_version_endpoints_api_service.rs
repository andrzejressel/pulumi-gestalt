#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionEndpointsApiService {
    /// Endpoints service configuration ID as specified by the Service Management API. For example "2016-09-19r1".
    /// By default, the rollout strategy for Endpoints is "FIXED". This means that Endpoints starts up with a particular configuration ID.
    /// When a new configuration is rolled out, Endpoints must be given the new configuration ID. The configId field is used to give the configuration ID
    /// and is required in this case.
    /// Endpoints also has a rollout strategy called "MANAGED". When using this, Endpoints fetches the latest configuration and does not need
    /// the configuration ID. In this case, configId must be omitted.
    #[builder(into)]
    #[serde(rename = "configId")]
    pub r#config_id: Option<String>,
    /// Enable or disable trace sampling. By default, this is set to false for enabled.
    #[builder(into)]
    #[serde(rename = "disableTraceSampling")]
    pub r#disable_trace_sampling: Option<bool>,
    /// Endpoints service name which is the name of the "service" resource in the Service Management API.
    /// For example "myapi.endpoints.myproject.cloud.goog"
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Endpoints rollout strategy. If FIXED, configId must be specified. If MANAGED, configId must be omitted.
    /// Default value is `FIXED`.
    /// Possible values are: `FIXED`, `MANAGED`.
    #[builder(into)]
    #[serde(rename = "rolloutStrategy")]
    pub r#rollout_strategy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionEndpointsApiService {
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
                "config_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#config_id,
                )
                .await,
            );
            map.insert(
                "disable_trace_sampling".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_trace_sampling,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "rollout_strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rollout_strategy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionEndpointsApiService {
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
                    r#config_id: {
                        let field_value = match fields_map.get("config_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'config_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_trace_sampling: {
                        let field_value = match fields_map.get("disable_trace_sampling") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_trace_sampling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#rollout_strategy: {
                        let field_value = match fields_map.get("rollout_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'rollout_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
