#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SingleNodeVirtualInstanceSingleServerConfigurationDiskVolumeConfiguration {
    /// The total number of disks required for the concerned volume. Possible values are at least `1`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "numberOfDisks")]
    pub r#number_of_disks: i32,
    /// The size of the Disk in GB. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: i32,
    /// The name of the Disk SKU. Possible values are `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "skuName")]
    pub r#sku_name: String,
    /// Specifies the volumn name of the database disk. Possible values are `backup`, `hana/data`, `hana/log`, `hana/shared`, `os` and `usr/sap`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SingleNodeVirtualInstanceSingleServerConfigurationDiskVolumeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "number_of_disks",
                    &self.r#number_of_disks,
                ),
                to_pulumi_object_field(
                    "size_in_gb",
                    &self.r#size_in_gb,
                ),
                to_pulumi_object_field(
                    "sku_name",
                    &self.r#sku_name,
                ),
                to_pulumi_object_field(
                    "volume_name",
                    &self.r#volume_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SingleNodeVirtualInstanceSingleServerConfigurationDiskVolumeConfiguration {
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
                    r#number_of_disks: {
                        let field_value = match fields_map.get("number_of_disks") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_disks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_in_gb: {
                        let field_value = match fields_map.get("size_in_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_in_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sku_name: {
                        let field_value = match fields_map.get("sku_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'sku_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_name: {
                        let field_value = match fields_map.get("volume_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
