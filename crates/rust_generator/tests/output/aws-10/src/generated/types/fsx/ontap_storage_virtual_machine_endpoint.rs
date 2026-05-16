#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OntapStorageVirtualMachineEndpoint {
    /// An endpoint for accessing data on your storage virtual machine via iSCSI protocol. See Endpoint.
    #[builder(into)]
    #[serde(rename = "iscsis")]
    pub r#iscsis: Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointIscsi>>,
    /// An endpoint for managing your file system using the NetApp ONTAP CLI and NetApp ONTAP API. See Endpoint.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointManagement>>,
    /// An endpoint for accessing data on your storage virtual machine via NFS protocol. See Endpoint.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointNf>>,
    /// An endpoint for accessing data on your storage virtual machine via SMB protocol. This is only set if an active_directory_configuration has been set. See Endpoint.
    #[builder(into)]
    #[serde(rename = "smbs")]
    pub r#smbs: Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointSmb>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OntapStorageVirtualMachineEndpoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("iscsis".to_string(), self.r#iscsis.to_pulumi_value().await);
            map.insert("managements".to_string(), self.r#managements.to_pulumi_value().await);
            map.insert("nfs".to_string(), self.r#nfs.to_pulumi_value().await);
            map.insert("smbs".to_string(), self.r#smbs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OntapStorageVirtualMachineEndpoint {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#iscsis: {
                        let field_value = match fields_map.get("iscsis") {
                            Some(value) => value,
                            None => bail!("Missing field 'iscsis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointIscsi>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#managements: {
                        let field_value = match fields_map.get("managements") {
                            Some(value) => value,
                            None => bail!("Missing field 'managements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointManagement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#nfs: {
                        let field_value = match fields_map.get("nfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointNf>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#smbs: {
                        let field_value = match fields_map.get("smbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'smbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointSmb>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
