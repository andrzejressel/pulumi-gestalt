#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CacheAccessPolicyAccessRule {
    /// The access level for this rule. Possible values are: `rw`, `ro`, `no`.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: String,
    /// The anonymous GID used when `root_squash_enabled` is `true`.
    #[builder(into)]
    #[serde(rename = "anonymousGid")]
    pub r#anonymous_gid: Option<i32>,
    /// The anonymous UID used when `root_squash_enabled` is `true`.
    #[builder(into)]
    #[serde(rename = "anonymousUid")]
    pub r#anonymous_uid: Option<i32>,
    /// The filter applied to the `scope` for this rule. The filter's format depends on its scope: `default` scope matches all clients and has no filter value; `network` scope takes a CIDR format; `host` takes an IP address or fully qualified domain name. If a client does not match any filter rule and there is no default rule, access is denied.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// Whether to enable [root squash](https://docs.microsoft.com/azure/hpc-cache/access-policies#root-squash)?
    #[builder(into)]
    #[serde(rename = "rootSquashEnabled")]
    pub r#root_squash_enabled: Option<bool>,
    /// The scope of this rule. The `scope` and (potentially) the `filter` determine which clients match the rule. Possible values are: `default`, `network`, `host`.
    /// 
    /// > **NOTE:** Each `access_rule` should set a unique `scope`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
    /// Whether allow access to subdirectories under the root export?
    #[builder(into)]
    #[serde(rename = "submountAccessEnabled")]
    pub r#submount_access_enabled: Option<bool>,
    /// Whether [SUID](https://docs.microsoft.com/azure/hpc-cache/access-policies#suid) is allowed?
    #[builder(into)]
    #[serde(rename = "suidEnabled")]
    pub r#suid_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CacheAccessPolicyAccessRule {
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
                    "access",
                    &self.r#access,
                ),
                to_pulumi_object_field(
                    "anonymous_gid",
                    &self.r#anonymous_gid,
                ),
                to_pulumi_object_field(
                    "anonymous_uid",
                    &self.r#anonymous_uid,
                ),
                to_pulumi_object_field(
                    "filter",
                    &self.r#filter,
                ),
                to_pulumi_object_field(
                    "root_squash_enabled",
                    &self.r#root_squash_enabled,
                ),
                to_pulumi_object_field(
                    "scope",
                    &self.r#scope,
                ),
                to_pulumi_object_field(
                    "submount_access_enabled",
                    &self.r#submount_access_enabled,
                ),
                to_pulumi_object_field(
                    "suid_enabled",
                    &self.r#suid_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CacheAccessPolicyAccessRule {
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
                    r#access: {
                        let field_value = match fields_map.get("access") {
                            Some(value) => value,
                            None => bail!("Missing field 'access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#anonymous_gid: {
                        let field_value = match fields_map.get("anonymous_gid") {
                            Some(value) => value,
                            None => bail!("Missing field 'anonymous_gid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#anonymous_uid: {
                        let field_value = match fields_map.get("anonymous_uid") {
                            Some(value) => value,
                            None => bail!("Missing field 'anonymous_uid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_squash_enabled: {
                        let field_value = match fields_map.get("root_squash_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_squash_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#submount_access_enabled: {
                        let field_value = match fields_map.get("submount_access_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'submount_access_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suid_enabled: {
                        let field_value = match fields_map.get("suid_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'suid_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
