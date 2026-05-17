#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptions {
    /// Name of the driver to use to create the volume
    #[builder(into)]
    #[serde(rename = "driverName")]
    pub r#driver_name: Option<String>,
    /// key/value map of driver specific options
    #[builder(into)]
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Option<std::collections::HashMap<String, String>>,
    /// User-defined key/value metadata
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<super::types::ServiceTaskSpecContainerSpecMountVolumeOptionsLabel>>,
    /// Populate volume with data from the target
    #[builder(into)]
    #[serde(rename = "noCopy")]
    pub r#no_copy: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTaskSpecContainerSpecMountVolumeOptions {
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
                "driver_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#driver_name,
                )
                .await,
            );
            map.insert(
                "driver_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#driver_options,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "no_copy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#no_copy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTaskSpecContainerSpecMountVolumeOptions {
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
                    r#driver_name: {
                        let field_value = match fields_map.get("driver_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'driver_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#driver_options: {
                        let field_value = match fields_map.get("driver_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'driver_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_copy: {
                        let field_value = match fields_map.get("no_copy") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_copy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
