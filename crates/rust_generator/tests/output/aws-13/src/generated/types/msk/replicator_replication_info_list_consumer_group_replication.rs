#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatorReplicationInfoListConsumerGroupReplication {
    /// List of regular expression patterns indicating the consumer groups that should not be replicated.
    #[builder(into)]
    #[serde(rename = "consumerGroupsToExcludes")]
    pub r#consumer_groups_to_excludes: Option<Vec<String>>,
    /// List of regular expression patterns indicating the consumer groups to copy.
    #[builder(into)]
    #[serde(rename = "consumerGroupsToReplicates")]
    pub r#consumer_groups_to_replicates: Vec<String>,
    /// Whether to periodically check for new consumer groups.
    #[builder(into)]
    #[serde(rename = "detectAndCopyNewConsumerGroups")]
    pub r#detect_and_copy_new_consumer_groups: Option<bool>,
    /// Whether to periodically write the translated offsets to __consumer_offsets topic in target cluster.
    #[builder(into)]
    #[serde(rename = "synchroniseConsumerGroupOffsets")]
    pub r#synchronise_consumer_group_offsets: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatorReplicationInfoListConsumerGroupReplication {
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
                "consumer_groups_to_excludes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consumer_groups_to_excludes,
                )
                .await,
            );
            map.insert(
                "consumer_groups_to_replicates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consumer_groups_to_replicates,
                )
                .await,
            );
            map.insert(
                "detect_and_copy_new_consumer_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detect_and_copy_new_consumer_groups,
                )
                .await,
            );
            map.insert(
                "synchronise_consumer_group_offsets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#synchronise_consumer_group_offsets,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatorReplicationInfoListConsumerGroupReplication {
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
                    r#consumer_groups_to_excludes: {
                        let field_value = match fields_map.get("consumer_groups_to_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_groups_to_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consumer_groups_to_replicates: {
                        let field_value = match fields_map.get("consumer_groups_to_replicates") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_groups_to_replicates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detect_and_copy_new_consumer_groups: {
                        let field_value = match fields_map.get("detect_and_copy_new_consumer_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'detect_and_copy_new_consumer_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#synchronise_consumer_group_offsets: {
                        let field_value = match fields_map.get("synchronise_consumer_group_offsets") {
                            Some(value) => value,
                            None => bail!("Missing field 'synchronise_consumer_group_offsets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
