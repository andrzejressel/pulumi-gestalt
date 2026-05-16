#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity {
    /// A number that CloudFront uses as the value for the `max-age` directive in the `Strict-Transport-Security` HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlMaxAgeSec")]
    pub r#access_control_max_age_sec: i32,
    /// Whether CloudFront includes the `includeSubDomains` directive in the `Strict-Transport-Security` HTTP response header.
    #[builder(into)]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<bool>,
    /// Whether CloudFront overrides the `Strict-Transport-Security` HTTP response header received from the origin with the one specified in this response headers policy.
    #[builder(into)]
    #[serde(rename = "override")]
    pub r#override_: bool,
    /// Whether CloudFront includes the `preload` directive in the `Strict-Transport-Security` HTTP response header.
    #[builder(into)]
    #[serde(rename = "preload")]
    pub r#preload: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("access_control_max_age_sec".to_string(), self.r#access_control_max_age_sec.to_pulumi_value().await);
            map.insert("include_subdomains".to_string(), self.r#include_subdomains.to_pulumi_value().await);
            map.insert("override_".to_string(), self.r#override_.to_pulumi_value().await);
            map.insert("preload".to_string(), self.r#preload.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity {
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
                    r#access_control_max_age_sec: {
                        let field_value = match fields_map.get("access_control_max_age_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_max_age_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#include_subdomains: {
                        let field_value = match fields_map.get("include_subdomains") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_subdomains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#override_: {
                        let field_value = match fields_map.get("override_") {
                            Some(value) => value,
                            None => bail!("Missing field 'override_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#preload: {
                        let field_value = match fields_map.get("preload") {
                            Some(value) => value,
                            None => bail!("Missing field 'preload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
