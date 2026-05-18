#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardWebTestValidationRulesContent {
    /// A string value containing the content to match on.
    #[builder(into)]
    #[serde(rename = "contentMatch")]
    pub r#content_match: String,
    /// Ignore the casing in the `content_match` value.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Option<bool>,
    /// If the content of `content_match` is found, pass the test. If set to `false`, the WebTest is failing if the content of `content_match` is found.
    #[builder(into)]
    #[serde(rename = "passIfTextFound")]
    pub r#pass_if_text_found: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StandardWebTestValidationRulesContent {
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
                    "content_match",
                    &self.r#content_match,
                ),
                to_pulumi_object_field(
                    "ignore_case",
                    &self.r#ignore_case,
                ),
                to_pulumi_object_field(
                    "pass_if_text_found",
                    &self.r#pass_if_text_found,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StandardWebTestValidationRulesContent {
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
                    r#content_match: {
                        let field_value = match fields_map.get("content_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_case: {
                        let field_value = match fields_map.get("ignore_case") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_case' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pass_if_text_found: {
                        let field_value = match fields_map.get("pass_if_text_found") {
                            Some(value) => value,
                            None => bail!("Missing field 'pass_if_text_found' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
