#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpotInstanceRequestMetadataOptions {
    /// Whether the metadata service is available. Valid values include `enabled` or `disabled`. Defaults to `enabled`.
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Option<String>,
    /// Whether the IPv6 endpoint for the instance metadata service is enabled. Defaults to `disabled`.
    #[builder(into)]
    #[serde(rename = "httpProtocolIpv6")]
    pub r#http_protocol_ipv_6: Option<String>,
    /// Desired HTTP PUT response hop limit for instance metadata requests. The larger the number, the further instance metadata requests can travel. Valid values are integer from `1` to `64`. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Option<i32>,
    /// Whether or not the metadata service requires session tokens, also referred to as _Instance Metadata Service Version 2 (IMDSv2)_. Valid values include `optional` or `required`.
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Option<String>,
    /// Enables or disables access to instance tags from the instance metadata service. Valid values include `enabled` or `disabled`. Defaults to `disabled`.
    /// 
    /// For more information, see the documentation on the [Instance Metadata Service](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html).
    #[builder(into)]
    #[serde(rename = "instanceMetadataTags")]
    pub r#instance_metadata_tags: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpotInstanceRequestMetadataOptions {
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
                "http_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_endpoint,
                )
                .await,
            );
            map.insert(
                "http_protocol_ipv_6".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_protocol_ipv_6,
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
            map.insert(
                "instance_metadata_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_metadata_tags,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpotInstanceRequestMetadataOptions {
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
                    r#http_protocol_ipv_6: {
                        let field_value = match fields_map.get("http_protocol_ipv_6") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_protocol_ipv_6' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#instance_metadata_tags: {
                        let field_value = match fields_map.get("instance_metadata_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_metadata_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
