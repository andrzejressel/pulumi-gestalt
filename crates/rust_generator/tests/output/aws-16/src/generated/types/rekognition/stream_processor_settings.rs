#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamProcessorSettings {
    /// Label detection settings to use on a streaming video. See `connected_home`.
    #[builder(into)]
    #[serde(rename = "connectedHome")]
    pub r#connected_home: Option<Box<super::super::types::rekognition::StreamProcessorSettingsConnectedHome>>,
    /// Input face recognition parameters for an Amazon Rekognition stream processor. See `face_search`.
    #[builder(into)]
    #[serde(rename = "faceSearch")]
    pub r#face_search: Option<Box<super::super::types::rekognition::StreamProcessorSettingsFaceSearch>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamProcessorSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("connected_home".to_string(), self.r#connected_home.to_pulumi_value().await);
            map.insert("face_search".to_string(), self.r#face_search.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamProcessorSettings {
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
                    r#connected_home: {
                        let field_value = match fields_map.get("connected_home") {
                            Some(value) => value,
                            None => bail!("Missing field 'connected_home' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::rekognition::StreamProcessorSettingsConnectedHome>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#face_search: {
                        let field_value = match fields_map.get("face_search") {
                            Some(value) => value,
                            None => bail!("Missing field 'face_search' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::rekognition::StreamProcessorSettingsFaceSearch>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
