#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostingVersionConfigRewrite {
    /// The function to proxy requests to. Must match the exported function name exactly.
    #[builder(into)]
    #[serde(rename = "function")]
    pub r#function: Option<String>,
    /// The user-supplied glob to match against the request URL path.
    #[builder(into)]
    #[serde(rename = "glob")]
    pub r#glob: Option<String>,
    /// The URL path to rewrite the request to.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<String>,
    /// The request will be forwarded to Cloud Run.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "run")]
    pub r#run: Option<Box<super::super::types::firebase::HostingVersionConfigRewriteRun>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HostingVersionConfigRewrite {
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
                    "function",
                    &self.r#function,
                ),
                to_pulumi_object_field(
                    "glob",
                    &self.r#glob,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "regex",
                    &self.r#regex,
                ),
                to_pulumi_object_field(
                    "run",
                    &self.r#run,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HostingVersionConfigRewrite {
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
                    r#function: {
                        let field_value = match fields_map.get("function") {
                            Some(value) => value,
                            None => bail!("Missing field 'function' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#glob: {
                        let field_value = match fields_map.get("glob") {
                            Some(value) => value,
                            None => bail!("Missing field 'glob' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex: {
                        let field_value = match fields_map.get("regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run: {
                        let field_value = match fields_map.get("run") {
                            Some(value) => value,
                            None => bail!("Missing field 'run' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
