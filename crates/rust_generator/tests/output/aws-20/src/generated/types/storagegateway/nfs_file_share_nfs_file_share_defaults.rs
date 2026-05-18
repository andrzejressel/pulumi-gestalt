#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NfsFileShareNfsFileShareDefaults {
    /// The Unix directory mode in the string form "nnnn". Defaults to `"0777"`.
    #[builder(into)]
    #[serde(rename = "directoryMode")]
    pub r#directory_mode: Option<String>,
    /// The Unix file mode in the string form "nnnn". Defaults to `"0666"`.
    #[builder(into)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Option<String>,
    /// The default group ID for the file share (unless the files have another group ID specified). Defaults to `65534` (`nfsnobody`). Valid values: `0` through `4294967294`.
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: Option<String>,
    /// The default owner ID for the file share (unless the files have another owner ID specified). Defaults to `65534` (`nfsnobody`). Valid values: `0` through `4294967294`.
    #[builder(into)]
    #[serde(rename = "ownerId")]
    pub r#owner_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NfsFileShareNfsFileShareDefaults {
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
                    "directory_mode",
                    &self.r#directory_mode,
                ),
                to_pulumi_object_field(
                    "file_mode",
                    &self.r#file_mode,
                ),
                to_pulumi_object_field(
                    "group_id",
                    &self.r#group_id,
                ),
                to_pulumi_object_field(
                    "owner_id",
                    &self.r#owner_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NfsFileShareNfsFileShareDefaults {
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
                    r#directory_mode: {
                        let field_value = match fields_map.get("directory_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'directory_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_mode: {
                        let field_value = match fields_map.get("file_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_id: {
                        let field_value = match fields_map.get("group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#owner_id: {
                        let field_value = match fields_map.get("owner_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'owner_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
