#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessLevelBasicConditionDevicePolicy {
    /// A list of allowed device management levels.
    /// An empty list allows all management levels.
    /// Each value may be one of: `MANAGEMENT_UNSPECIFIED`, `NONE`, `BASIC`, `COMPLETE`.
    #[builder(into)]
    #[serde(rename = "allowedDeviceManagementLevels")]
    pub r#allowed_device_management_levels: Option<Vec<String>>,
    /// A list of allowed encryptions statuses.
    /// An empty list allows all statuses.
    /// Each value may be one of: `ENCRYPTION_UNSPECIFIED`, `ENCRYPTION_UNSUPPORTED`, `UNENCRYPTED`, `ENCRYPTED`.
    #[builder(into)]
    #[serde(rename = "allowedEncryptionStatuses")]
    pub r#allowed_encryption_statuses: Option<Vec<String>>,
    /// A list of allowed OS versions.
    /// An empty list allows all types and all versions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "osConstraints")]
    pub r#os_constraints: Option<Vec<super::super::types::accesscontextmanager::AccessLevelBasicConditionDevicePolicyOsConstraint>>,
    /// Whether the device needs to be approved by the customer admin.
    #[builder(into)]
    #[serde(rename = "requireAdminApproval")]
    pub r#require_admin_approval: Option<bool>,
    /// Whether the device needs to be corp owned.
    #[builder(into)]
    #[serde(rename = "requireCorpOwned")]
    pub r#require_corp_owned: Option<bool>,
    /// Whether or not screenlock is required for the DevicePolicy
    /// to be true. Defaults to false.
    #[builder(into)]
    #[serde(rename = "requireScreenLock")]
    pub r#require_screen_lock: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessLevelBasicConditionDevicePolicy {
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
                "allowed_device_management_levels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_device_management_levels,
                )
                .await,
            );
            map.insert(
                "allowed_encryption_statuses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_encryption_statuses,
                )
                .await,
            );
            map.insert(
                "os_constraints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_constraints,
                )
                .await,
            );
            map.insert(
                "require_admin_approval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_admin_approval,
                )
                .await,
            );
            map.insert(
                "require_corp_owned".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_corp_owned,
                )
                .await,
            );
            map.insert(
                "require_screen_lock".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_screen_lock,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessLevelBasicConditionDevicePolicy {
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
                    r#allowed_device_management_levels: {
                        let field_value = match fields_map.get("allowed_device_management_levels") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_device_management_levels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_encryption_statuses: {
                        let field_value = match fields_map.get("allowed_encryption_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_encryption_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_constraints: {
                        let field_value = match fields_map.get("os_constraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_constraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_admin_approval: {
                        let field_value = match fields_map.get("require_admin_approval") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_admin_approval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_corp_owned: {
                        let field_value = match fields_map.get("require_corp_owned") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_corp_owned' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_screen_lock: {
                        let field_value = match fields_map.get("require_screen_lock") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_screen_lock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
