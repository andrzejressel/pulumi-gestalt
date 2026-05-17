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
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "apt",
                    &self.r#apt,
                ),
                to_pulumi_object_field(
                    "deb",
                    &self.r#deb,
                ),
                to_pulumi_object_field(
                    "desired_state",
                    &self.r#desired_state,
                ),
                to_pulumi_object_field(
                    "googet",
                    &self.r#googet,
                ),
                to_pulumi_object_field(
                    "msi",
                    &self.r#msi,
                ),
                to_pulumi_object_field(
                    "rpm",
                    &self.r#rpm,
                ),
                to_pulumi_object_field(
                    "yum",
                    &self.r#yum,
                ),
                to_pulumi_object_field(
                    "zypper",
                    &self.r#zypper,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
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
