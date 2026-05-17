#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRule {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    /// Specifies a list of destination FQDN tags.
    #[builder(into)]
    #[serde(rename = "destinationFqdnTags")]
    pub r#destination_fqdn_tags: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "destinationFqdns")]
    pub r#destination_fqdns: Option<Vec<String>>,
    /// Specifies a list of destination URLs for which policy should hold. Needs Premium SKU for Firewall Policy. Conflicts with `destination_fqdns`.
    #[builder(into)]
    #[serde(rename = "destinationUrls")]
    pub r#destination_urls: Option<Vec<String>>,
    /// Specifies a list of HTTP/HTTPS headers to insert. One or more `http_headers` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Option<Vec<super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRuleHttpHeader>>,
    /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Option<Vec<super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRuleProtocol>>,
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Option<Vec<String>>,
    /// Boolean specifying if TLS shall be terminated (true) or not (false). Must be `true` when using `destination_urls`. Needs Premium SKU for Firewall Policy.
    #[builder(into)]
    #[serde(rename = "terminateTls")]
    pub r#terminate_tls: Option<bool>,
    /// Specifies a list of web categories to which access is denied or allowed depending on the value of `action` above. Needs Premium SKU for Firewall Policy.
    #[builder(into)]
    #[serde(rename = "webCategories")]
    pub r#web_categories: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "destination_addresses",
                    &self.r#destination_addresses,
                ),
                to_pulumi_object_field(
                    "destination_fqdn_tags",
                    &self.r#destination_fqdn_tags,
                ),
                to_pulumi_object_field(
                    "destination_fqdns",
                    &self.r#destination_fqdns,
                ),
                to_pulumi_object_field(
                    "destination_urls",
                    &self.r#destination_urls,
                ),
                to_pulumi_object_field(
                    "http_headers",
                    &self.r#http_headers,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "protocols",
                    &self.r#protocols,
                ),
                to_pulumi_object_field(
                    "source_addresses",
                    &self.r#source_addresses,
                ),
                to_pulumi_object_field(
                    "source_ip_groups",
                    &self.r#source_ip_groups,
                ),
                to_pulumi_object_field(
                    "terminate_tls",
                    &self.r#terminate_tls,
                ),
                to_pulumi_object_field(
                    "web_categories",
                    &self.r#web_categories,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRule {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_addresses: {
                        let field_value = match fields_map.get("destination_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_fqdn_tags: {
                        let field_value = match fields_map.get("destination_fqdn_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_fqdn_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_fqdns: {
                        let field_value = match fields_map.get("destination_fqdns") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_fqdns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_urls: {
                        let field_value = match fields_map.get("destination_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_headers: {
                        let field_value = match fields_map.get("http_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocols: {
                        let field_value = match fields_map.get("protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_addresses: {
                        let field_value = match fields_map.get("source_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_ip_groups: {
                        let field_value = match fields_map.get("source_ip_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_ip_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#terminate_tls: {
                        let field_value = match fields_map.get("terminate_tls") {
                            Some(value) => value,
                            None => bail!("Missing field 'terminate_tls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_categories: {
                        let field_value = match fields_map.get("web_categories") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_categories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
