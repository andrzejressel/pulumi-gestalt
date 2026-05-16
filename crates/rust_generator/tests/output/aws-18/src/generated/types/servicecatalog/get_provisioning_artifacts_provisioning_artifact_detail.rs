#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetProvisioningArtifactsProvisioningArtifactDetail {
    /// Indicates whether the product version is active.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: bool,
    /// The UTC time stamp of the creation time.
    #[builder(into)]
    #[serde(rename = "createdTime")]
    pub r#created_time: String,
    /// The description of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Information set by the administrator to provide guidance to end users about which provisioning artifacts to use.
    #[builder(into)]
    #[serde(rename = "guidance")]
    pub r#guidance: String,
    /// The identifier of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type of provisioning artifact.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetProvisioningArtifactsProvisioningArtifactDetail {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("active".to_string(), self.r#active.to_pulumi_value().await);
            map.insert("created_time".to_string(), self.r#created_time.to_pulumi_value().await);
            map.insert("description".to_string(), self.r#description.to_pulumi_value().await);
            map.insert("guidance".to_string(), self.r#guidance.to_pulumi_value().await);
            map.insert("id".to_string(), self.r#id.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetProvisioningArtifactsProvisioningArtifactDetail {
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
                    r#active: {
                        let field_value = match fields_map.get("active") {
                            Some(value) => value,
                            None => bail!("Missing field 'active' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#created_time: {
                        let field_value = match fields_map.get("created_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#guidance: {
                        let field_value = match fields_map.get("guidance") {
                            Some(value) => value,
                            None => bail!("Missing field 'guidance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
