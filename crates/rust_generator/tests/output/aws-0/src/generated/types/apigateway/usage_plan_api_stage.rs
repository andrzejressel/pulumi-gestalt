#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UsagePlanApiStage {
    /// API Id of the associated API stage in a usage plan.
    #[builder(into)]
    #[serde(rename = "apiId")]
    pub r#api_id: String,
    /// API stage name of the associated API stage in a usage plan.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: String,
    /// The throttling limits of the usage plan.
    #[builder(into)]
    #[serde(rename = "throttles")]
    pub r#throttles: Option<Vec<super::super::types::apigateway::UsagePlanApiStageThrottle>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UsagePlanApiStage {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("api_id".to_string(), self.r#api_id.to_pulumi_value().await);
            map.insert("stage".to_string(), self.r#stage.to_pulumi_value().await);
            map.insert("throttles".to_string(), self.r#throttles.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UsagePlanApiStage {
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
                    r#api_id: {
                        let field_value = match fields_map.get("api_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stage: {
                        let field_value = match fields_map.get("stage") {
                            Some(value) => value,
                            None => bail!("Missing field 'stage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#throttles: {
                        let field_value = match fields_map.get("throttles") {
                            Some(value) => value,
                            None => bail!("Missing field 'throttles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::apigateway::UsagePlanApiStageThrottle>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
