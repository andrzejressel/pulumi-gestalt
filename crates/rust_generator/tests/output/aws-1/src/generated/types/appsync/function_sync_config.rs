#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionSyncConfig {
    /// Conflict Detection strategy to use. Valid values are `NONE` and `VERSION`.
    #[builder(into)]
    #[serde(rename = "conflictDetection")]
    pub r#conflict_detection: Option<String>,
    /// Conflict Resolution strategy to perform in the event of a conflict. Valid values are `NONE`, `OPTIMISTIC_CONCURRENCY`, `AUTOMERGE`, and `LAMBDA`.
    #[builder(into)]
    #[serde(rename = "conflictHandler")]
    pub r#conflict_handler: Option<String>,
    /// Lambda Conflict Handler Config when configuring `LAMBDA` as the Conflict Handler. See `lambda_conflict_handler_config` Block for details.
    #[builder(into)]
    #[serde(rename = "lambdaConflictHandlerConfig")]
    pub r#lambda_conflict_handler_config: Option<Box<super::super::types::appsync::FunctionSyncConfigLambdaConflictHandlerConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionSyncConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("conflict_detection".to_string(), self.r#conflict_detection.to_pulumi_value().await);
            map.insert("conflict_handler".to_string(), self.r#conflict_handler.to_pulumi_value().await);
            map.insert("lambda_conflict_handler_config".to_string(), self.r#lambda_conflict_handler_config.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionSyncConfig {
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
                    r#conflict_detection: {
                        let field_value = match fields_map.get("conflict_detection") {
                            Some(value) => value,
                            None => bail!("Missing field 'conflict_detection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#conflict_handler: {
                        let field_value = match fields_map.get("conflict_handler") {
                            Some(value) => value,
                            None => bail!("Missing field 'conflict_handler' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lambda_conflict_handler_config: {
                        let field_value = match fields_map.get("lambda_conflict_handler_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_conflict_handler_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appsync::FunctionSyncConfigLambdaConflictHandlerConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
