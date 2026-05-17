#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoadBalancerRuleOverrideSessionAffinityAttribute {
    /// Configures the HTTP header names to use when header session affinity is enabled.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<String>>,
    /// Configures how headers are used when header session affinity is enabled. Set to true to require all headers to be present on requests in order for sessions to be created or false to require at least one header to be present. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Option<bool>,
    /// Configures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set `secure="Never"`. Available values: `Auto`, `Lax`, `None`, `Strict`.
    #[builder(into)]
    #[serde(rename = "samesite")]
    pub r#samesite: Option<String>,
    /// Configures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`.
    #[builder(into)]
    #[serde(rename = "secure")]
    pub r#secure: Option<String>,
    /// Configures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`.
    #[builder(into)]
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LoadBalancerRuleOverrideSessionAffinityAttribute {
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
                "headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers,
                )
                .await,
            );
            map.insert(
                "require_all_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_all_headers,
                )
                .await,
            );
            map.insert(
                "samesite".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#samesite,
                )
                .await,
            );
            map.insert(
                "secure".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secure,
                )
                .await,
            );
            map.insert(
                "zero_downtime_failover".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zero_downtime_failover,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LoadBalancerRuleOverrideSessionAffinityAttribute {
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
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_all_headers: {
                        let field_value = match fields_map.get("require_all_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_all_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#samesite: {
                        let field_value = match fields_map.get("samesite") {
                            Some(value) => value,
                            None => bail!("Missing field 'samesite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secure: {
                        let field_value = match fields_map.get("secure") {
                            Some(value) => value,
                            None => bail!("Missing field 'secure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zero_downtime_failover: {
                        let field_value = match fields_map.get("zero_downtime_failover") {
                            Some(value) => value,
                            None => bail!("Missing field 'zero_downtime_failover' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
