#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DirectoryWorkspaceAccessProperties {
    /// Indicates whether users can use Android devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeAndroid")]
    pub r#device_type_android: Option<String>,
    /// Indicates whether users can use Chromebooks to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeChromeos")]
    pub r#device_type_chromeos: Option<String>,
    /// Indicates whether users can use iOS devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeIos")]
    pub r#device_type_ios: Option<String>,
    /// Indicates whether users can use Linux clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeLinux")]
    pub r#device_type_linux: Option<String>,
    /// Indicates whether users can use macOS clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeOsx")]
    pub r#device_type_osx: Option<String>,
    /// Indicates whether users can access their WorkSpaces through a web browser.
    #[builder(into)]
    #[serde(rename = "deviceTypeWeb")]
    pub r#device_type_web: Option<String>,
    /// Indicates whether users can use Windows clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeWindows")]
    pub r#device_type_windows: Option<String>,
    /// Indicates whether users can use zero client devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeZeroclient")]
    pub r#device_type_zeroclient: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DirectoryWorkspaceAccessProperties {
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
                "device_type_android".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_android,
                )
                .await,
            );
            map.insert(
                "device_type_chromeos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_chromeos,
                )
                .await,
            );
            map.insert(
                "device_type_ios".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_ios,
                )
                .await,
            );
            map.insert(
                "device_type_linux".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_linux,
                )
                .await,
            );
            map.insert(
                "device_type_osx".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_osx,
                )
                .await,
            );
            map.insert(
                "device_type_web".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_web,
                )
                .await,
            );
            map.insert(
                "device_type_windows".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_windows,
                )
                .await,
            );
            map.insert(
                "device_type_zeroclient".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type_zeroclient,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DirectoryWorkspaceAccessProperties {
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
                    r#device_type_android: {
                        let field_value = match fields_map.get("device_type_android") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_android' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type_chromeos: {
                        let field_value = match fields_map.get("device_type_chromeos") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_chromeos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type_ios: {
                        let field_value = match fields_map.get("device_type_ios") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_ios' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type_linux: {
                        let field_value = match fields_map.get("device_type_linux") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_linux' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type_osx: {
                        let field_value = match fields_map.get("device_type_osx") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_osx' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type_web: {
                        let field_value = match fields_map.get("device_type_web") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_web' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type_windows: {
                        let field_value = match fields_map.get("device_type_windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type_zeroclient: {
                        let field_value = match fields_map.get("device_type_zeroclient") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type_zeroclient' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
