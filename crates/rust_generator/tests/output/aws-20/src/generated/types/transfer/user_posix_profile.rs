#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPosixProfile {
    /// The POSIX group ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: i32,
    /// The secondary POSIX group IDs used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "secondaryGids")]
    pub r#secondary_gids: Option<Vec<i32>>,
    /// The POSIX user ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPosixProfile {
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
                "gid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gid,
                )
                .await,
            );
            map.insert(
                "secondary_gids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_gids,
                )
                .await,
            );
            map.insert(
                "uid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uid,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPosixProfile {
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
                    r#gid: {
                        let field_value = match fields_map.get("gid") {
                            Some(value) => value,
                            None => bail!("Missing field 'gid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_gids: {
                        let field_value = match fields_map.get("secondary_gids") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_gids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uid: {
                        let field_value = match fields_map.get("uid") {
                            Some(value) => value,
                            None => bail!("Missing field 'uid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
