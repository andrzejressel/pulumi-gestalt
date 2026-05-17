#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardAppVersionHandlerStaticFiles {
    /// Whether files should also be uploaded as code data. By default, files declared in static file handlers are uploaded as
    /// static data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged
    /// against both your code and static data storage resource quotas.
    #[builder(into)]
    #[serde(rename = "applicationReadable")]
    pub r#application_readable: Option<bool>,
    /// Time a static file served by this handler should be cached by web proxies and browsers.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example "3.5s".
    #[builder(into)]
    #[serde(rename = "expiration")]
    pub r#expiration: Option<String>,
    /// HTTP headers to use for all responses from these URLs.
    /// An object containing a list of "key:value" value pairs.".
    #[builder(into)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Option<std::collections::HashMap<String, String>>,
    /// MIME type used to serve all files served by this handler.
    /// Defaults to file-specific MIME types, which are derived from each file's filename extension.
    #[builder(into)]
    #[serde(rename = "mimeType")]
    pub r#mime_type: Option<String>,
    /// Path to the static files matched by the URL pattern, from the application root directory. The path can refer to text matched in groupings in the URL pattern.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Whether this handler should match the request if the file referenced by the handler does not exist.
    #[builder(into)]
    #[serde(rename = "requireMatchingFile")]
    pub r#require_matching_file: Option<bool>,
    /// Regular expression that matches the file paths for all files that should be referenced by this handler.
    #[builder(into)]
    #[serde(rename = "uploadPathRegex")]
    pub r#upload_path_regex: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StandardAppVersionHandlerStaticFiles {
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
                "application_readable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_readable,
                )
                .await,
            );
            map.insert(
                "expiration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiration,
                )
                .await,
            );
            map.insert(
                "http_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_headers,
                )
                .await,
            );
            map.insert(
                "mime_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mime_type,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "require_matching_file".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_matching_file,
                )
                .await,
            );
            map.insert(
                "upload_path_regex".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upload_path_regex,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StandardAppVersionHandlerStaticFiles {
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
                    r#application_readable: {
                        let field_value = match fields_map.get("application_readable") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_readable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiration: {
                        let field_value = match fields_map.get("expiration") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_headers: {
                        let field_value = match fields_map.get("http_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mime_type: {
                        let field_value = match fields_map.get("mime_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'mime_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_matching_file: {
                        let field_value = match fields_map.get("require_matching_file") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_matching_file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upload_path_regex: {
                        let field_value = match fields_map.get("upload_path_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'upload_path_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
