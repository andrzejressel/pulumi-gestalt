#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnterpriseKeyWebSettings {
    /// If set to true, it means allowed_domains will not be enforced.
    #[builder(into)]
    #[serde(rename = "allowAllDomains")]
    pub r#allow_all_domains: Option<bool>,
    /// If set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type.
    #[builder(into)]
    #[serde(rename = "allowAmpTraffic")]
    pub r#allow_amp_traffic: Option<bool>,
    /// Domains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'
    #[builder(into)]
    #[serde(rename = "allowedDomains")]
    pub r#allowed_domains: Option<Vec<String>>,
    /// Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE. Possible values: CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED, USABILITY, BALANCE, SECURITY
    #[builder(into)]
    #[serde(rename = "challengeSecurityPreference")]
    pub r#challenge_security_preference: Option<String>,
    /// Required. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE
    #[builder(into)]
    #[serde(rename = "integrationType")]
    pub r#integration_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnterpriseKeyWebSettings {
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
                    "allow_all_domains",
                    &self.r#allow_all_domains,
                ),
                to_pulumi_object_field(
                    "allow_amp_traffic",
                    &self.r#allow_amp_traffic,
                ),
                to_pulumi_object_field(
                    "allowed_domains",
                    &self.r#allowed_domains,
                ),
                to_pulumi_object_field(
                    "challenge_security_preference",
                    &self.r#challenge_security_preference,
                ),
                to_pulumi_object_field(
                    "integration_type",
                    &self.r#integration_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EnterpriseKeyWebSettings {
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
                    r#allow_all_domains: {
                        let field_value = match fields_map.get("allow_all_domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_all_domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_amp_traffic: {
                        let field_value = match fields_map.get("allow_amp_traffic") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_amp_traffic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_domains: {
                        let field_value = match fields_map.get("allowed_domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#challenge_security_preference: {
                        let field_value = match fields_map.get("challenge_security_preference") {
                            Some(value) => value,
                            None => bail!("Missing field 'challenge_security_preference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integration_type: {
                        let field_value = match fields_map.get("integration_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'integration_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
