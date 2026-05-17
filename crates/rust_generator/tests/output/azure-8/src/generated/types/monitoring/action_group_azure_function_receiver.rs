#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupAzureFunctionReceiver {
    /// The Azure resource ID of the function app.
    #[builder(into)]
    #[serde(rename = "functionAppResourceId")]
    pub r#function_app_resource_id: String,
    /// The function name in the function app.
    #[builder(into)]
    #[serde(rename = "functionName")]
    pub r#function_name: String,
    /// The HTTP trigger url where HTTP request sent to.
    #[builder(into)]
    #[serde(rename = "httpTriggerUrl")]
    pub r#http_trigger_url: String,
    /// The name of the Azure Function receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Enables or disables the common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActionGroupAzureFunctionReceiver {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "function_app_resource_id",
                    &self.r#function_app_resource_id,
                ),
                to_pulumi_object_field(
                    "function_name",
                    &self.r#function_name,
                ),
                to_pulumi_object_field(
                    "http_trigger_url",
                    &self.r#http_trigger_url,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "use_common_alert_schema",
                    &self.r#use_common_alert_schema,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ActionGroupAzureFunctionReceiver {
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
                    r#function_app_resource_id: {
                        let field_value = match fields_map.get("function_app_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'function_app_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#function_name: {
                        let field_value = match fields_map.get("function_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'function_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_trigger_url: {
                        let field_value = match fields_map.get("http_trigger_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_trigger_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#use_common_alert_schema: {
                        let field_value = match fields_map.get("use_common_alert_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_common_alert_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
