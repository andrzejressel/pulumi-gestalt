#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterProfile {
    /// The custom domain for the cluster. For more info, see [Prepare a custom domain for your cluster](https://docs.microsoft.com/azure/openshift/tutorial-create-cluster#prepare-a-custom-domain-for-your-cluster-optional). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: String,
    /// Whether Federal Information Processing Standard (FIPS) validated cryptographic modules are used. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "fipsEnabled")]
    pub r#fips_enabled: Option<bool>,
    /// The name of a Resource Group which will be created to host VMs of Azure Red Hat OpenShift Cluster. The value cannot contain uppercase characters. Defaults to `aro-{domain}`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "managedResourceGroupName")]
    pub r#managed_resource_group_name: Option<String>,
    /// The Red Hat pull secret for the cluster. For more info, see [Get a Red Hat pull secret](https://learn.microsoft.com/azure/openshift/tutorial-create-cluster#get-a-red-hat-pull-secret-optional). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "pullSecret")]
    pub r#pull_secret: Option<String>,
    /// The resource group that the cluster profile is attached to.
    #[builder(into)]
    #[serde(rename = "resourceGroupId")]
    pub r#resource_group_id: Option<String>,
    /// The version of the OpenShift cluster. Available versions can be found with the Azure CLI command `az aro get-versions --location <region>`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterProfile {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain,
                )
                .await,
            );
            map.insert(
                "fips_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fips_enabled,
                )
                .await,
            );
            map.insert(
                "managed_resource_group_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_resource_group_name,
                )
                .await,
            );
            map.insert(
                "pull_secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pull_secret,
                )
                .await,
            );
            map.insert(
                "resource_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_group_id,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterProfile {
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
                    r#domain: {
                        let field_value = match fields_map.get("domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fips_enabled: {
                        let field_value = match fields_map.get("fips_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'fips_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_resource_group_name: {
                        let field_value = match fields_map.get("managed_resource_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_resource_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pull_secret: {
                        let field_value = match fields_map.get("pull_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'pull_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_group_id: {
                        let field_value = match fields_map.get("resource_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
