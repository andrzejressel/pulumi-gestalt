#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RunCommandInstanceView {
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    #[builder(into)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Option<String>,
    #[builder(into)]
    #[serde(rename = "executionMessage")]
    pub r#execution_message: Option<String>,
    #[builder(into)]
    #[serde(rename = "executionState")]
    pub r#execution_state: Option<String>,
    #[builder(into)]
    #[serde(rename = "exitCode")]
    pub r#exit_code: Option<i32>,
    #[builder(into)]
    #[serde(rename = "output")]
    pub r#output: Option<String>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RunCommandInstanceView {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "end_time",
                    &self.r#end_time,
                ),
                to_pulumi_object_field(
                    "error_message",
                    &self.r#error_message,
                ),
                to_pulumi_object_field(
                    "execution_message",
                    &self.r#execution_message,
                ),
                to_pulumi_object_field(
                    "execution_state",
                    &self.r#execution_state,
                ),
                to_pulumi_object_field(
                    "exit_code",
                    &self.r#exit_code,
                ),
                to_pulumi_object_field(
                    "output",
                    &self.r#output,
                ),
                to_pulumi_object_field(
                    "start_time",
                    &self.r#start_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RunCommandInstanceView {
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
                    r#end_time: {
                        let field_value = match fields_map.get("end_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_message: {
                        let field_value = match fields_map.get("error_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_message: {
                        let field_value = match fields_map.get("execution_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_state: {
                        let field_value = match fields_map.get("execution_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exit_code: {
                        let field_value = match fields_map.get("exit_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'exit_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output: {
                        let field_value = match fields_map.get("output") {
                            Some(value) => value,
                            None => bail!("Missing field 'output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
