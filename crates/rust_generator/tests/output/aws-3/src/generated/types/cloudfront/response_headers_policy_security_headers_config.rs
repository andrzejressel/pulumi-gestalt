#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponseHeadersPolicySecurityHeadersConfig {
    /// The policy directives and their values that CloudFront includes as values for the `Content-Security-Policy` HTTP response header. See Content Security Policy for more information.
    #[builder(into)]
    #[serde(rename = "contentSecurityPolicy")]
    pub r#content_security_policy: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy>>,
    /// Determines whether CloudFront includes the `X-Content-Type-Options` HTTP response header with its value set to `nosniff`. See Content Type Options for more information.
    #[builder(into)]
    #[serde(rename = "contentTypeOptions")]
    pub r#content_type_options: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigContentTypeOptions>>,
    /// Determines whether CloudFront includes the `X-Frame-Options` HTTP response header and the header’s value. See Frame Options for more information.
    #[builder(into)]
    #[serde(rename = "frameOptions")]
    pub r#frame_options: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigFrameOptions>>,
    /// Determines whether CloudFront includes the `Referrer-Policy` HTTP response header and the header’s value. See Referrer Policy for more information.
    #[builder(into)]
    #[serde(rename = "referrerPolicy")]
    pub r#referrer_policy: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigReferrerPolicy>>,
    /// Determines whether CloudFront includes the `Strict-Transport-Security` HTTP response header and the header’s value. See Strict Transport Security for more information.
    #[builder(into)]
    #[serde(rename = "strictTransportSecurity")]
    pub r#strict_transport_security: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity>>,
    /// Determine whether CloudFront includes the `X-XSS-Protection` HTTP response header and the header’s value. See XSS Protection for more information.
    #[builder(into)]
    #[serde(rename = "xssProtection")]
    pub r#xss_protection: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigXssProtection>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResponseHeadersPolicySecurityHeadersConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "content_security_policy",
                    &self.r#content_security_policy,
                ),
                to_pulumi_object_field(
                    "content_type_options",
                    &self.r#content_type_options,
                ),
                to_pulumi_object_field(
                    "frame_options",
                    &self.r#frame_options,
                ),
                to_pulumi_object_field(
                    "referrer_policy",
                    &self.r#referrer_policy,
                ),
                to_pulumi_object_field(
                    "strict_transport_security",
                    &self.r#strict_transport_security,
                ),
                to_pulumi_object_field(
                    "xss_protection",
                    &self.r#xss_protection,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResponseHeadersPolicySecurityHeadersConfig {
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
                    r#content_security_policy: {
                        let field_value = match fields_map.get("content_security_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_security_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_type_options: {
                        let field_value = match fields_map.get("content_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frame_options: {
                        let field_value = match fields_map.get("frame_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'frame_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#referrer_policy: {
                        let field_value = match fields_map.get("referrer_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'referrer_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strict_transport_security: {
                        let field_value = match fields_map.get("strict_transport_security") {
                            Some(value) => value,
                            None => bail!("Missing field 'strict_transport_security' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#xss_protection: {
                        let field_value = match fields_map.get("xss_protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'xss_protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
