#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeMountOption {
    /// (Output)
    /// Export path of the volume.
    #[builder(into)]
    #[serde(rename = "export")]
    pub r#export: Option<String>,
    /// (Output)
    /// Full export path of the volume.
    /// Format for NFS volumes: `<export_ip>:/<shareName>`
    /// Format for SMB volumes: `\\\\netbios_prefix-four_random_hex_letters.domain_name\\shareName`
    #[builder(into)]
    #[serde(rename = "exportFull")]
    pub r#export_full: Option<String>,
    /// (Output)
    /// Human-readable mount instructions.
    #[builder(into)]
    #[serde(rename = "instructions")]
    pub r#instructions: Option<String>,
    /// (Output)
    /// Protocol to mount with.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeMountOption {
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
                "export".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#export,
                )
                .await,
            );
            map.insert(
                "export_full".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#export_full,
                )
                .await,
            );
            map.insert(
                "instructions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instructions,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeMountOption {
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
                    r#export: {
                        let field_value = match fields_map.get("export") {
                            Some(value) => value,
                            None => bail!("Missing field 'export' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#export_full: {
                        let field_value = match fields_map.get("export_full") {
                            Some(value) => value,
                            None => bail!("Missing field 'export_full' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instructions: {
                        let field_value = match fields_map.get("instructions") {
                            Some(value) => value,
                            None => bail!("Missing field 'instructions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
