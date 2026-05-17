#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesVirtualMachine {
    #[builder(into)]
    #[serde(rename = "deleteOsDiskOnDeletion")]
    pub r#delete_os_disk_on_deletion: Option<bool>,
    #[builder(into)]
    #[serde(rename = "detachImplicitDataDiskOnDeletion")]
    pub r#detach_implicit_data_disk_on_deletion: Option<bool>,
    #[builder(into)]
    #[serde(rename = "gracefulShutdown")]
    pub r#graceful_shutdown: Option<bool>,
    #[builder(into)]
    #[serde(rename = "skipShutdownAndForceDelete")]
    pub r#skip_shutdown_and_force_delete: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderFeaturesVirtualMachine {
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
                    "delete_os_disk_on_deletion",
                    &self.r#delete_os_disk_on_deletion,
                ),
                to_pulumi_object_field(
                    "detach_implicit_data_disk_on_deletion",
                    &self.r#detach_implicit_data_disk_on_deletion,
                ),
                to_pulumi_object_field(
                    "graceful_shutdown",
                    &self.r#graceful_shutdown,
                ),
                to_pulumi_object_field(
                    "skip_shutdown_and_force_delete",
                    &self.r#skip_shutdown_and_force_delete,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProviderFeaturesVirtualMachine {
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
                    r#delete_os_disk_on_deletion: {
                        let field_value = match fields_map.get("delete_os_disk_on_deletion") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_os_disk_on_deletion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detach_implicit_data_disk_on_deletion: {
                        let field_value = match fields_map.get("detach_implicit_data_disk_on_deletion") {
                            Some(value) => value,
                            None => bail!("Missing field 'detach_implicit_data_disk_on_deletion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#graceful_shutdown: {
                        let field_value = match fields_map.get("graceful_shutdown") {
                            Some(value) => value,
                            None => bail!("Missing field 'graceful_shutdown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_shutdown_and_force_delete: {
                        let field_value = match fields_map.get("skip_shutdown_and_force_delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'skip_shutdown_and_force_delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
