#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceFileSharesNfsExportOption {
    /// Either READ_ONLY, for allowing only read requests on the exported directory,
    /// or READ_WRITE, for allowing both read and write requests. The default is READ_WRITE.
    /// Default value is `READ_WRITE`.
    /// Possible values are: `READ_ONLY`, `READ_WRITE`.
    #[builder(into)]
    #[serde(rename = "accessMode")]
    pub r#access_mode: Option<String>,
    /// An integer representing the anonymous group id with a default value of 65534.
    /// Anon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned
    /// if this field is specified for other squashMode settings.
    #[builder(into)]
    #[serde(rename = "anonGid")]
    pub r#anon_gid: Option<i32>,
    /// An integer representing the anonymous user id with a default value of 65534.
    /// Anon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned
    /// if this field is specified for other squashMode settings.
    #[builder(into)]
    #[serde(rename = "anonUid")]
    pub r#anon_uid: Option<i32>,
    /// List of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.
    /// Overlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.
    /// The limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions.
    #[builder(into)]
    #[serde(rename = "ipRanges")]
    pub r#ip_ranges: Option<Vec<String>>,
    /// Either NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,
    /// for not allowing root access. The default is NO_ROOT_SQUASH.
    /// Default value is `NO_ROOT_SQUASH`.
    /// Possible values are: `NO_ROOT_SQUASH`, `ROOT_SQUASH`.
    #[builder(into)]
    #[serde(rename = "squashMode")]
    pub r#squash_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceFileSharesNfsExportOption {
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
                    "access_mode",
                    &self.r#access_mode,
                ),
                to_pulumi_object_field(
                    "anon_gid",
                    &self.r#anon_gid,
                ),
                to_pulumi_object_field(
                    "anon_uid",
                    &self.r#anon_uid,
                ),
                to_pulumi_object_field(
                    "ip_ranges",
                    &self.r#ip_ranges,
                ),
                to_pulumi_object_field(
                    "squash_mode",
                    &self.r#squash_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceFileSharesNfsExportOption {
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
                    r#access_mode: {
                        let field_value = match fields_map.get("access_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#anon_gid: {
                        let field_value = match fields_map.get("anon_gid") {
                            Some(value) => value,
                            None => bail!("Missing field 'anon_gid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#anon_uid: {
                        let field_value = match fields_map.get("anon_uid") {
                            Some(value) => value,
                            None => bail!("Missing field 'anon_uid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_ranges: {
                        let field_value = match fields_map.get("ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#squash_mode: {
                        let field_value = match fields_map.get("squash_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'squash_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
