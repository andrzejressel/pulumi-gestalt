#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventHubNamespaceNetworkRulesets {
    /// The default action to take when a rule is not matched. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    /// One or more `ip_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<super::super::types::eventhub::EventHubNamespaceNetworkRulesetsIpRule>>,
    /// Is public network access enabled for the EventHub Namespace? Defaults to `true`.
    /// 
    /// > **Note:** The public network access setting at the network rule sets level should be the same as it's at the namespace level.
    #[builder(into)]
    #[serde(rename = "publicNetworkAccessEnabled")]
    pub r#public_network_access_enabled: Option<bool>,
    /// Whether Trusted Microsoft Services are allowed to bypass firewall.
    #[builder(into)]
    #[serde(rename = "trustedServiceAccessEnabled")]
    pub r#trusted_service_access_enabled: Option<bool>,
    /// One or more `virtual_network_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "virtualNetworkRules")]
    pub r#virtual_network_rules: Option<Vec<super::super::types::eventhub::EventHubNamespaceNetworkRulesetsVirtualNetworkRule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventHubNamespaceNetworkRulesets {
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
                    "defaultAction",
                    &self.r#default_action,
                ),
                to_pulumi_object_field(
                    "ipRules",
                    &self.r#ip_rules,
                ),
                to_pulumi_object_field(
                    "publicNetworkAccessEnabled",
                    &self.r#public_network_access_enabled,
                ),
                to_pulumi_object_field(
                    "trustedServiceAccessEnabled",
                    &self.r#trusted_service_access_enabled,
                ),
                to_pulumi_object_field(
                    "virtualNetworkRules",
                    &self.r#virtual_network_rules,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventHubNamespaceNetworkRulesets {
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
                    r#default_action: {
                        let field_value = match fields_map.get("defaultAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_rules: {
                        let field_value = match fields_map.get("ipRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_network_access_enabled: {
                        let field_value = match fields_map.get("publicNetworkAccessEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicNetworkAccessEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_service_access_enabled: {
                        let field_value = match fields_map.get("trustedServiceAccessEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'trustedServiceAccessEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_rules: {
                        let field_value = match fields_map.get("virtualNetworkRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualNetworkRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
