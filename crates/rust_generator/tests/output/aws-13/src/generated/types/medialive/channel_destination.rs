#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelDestination {
    /// User-specified id. Ths is used in an output group or an output.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Destination settings for a MediaPackage output; one destination for both encoders. See Media Package Settings for more details.
    #[builder(into)]
    #[serde(rename = "mediaPackageSettings")]
    pub r#media_package_settings: Option<Vec<super::super::types::medialive::ChannelDestinationMediaPackageSetting>>,
    /// Destination settings for a Multiplex output; one destination for both encoders. See Multiplex Settings for more details.
    #[builder(into)]
    #[serde(rename = "multiplexSettings")]
    pub r#multiplex_settings: Option<Box<super::super::types::medialive::ChannelDestinationMultiplexSettings>>,
    /// Destination settings for a standard output; one destination for each redundant encoder. See Settings for more details.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Option<Vec<super::super::types::medialive::ChannelDestinationSetting>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelDestination {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("id".to_string(), self.r#id.to_pulumi_value().await);
            map.insert("media_package_settings".to_string(), self.r#media_package_settings.to_pulumi_value().await);
            map.insert("multiplex_settings".to_string(), self.r#multiplex_settings.to_pulumi_value().await);
            map.insert("settings".to_string(), self.r#settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelDestination {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#media_package_settings: {
                        let field_value = match fields_map.get("media_package_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'media_package_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelDestinationMediaPackageSetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#multiplex_settings: {
                        let field_value = match fields_map.get("multiplex_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplex_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelDestinationMultiplexSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#settings: {
                        let field_value = match fields_map.get("settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelDestinationSetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
