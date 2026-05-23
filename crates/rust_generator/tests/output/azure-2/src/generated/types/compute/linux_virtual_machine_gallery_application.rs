#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxVirtualMachineGalleryApplication {
    /// Specifies whether the version will be automatically updated for the VM when a new Gallery Application version is available in PIR/SIG. Defaults to `false`.
    #[builder(into)]
    pub r#automatic_upgrade_enabled: Option<bool>,
    /// Specifies the URI to an Azure Blob that will replace the default configuration for the package if provided.
    #[builder(into)]
    pub r#configuration_blob_uri: Option<String>,
    /// Specifies the order in which the packages have to be installed. Possible values are between `0` and `2147483647`. Defaults to `0`.
    #[builder(into)]
    pub r#order: Option<i32>,
    /// Specifies a passthrough value for more generic context. This field can be any valid `string` value.
    #[builder(into)]
    pub r#tag: Option<String>,
    /// Specifies whether any failure for any operation in the VmApplication will fail the deployment of the VM. Defaults to `false`.
    #[builder(into)]
    pub r#treat_failure_as_deployment_failure_enabled: Option<bool>,
    /// Specifies the Gallery Application Version resource ID.
    #[builder(into)]
    pub r#version_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxVirtualMachineGalleryApplication {
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
                    "automaticUpgradeEnabled",
                    &self.r#automatic_upgrade_enabled,
                ),
                to_pulumi_object_field(
                    "configurationBlobUri",
                    &self.r#configuration_blob_uri,
                ),
                to_pulumi_object_field(
                    "order",
                    &self.r#order,
                ),
                to_pulumi_object_field(
                    "tag",
                    &self.r#tag,
                ),
                to_pulumi_object_field(
                    "treatFailureAsDeploymentFailureEnabled",
                    &self.r#treat_failure_as_deployment_failure_enabled,
                ),
                to_pulumi_object_field(
                    "versionId",
                    &self.r#version_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxVirtualMachineGalleryApplication {
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
                    r#automatic_upgrade_enabled: {
                        let field_value = match fields_map.get("automaticUpgradeEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'automaticUpgradeEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configuration_blob_uri: {
                        let field_value = match fields_map.get("configurationBlobUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'configurationBlobUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag: {
                        let field_value = match fields_map.get("tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#treat_failure_as_deployment_failure_enabled: {
                        let field_value = match fields_map.get("treatFailureAsDeploymentFailureEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'treatFailureAsDeploymentFailureEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_id: {
                        let field_value = match fields_map.get("versionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'versionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
