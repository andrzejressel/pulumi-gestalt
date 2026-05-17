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
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "iscsis".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#iscsis,
                )
                .await,
            );
            map.insert(
                "managements".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managements,
                )
                .await,
            );
            map.insert(
                "nfs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfs,
                )
                .await,
            );
            map.insert(
                "smbs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#smbs,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OntapStorageVirtualMachineEndpoint {
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
