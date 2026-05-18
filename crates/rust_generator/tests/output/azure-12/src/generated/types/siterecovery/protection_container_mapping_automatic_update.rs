#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProtectionContainerMappingAutomaticUpdate {
    /// The authentication type used for automation account. Possible values are `RunAsAccount` and `SystemAssignedIdentity`. Defaults to `SystemAssignedIdentity`.
    /// 
    /// > **Note:** `RunAsAccount` of `authentication_type` is deprecated and will retire on September 30, 2023. Details could be found [here](https://learn.microsoft.com/en-us/azure/automation/whats-new#support-for-run-as-accounts).
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Option<String>,
    /// The automation account ID which holds the automatic update runbook and authenticates to Azure resources.
    /// 
    /// > **Note:** `automation_account_id` is required when `enabled` is specified.
    #[builder(into)]
    #[serde(rename = "automationAccountId")]
    pub r#automation_account_id: Option<String>,
    /// Should the Mobility service installed on Azure virtual machines be automatically updated. Defaults to `false`.
    /// 
    /// > **Note:** The setting applies to all Azure VMs protected in the same container. For more details see [this document](https://learn.microsoft.com/en-us/azure/site-recovery/azure-to-azure-autoupdate#enable-automatic-updates)
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProtectionContainerMappingAutomaticUpdate {
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
                    "authentication_type",
                    &self.r#authentication_type,
                ),
                to_pulumi_object_field(
                    "automation_account_id",
                    &self.r#automation_account_id,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProtectionContainerMappingAutomaticUpdate {
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
                    r#authentication_type: {
                        let field_value = match fields_map.get("authentication_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#automation_account_id: {
                        let field_value = match fields_map.get("automation_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'automation_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
