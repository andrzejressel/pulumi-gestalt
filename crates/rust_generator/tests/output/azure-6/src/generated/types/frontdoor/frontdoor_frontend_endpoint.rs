#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorFrontendEndpoint {
    /// Specifies the host name of the `frontend_endpoint`. Must be a domain name. In order to use a name.azurefd.net domain, the name value must match the Front Door name.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// The ID of the FrontDoor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the name of the `frontend_endpoint`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Whether to allow session affinity on this host. Valid options are `true` or `false` Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sessionAffinityEnabled")]
    pub r#session_affinity_enabled: Option<bool>,
    /// The TTL to use in seconds for session affinity, if applicable. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "sessionAffinityTtlSeconds")]
    pub r#session_affinity_ttl_seconds: Option<i32>,
    /// Defines the Web Application Firewall policy `ID` for each host.
    #[builder(into)]
    #[serde(rename = "webApplicationFirewallPolicyLinkId")]
    pub r#web_application_firewall_policy_link_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorFrontendEndpoint {
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
                    "host_name",
                    &self.r#host_name,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "session_affinity_enabled",
                    &self.r#session_affinity_enabled,
                ),
                to_pulumi_object_field(
                    "session_affinity_ttl_seconds",
                    &self.r#session_affinity_ttl_seconds,
                ),
                to_pulumi_object_field(
                    "web_application_firewall_policy_link_id",
                    &self.r#web_application_firewall_policy_link_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorFrontendEndpoint {
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
                    r#host_name: {
                        let field_value = match fields_map.get("host_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#session_affinity_enabled: {
                        let field_value = match fields_map.get("session_affinity_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_affinity_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_affinity_ttl_seconds: {
                        let field_value = match fields_map.get("session_affinity_ttl_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_affinity_ttl_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_application_firewall_policy_link_id: {
                        let field_value = match fields_map.get("web_application_firewall_policy_link_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_application_firewall_policy_link_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
