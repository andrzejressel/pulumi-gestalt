#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesRecoveryService {
    #[builder(into)]
    #[serde(rename = "purgeProtectedItemsFromVaultOnDestroy")]
    pub r#purge_protected_items_from_vault_on_destroy: Option<bool>,
    #[builder(into)]
    #[serde(rename = "vmBackupStopProtectionAndRetainDataOnDestroy")]
    pub r#vm_backup_stop_protection_and_retain_data_on_destroy: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderFeaturesRecoveryService {
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
                    "purge_protected_items_from_vault_on_destroy",
                    &self.r#purge_protected_items_from_vault_on_destroy,
                ),
                to_pulumi_object_field(
                    "vm_backup_stop_protection_and_retain_data_on_destroy",
                    &self.r#vm_backup_stop_protection_and_retain_data_on_destroy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProviderFeaturesRecoveryService {
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
                    r#purge_protected_items_from_vault_on_destroy: {
                        let field_value = match fields_map.get("purge_protected_items_from_vault_on_destroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'purge_protected_items_from_vault_on_destroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_backup_stop_protection_and_retain_data_on_destroy: {
                        let field_value = match fields_map.get("vm_backup_stop_protection_and_retain_data_on_destroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_backup_stop_protection_and_retain_data_on_destroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
