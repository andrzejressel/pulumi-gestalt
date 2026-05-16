#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MaintenanceWindowTaskTaskInvocationParametersLambdaParameters {
    /// Pass client-specific information to the Lambda function that you are invoking.
    #[builder(into)]
    #[serde(rename = "clientContext")]
    pub r#client_context: Option<String>,
    /// JSON to provide to your Lambda function as input.
    #[builder(into)]
    #[serde(rename = "payload")]
    pub r#payload: Option<String>,
    /// Specify a Lambda function version or alias name.
    #[builder(into)]
    #[serde(rename = "qualifier")]
    pub r#qualifier: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MaintenanceWindowTaskTaskInvocationParametersLambdaParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("client_context".to_string(), self.r#client_context.to_pulumi_value().await);
            map.insert("payload".to_string(), self.r#payload.to_pulumi_value().await);
            map.insert("qualifier".to_string(), self.r#qualifier.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MaintenanceWindowTaskTaskInvocationParametersLambdaParameters {
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
                    r#client_context: {
                        let field_value = match fields_map.get("client_context") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#payload: {
                        let field_value = match fields_map.get("payload") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#qualifier: {
                        let field_value = match fields_map.get("qualifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'qualifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
