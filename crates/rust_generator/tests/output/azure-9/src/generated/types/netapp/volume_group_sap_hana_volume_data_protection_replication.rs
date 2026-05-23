#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeGroupSapHanaVolumeDataProtectionReplication {
    /// The endpoint type. Possible values are `dst` and `src`. Defaults to `dst`.
    #[builder(into)]
    pub r#endpoint_type: Option<String>,
    /// Location of the primary volume.
    #[builder(into)]
    pub r#remote_volume_location: String,
    /// Resource ID of the primary volume.
    #[builder(into)]
    pub r#remote_volume_resource_id: String,
    /// eplication frequency. Possible values are `10minutes`, `daily` and `hourly`.
    #[builder(into)]
    pub r#replication_frequency: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeGroupSapHanaVolumeDataProtectionReplication {
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
                    "endpointType",
                    &self.r#endpoint_type,
                ),
                to_pulumi_object_field(
                    "remoteVolumeLocation",
                    &self.r#remote_volume_location,
                ),
                to_pulumi_object_field(
                    "remoteVolumeResourceId",
                    &self.r#remote_volume_resource_id,
                ),
                to_pulumi_object_field(
                    "replicationFrequency",
                    &self.r#replication_frequency,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeGroupSapHanaVolumeDataProtectionReplication {
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
                        let field_value = match fields_map.get("endpointType") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpointType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_volume_location: {
                        let field_value = match fields_map.get("remoteVolumeLocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'remoteVolumeLocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_volume_resource_id: {
                        let field_value = match fields_map.get("remoteVolumeResourceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'remoteVolumeResourceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replication_frequency: {
                        let field_value = match fields_map.get("replicationFrequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicationFrequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
