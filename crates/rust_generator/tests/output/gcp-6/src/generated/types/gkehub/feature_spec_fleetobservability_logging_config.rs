#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureSpecFleetobservabilityLoggingConfig {
    /// Specified if applying the default routing config to logs not specified in other configs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "defaultConfig")]
    pub r#default_config: Option<Box<super::super::types::gkehub::FeatureSpecFleetobservabilityLoggingConfigDefaultConfig>>,
    /// Specified if applying the routing config to all logs for all fleet scopes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fleetScopeLogsConfig")]
    pub r#fleet_scope_logs_config: Option<Box<super::super::types::gkehub::FeatureSpecFleetobservabilityLoggingConfigFleetScopeLogsConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureSpecFleetobservabilityLoggingConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "default_config",
                    &self.r#default_config,
                ),
                to_pulumi_object_field(
                    "fleet_scope_logs_config",
                    &self.r#fleet_scope_logs_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureSpecFleetobservabilityLoggingConfig {
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
                    r#default_config: {
                        let field_value = match fields_map.get("default_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fleet_scope_logs_config: {
                        let field_value = match fields_map.get("fleet_scope_logs_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'fleet_scope_logs_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
