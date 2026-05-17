#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerVolume {
    /// The path in the container where the volume will be mounted.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: Option<String>,
    /// The container where the volume is coming from.
    #[builder(into)]
    #[serde(rename = "fromContainer")]
    pub r#from_container: Option<String>,
    /// The path on the host where the volume is coming from.
    #[builder(into)]
    #[serde(rename = "hostPath")]
    pub r#host_path: Option<String>,
    /// If `true`, this volume will be readonly. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    /// The name of the docker volume which should be mounted.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerVolume {
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
                    "container_path",
                    &self.r#container_path,
                ),
                to_pulumi_object_field(
                    "from_container",
                    &self.r#from_container,
                ),
                to_pulumi_object_field(
                    "host_path",
                    &self.r#host_path,
                ),
                to_pulumi_object_field(
                    "read_only",
                    &self.r#read_only,
                ),
                to_pulumi_object_field(
                    "volume_name",
                    &self.r#volume_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("container_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_container: {
                        let field_value = match fields_map.get("from_container") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_container' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_path: {
                        let field_value = match fields_map.get("host_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_only: {
                        let field_value = match fields_map.get("read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
