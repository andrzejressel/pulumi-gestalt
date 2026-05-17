#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterAciConnectorLinux {
    /// A `connector_identity` block is exported. The exported attributes are defined below.
    #[builder(into)]
    #[serde(rename = "connectorIdentities")]
    pub r#connector_identities: Option<Vec<super::super::types::containerservice::KubernetesClusterAciConnectorLinuxConnectorIdentity>>,
    /// The subnet name for the virtual nodes to run.
    /// 
    /// > **Note:** At this time ACI Connectors are not supported in Azure China.
    /// 
    /// > **Note:** AKS will add a delegation to the subnet named here. To prevent further runs from failing you should make sure that the subnet you create for virtual nodes has a delegation, like so.
    /// 
    /// ```yaml
    /// resources:
    ///   virtual:
    ///     type: azure:network:Subnet
    ///     properties:
    ///       delegations:
    ///         - name: aciDelegation
    ///           serviceDelegation:
    ///             name: Microsoft.ContainerInstance/containerGroups
    ///             actions:
    ///               - Microsoft.Network/virtualNetworks/subnets/action
    /// ```
    #[builder(into)]
    #[serde(rename = "subnetName")]
    pub r#subnet_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterAciConnectorLinux {
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
                    "connector_identities",
                    &self.r#connector_identities,
                ),
                to_pulumi_object_field(
                    "subnet_name",
                    &self.r#subnet_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterAciConnectorLinux {
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
                    r#connector_identities: {
                        let field_value = match fields_map.get("connector_identities") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_identities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_name: {
                        let field_value = match fields_map.get("subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
