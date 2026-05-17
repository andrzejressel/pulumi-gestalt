#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceRepository {
    /// An Apt Repository. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "apt")]
    pub r#apt: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryApt>>,
    /// A Goo Repository. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "goo")]
    pub r#goo: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryGoo>>,
    /// A Yum Repository. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "yum")]
    pub r#yum: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryYum>>,
    /// A Zypper Repository. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "zypper")]
    pub r#zypper: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryZypper>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceRepository {
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
                "goo".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#goo,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceRepository {
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
                    r#goo: {
                        let field_value = match fields_map.get("goo") {
                            Some(value) => value,
                            None => bail!("Missing field 'goo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
