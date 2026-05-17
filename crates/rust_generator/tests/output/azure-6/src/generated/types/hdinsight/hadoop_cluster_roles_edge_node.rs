#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HadoopClusterRolesEdgeNode {
    /// The HTTPS Connectivity Endpoint for this HDInsight Hadoop Cluster. One or more `https_endpoints` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "httpsEndpoints")]
    pub r#https_endpoints: Option<Vec<super::super::types::hdinsight::HadoopClusterRolesEdgeNodeHttpsEndpoint>>,
    /// A `install_script_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "installScriptActions")]
    pub r#install_script_actions: Vec<super::super::types::hdinsight::HadoopClusterRolesEdgeNodeInstallScriptAction>,
    /// The number of instances which should be run for the Worker Nodes.
    #[builder(into)]
    #[serde(rename = "targetInstanceCount")]
    pub r#target_instance_count: i32,
    /// A `uninstall_script_actions` block as defined below.
    #[builder(into)]
    #[serde(rename = "uninstallScriptActions")]
    pub r#uninstall_script_actions: Option<Vec<super::super::types::hdinsight::HadoopClusterRolesEdgeNodeUninstallScriptAction>>,
    /// The Size of the Virtual Machine which should be used as the Edge Nodes. Possible values are `ExtraSmall`, `Small`, `Medium`, `Large`, `ExtraLarge`, `A5`, `A6`, `A7`, `A8`, `A9`, `A10`, `A11`, `Standard_A1_V2`, `Standard_A2_V2`, `Standard_A2m_V2`, `Standard_A3`, `Standard_A4_V2`, `Standard_A4m_V2`, `Standard_A8_V2`, `Standard_A8m_V2`, `Standard_D1`, `Standard_D2`, `Standard_D3`, `Standard_D4`, `Standard_D11`, `Standard_D12`, `Standard_D13`, `Standard_D14`, `Standard_D1_V2`, `Standard_D2_V2`, `Standard_D3_V2`, `Standard_D4_V2`, `Standard_D5_V2`, `Standard_D11_V2`, `Standard_D12_V2`, `Standard_D13_V2`, `Standard_D14_V2`, `Standard_DS1_V2`, `Standard_DS2_V2`, `Standard_DS3_V2`, `Standard_DS4_V2`, `Standard_DS5_V2`, `Standard_DS11_V2`, `Standard_DS12_V2`, `Standard_DS13_V2`, `Standard_DS14_V2`, `Standard_E2_V3`, `Standard_E4_V3`, `Standard_E8_V3`, `Standard_E16_V3`, `Standard_E20_V3`, `Standard_E32_V3`, `Standard_E64_V3`, `Standard_E64i_V3`, `Standard_E2s_V3`, `Standard_E4s_V3`, `Standard_E8s_V3`, `Standard_E16s_V3`, `Standard_E20s_V3`, `Standard_E32s_V3`, `Standard_E64s_V3`, `Standard_E64is_V3`, `Standard_D2a_V4`, `Standard_D4a_V4`, `Standard_D8a_V4`, `Standard_D16a_V4`, `Standard_D32a_V4`, `Standard_D48a_V4`, `Standard_D64a_V4`, `Standard_D96a_V4`, `Standard_E2a_V4`, `Standard_E4a_V4`, `Standard_E8a_V4`, `Standard_E16a_V4`, `Standard_E20a_V4`, `Standard_E32a_V4`, `Standard_E48a_V4`, `Standard_E64a_V4`, `Standard_E96a_V4`, `Standard_D2ads_V5`, `Standard_D4ads_V5`, `Standard_D8ads_V5`, `Standard_D16ads_V5`, `Standard_D32ads_V5`, `Standard_D48ads_V5`, `Standard_D64ads_V5`, `Standard_D96ads_V5`, `Standard_E2ads_V5`, `Standard_E4ads_V5`, `Standard_E8ads_V5`, `Standard_E16ads_V5`, `Standard_E20ads_V5`, `Standard_E32ads_V5`, `Standard_E48ads_V5`, `Standard_E64ads_V5`, `Standard_E96ads_V5`, `Standard_G1`, `Standard_G2`, `Standard_G3`, `Standard_G4`, `Standard_G5`, `Standard_F2s_V2`, `Standard_F4s_V2`, `Standard_F8s_V2`, `Standard_F16s_V2`, `Standard_F32s_V2`, `Standard_F64s_V2`, `Standard_F72s_V2`, `Standard_GS1`, `Standard_GS2`, `Standard_GS3`, `Standard_GS4`, `Standard_GS5` and `Standard_NC24`.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HadoopClusterRolesEdgeNode {
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
                    "https_endpoints",
                    &self.r#https_endpoints,
                ),
                to_pulumi_object_field(
                    "install_script_actions",
                    &self.r#install_script_actions,
                ),
                to_pulumi_object_field(
                    "target_instance_count",
                    &self.r#target_instance_count,
                ),
                to_pulumi_object_field(
                    "uninstall_script_actions",
                    &self.r#uninstall_script_actions,
                ),
                to_pulumi_object_field(
                    "vm_size",
                    &self.r#vm_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HadoopClusterRolesEdgeNode {
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
                    r#https_endpoints: {
                        let field_value = match fields_map.get("https_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#install_script_actions: {
                        let field_value = match fields_map.get("install_script_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'install_script_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_instance_count: {
                        let field_value = match fields_map.get("target_instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uninstall_script_actions: {
                        let field_value = match fields_map.get("uninstall_script_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'uninstall_script_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vm_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
