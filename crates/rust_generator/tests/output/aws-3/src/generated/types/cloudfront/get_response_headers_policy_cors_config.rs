#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResponseHeadersPolicyCorsConfig {
    /// A Boolean value that CloudFront uses as the value for the Access-Control-Allow-Credentials HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowCredentials")]
    pub r#access_control_allow_credentials: bool,
    /// Object that contains an attribute `items` that contains a list of HTTP header names that CloudFront includes as values for the Access-Control-Allow-Headers HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowHeaders")]
    pub r#access_control_allow_headers: Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowHeader>,
    /// Object that contains an attribute `items` that contains a list of HTTP methods that CloudFront includes as values for the Access-Control-Allow-Methods HTTP response header. Valid values: `GET` | `POST` | `OPTIONS` | `PUT` | `DELETE` | `HEAD` | `ALL`
    #[builder(into)]
    #[serde(rename = "accessControlAllowMethods")]
    pub r#access_control_allow_methods: Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowMethod>,
    /// Object that contains an attribute `items` that contains a list of origins that CloudFront can use as the value for the Access-Control-Allow-Origin HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowOrigins")]
    pub r#access_control_allow_origins: Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowOrigin>,
    /// Object that contains an attribute `items` that contains a list of HTTP headers that CloudFront includes as values for the Access-Control-Expose-Headers HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlExposeHeaders")]
    pub r#access_control_expose_headers: Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlExposeHeader>,
    /// A number that CloudFront uses as the value for the max-age directive in the Strict-Transport-Security HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlMaxAgeSec")]
    pub r#access_control_max_age_sec: i32,
    #[builder(into)]
    #[serde(rename = "originOverride")]
    pub r#origin_override: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResponseHeadersPolicyCorsConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("access_control_allow_credentials".to_string(), self.r#access_control_allow_credentials.to_pulumi_value().await);
            map.insert("access_control_allow_headers".to_string(), self.r#access_control_allow_headers.to_pulumi_value().await);
            map.insert("access_control_allow_methods".to_string(), self.r#access_control_allow_methods.to_pulumi_value().await);
            map.insert("access_control_allow_origins".to_string(), self.r#access_control_allow_origins.to_pulumi_value().await);
            map.insert("access_control_expose_headers".to_string(), self.r#access_control_expose_headers.to_pulumi_value().await);
            map.insert("access_control_max_age_sec".to_string(), self.r#access_control_max_age_sec.to_pulumi_value().await);
            map.insert("origin_override".to_string(), self.r#origin_override.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResponseHeadersPolicyCorsConfig {
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
                    r#access_control_allow_credentials: {
                        let field_value = match fields_map.get("access_control_allow_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_headers: {
                        let field_value = match fields_map.get("access_control_allow_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowHeader> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_methods: {
                        let field_value = match fields_map.get("access_control_allow_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowMethod> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_origins: {
                        let field_value = match fields_map.get("access_control_allow_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowOrigin> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#access_control_expose_headers: {
                        let field_value = match fields_map.get("access_control_expose_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_expose_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlExposeHeader> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#access_control_max_age_sec: {
                        let field_value = match fields_map.get("access_control_max_age_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_max_age_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#origin_override: {
                        let field_value = match fields_map.get("origin_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
