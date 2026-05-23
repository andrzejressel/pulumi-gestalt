#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherPathRuleCustomErrorResponsePolicyErrorResponseRule {
    /// Valid values include:
    /// - A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.
    /// - 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.
    /// - 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.
    /// Values must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy.
    #[builder(into)]
    pub r#match_response_codes: Option<Vec<String>>,
    /// The HTTP status code returned with the response containing the custom error content.
    /// If overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client.
    #[builder(into)]
    pub r#override_response_code: Option<i32>,
    /// The full path to a file within backendBucket. For example: /errors/defaultError.html
    /// path must start with a leading slash. path cannot have trailing slashes.
    /// If the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.
    /// The value must be from 1 to 1024 characters.
    #[builder(into)]
    pub r#path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherPathRuleCustomErrorResponsePolicyErrorResponseRule {
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
                    "matchResponseCodes",
                    &self.r#match_response_codes,
                ),
                to_pulumi_object_field(
                    "overrideResponseCode",
                    &self.r#override_response_code,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherPathRuleCustomErrorResponsePolicyErrorResponseRule {
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
                    r#match_response_codes: {
                        let field_value = match fields_map.get("matchResponseCodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'matchResponseCodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#override_response_code: {
                        let field_value = match fields_map.get("overrideResponseCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'overrideResponseCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
