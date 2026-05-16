#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRuntimeVersionsRuntimeVersion {
    /// Date of deprecation if the runtme version is deprecated.
    #[builder(into)]
    #[serde(rename = "deprecationDate")]
    pub r#deprecation_date: String,
    /// Description of the runtime version, created by Amazon.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Date that the runtime version was released.
    #[builder(into)]
    #[serde(rename = "releaseDate")]
    pub r#release_date: String,
    /// Name of the runtime version.
    /// For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
    #[builder(into)]
    #[serde(rename = "versionName")]
    pub r#version_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRuntimeVersionsRuntimeVersion {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("deprecation_date".to_string(), self.r#deprecation_date.to_pulumi_value().await);
            map.insert("description".to_string(), self.r#description.to_pulumi_value().await);
            map.insert("release_date".to_string(), self.r#release_date.to_pulumi_value().await);
            map.insert("version_name".to_string(), self.r#version_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRuntimeVersionsRuntimeVersion {
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
                    r#deprecation_date: {
                        let field_value = match fields_map.get("deprecation_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'deprecation_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#release_date: {
                        let field_value = match fields_map.get("release_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'release_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#version_name: {
                        let field_value = match fields_map.get("version_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
