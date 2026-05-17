#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterKerberosAttributes {
    /// Active Directory password for `ad_domain_join_user`. This provider cannot perform drift detection of this configuration.
    #[builder(into)]
    #[serde(rename = "adDomainJoinPassword")]
    pub r#ad_domain_join_password: Option<String>,
    /// Required only when establishing a cross-realm trust with an Active Directory domain. A user with sufficient privileges to join resources to the domain. This provider cannot perform drift detection of this configuration.
    #[builder(into)]
    #[serde(rename = "adDomainJoinUser")]
    pub r#ad_domain_join_user: Option<String>,
    /// Required only when establishing a cross-realm trust with a KDC in a different realm. The cross-realm principal password, which must be identical across realms. This provider cannot perform drift detection of this configuration.
    #[builder(into)]
    #[serde(rename = "crossRealmTrustPrincipalPassword")]
    pub r#cross_realm_trust_principal_password: Option<String>,
    /// Password used within the cluster for the kadmin service on the cluster-dedicated KDC, which maintains Kerberos principals, password policies, and keytabs for the cluster. This provider cannot perform drift detection of this configuration.
    #[builder(into)]
    #[serde(rename = "kdcAdminPassword")]
    pub r#kdc_admin_password: String,
    /// Name of the Kerberos realm to which all nodes in a cluster belong. For example, `EC2.INTERNAL`
    #[builder(into)]
    #[serde(rename = "realm")]
    pub r#realm: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterKerberosAttributes {
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
                "ad_domain_join_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ad_domain_join_password,
                )
                .await,
            );
            map.insert(
                "ad_domain_join_user".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ad_domain_join_user,
                )
                .await,
            );
            map.insert(
                "cross_realm_trust_principal_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cross_realm_trust_principal_password,
                )
                .await,
            );
            map.insert(
                "kdc_admin_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kdc_admin_password,
                )
                .await,
            );
            map.insert(
                "realm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#realm,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterKerberosAttributes {
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
                    r#ad_domain_join_password: {
                        let field_value = match fields_map.get("ad_domain_join_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'ad_domain_join_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ad_domain_join_user: {
                        let field_value = match fields_map.get("ad_domain_join_user") {
                            Some(value) => value,
                            None => bail!("Missing field 'ad_domain_join_user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cross_realm_trust_principal_password: {
                        let field_value = match fields_map.get("cross_realm_trust_principal_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'cross_realm_trust_principal_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kdc_admin_password: {
                        let field_value = match fields_map.get("kdc_admin_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'kdc_admin_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#realm: {
                        let field_value = match fields_map.get("realm") {
                            Some(value) => value,
                            None => bail!("Missing field 'realm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
