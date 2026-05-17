#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionEventInvokeConfigDestinationConfig {
    /// Configuration block with destination configuration for failed asynchronous invocations. See below for details.
    #[builder(into)]
    #[serde(rename = "onFailure")]
    pub r#on_failure: Option<Box<super::super::types::lambda::FunctionEventInvokeConfigDestinationConfigOnFailure>>,
    /// Configuration block with destination configuration for successful asynchronous invocations. See below for details.
    #[builder(into)]
    #[serde(rename = "onSuccess")]
    pub r#on_success: Option<Box<super::super::types::lambda::FunctionEventInvokeConfigDestinationConfigOnSuccess>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionEventInvokeConfigDestinationConfig {
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
                "on_failure".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_failure,
                )
                .await,
            );
            map.insert(
                "on_success".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_success,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionEventInvokeConfigDestinationConfig {
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
                    r#on_failure: {
                        let field_value = match fields_map.get("on_failure") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_failure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_success: {
                        let field_value = match fields_map.get("on_success") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_success' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
