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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "media_package_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#media_package_settings,
                )
                .await,
            );
            map.insert(
                "multiplex_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#multiplex_settings,
                )
                .await,
            );
            map.insert(
                "settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#settings,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelDestination {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#media_package_settings: {
                        let field_value = match fields_map.get("media_package_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'media_package_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiplex_settings: {
                        let field_value = match fields_map.get("multiplex_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplex_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#settings: {
                        let field_value = match fields_map.get("settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
