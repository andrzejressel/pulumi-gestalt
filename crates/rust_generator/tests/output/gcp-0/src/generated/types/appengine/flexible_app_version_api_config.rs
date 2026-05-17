#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionApiConfig {
    /// Action to take when users access resources that require authentication.
    /// Default value is `AUTH_FAIL_ACTION_REDIRECT`.
    /// Possible values are: `AUTH_FAIL_ACTION_REDIRECT`, `AUTH_FAIL_ACTION_UNAUTHORIZED`.
    #[builder(into)]
    #[serde(rename = "authFailAction")]
    pub r#auth_fail_action: Option<String>,
    /// Level of login required to access this resource.
    /// Default value is `LOGIN_OPTIONAL`.
    /// Possible values are: `LOGIN_OPTIONAL`, `LOGIN_ADMIN`, `LOGIN_REQUIRED`.
    #[builder(into)]
    #[serde(rename = "login")]
    pub r#login: Option<String>,
    /// Path to the script from the application root directory.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: String,
    /// Security (HTTPS) enforcement for this URL.
    /// Possible values are: `SECURE_DEFAULT`, `SECURE_NEVER`, `SECURE_OPTIONAL`, `SECURE_ALWAYS`.
    #[builder(into)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    /// URL to serve the endpoint at.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionApiConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "auth_fail_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_fail_action,
                )
                .await,
            );
            map.insert(
                "login".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#login,
                )
                .await,
            );
            map.insert(
                "script".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#script,
                )
                .await,
            );
            map.insert(
                "security_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_level,
                )
                .await,
            );
            map.insert(
                "url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionApiConfig {
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
                    r#auth_fail_action: {
                        let field_value = match fields_map.get("auth_fail_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_fail_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login: {
                        let field_value = match fields_map.get("login") {
                            Some(value) => value,
                            None => bail!("Missing field 'login' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script: {
                        let field_value = match fields_map.get("script") {
                            Some(value) => value,
                            None => bail!("Missing field 'script' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_level: {
                        let field_value = match fields_map.get("security_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
