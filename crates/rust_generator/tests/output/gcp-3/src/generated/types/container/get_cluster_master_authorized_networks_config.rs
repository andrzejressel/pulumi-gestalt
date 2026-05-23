#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterMasterAuthorizedNetworksConfig {
    /// External networks that can access the Kubernetes cluster master through HTTPS.
    #[builder(into)]
    pub r#cidr_blocks: Vec<super::super::types::container::GetClusterMasterAuthorizedNetworksConfigCidrBlock>,
    /// Whether Kubernetes master is accessible via Google Compute Engine Public IPs.
    #[builder(into)]
    pub r#gcp_public_cidrs_access_enabled: bool,
    /// Whether authorized networks is enforced on the private endpoint or not. Defaults to false.
    #[builder(into)]
    pub r#private_endpoint_enforcement_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterMasterAuthorizedNetworksConfig {
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
                    "cidrBlocks",
                    &self.r#cidr_blocks,
                ),
                to_pulumi_object_field(
                    "gcpPublicCidrsAccessEnabled",
                    &self.r#gcp_public_cidrs_access_enabled,
                ),
                to_pulumi_object_field(
                    "privateEndpointEnforcementEnabled",
                    &self.r#private_endpoint_enforcement_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("cidrBlocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidrBlocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_public_cidrs_access_enabled: {
                        let field_value = match fields_map.get("gcpPublicCidrsAccessEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcpPublicCidrsAccessEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint_enforcement_enabled: {
                        let field_value = match fields_map.get("privateEndpointEnforcementEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateEndpointEnforcementEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
