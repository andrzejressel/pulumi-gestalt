#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HadoopClusterSecurityProfile {
    /// The resource ID of the Azure Active Directory Domain Service. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "aaddsResourceId")]
    pub r#aadds_resource_id: String,
    /// A list of the distinguished names for the cluster user groups. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterUsersGroupDns")]
    pub r#cluster_users_group_dns: Option<Vec<String>>,
    /// The name of the Azure Active Directory Domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// The user password of the Azure Active Directory Domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domainUserPassword")]
    pub r#domain_user_password: String,
    /// The username of the Azure Active Directory Domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domainUsername")]
    pub r#domain_username: String,
    /// A list of the LDAPS URLs to communicate with the Azure Active Directory. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "ldapsUrls")]
    pub r#ldaps_urls: Vec<String>,
    /// The User Assigned Identity for the HDInsight Cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "msiResourceId")]
    pub r#msi_resource_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HadoopClusterSecurityProfile {
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
                    "aadds_resource_id",
                    &self.r#aadds_resource_id,
                ),
                to_pulumi_object_field(
                    "cluster_users_group_dns",
                    &self.r#cluster_users_group_dns,
                ),
                to_pulumi_object_field(
                    "domain_name",
                    &self.r#domain_name,
                ),
                to_pulumi_object_field(
                    "domain_user_password",
                    &self.r#domain_user_password,
                ),
                to_pulumi_object_field(
                    "domain_username",
                    &self.r#domain_username,
                ),
                to_pulumi_object_field(
                    "ldaps_urls",
                    &self.r#ldaps_urls,
                ),
                to_pulumi_object_field(
                    "msi_resource_id",
                    &self.r#msi_resource_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HadoopClusterSecurityProfile {
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
                    r#aadds_resource_id: {
                        let field_value = match fields_map.get("aadds_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'aadds_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_users_group_dns: {
                        let field_value = match fields_map.get("cluster_users_group_dns") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_users_group_dns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_name: {
                        let field_value = match fields_map.get("domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_user_password: {
                        let field_value = match fields_map.get("domain_user_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_user_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_username: {
                        let field_value = match fields_map.get("domain_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ldaps_urls: {
                        let field_value = match fields_map.get("ldaps_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'ldaps_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#msi_resource_id: {
                        let field_value = match fields_map.get("msi_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'msi_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
