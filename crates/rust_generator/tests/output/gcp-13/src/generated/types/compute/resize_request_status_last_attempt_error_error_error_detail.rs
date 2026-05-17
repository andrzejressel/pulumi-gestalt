#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResizeRequestStatusLastAttemptErrorErrorErrorDetail {
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errorInfos")]
    pub r#error_infos: Option<Vec<super::super::types::compute::ResizeRequestStatusLastAttemptErrorErrorErrorDetailErrorInfo>>,
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "helps")]
    pub r#helps: Option<Vec<super::super::types::compute::ResizeRequestStatusLastAttemptErrorErrorErrorDetailHelp>>,
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "localizedMessages")]
    pub r#localized_messages: Option<Vec<super::super::types::compute::ResizeRequestStatusLastAttemptErrorErrorErrorDetailLocalizedMessage>>,
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "quotaInfos")]
    pub r#quota_infos: Option<Vec<super::super::types::compute::ResizeRequestStatusLastAttemptErrorErrorErrorDetailQuotaInfo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResizeRequestStatusLastAttemptErrorErrorErrorDetail {
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
                    "error_infos",
                    &self.r#error_infos,
                ),
                to_pulumi_object_field(
                    "helps",
                    &self.r#helps,
                ),
                to_pulumi_object_field(
                    "localized_messages",
                    &self.r#localized_messages,
                ),
                to_pulumi_object_field(
                    "quota_infos",
                    &self.r#quota_infos,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResizeRequestStatusLastAttemptErrorErrorErrorDetail {
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
                    r#error_infos: {
                        let field_value = match fields_map.get("error_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#helps: {
                        let field_value = match fields_map.get("helps") {
                            Some(value) => value,
                            None => bail!("Missing field 'helps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#localized_messages: {
                        let field_value = match fields_map.get("localized_messages") {
                            Some(value) => value,
                            None => bail!("Missing field 'localized_messages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_infos: {
                        let field_value = match fields_map.get("quota_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'quota_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
