#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketWebsiteConfigurationV2RoutingRuleCondition {
    /// HTTP error code when the redirect is applied. If specified with `key_prefix_equals`, then both must be true for the redirect to be applied.
    #[builder(into)]
    #[serde(rename = "httpErrorCodeReturnedEquals")]
    pub r#http_error_code_returned_equals: Option<String>,
    /// Object key name prefix when the redirect is applied. If specified with `http_error_code_returned_equals`, then both must be true for the redirect to be applied.
    #[builder(into)]
    #[serde(rename = "keyPrefixEquals")]
    pub r#key_prefix_equals: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketWebsiteConfigurationV2RoutingRuleCondition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("http_error_code_returned_equals".to_string(), self.r#http_error_code_returned_equals.to_pulumi_value().await);
            map.insert("key_prefix_equals".to_string(), self.r#key_prefix_equals.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketWebsiteConfigurationV2RoutingRuleCondition {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#http_error_code_returned_equals: {
                        let field_value = match fields_map.get("http_error_code_returned_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_error_code_returned_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#key_prefix_equals: {
                        let field_value = match fields_map.get("key_prefix_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_prefix_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
