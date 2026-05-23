#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerVolume {
    /// The path in the container where the volume will be mounted.
    #[builder(into)]
    pub r#container_path: Option<String>,
    /// The container where the volume is coming from.
    #[builder(into)]
    pub r#from_container: Option<String>,
    /// The path on the host where the volume is coming from.
    #[builder(into)]
    pub r#host_path: Option<String>,
    /// If `true`, this volume will be readonly. Defaults to `false`.
    #[builder(into)]
    pub r#read_only: Option<bool>,
    /// The name of the docker volume which should be mounted.
    #[builder(into)]
    pub r#volume_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerVolume {
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
                    "containerPath",
                    &self.r#container_path,
                ),
                to_pulumi_object_field(
                    "fromContainer",
                    &self.r#from_container,
                ),
                to_pulumi_object_field(
                    "hostPath",
                    &self.r#host_path,
                ),
                to_pulumi_object_field(
                    "readOnly",
                    &self.r#read_only,
                ),
                to_pulumi_object_field(
                    "volumeName",
                    &self.r#volume_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContainerVolume {
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
                    r#container_path: {
                        let field_value = match fields_map.get("containerPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_container: {
                        let field_value = match fields_map.get("fromContainer") {
                            Some(value) => value,
                            None => bail!("Missing field 'fromContainer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_path: {
                        let field_value = match fields_map.get("hostPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_only: {
                        let field_value = match fields_map.get("readOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'readOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_name: {
                        let field_value = match fields_map.get("volumeName") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumeName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
