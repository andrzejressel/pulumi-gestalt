#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResponseHeadersPolicySecurityHeadersConfig {
    /// The policy directives and their values that CloudFront includes as values for the Content-Security-Policy HTTP response header.
    #[builder(into)]
    #[serde(rename = "contentSecurityPolicies")]
    pub r#content_security_policies: Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy>,
    /// A setting that determines whether CloudFront includes the X-Content-Type-Options HTTP response header with its value set to nosniff. See Content Type Options for more information.
    #[builder(into)]
    #[serde(rename = "contentTypeOptions")]
    pub r#content_type_options: Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigContentTypeOption>,
    /// Setting that determines whether CloudFront includes the X-Frame-Options HTTP response header and the header’s value. See Frame Options for more information.
    #[builder(into)]
    #[serde(rename = "frameOptions")]
    pub r#frame_options: Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigFrameOption>,
    /// Value of the Referrer-Policy HTTP response header. Valid Values: `no-referrer` | `no-referrer-when-downgrade` | `origin` | `origin-when-cross-origin` | `same-origin` | `strict-origin` | `strict-origin-when-cross-origin` | `unsafe-url`
    #[builder(into)]
    #[serde(rename = "referrerPolicies")]
    pub r#referrer_policies: Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigReferrerPolicy>,
    /// Settings that determine whether CloudFront includes the Strict-Transport-Security HTTP response header and the header’s value. See Strict Transport Security for more information.
    #[builder(into)]
    #[serde(rename = "strictTransportSecurities")]
    pub r#strict_transport_securities: Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity>,
    /// Settings that determine whether CloudFront includes the X-XSS-Protection HTTP response header and the header’s value. See XSS Protection for more information.
    #[builder(into)]
    #[serde(rename = "xssProtections")]
    pub r#xss_protections: Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigXssProtection>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResponseHeadersPolicySecurityHeadersConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("content_security_policies".to_string(), self.r#content_security_policies.to_pulumi_value().await);
            map.insert("content_type_options".to_string(), self.r#content_type_options.to_pulumi_value().await);
            map.insert("frame_options".to_string(), self.r#frame_options.to_pulumi_value().await);
            map.insert("referrer_policies".to_string(), self.r#referrer_policies.to_pulumi_value().await);
            map.insert("strict_transport_securities".to_string(), self.r#strict_transport_securities.to_pulumi_value().await);
            map.insert("xss_protections".to_string(), self.r#xss_protections.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResponseHeadersPolicySecurityHeadersConfig {
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
                    r#content_security_policies: {
                        let field_value = match fields_map.get("content_security_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_security_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#content_type_options: {
                        let field_value = match fields_map.get("content_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigContentTypeOption> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#frame_options: {
                        let field_value = match fields_map.get("frame_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigFrameOption> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#referrer_policies: {
                        let field_value = match fields_map.get("referrer_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'referrer_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigReferrerPolicy> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#strict_transport_securities: {
                        let field_value = match fields_map.get("strict_transport_securities") {
                            Some(value) => value,
                            None => bail!("Missing field 'strict_transport_securities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#xss_protections: {
                        let field_value = match fields_map.get("xss_protections") {
                            Some(value) => value,
                            None => bail!("Missing field 'xss_protections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigXssProtection> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
