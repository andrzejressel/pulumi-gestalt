#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchConfigurationRootBlockDevice {
    /// Whether the EBS Volume will be deleted on instance termination.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: bool,
    /// Whether the volume is Encrypted.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: bool,
    /// Provisioned IOPs of the volume.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: i32,
    /// Throughput of the volume.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: i32,
    /// Size of the volume.
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: i32,
    /// Type of the volume.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchConfigurationRootBlockDevice {
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
                "delete_on_termination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#delete_on_termination,
                )
                .await,
            );
            map.insert(
                "encrypted".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encrypted,
                )
                .await,
            );
            map.insert(
                "iops".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#iops,
                )
                .await,
            );
            map.insert(
                "throughput".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#throughput,
                )
                .await,
            );
            map.insert(
                "volume_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volume_size,
                )
                .await,
            );
            map.insert(
                "volume_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volume_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLaunchConfigurationRootBlockDevice {
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
                    r#delete_on_termination: {
                        let field_value = match fields_map.get("delete_on_termination") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_on_termination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encrypted: {
                        let field_value = match fields_map.get("encrypted") {
                            Some(value) => value,
                            None => bail!("Missing field 'encrypted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iops: {
                        let field_value = match fields_map.get("iops") {
                            Some(value) => value,
                            None => bail!("Missing field 'iops' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throughput: {
                        let field_value = match fields_map.get("throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_size: {
                        let field_value = match fields_map.get("volume_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_type: {
                        let field_value = match fields_map.get("volume_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
