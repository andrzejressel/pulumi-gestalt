#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserProfileUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings {
    /// The default size of the EBS storage volume for a private space.
    #[builder(into)]
    #[serde(rename = "defaultEbsVolumeSizeInGb")]
    pub r#default_ebs_volume_size_in_gb: i32,
    /// The maximum size of the EBS storage volume for a private space.
    #[builder(into)]
    #[serde(rename = "maximumEbsVolumeSizeInGb")]
    pub r#maximum_ebs_volume_size_in_gb: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserProfileUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("default_ebs_volume_size_in_gb".to_string(), self.r#default_ebs_volume_size_in_gb.to_pulumi_value().await);
            map.insert("maximum_ebs_volume_size_in_gb".to_string(), self.r#maximum_ebs_volume_size_in_gb.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserProfileUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings {
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
                    r#default_ebs_volume_size_in_gb: {
                        let field_value = match fields_map.get("default_ebs_volume_size_in_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ebs_volume_size_in_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#maximum_ebs_volume_size_in_gb: {
                        let field_value = match fields_map.get("maximum_ebs_volume_size_in_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_ebs_volume_size_in_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
