#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnterpriseKeyWebSettings {
    /// If set to true, it means allowed_domains will not be enforced.
    #[builder(into)]
    pub r#allow_all_domains: Option<bool>,
    /// If set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type.
    #[builder(into)]
    pub r#allow_amp_traffic: Option<bool>,
    /// Domains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'
    #[builder(into)]
    pub r#allowed_domains: Option<Vec<String>>,
    /// Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE. Possible values: CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED, USABILITY, BALANCE, SECURITY
    #[builder(into)]
    pub r#challenge_security_preference: Option<String>,
    /// Required. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE
    #[builder(into)]
    pub r#integration_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnterpriseKeyWebSettings {
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
                    "allowAllDomains",
                    &self.r#allow_all_domains,
                ),
                to_pulumi_object_field(
                    "allowAmpTraffic",
                    &self.r#allow_amp_traffic,
                ),
                to_pulumi_object_field(
                    "allowedDomains",
                    &self.r#allowed_domains,
                ),
                to_pulumi_object_field(
                    "challengeSecurityPreference",
                    &self.r#challenge_security_preference,
                ),
                to_pulumi_object_field(
                    "integrationType",
                    &self.r#integration_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("allowAllDomains") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowAllDomains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_amp_traffic: {
                        let field_value = match fields_map.get("allowAmpTraffic") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowAmpTraffic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_domains: {
                        let field_value = match fields_map.get("allowedDomains") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedDomains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#challenge_security_preference: {
                        let field_value = match fields_map.get("challengeSecurityPreference") {
                            Some(value) => value,
                            None => bail!("Missing field 'challengeSecurityPreference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integration_type: {
                        let field_value = match fields_map.get("integrationType") {
                            Some(value) => value,
                            None => bail!("Missing field 'integrationType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
