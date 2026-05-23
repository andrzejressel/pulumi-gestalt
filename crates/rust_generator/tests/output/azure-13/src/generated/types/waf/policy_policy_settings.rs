#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyPolicySettings {
    /// Describes if the policy is in enabled state or disabled state. Defaults to `true`.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// Whether the firewall should block a request with upload size greater then `file_upload_limit_in_mb`.
    #[builder(into)]
    pub r#file_upload_enforcement: Option<bool>,
    /// The File Upload Limit in MB. Accepted values are in the range `1` to `4000`. Defaults to `100`.
    #[builder(into)]
    pub r#file_upload_limit_in_mb: Option<i32>,
    /// Specifies the JavaScript challenge cookie validity lifetime in minutes. The user is challenged after the lifetime expires. Accepted values are in the range `5` to `1440`. Defaults to `30`.
    #[builder(into)]
    pub r#js_challenge_cookie_expiration_in_minutes: Option<i32>,
    /// One `log_scrubbing` block as defined below.
    #[builder(into)]
    pub r#log_scrubbing: Option<Box<super::super::types::waf::PolicyPolicySettingsLogScrubbing>>,
    /// The Maximum Request Body Size in KB. Accepted values are in the range `8` to `2000`. Defaults to `128`.
    #[builder(into)]
    pub r#max_request_body_size_in_kb: Option<i32>,
    /// Describes if it is in detection mode or prevention mode at the policy level. Valid values are `Detection` and `Prevention`. Defaults to `Prevention`.
    #[builder(into)]
    pub r#mode: Option<String>,
    /// Is Request Body Inspection enabled? Defaults to `true`.
    #[builder(into)]
    pub r#request_body_check: Option<bool>,
    /// Whether the firewall should block a request with body size greater then `max_request_body_size_in_kb`. Defaults to `true`.
    #[builder(into)]
    pub r#request_body_enforcement: Option<bool>,
    /// Specifies the maximum request body inspection limit in KB for the Web Application Firewall. Defaults to `128`.
    #[builder(into)]
    pub r#request_body_inspect_limit_in_kb: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyPolicySettings {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "fileUploadEnforcement",
                    &self.r#file_upload_enforcement,
                ),
                to_pulumi_object_field(
                    "fileUploadLimitInMb",
                    &self.r#file_upload_limit_in_mb,
                ),
                to_pulumi_object_field(
                    "jsChallengeCookieExpirationInMinutes",
                    &self.r#js_challenge_cookie_expiration_in_minutes,
                ),
                to_pulumi_object_field(
                    "logScrubbing",
                    &self.r#log_scrubbing,
                ),
                to_pulumi_object_field(
                    "maxRequestBodySizeInKb",
                    &self.r#max_request_body_size_in_kb,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "requestBodyCheck",
                    &self.r#request_body_check,
                ),
                to_pulumi_object_field(
                    "requestBodyEnforcement",
                    &self.r#request_body_enforcement,
                ),
                to_pulumi_object_field(
                    "requestBodyInspectLimitInKb",
                    &self.r#request_body_inspect_limit_in_kb,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyPolicySettings {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_upload_enforcement: {
                        let field_value = match fields_map.get("fileUploadEnforcement") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileUploadEnforcement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_upload_limit_in_mb: {
                        let field_value = match fields_map.get("fileUploadLimitInMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileUploadLimitInMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#js_challenge_cookie_expiration_in_minutes: {
                        let field_value = match fields_map.get("jsChallengeCookieExpirationInMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'jsChallengeCookieExpirationInMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_scrubbing: {
                        let field_value = match fields_map.get("logScrubbing") {
                            Some(value) => value,
                            None => bail!("Missing field 'logScrubbing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_request_body_size_in_kb: {
                        let field_value = match fields_map.get("maxRequestBodySizeInKb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRequestBodySizeInKb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_body_check: {
                        let field_value = match fields_map.get("requestBodyCheck") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestBodyCheck' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_body_enforcement: {
                        let field_value = match fields_map.get("requestBodyEnforcement") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestBodyEnforcement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_body_inspect_limit_in_kb: {
                        let field_value = match fields_map.get("requestBodyInspectLimitInKb") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestBodyInspectLimitInKb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
