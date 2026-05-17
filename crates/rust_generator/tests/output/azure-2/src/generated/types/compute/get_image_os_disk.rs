#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetImageOsDisk {
    /// the URI in Azure storage of the blob used to create the image.
    #[builder(into)]
    #[serde(rename = "blobUri")]
    pub r#blob_uri: String,
    /// the caching mode for the Data Disk, such as `ReadWrite`, `ReadOnly`, or `None`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: String,
    /// the ID of the Managed Disk used as the Data Disk Image.
    #[builder(into)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: String,
    /// the State of the OS used in the Image, such as `Generalized`.
    #[builder(into)]
    #[serde(rename = "osState")]
    pub r#os_state: String,
    /// the type of Operating System used on the OS Disk. such as `Linux` or `Windows`.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: String,
    /// the size of this Data Disk in GB.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetImageOsDisk {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "blob_uri",
                    &self.r#blob_uri,
                ),
                to_pulumi_object_field(
                    "caching",
                    &self.r#caching,
                ),
                to_pulumi_object_field(
                    "managed_disk_id",
                    &self.r#managed_disk_id,
                ),
                to_pulumi_object_field(
                    "os_state",
                    &self.r#os_state,
                ),
                to_pulumi_object_field(
                    "os_type",
                    &self.r#os_type,
                ),
                to_pulumi_object_field(
                    "size_gb",
                    &self.r#size_gb,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetImageOsDisk {
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
                    r#blob_uri: {
                        let field_value = match fields_map.get("blob_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'blob_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caching: {
                        let field_value = match fields_map.get("caching") {
                            Some(value) => value,
                            None => bail!("Missing field 'caching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_disk_id: {
                        let field_value = match fields_map.get("managed_disk_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_disk_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_state: {
                        let field_value = match fields_map.get("os_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_type: {
                        let field_value = match fields_map.get("os_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_gb: {
                        let field_value = match fields_map.get("size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
