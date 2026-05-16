#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainSamlOptionsSamlOptions {
    /// Whether SAML authentication is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Information from your identity provider.
    #[builder(into)]
    #[serde(rename = "idp")]
    pub r#idp: Option<Box<super::super::types::opensearch::DomainSamlOptionsSamlOptionsIdp>>,
    /// This backend role from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into)]
    #[serde(rename = "masterBackendRole")]
    pub r#master_backend_role: Option<String>,
    /// This username from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into)]
    #[serde(rename = "masterUserName")]
    pub r#master_user_name: Option<String>,
    /// Element of the SAML assertion to use for backend roles. Default is roles.
    #[builder(into)]
    #[serde(rename = "rolesKey")]
    pub r#roles_key: Option<String>,
    /// Duration of a session in minutes after a user logs in. Default is 60. Maximum value is 1,440.
    #[builder(into)]
    #[serde(rename = "sessionTimeoutMinutes")]
    pub r#session_timeout_minutes: Option<i32>,
    /// Element of the SAML assertion to use for username. Default is NameID.
    #[builder(into)]
    #[serde(rename = "subjectKey")]
    pub r#subject_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainSamlOptionsSamlOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("enabled".to_string(), self.r#enabled.to_pulumi_value().await);
            map.insert("idp".to_string(), self.r#idp.to_pulumi_value().await);
            map.insert("master_backend_role".to_string(), self.r#master_backend_role.to_pulumi_value().await);
            map.insert("master_user_name".to_string(), self.r#master_user_name.to_pulumi_value().await);
            map.insert("roles_key".to_string(), self.r#roles_key.to_pulumi_value().await);
            map.insert("session_timeout_minutes".to_string(), self.r#session_timeout_minutes.to_pulumi_value().await);
            map.insert("subject_key".to_string(), self.r#subject_key.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainSamlOptionsSamlOptions {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#idp: {
                        let field_value = match fields_map.get("idp") {
                            Some(value) => value,
                            None => bail!("Missing field 'idp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::opensearch::DomainSamlOptionsSamlOptionsIdp>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#master_backend_role: {
                        let field_value = match fields_map.get("master_backend_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_backend_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#master_user_name: {
                        let field_value = match fields_map.get("master_user_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_user_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#roles_key: {
                        let field_value = match fields_map.get("roles_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'roles_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#session_timeout_minutes: {
                        let field_value = match fields_map.get("session_timeout_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_timeout_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subject_key: {
                        let field_value = match fields_map.get("subject_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
