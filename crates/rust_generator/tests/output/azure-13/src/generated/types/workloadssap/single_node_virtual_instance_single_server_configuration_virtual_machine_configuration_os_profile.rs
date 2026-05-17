#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfigurationOsProfile {
    /// The name of the administrator account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: String,
    /// The SSH public key that is used to authenticate with the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sshPrivateKey")]
    pub r#ssh_private_key: String,
    /// The SSH private key that is used to authenticate with the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sshPublicKey")]
    pub r#ssh_public_key: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfigurationOsProfile {
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
                "admin_username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#admin_username,
                )
                .await,
            );
            map.insert(
                "ssh_private_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_private_key,
                )
                .await,
            );
            map.insert(
                "ssh_public_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_public_key,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfigurationOsProfile {
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
                    r#admin_username: {
                        let field_value = match fields_map.get("admin_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'admin_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_private_key: {
                        let field_value = match fields_map.get("ssh_private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_public_key: {
                        let field_value = match fields_map.get("ssh_public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
