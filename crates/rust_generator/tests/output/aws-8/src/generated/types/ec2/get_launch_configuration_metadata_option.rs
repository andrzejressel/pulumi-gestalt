#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchConfigurationMetadataOption {
    /// State of the metadata service: `enabled`, `disabled`.
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: String,
    /// The desired HTTP PUT response hop limit for instance metadata requests.
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: i32,
    /// If session tokens are required: `optional`, `required`.
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchConfigurationMetadataOption {
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
                "http_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_endpoint,
                )
                .await,
            );
            map.insert(
                "http_put_response_hop_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_put_response_hop_limit,
                )
                .await,
            );
            map.insert(
                "http_tokens".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_tokens,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLaunchConfigurationMetadataOption {
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
                    r#http_endpoint: {
                        let field_value = match fields_map.get("http_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_put_response_hop_limit: {
                        let field_value = match fields_map.get("http_put_response_hop_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_put_response_hop_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_tokens: {
                        let field_value = match fields_map.get("http_tokens") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_tokens' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
