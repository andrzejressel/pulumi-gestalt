#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetOntapStorageVirtualMachineEndpoint {
    #[builder(into)]
    #[serde(rename = "iscsis")]
    pub r#iscsis: Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointIscsi>,
    /// An endpoint for managing SVMs using the NetApp ONTAP CLI, NetApp ONTAP API, or NetApp CloudManager. See SVM Endpoint below.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointManagement>,
    /// An endpoint for connecting using the Network File System (NFS) protocol. See SVM Endpoint below.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointNf>,
    /// An endpoint for connecting using the Server Message Block (SMB) protocol. See SVM Endpoint below.
    #[builder(into)]
    #[serde(rename = "smbs")]
    pub r#smbs: Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointSmb>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetOntapStorageVirtualMachineEndpoint {
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
                    "iscsis",
                    &self.r#iscsis,
                ),
                to_pulumi_object_field(
                    "managements",
                    &self.r#managements,
                ),
                to_pulumi_object_field(
                    "nfs",
                    &self.r#nfs,
                ),
                to_pulumi_object_field(
                    "smbs",
                    &self.r#smbs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetOntapStorageVirtualMachineEndpoint {
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
                    r#iscsis: {
                        let field_value = match fields_map.get("iscsis") {
                            Some(value) => value,
                            None => bail!("Missing field 'iscsis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managements: {
                        let field_value = match fields_map.get("managements") {
                            Some(value) => value,
                            None => bail!("Missing field 'managements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfs: {
                        let field_value = match fields_map.get("nfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smbs: {
                        let field_value = match fields_map.get("smbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'smbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
