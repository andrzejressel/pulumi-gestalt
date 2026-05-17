#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReservationSpecificReservationInstanceProperty {
    /// Guest accelerator type and count.
    #[builder(into)]
    #[serde(rename = "guestAccelerators")]
    pub r#guest_accelerators: Vec<super::super::types::compute::GetReservationSpecificReservationInstancePropertyGuestAccelerator>,
    /// The amount of local ssd to reserve with each instance. This
    /// reserves disks of type 'local-ssd'.
    #[builder(into)]
    #[serde(rename = "localSsds")]
    pub r#local_ssds: Vec<super::super::types::compute::GetReservationSpecificReservationInstancePropertyLocalSsd>,
    /// The name of the machine type to reserve.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: String,
    /// The minimum CPU platform for the reservation. For example,
    /// '"Intel Skylake"'. See
    /// the CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)
    /// for information on available CPU platforms.
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetReservationSpecificReservationInstanceProperty {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "guest_accelerators",
                    &self.r#guest_accelerators,
                ),
                to_pulumi_object_field(
                    "local_ssds",
                    &self.r#local_ssds,
                ),
                to_pulumi_object_field(
                    "machine_type",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "min_cpu_platform",
                    &self.r#min_cpu_platform,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetReservationSpecificReservationInstanceProperty {
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
                    r#guest_accelerators: {
                        let field_value = match fields_map.get("guest_accelerators") {
                            Some(value) => value,
                            None => bail!("Missing field 'guest_accelerators' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssds: {
                        let field_value = match fields_map.get("local_ssds") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ssds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_cpu_platform: {
                        let field_value = match fields_map.get("min_cpu_platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_cpu_platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
