#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFunctionUrlCor {
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: bool,
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Vec<String>,
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Vec<String>,
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Vec<String>,
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Vec<String>,
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFunctionUrlCor {
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
                    "allow_credentials",
                    &self.r#allow_credentials,
                ),
                to_pulumi_object_field(
                    "allow_headers",
                    &self.r#allow_headers,
                ),
                to_pulumi_object_field(
                    "allow_methods",
                    &self.r#allow_methods,
                ),
                to_pulumi_object_field(
                    "allow_origins",
                    &self.r#allow_origins,
                ),
                to_pulumi_object_field(
                    "expose_headers",
                    &self.r#expose_headers,
                ),
                to_pulumi_object_field(
                    "max_age",
                    &self.r#max_age,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFunctionUrlCor {
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
                    r#allow_credentials: {
                        let field_value = match fields_map.get("allow_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_headers: {
                        let field_value = match fields_map.get("allow_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_methods: {
                        let field_value = match fields_map.get("allow_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_origins: {
                        let field_value = match fields_map.get("allow_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expose_headers: {
                        let field_value = match fields_map.get("expose_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'expose_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_age: {
                        let field_value = match fields_map.get("max_age") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
