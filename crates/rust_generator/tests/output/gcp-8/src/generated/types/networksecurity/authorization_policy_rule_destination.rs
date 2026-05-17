#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthorizationPolicyRuleDestination {
    /// List of host names to match. Matched against the ":authority" header in http requests. At least one host should match. Each host can be an exact match, or a prefix match (example "mydomain.*") or a suffix match (example "*.myorg.com") or a presence (any) match "*".
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Vec<String>,
    /// Match against key:value pair in http header. Provides a flexible match based on HTTP headers, for potentially advanced use cases. At least one header should match.
    /// Avoid using header matches to make authorization decisions unless there is a strong guarantee that requests arrive through a trusted client or proxy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "httpHeaderMatch")]
    pub r#http_header_match: Option<Box<super::super::types::networksecurity::AuthorizationPolicyRuleDestinationHttpHeaderMatch>>,
    /// A list of HTTP methods to match. At least one method should match. Should not be set for gRPC services.
    #[builder(into)]
    #[serde(rename = "methods")]
    pub r#methods: Vec<String>,
    /// List of destination ports to match. At least one port should match.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Vec<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthorizationPolicyRuleDestination {
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
                "hosts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hosts,
                )
                .await,
            );
            map.insert(
                "http_header_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_header_match,
                )
                .await,
            );
            map.insert(
                "methods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#methods,
                )
                .await,
            );
            map.insert(
                "ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ports,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthorizationPolicyRuleDestination {
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
                    r#hosts: {
                        let field_value = match fields_map.get("hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_header_match: {
                        let field_value = match fields_map.get("http_header_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_header_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#methods: {
                        let field_value = match fields_map.get("methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ports: {
                        let field_value = match fields_map.get("ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
