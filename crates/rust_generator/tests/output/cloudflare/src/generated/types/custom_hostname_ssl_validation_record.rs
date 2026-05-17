#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomHostnameSslValidationRecord {
    #[builder(into)]
    #[serde(rename = "cnameName")]
    pub r#cname_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "cnameTarget")]
    pub r#cname_target: Option<String>,
    #[builder(into)]
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "httpBody")]
    pub r#http_body: Option<String>,
    #[builder(into)]
    #[serde(rename = "httpUrl")]
    pub r#http_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "txtName")]
    pub r#txt_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "txtValue")]
    pub r#txt_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomHostnameSslValidationRecord {
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
                    "cname_name",
                    &self.r#cname_name,
                ),
                to_pulumi_object_field(
                    "cname_target",
                    &self.r#cname_target,
                ),
                to_pulumi_object_field(
                    "emails",
                    &self.r#emails,
                ),
                to_pulumi_object_field(
                    "http_body",
                    &self.r#http_body,
                ),
                to_pulumi_object_field(
                    "http_url",
                    &self.r#http_url,
                ),
                to_pulumi_object_field(
                    "txt_name",
                    &self.r#txt_name,
                ),
                to_pulumi_object_field(
                    "txt_value",
                    &self.r#txt_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomHostnameSslValidationRecord {
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
                    r#cname_name: {
                        let field_value = match fields_map.get("cname_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cname_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cname_target: {
                        let field_value = match fields_map.get("cname_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'cname_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emails: {
                        let field_value = match fields_map.get("emails") {
                            Some(value) => value,
                            None => bail!("Missing field 'emails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_body: {
                        let field_value = match fields_map.get("http_body") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_url: {
                        let field_value = match fields_map.get("http_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#txt_name: {
                        let field_value = match fields_map.get("txt_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'txt_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#txt_value: {
                        let field_value = match fields_map.get("txt_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'txt_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
