#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExtensionsInstanceConfig {
    /// List of extension events selected by consumer that extension is allowed to
    /// emit, identified by their types.
    #[builder(into)]
    #[serde(rename = "allowedEventTypes")]
    pub r#allowed_event_types: Option<Vec<String>>,
    /// (Output)
    /// The time at which the Extension Instance Config was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Option<String>,
    /// Fully qualified Eventarc resource name that consumers should use for event triggers.
    #[builder(into)]
    #[serde(rename = "eventarcChannel")]
    pub r#eventarc_channel: Option<String>,
    /// The ref of the Extension from the Registry (e.g. publisher-id/awesome-extension)
    #[builder(into)]
    #[serde(rename = "extensionRef")]
    pub r#extension_ref: String,
    /// The version of the Extension from the Registry (e.g. 1.0.3). If left blank, latest is assumed.
    #[builder(into)]
    #[serde(rename = "extensionVersion")]
    pub r#extension_version: Option<String>,
    /// (Output)
    /// The unique identifier for this configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Environment variables that may be configured for the Extension
    #[builder(into)]
    #[serde(rename = "params")]
    pub r#params: std::collections::HashMap<String, String>,
    /// (Output)
    /// Postinstall instructions to be shown for this Extension, with
    /// template strings representing function and parameter values substituted
    /// with actual values. These strings include: ${param:FOO},
    /// ${function:myFunc.url},
    /// ${function:myFunc.name}, and ${function:myFunc.location}
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "populatedPostinstallContent")]
    pub r#populated_postinstall_content: Option<String>,
    /// Params whose values are only available at deployment time.
    /// Unlike other params, these will not be set as environment variables on
    /// functions. See a full list of system parameters at
    /// https://firebase.google.com/docs/extensions/publishers/parameters#system_parameters
    #[builder(into)]
    #[serde(rename = "systemParams")]
    pub r#system_params: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExtensionsInstanceConfig {
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
                    "allowed_event_types",
                    &self.r#allowed_event_types,
                ),
                to_pulumi_object_field(
                    "create_time",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "eventarc_channel",
                    &self.r#eventarc_channel,
                ),
                to_pulumi_object_field(
                    "extension_ref",
                    &self.r#extension_ref,
                ),
                to_pulumi_object_field(
                    "extension_version",
                    &self.r#extension_version,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "params",
                    &self.r#params,
                ),
                to_pulumi_object_field(
                    "populated_postinstall_content",
                    &self.r#populated_postinstall_content,
                ),
                to_pulumi_object_field(
                    "system_params",
                    &self.r#system_params,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExtensionsInstanceConfig {
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
                    r#allowed_event_types: {
                        let field_value = match fields_map.get("allowed_event_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_event_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eventarc_channel: {
                        let field_value = match fields_map.get("eventarc_channel") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventarc_channel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extension_ref: {
                        let field_value = match fields_map.get("extension_ref") {
                            Some(value) => value,
                            None => bail!("Missing field 'extension_ref' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extension_version: {
                        let field_value = match fields_map.get("extension_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'extension_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#params: {
                        let field_value = match fields_map.get("params") {
                            Some(value) => value,
                            None => bail!("Missing field 'params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#populated_postinstall_content: {
                        let field_value = match fields_map.get("populated_postinstall_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'populated_postinstall_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_params: {
                        let field_value = match fields_map.get("system_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'system_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
