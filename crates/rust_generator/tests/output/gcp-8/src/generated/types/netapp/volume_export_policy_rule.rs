#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeExportPolicyRule {
    /// Defines the access type for clients matching the `allowedClients` specification.
    /// Possible values are: `READ_ONLY`, `READ_WRITE`, `READ_NONE`.
    #[builder(into)]
    pub r#access_type: Option<String>,
    /// Defines the client ingress specification (allowed clients) as a comma separated list with IPv4 CIDRs or IPv4 host addresses.
    #[builder(into)]
    pub r#allowed_clients: Option<String>,
    /// If enabled, the root user (UID = 0) of the specified clients doesn't get mapped to nobody (UID = 65534). This is also known as no_root_squash.
    #[builder(into)]
    pub r#has_root_access: Option<String>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode.
    #[builder(into)]
    pub r#kerberos_5_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode. The 'kerberos5ReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    pub r#kerberos_5_read_write: Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode.
    #[builder(into)]
    pub r#kerberos_5_i_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    pub r#kerberos_5_i_read_write: Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode.
    #[builder(into)]
    pub r#kerberos_5_p_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    pub r#kerberos_5_p_read_write: Option<bool>,
    /// Enable to apply the export rule to NFSV3 clients.
    #[builder(into)]
    pub r#nfsv_3: Option<bool>,
    /// Enable to apply the export rule to NFSV4.1 clients.
    #[builder(into)]
    pub r#nfsv_4: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeExportPolicyRule {
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
                    "accessType",
                    &self.r#access_type,
                ),
                to_pulumi_object_field(
                    "allowedClients",
                    &self.r#allowed_clients,
                ),
                to_pulumi_object_field(
                    "hasRootAccess",
                    &self.r#has_root_access,
                ),
                to_pulumi_object_field(
                    "kerberos5ReadOnly",
                    &self.r#kerberos_5_read_only,
                ),
                to_pulumi_object_field(
                    "kerberos5ReadWrite",
                    &self.r#kerberos_5_read_write,
                ),
                to_pulumi_object_field(
                    "kerberos5iReadOnly",
                    &self.r#kerberos_5_i_read_only,
                ),
                to_pulumi_object_field(
                    "kerberos5iReadWrite",
                    &self.r#kerberos_5_i_read_write,
                ),
                to_pulumi_object_field(
                    "kerberos5pReadOnly",
                    &self.r#kerberos_5_p_read_only,
                ),
                to_pulumi_object_field(
                    "kerberos5pReadWrite",
                    &self.r#kerberos_5_p_read_write,
                ),
                to_pulumi_object_field(
                    "nfsv3",
                    &self.r#nfsv_3,
                ),
                to_pulumi_object_field(
                    "nfsv4",
                    &self.r#nfsv_4,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeExportPolicyRule {
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
                    r#access_type: {
                        let field_value = match fields_map.get("accessType") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_clients: {
                        let field_value = match fields_map.get("allowedClients") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedClients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#has_root_access: {
                        let field_value = match fields_map.get("hasRootAccess") {
                            Some(value) => value,
                            None => bail!("Missing field 'hasRootAccess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_only: {
                        let field_value = match fields_map.get("kerberos5ReadOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5ReadOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_write: {
                        let field_value = match fields_map.get("kerberos5ReadWrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5ReadWrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_only: {
                        let field_value = match fields_map.get("kerberos5iReadOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5iReadOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_write: {
                        let field_value = match fields_map.get("kerberos5iReadWrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5iReadWrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_only: {
                        let field_value = match fields_map.get("kerberos5pReadOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5pReadOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_write: {
                        let field_value = match fields_map.get("kerberos5pReadWrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5pReadWrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfsv_3: {
                        let field_value = match fields_map.get("nfsv3") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfsv3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfsv_4: {
                        let field_value = match fields_map.get("nfsv4") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfsv4' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
