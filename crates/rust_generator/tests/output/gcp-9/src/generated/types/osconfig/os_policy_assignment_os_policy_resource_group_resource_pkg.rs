#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkg {
    /// A package managed by Apt. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "apt")]
    pub r#apt: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgApt>>,
    /// A deb package file. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "deb")]
    pub r#deb: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgDeb>>,
    /// The desired state the agent should maintain for
    /// this package. Possible values are: `DESIRED_STATE_UNSPECIFIED`, `INSTALLED`,
    /// `REMOVED`.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: String,
    /// A package managed by GooGet. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "googet")]
    pub r#googet: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgGooget>>,
    /// An MSI package. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "msi")]
    pub r#msi: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgMsi>>,
    /// An rpm package file. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "rpm")]
    pub r#rpm: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgRpm>>,
    /// A package managed by YUM. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "yum")]
    pub r#yum: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgYum>>,
    /// A package managed by Zypper. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "zypper")]
    pub r#zypper: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgZypper>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourcePkg {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "apt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apt,
                )
                .await,
            );
            map.insert(
                "deb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deb,
                )
                .await,
            );
            map.insert(
                "desired_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#desired_state,
                )
                .await,
            );
            map.insert(
                "googet".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#googet,
                )
                .await,
            );
            map.insert(
                "msi".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#msi,
                )
                .await,
            );
            map.insert(
                "rpm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rpm,
                )
                .await,
            );
            map.insert(
                "yum".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#yum,
                )
                .await,
            );
            map.insert(
                "zypper".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zypper,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourcePkg {
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
                    r#apt: {
                        let field_value = match fields_map.get("apt") {
                            Some(value) => value,
                            None => bail!("Missing field 'apt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deb: {
                        let field_value = match fields_map.get("deb") {
                            Some(value) => value,
                            None => bail!("Missing field 'deb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#desired_state: {
                        let field_value = match fields_map.get("desired_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#googet: {
                        let field_value = match fields_map.get("googet") {
                            Some(value) => value,
                            None => bail!("Missing field 'googet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#msi: {
                        let field_value = match fields_map.get("msi") {
                            Some(value) => value,
                            None => bail!("Missing field 'msi' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rpm: {
                        let field_value = match fields_map.get("rpm") {
                            Some(value) => value,
                            None => bail!("Missing field 'rpm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#yum: {
                        let field_value = match fields_map.get("yum") {
                            Some(value) => value,
                            None => bail!("Missing field 'yum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zypper: {
                        let field_value = match fields_map.get("zypper") {
                            Some(value) => value,
                            None => bail!("Missing field 'zypper' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
