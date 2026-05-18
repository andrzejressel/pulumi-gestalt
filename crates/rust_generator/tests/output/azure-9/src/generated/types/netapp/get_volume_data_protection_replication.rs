#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVolumeDataProtectionReplication {
    /// The endpoint type.
    #[builder(into)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: String,
    /// Location of the primary volume.
    #[builder(into)]
    #[serde(rename = "remoteVolumeLocation")]
    pub r#remote_volume_location: String,
    /// Resource ID of the primary volume.
    #[builder(into)]
    #[serde(rename = "remoteVolumeResourceId")]
    pub r#remote_volume_resource_id: String,
    /// Frequency of replication.
    #[builder(into)]
    #[serde(rename = "replicationFrequency")]
    pub r#replication_frequency: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVolumeDataProtectionReplication {
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
                    "endpoint_type",
                    &self.r#endpoint_type,
                ),
                to_pulumi_object_field(
                    "remote_volume_location",
                    &self.r#remote_volume_location,
                ),
                to_pulumi_object_field(
                    "remote_volume_resource_id",
                    &self.r#remote_volume_resource_id,
                ),
                to_pulumi_object_field(
                    "replication_frequency",
                    &self.r#replication_frequency,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVolumeDataProtectionReplication {
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
                    r#endpoint_type: {
                        let field_value = match fields_map.get("endpoint_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_volume_location: {
                        let field_value = match fields_map.get("remote_volume_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_volume_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_volume_resource_id: {
                        let field_value = match fields_map.get("remote_volume_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_volume_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replication_frequency: {
                        let field_value = match fields_map.get("replication_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'replication_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
