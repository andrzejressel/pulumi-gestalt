#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterMasterAuthorizedNetworksConfig {
    /// External networks that can access the Kubernetes cluster master through HTTPS.
    #[builder(into)]
    #[serde(rename = "cidrBlocks")]
    pub r#cidr_blocks: Vec<super::super::types::container::GetClusterMasterAuthorizedNetworksConfigCidrBlock>,
    /// Whether Kubernetes master is accessible via Google Compute Engine Public IPs.
    #[builder(into)]
    #[serde(rename = "gcpPublicCidrsAccessEnabled")]
    pub r#gcp_public_cidrs_access_enabled: bool,
    /// Whether authorized networks is enforced on the private endpoint or not. Defaults to false.
    #[builder(into)]
    #[serde(rename = "privateEndpointEnforcementEnabled")]
    pub r#private_endpoint_enforcement_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterMasterAuthorizedNetworksConfig {
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
                    "cidr_blocks",
                    &self.r#cidr_blocks,
                ),
                to_pulumi_object_field(
                    "gcp_public_cidrs_access_enabled",
                    &self.r#gcp_public_cidrs_access_enabled,
                ),
                to_pulumi_object_field(
                    "private_endpoint_enforcement_enabled",
                    &self.r#private_endpoint_enforcement_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterMasterAuthorizedNetworksConfig {
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
                    r#cidr_blocks: {
                        let field_value = match fields_map.get("cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_public_cidrs_access_enabled: {
                        let field_value = match fields_map.get("gcp_public_cidrs_access_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcp_public_cidrs_access_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint_enforcement_enabled: {
                        let field_value = match fields_map.get("private_endpoint_enforcement_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint_enforcement_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
