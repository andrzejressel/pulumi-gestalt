#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionHandler {
    /// Actions to take when the user is not logged in.
    /// Possible values are: `AUTH_FAIL_ACTION_REDIRECT`, `AUTH_FAIL_ACTION_UNAUTHORIZED`.
    #[builder(into)]
    #[serde(rename = "authFailAction")]
    pub r#auth_fail_action: Option<String>,
    /// Methods to restrict access to a URL based on login status.
    /// Possible values are: `LOGIN_OPTIONAL`, `LOGIN_ADMIN`, `LOGIN_REQUIRED`.
    #[builder(into)]
    #[serde(rename = "login")]
    pub r#login: Option<String>,
    /// 30x code to use when performing redirects for the secure field.
    /// Possible values are: `REDIRECT_HTTP_RESPONSE_CODE_301`, `REDIRECT_HTTP_RESPONSE_CODE_302`, `REDIRECT_HTTP_RESPONSE_CODE_303`, `REDIRECT_HTTP_RESPONSE_CODE_307`.
    #[builder(into)]
    #[serde(rename = "redirectHttpResponseCode")]
    pub r#redirect_http_response_code: Option<String>,
    /// Executes a script to handle the requests that match this URL pattern.
    /// Only the auto value is supported for Node.js in the App Engine standard environment, for example "script:" "auto".
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Option<Box<super::super::types::appengine::FlexibleAppVersionHandlerScript>>,
    /// Security (HTTPS) enforcement for this URL.
    /// Possible values are: `SECURE_DEFAULT`, `SECURE_NEVER`, `SECURE_OPTIONAL`, `SECURE_ALWAYS`.
    #[builder(into)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    /// Files served directly to the user for a given URL, such as images, CSS stylesheets, or JavaScript source files.
    /// Static file handlers describe which files in the application directory are static files, and which URLs serve them.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "staticFiles")]
    pub r#static_files: Option<Box<super::super::types::appengine::FlexibleAppVersionHandlerStaticFiles>>,
    /// URL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings.
    /// All URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path.
    #[builder(into)]
    #[serde(rename = "urlRegex")]
    pub r#url_regex: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionHandler {
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
                    "auth_fail_action",
                    &self.r#auth_fail_action,
                ),
                to_pulumi_object_field(
                    "login",
                    &self.r#login,
                ),
                to_pulumi_object_field(
                    "redirect_http_response_code",
                    &self.r#redirect_http_response_code,
                ),
                to_pulumi_object_field(
                    "script",
                    &self.r#script,
                ),
                to_pulumi_object_field(
                    "security_level",
                    &self.r#security_level,
                ),
                to_pulumi_object_field(
                    "static_files",
                    &self.r#static_files,
                ),
                to_pulumi_object_field(
                    "url_regex",
                    &self.r#url_regex,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionHandler {
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
                    r#redirect_http_response_code: {
                        let field_value = match fields_map.get("redirect_http_response_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_http_response_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#static_files: {
                        let field_value = match fields_map.get("static_files") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_regex: {
                        let field_value = match fields_map.get("url_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
