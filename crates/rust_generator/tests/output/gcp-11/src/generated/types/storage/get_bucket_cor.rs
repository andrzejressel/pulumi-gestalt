#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBucketCor {
    /// The value, in seconds, to return in the Access-Control-Max-Age header used in preflight responses.
    #[builder(into)]
    #[serde(rename = "maxAgeSeconds")]
    pub r#max_age_seconds: i32,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: "*" is permitted in the list of methods, and means "any method".
    #[builder(into)]
    #[serde(rename = "methods")]
    pub r#methods: Vec<String>,
    /// The list of Origins eligible to receive CORS response headers. Note: "*" is permitted in the list of origins, and means "any Origin".
    #[builder(into)]
    #[serde(rename = "origins")]
    pub r#origins: Vec<String>,
    /// The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains.
    #[builder(into)]
    #[serde(rename = "responseHeaders")]
    pub r#response_headers: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBucketCor {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "max_age_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_age_seconds,
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
                "origins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origins,
                )
                .await,
            );
            map.insert(
                "response_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_headers,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBucketCor {
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
                    r#max_age_seconds: {
                        let field_value = match fields_map.get("max_age_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#origins: {
                        let field_value = match fields_map.get("origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_headers: {
                        let field_value = match fields_map.get("response_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
