#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalAdminClusterStorage {
    /// Specifies the config for local PersistentVolumes backed
    /// by mounted node disks. These disks need to be formatted and mounted by the
    /// user, which can be done before or after cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lvpNodeMountsConfig")]
    pub r#lvp_node_mounts_config: Box<super::super::types::gkeonprem::BareMetalAdminClusterStorageLvpNodeMountsConfig>,
    /// Specifies the config for local PersistentVolumes backed by
    /// subdirectories in a shared filesystem. These subdirectores are
    /// automatically created during cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lvpShareConfig")]
    pub r#lvp_share_config: Box<super::super::types::gkeonprem::BareMetalAdminClusterStorageLvpShareConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalAdminClusterStorage {
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
                    "lvp_node_mounts_config",
                    &self.r#lvp_node_mounts_config,
                ),
                to_pulumi_object_field(
                    "lvp_share_config",
                    &self.r#lvp_share_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalAdminClusterStorage {
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
                    r#lvp_node_mounts_config: {
                        let field_value = match fields_map.get("lvp_node_mounts_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'lvp_node_mounts_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lvp_share_config: {
                        let field_value = match fields_map.get("lvp_share_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'lvp_share_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
