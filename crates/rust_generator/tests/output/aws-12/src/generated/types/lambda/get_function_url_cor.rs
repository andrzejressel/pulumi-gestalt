#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFunctionUrlCor {
    #[builder(into)]
    pub r#allow_credentials: bool,
    #[builder(into)]
    pub r#allow_headers: Vec<String>,
    #[builder(into)]
    pub r#allow_methods: Vec<String>,
    #[builder(into)]
    pub r#allow_origins: Vec<String>,
    #[builder(into)]
    pub r#expose_headers: Vec<String>,
    #[builder(into)]
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
                    "allowCredentials",
                    &self.r#allow_credentials,
                ),
                to_pulumi_object_field(
                    "allowHeaders",
                    &self.r#allow_headers,
                ),
                to_pulumi_object_field(
                    "allowMethods",
                    &self.r#allow_methods,
                ),
                to_pulumi_object_field(
                    "allowOrigins",
                    &self.r#allow_origins,
                ),
                to_pulumi_object_field(
                    "exposeHeaders",
                    &self.r#expose_headers,
                ),
                to_pulumi_object_field(
                    "maxAge",
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
                        let field_value = match fields_map.get("allowCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_headers: {
                        let field_value = match fields_map.get("allowHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_methods: {
                        let field_value = match fields_map.get("allowMethods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowMethods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_origins: {
                        let field_value = match fields_map.get("allowOrigins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowOrigins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expose_headers: {
                        let field_value = match fields_map.get("exposeHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'exposeHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_age: {
                        let field_value = match fields_map.get("maxAge") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxAge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
