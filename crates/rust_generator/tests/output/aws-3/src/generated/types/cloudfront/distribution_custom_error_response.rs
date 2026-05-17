#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionCustomErrorResponse {
    /// Minimum amount of time you want HTTP error codes to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated.
    #[builder(into)]
    #[serde(rename = "errorCachingMinTtl")]
    pub r#error_caching_min_ttl: Option<i32>,
    /// 4xx or 5xx HTTP status code that you want to customize.
    #[builder(into)]
    #[serde(rename = "errorCode")]
    pub r#error_code: i32,
    /// HTTP status code that you want CloudFront to return with the custom error page to the viewer.
    #[builder(into)]
    #[serde(rename = "responseCode")]
    pub r#response_code: Option<i32>,
    /// Path of the custom error page (for example, `/custom_404.html`).
    #[builder(into)]
    #[serde(rename = "responsePagePath")]
    pub r#response_page_path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionCustomErrorResponse {
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
                    "error_caching_min_ttl",
                    &self.r#error_caching_min_ttl,
                ),
                to_pulumi_object_field(
                    "error_code",
                    &self.r#error_code,
                ),
                to_pulumi_object_field(
                    "response_code",
                    &self.r#response_code,
                ),
                to_pulumi_object_field(
                    "response_page_path",
                    &self.r#response_page_path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionCustomErrorResponse {
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
                    r#error_caching_min_ttl: {
                        let field_value = match fields_map.get("error_caching_min_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_caching_min_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_code: {
                        let field_value = match fields_map.get("error_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_code: {
                        let field_value = match fields_map.get("response_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_page_path: {
                        let field_value = match fields_map.get("response_page_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_page_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
