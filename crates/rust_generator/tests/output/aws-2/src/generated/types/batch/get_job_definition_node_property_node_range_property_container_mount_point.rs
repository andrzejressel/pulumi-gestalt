#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerMountPoint {
    /// The absolute file path in the container where the tmpfs volume is mounted.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: String,
    /// If this value is true, the container has read-only access to the volume.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: bool,
    /// The name of the volume to mount.
    #[builder(into)]
    #[serde(rename = "sourceVolume")]
    pub r#source_volume: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerMountPoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("container_path".to_string(), self.r#container_path.to_pulumi_value().await);
            map.insert("read_only".to_string(), self.r#read_only.to_pulumi_value().await);
            map.insert("source_volume".to_string(), self.r#source_volume.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerMountPoint {
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
                    r#container_path: {
                        let field_value = match fields_map.get("container_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#read_only: {
                        let field_value = match fields_map.get("read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source_volume: {
                        let field_value = match fields_map.get("source_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
