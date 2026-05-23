#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineGroupWsfcDomainProfile {
    /// The account name used for creating cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#cluster_bootstrap_account_name: Option<String>,
    /// The account name used for operating cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#cluster_operator_account_name: Option<String>,
    /// The subnet type of the SQL Virtual Machine cluster. Possible values are `MultiSubnet` and `SingleSubnet`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#cluster_subnet_type: String,
    /// The fully qualified name of the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#fqdn: String,
    /// The organizational Unit path in which the nodes and cluster will be present. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#organizational_unit_path: Option<String>,
    /// The account name under which SQL service will run on all participating SQL virtual machines in the cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#sql_service_account_name: Option<String>,
    /// The primary key of the Storage Account.
    #[builder(into)]
    pub r#storage_account_primary_key: Option<String>,
    /// The SAS URL to the Storage Container of the witness storage account. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#storage_account_url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineGroupWsfcDomainProfile {
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
                    "clusterBootstrapAccountName",
                    &self.r#cluster_bootstrap_account_name,
                ),
                to_pulumi_object_field(
                    "clusterOperatorAccountName",
                    &self.r#cluster_operator_account_name,
                ),
                to_pulumi_object_field(
                    "clusterSubnetType",
                    &self.r#cluster_subnet_type,
                ),
                to_pulumi_object_field(
                    "fqdn",
                    &self.r#fqdn,
                ),
                to_pulumi_object_field(
                    "organizationalUnitPath",
                    &self.r#organizational_unit_path,
                ),
                to_pulumi_object_field(
                    "sqlServiceAccountName",
                    &self.r#sql_service_account_name,
                ),
                to_pulumi_object_field(
                    "storageAccountPrimaryKey",
                    &self.r#storage_account_primary_key,
                ),
                to_pulumi_object_field(
                    "storageAccountUrl",
                    &self.r#storage_account_url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineGroupWsfcDomainProfile {
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
                    r#cluster_bootstrap_account_name: {
                        let field_value = match fields_map.get("clusterBootstrapAccountName") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterBootstrapAccountName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_operator_account_name: {
                        let field_value = match fields_map.get("clusterOperatorAccountName") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterOperatorAccountName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_subnet_type: {
                        let field_value = match fields_map.get("clusterSubnetType") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterSubnetType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fqdn: {
                        let field_value = match fields_map.get("fqdn") {
                            Some(value) => value,
                            None => bail!("Missing field 'fqdn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit_path: {
                        let field_value = match fields_map.get("organizationalUnitPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizationalUnitPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_service_account_name: {
                        let field_value = match fields_map.get("sqlServiceAccountName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqlServiceAccountName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_primary_key: {
                        let field_value = match fields_map.get("storageAccountPrimaryKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAccountPrimaryKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_url: {
                        let field_value = match fields_map.get("storageAccountUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAccountUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
