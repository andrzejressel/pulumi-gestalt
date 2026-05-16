#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetOntapFileSystemEndpoint {
    /// A FileSystemEndpoint for managing your file system by setting up NetApp SnapMirror with other ONTAP systems. See FileSystemEndpoint below.
    #[builder(into)]
    #[serde(rename = "interclusters")]
    pub r#interclusters: Vec<super::super::types::fsx::GetOntapFileSystemEndpointIntercluster>,
    /// A FileSystemEndpoint for managing your file system using the NetApp ONTAP CLI and NetApp ONTAP API. See FileSystemEndpoint below.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Vec<super::super::types::fsx::GetOntapFileSystemEndpointManagement>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetOntapFileSystemEndpoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("interclusters".to_string(), self.r#interclusters.to_pulumi_value().await);
            map.insert("managements".to_string(), self.r#managements.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetOntapFileSystemEndpoint {
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
                    r#interclusters: {
                        let field_value = match fields_map.get("interclusters") {
                            Some(value) => value,
                            None => bail!("Missing field 'interclusters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::fsx::GetOntapFileSystemEndpointIntercluster> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#managements: {
                        let field_value = match fields_map.get("managements") {
                            Some(value) => value,
                            None => bail!("Missing field 'managements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::fsx::GetOntapFileSystemEndpointManagement> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
