
awsAWS"6.66.2*�%
V
codeguruprofilerProfilingGroup2aws:codeguruprofiler/profilingGroup:ProfilingGroup�Resource for managing an AWS CodeGuru Profiler Profiling Group.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.codeguruprofiler.ProfilingGroup("example", {
    name: "example",
    computePlatform: "Default",
    agentOrchestrationConfig: {
        profilingEnabled: true,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codeguruprofiler.ProfilingGroup("example",
    name="example",
    compute_platform="Default",
    agent_orchestration_config={
        "profiling_enabled": True,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CodeGuruProfiler.ProfilingGroup("example", new()
    {
        Name = "example",
        ComputePlatform = "Default",
        AgentOrchestrationConfig = new Aws.CodeGuruProfiler.Inputs.ProfilingGroupAgentOrchestrationConfigArgs
        {
            ProfilingEnabled = true,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codeguruprofiler"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := codeguruprofiler.NewProfilingGroup(ctx, "example", &codeguruprofiler.ProfilingGroupArgs{
			Name:            pulumi.String("example"),
			ComputePlatform: pulumi.String("Default"),
			AgentOrchestrationConfig: &codeguruprofiler.ProfilingGroupAgentOrchestrationConfigArgs{
				ProfilingEnabled: pulumi.Bool(true),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codeguruprofiler.ProfilingGroup;
import com.pulumi.aws.codeguruprofiler.ProfilingGroupArgs;
import com.pulumi.aws.codeguruprofiler.inputs.ProfilingGroupAgentOrchestrationConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ProfilingGroup("example", ProfilingGroupArgs.builder()
            .name("example")
            .computePlatform("Default")
            .agentOrchestrationConfig(ProfilingGroupAgentOrchestrationConfigArgs.builder()
                .profilingEnabled(true)
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:codeguruprofiler:ProfilingGroup
    properties:
      name: example
      computePlatform: Default
      agentOrchestrationConfig:
        profilingEnabled: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodeGuru Profiler Profiling Group using the `id`. For example:

```sh
$ pulumi import aws:codeguruprofiler/profilingGroup:ProfilingGroup example profiling_group-name-12345678
```
�
agentOrchestrationConfig�B�:�
�
codeguruprofiler&ProfilingGroupAgentOrchestrationConfigbaws:codeguruprofiler/ProfilingGroupAgentOrchestrationConfig:ProfilingGroupAgentOrchestrationConfigSpecifies whether profiling is enabled or disabled for the created profiling. See Agent Orchestration Config for more details.
B
computePlatformB" )Compute platform of the profiling group.
R
nameB" DName of the profiling group.

The following arguments are optional:
�
tagsB2" �Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
agentOrchestrationConfig�B�:�
�
codeguruprofiler&ProfilingGroupAgentOrchestrationConfigbaws:codeguruprofiler/ProfilingGroupAgentOrchestrationConfig:ProfilingGroupAgentOrchestrationConfigSpecifies whether profiling is enabled or disabled for the created profiling. See Agent Orchestration Config for more details.
"'
arn" ARN of the profiling group.
"@
computePlatform" )Compute platform of the profiling group.
"P
name" DName of the profiling group.

The following arguments are optional:
"�
tagsB2" �Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�?
k
codegurureviewerRepositoryAssociation@aws:codegurureviewer/repositoryAssociation:RepositoryAssociation�+Resource for managing an AWS CodeGuru Reviewer Repository Association.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.kms.Key("example", {});
const exampleRepository = new aws.codecommit.Repository("example", {repositoryName: "example-repo"});
const exampleRepositoryAssociation = new aws.codegurureviewer.RepositoryAssociation("example", {
    repository: {
        codecommit: {
            name: exampleRepository.repositoryName,
        },
    },
    kmsKeyDetails: {
        encryptionOption: "CUSTOMER_MANAGED_CMK",
        kmsKeyId: example.keyId,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.kms.Key("example")
example_repository = aws.codecommit.Repository("example", repository_name="example-repo")
example_repository_association = aws.codegurureviewer.RepositoryAssociation("example",
    repository={
        "codecommit": {
            "name": example_repository.repository_name,
        },
    },
    kms_key_details={
        "encryption_option": "CUSTOMER_MANAGED_CMK",
        "kms_key_id": example.key_id,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Kms.Key("example");

    var exampleRepository = new Aws.CodeCommit.Repository("example", new()
    {
        RepositoryName = "example-repo",
    });

    var exampleRepositoryAssociation = new Aws.CodeGuruReviewer.RepositoryAssociation("example", new()
    {
        Repository = new Aws.CodeGuruReviewer.Inputs.RepositoryAssociationRepositoryArgs
        {
            Codecommit = new Aws.CodeGuruReviewer.Inputs.RepositoryAssociationRepositoryCodecommitArgs
            {
                Name = exampleRepository.RepositoryName,
            },
        },
        KmsKeyDetails = new Aws.CodeGuruReviewer.Inputs.RepositoryAssociationKmsKeyDetailsArgs
        {
            EncryptionOption = "CUSTOMER_MANAGED_CMK",
            KmsKeyId = example.KeyId,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codecommit"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codegurureviewer"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kms"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := kms.NewKey(ctx, "example", nil)
		if err != nil {
			return err
		}
		exampleRepository, err := codecommit.NewRepository(ctx, "example", &codecommit.RepositoryArgs{
			RepositoryName: pulumi.String("example-repo"),
		})
		if err != nil {
			return err
		}
		_, err = codegurureviewer.NewRepositoryAssociation(ctx, "example", &codegurureviewer.RepositoryAssociationArgs{
			Repository: &codegurureviewer.RepositoryAssociationRepositoryArgs{
				Codecommit: &codegurureviewer.RepositoryAssociationRepositoryCodecommitArgs{
					Name: exampleRepository.RepositoryName,
				},
			},
			KmsKeyDetails: &codegurureviewer.RepositoryAssociationKmsKeyDetailsArgs{
				EncryptionOption: pulumi.String("CUSTOMER_MANAGED_CMK"),
				KmsKeyId:         example.KeyId,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.kms.Key;
import com.pulumi.aws.codecommit.Repository;
import com.pulumi.aws.codecommit.RepositoryArgs;
import com.pulumi.aws.codegurureviewer.RepositoryAssociation;
import com.pulumi.aws.codegurureviewer.RepositoryAssociationArgs;
import com.pulumi.aws.codegurureviewer.inputs.RepositoryAssociationRepositoryArgs;
import com.pulumi.aws.codegurureviewer.inputs.RepositoryAssociationRepositoryCodecommitArgs;
import com.pulumi.aws.codegurureviewer.inputs.RepositoryAssociationKmsKeyDetailsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Key("example");

        var exampleRepository = new Repository("exampleRepository", RepositoryArgs.builder()
            .repositoryName("example-repo")
            .build());

        var exampleRepositoryAssociation = new RepositoryAssociation("exampleRepositoryAssociation", RepositoryAssociationArgs.builder()
            .repository(RepositoryAssociationRepositoryArgs.builder()
                .codecommit(RepositoryAssociationRepositoryCodecommitArgs.builder()
                    .name(exampleRepository.repositoryName())
                    .build())
                .build())
            .kmsKeyDetails(RepositoryAssociationKmsKeyDetailsArgs.builder()
                .encryptionOption("CUSTOMER_MANAGED_CMK")
                .kmsKeyId(example.keyId())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:kms:Key
  exampleRepository:
    type: aws:codecommit:Repository
    name: example
    properties:
      repositoryName: example-repo
  exampleRepositoryAssociation:
    type: aws:codegurureviewer:RepositoryAssociation
    name: example
    properties:
      repository:
        codecommit:
          name: ${exampleRepository.repositoryName}
      kmsKeyDetails:
        encryptionOption: CUSTOMER_MANAGED_CMK
        kmsKeyId: ${example.keyId}
```
<!--End PulumiCodeChooser -->
�
kmsKeyDetails�B�:�
�
codegurureviewer"RepositoryAssociationKmsKeyDetailsZaws:codegurureviewer/RepositoryAssociationKmsKeyDetails:RepositoryAssociationKmsKeyDetailsKAn object describing the KMS key to asssociate. Block is documented below.
�

repository�:�
�
codegurureviewerRepositoryAssociationRepositoryTaws:codegurureviewer/RepositoryAssociationRepository:RepositoryAssociationRepository�An object describing the repository to associate. Valid values: `bitbucket`, `codecommit`, `github_enterprise_server`, or `s3_bucket`. Block is documented below. Note: for repositories that leverage CodeStar connections (ex. `bitbucket`, `github_enterprise_server`) the connection must be in `Available` status prior to creating this resource.

The following arguments are optional:

tagsB2" "R
arn" GThe Amazon Resource Name (ARN) identifying the repository association.
";
associationId" &The ID of the repository association.
"_
connectionArn" JThe Amazon Resource Name (ARN) of an AWS CodeStar Connections connection.
"�
kmsKeyDetails�B�:�
�
codegurureviewer"RepositoryAssociationKmsKeyDetailsZaws:codegurureviewer/RepositoryAssociationKmsKeyDetails:RepositoryAssociationKmsKeyDetailsKAn object describing the KMS key to asssociate. Block is documented below.
"(
name" The name of the repository.
"*
owner" The owner of the repository.
"E
providerType" 1The provider type of the repository association.
"�

repository�:�
�
codegurureviewerRepositoryAssociationRepositoryTaws:codegurureviewer/RepositoryAssociationRepository:RepositoryAssociationRepository�An object describing the repository to associate. Valid values: `bitbucket`, `codecommit`, `github_enterprise_server`, or `s3_bucket`. Block is documented below. Note: for repositories that leverage CodeStar connections (ex. `bitbucket`, `github_enterprise_server`) the connection must be in `Available` status prior to creating this resource.

The following arguments are optional:
"�
s3RepositoryDetails�*�:�
�
codegurureviewer'RepositoryAssociationS3RepositoryDetaildaws:codegurureviewer/RepositoryAssociationS3RepositoryDetail:RepositoryAssociationS3RepositoryDetail"6
state" )The state of the repository association.
"\
stateReason" IA description of why the repository association is in the current state.
"
tagsB2" "
tagsAll2" *�1
T
codepipelineCustomActionType2aws:codepipeline/customActionType:CustomActionType� Provides a CodeDeploy CustomActionType

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.codepipeline.CustomActionType("example", {
    category: "Build",
    inputArtifactDetails: {
        maximumCount: 1,
        minimumCount: 0,
    },
    outputArtifactDetails: {
        maximumCount: 1,
        minimumCount: 0,
    },
    providerName: "example",
    version: "1",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codepipeline.CustomActionType("example",
    category="Build",
    input_artifact_details={
        "maximum_count": 1,
        "minimum_count": 0,
    },
    output_artifact_details={
        "maximum_count": 1,
        "minimum_count": 0,
    },
    provider_name="example",
    version="1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CodePipeline.CustomActionType("example", new()
    {
        Category = "Build",
        InputArtifactDetails = new Aws.CodePipeline.Inputs.CustomActionTypeInputArtifactDetailsArgs
        {
            MaximumCount = 1,
            MinimumCount = 0,
        },
        OutputArtifactDetails = new Aws.CodePipeline.Inputs.CustomActionTypeOutputArtifactDetailsArgs
        {
            MaximumCount = 1,
            MinimumCount = 0,
        },
        ProviderName = "example",
        Version = "1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codepipeline"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := codepipeline.NewCustomActionType(ctx, "example", &codepipeline.CustomActionTypeArgs{
			Category: pulumi.String("Build"),
			InputArtifactDetails: &codepipeline.CustomActionTypeInputArtifactDetailsArgs{
				MaximumCount: pulumi.Int(1),
				MinimumCount: pulumi.Int(0),
			},
			OutputArtifactDetails: &codepipeline.CustomActionTypeOutputArtifactDetailsArgs{
				MaximumCount: pulumi.Int(1),
				MinimumCount: pulumi.Int(0),
			},
			ProviderName: pulumi.String("example"),
			Version:      pulumi.String("1"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codepipeline.CustomActionType;
import com.pulumi.aws.codepipeline.CustomActionTypeArgs;
import com.pulumi.aws.codepipeline.inputs.CustomActionTypeInputArtifactDetailsArgs;
import com.pulumi.aws.codepipeline.inputs.CustomActionTypeOutputArtifactDetailsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new CustomActionType("example", CustomActionTypeArgs.builder()
            .category("Build")
            .inputArtifactDetails(CustomActionTypeInputArtifactDetailsArgs.builder()
                .maximumCount(1)
                .minimumCount(0)
                .build())
            .outputArtifactDetails(CustomActionTypeOutputArtifactDetailsArgs.builder()
                .maximumCount(1)
                .minimumCount(0)
                .build())
            .providerName("example")
            .version("1")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:codepipeline:CustomActionType
    properties:
      category: Build
      inputArtifactDetails:
        maximumCount: 1
        minimumCount: 0
      outputArtifactDetails:
        maximumCount: 1
        minimumCount: 0
      providerName: example
      version: '1'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodeDeploy CustomActionType using the `id`. For example:

```sh
$ pulumi import aws:codepipeline/customActionType:CustomActionType example Build:pulumi:1
```
{
category" kThe category of the custom action. Valid values: `Source`, `Build`, `Deploy`, `Test`, `Invoke`, `Approval`
�
configurationProperties�B�*�:�
�
codepipeline%CustomActionTypeConfigurationProperty\aws:codepipeline/CustomActionTypeConfigurationProperty:CustomActionTypeConfigurationPropertyBThe configuration properties for the custom action. Max 10 items.
�
inputArtifactDetails�:�
�
codepipeline$CustomActionTypeInputArtifactDetailsZaws:codepipeline/CustomActionTypeInputArtifactDetails:CustomActionTypeInputArtifactDetails�
outputArtifactDetails�:�
�
codepipeline%CustomActionTypeOutputArtifactDetails\aws:codepipeline/CustomActionTypeOutputArtifactDetails:CustomActionTypeOutputArtifactDetails
providerName" ~
settingsrBp:n
l
codepipelineCustomActionTypeSettingsBaws:codepipeline/CustomActionTypeSettings:CustomActionTypeSettings
tagsB2" 
version" "
arn" The action ARN.
"{
category" kThe category of the custom action. Valid values: `Source`, `Build`, `Deploy`, `Test`, `Invoke`, `Approval`
"�
configurationProperties�B�*�:�
�
codepipeline%CustomActionTypeConfigurationProperty\aws:codepipeline/CustomActionTypeConfigurationProperty:CustomActionTypeConfigurationPropertyBThe configuration properties for the custom action. Max 10 items.
"�
inputArtifactDetails�:�
�
codepipeline$CustomActionTypeInputArtifactDetailsZaws:codepipeline/CustomActionTypeInputArtifactDetails:CustomActionTypeInputArtifactDetails"�
outputArtifactDetails�:�
�
codepipeline%CustomActionTypeOutputArtifactDetails\aws:codepipeline/CustomActionTypeOutputArtifactDetails:CustomActionTypeOutputArtifactDetails"5
owner" (The creator of the action being called.
"
providerName" "~
settingsrBp:n
l
codepipelineCustomActionTypeSettingsBaws:codepipeline/CustomActionTypeSettings:CustomActionTypeSettings"
tagsB2" "�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"
version" *��
<
codepipelinePipeline"aws:codepipeline/pipeline:Pipeline��Provides a CodePipeline.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.codestarconnections.Connection("example", {
    name: "example-connection",
    providerType: "GitHub",
});
const codepipelineBucket = new aws.s3.BucketV2("codepipeline_bucket", {bucket: "test-bucket"});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["codepipeline.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const codepipelineRole = new aws.iam.Role("codepipeline_role", {
    name: "test-role",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const s3kmskey = aws.kms.getAlias({
    name: "alias/myKmsKey",
});
const codepipeline = new aws.codepipeline.Pipeline("codepipeline", {
    name: "tf-test-pipeline",
    roleArn: codepipelineRole.arn,
    artifactStores: [{
        location: codepipelineBucket.bucket,
        type: "S3",
        encryptionKey: {
            id: s3kmskey.then(s3kmskey => s3kmskey.arn),
            type: "KMS",
        },
    }],
    stages: [
        {
            name: "Source",
            actions: [{
                name: "Source",
                category: "Source",
                owner: "AWS",
                provider: "CodeStarSourceConnection",
                version: "1",
                outputArtifacts: ["source_output"],
                configuration: {
                    ConnectionArn: example.arn,
                    FullRepositoryId: "my-organization/example",
                    BranchName: "main",
                },
            }],
        },
        {
            name: "Build",
            actions: [{
                name: "Build",
                category: "Build",
                owner: "AWS",
                provider: "CodeBuild",
                inputArtifacts: ["source_output"],
                outputArtifacts: ["build_output"],
                version: "1",
                configuration: {
                    ProjectName: "test",
                },
            }],
        },
        {
            name: "Deploy",
            actions: [{
                name: "Deploy",
                category: "Deploy",
                owner: "AWS",
                provider: "CloudFormation",
                inputArtifacts: ["build_output"],
                version: "1",
                configuration: {
                    ActionMode: "REPLACE_ON_FAILURE",
                    Capabilities: "CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM",
                    OutputFileName: "CreateStackOutput.json",
                    StackName: "MyStack",
                    TemplatePath: "build_output::sam-templated.yaml",
                },
            }],
        },
    ],
});
const codepipelineBucketPab = new aws.s3.BucketPublicAccessBlock("codepipeline_bucket_pab", {
    bucket: codepipelineBucket.id,
    blockPublicAcls: true,
    blockPublicPolicy: true,
    ignorePublicAcls: true,
    restrictPublicBuckets: true,
});
const codepipelinePolicy = aws.iam.getPolicyDocumentOutput({
    statements: [
        {
            effect: "Allow",
            actions: [
                "s3:GetObject",
                "s3:GetObjectVersion",
                "s3:GetBucketVersioning",
                "s3:PutObjectAcl",
                "s3:PutObject",
            ],
            resources: [
                codepipelineBucket.arn,
                pulumi.interpolate`${codepipelineBucket.arn}/*`,
            ],
        },
        {
            effect: "Allow",
            actions: ["codestar-connections:UseConnection"],
            resources: [example.arn],
        },
        {
            effect: "Allow",
            actions: [
                "codebuild:BatchGetBuilds",
                "codebuild:StartBuild",
            ],
            resources: ["*"],
        },
    ],
});
const codepipelinePolicyRolePolicy = new aws.iam.RolePolicy("codepipeline_policy", {
    name: "codepipeline_policy",
    role: codepipelineRole.id,
    policy: codepipelinePolicy.apply(codepipelinePolicy => codepipelinePolicy.json),
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codestarconnections.Connection("example",
    name="example-connection",
    provider_type="GitHub")
codepipeline_bucket = aws.s3.BucketV2("codepipeline_bucket", bucket="test-bucket")
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["codepipeline.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
codepipeline_role = aws.iam.Role("codepipeline_role",
    name="test-role",
    assume_role_policy=assume_role.json)
s3kmskey = aws.kms.get_alias(name="alias/myKmsKey")
codepipeline = aws.codepipeline.Pipeline("codepipeline",
    name="tf-test-pipeline",
    role_arn=codepipeline_role.arn,
    artifact_stores=[{
        "location": codepipeline_bucket.bucket,
        "type": "S3",
        "encryption_key": {
            "id": s3kmskey.arn,
            "type": "KMS",
        },
    }],
    stages=[
        {
            "name": "Source",
            "actions": [{
                "name": "Source",
                "category": "Source",
                "owner": "AWS",
                "provider": "CodeStarSourceConnection",
                "version": "1",
                "output_artifacts": ["source_output"],
                "configuration": {
                    "ConnectionArn": example.arn,
                    "FullRepositoryId": "my-organization/example",
                    "BranchName": "main",
                },
            }],
        },
        {
            "name": "Build",
            "actions": [{
                "name": "Build",
                "category": "Build",
                "owner": "AWS",
                "provider": "CodeBuild",
                "input_artifacts": ["source_output"],
                "output_artifacts": ["build_output"],
                "version": "1",
                "configuration": {
                    "ProjectName": "test",
                },
            }],
        },
        {
            "name": "Deploy",
            "actions": [{
                "name": "Deploy",
                "category": "Deploy",
                "owner": "AWS",
                "provider": "CloudFormation",
                "input_artifacts": ["build_output"],
                "version": "1",
                "configuration": {
                    "ActionMode": "REPLACE_ON_FAILURE",
                    "Capabilities": "CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM",
                    "OutputFileName": "CreateStackOutput.json",
                    "StackName": "MyStack",
                    "TemplatePath": "build_output::sam-templated.yaml",
                },
            }],
        },
    ])
codepipeline_bucket_pab = aws.s3.BucketPublicAccessBlock("codepipeline_bucket_pab",
    bucket=codepipeline_bucket.id,
    block_public_acls=True,
    block_public_policy=True,
    ignore_public_acls=True,
    restrict_public_buckets=True)
codepipeline_policy = aws.iam.get_policy_document_output(statements=[
    {
        "effect": "Allow",
        "actions": [
            "s3:GetObject",
            "s3:GetObjectVersion",
            "s3:GetBucketVersioning",
            "s3:PutObjectAcl",
            "s3:PutObject",
        ],
        "resources": [
            codepipeline_bucket.arn,
            codepipeline_bucket.arn.apply(lambda arn: f"{arn}/*"),
        ],
    },
    {
        "effect": "Allow",
        "actions": ["codestar-connections:UseConnection"],
        "resources": [example.arn],
    },
    {
        "effect": "Allow",
        "actions": [
            "codebuild:BatchGetBuilds",
            "codebuild:StartBuild",
        ],
        "resources": ["*"],
    },
])
codepipeline_policy_role_policy = aws.iam.RolePolicy("codepipeline_policy",
    name="codepipeline_policy",
    role=codepipeline_role.id,
    policy=codepipeline_policy.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CodeStarConnections.Connection("example", new()
    {
        Name = "example-connection",
        ProviderType = "GitHub",
    });

    var codepipelineBucket = new Aws.S3.BucketV2("codepipeline_bucket", new()
    {
        Bucket = "test-bucket",
    });

    var assumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "codepipeline.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var codepipelineRole = new Aws.Iam.Role("codepipeline_role", new()
    {
        Name = "test-role",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var s3kmskey = Aws.Kms.GetAlias.Invoke(new()
    {
        Name = "alias/myKmsKey",
    });

    var codepipeline = new Aws.CodePipeline.Pipeline("codepipeline", new()
    {
        Name = "tf-test-pipeline",
        RoleArn = codepipelineRole.Arn,
        ArtifactStores = new[]
        {
            new Aws.CodePipeline.Inputs.PipelineArtifactStoreArgs
            {
                Location = codepipelineBucket.Bucket,
                Type = "S3",
                EncryptionKey = new Aws.CodePipeline.Inputs.PipelineArtifactStoreEncryptionKeyArgs
                {
                    Id = s3kmskey.Apply(getAliasResult => getAliasResult.Arn),
                    Type = "KMS",
                },
            },
        },
        Stages = new[]
        {
            new Aws.CodePipeline.Inputs.PipelineStageArgs
            {
                Name = "Source",
                Actions = new[]
                {
                    new Aws.CodePipeline.Inputs.PipelineStageActionArgs
                    {
                        Name = "Source",
                        Category = "Source",
                        Owner = "AWS",
                        Provider = "CodeStarSourceConnection",
                        Version = "1",
                        OutputArtifacts = new[]
                        {
                            "source_output",
                        },
                        Configuration = 
                        {
                            { "ConnectionArn", example.Arn },
                            { "FullRepositoryId", "my-organization/example" },
                            { "BranchName", "main" },
                        },
                    },
                },
            },
            new Aws.CodePipeline.Inputs.PipelineStageArgs
            {
                Name = "Build",
                Actions = new[]
                {
                    new Aws.CodePipeline.Inputs.PipelineStageActionArgs
                    {
                        Name = "Build",
                        Category = "Build",
                        Owner = "AWS",
                        Provider = "CodeBuild",
                        InputArtifacts = new[]
                        {
                            "source_output",
                        },
                        OutputArtifacts = new[]
                        {
                            "build_output",
                        },
                        Version = "1",
                        Configuration = 
                        {
                            { "ProjectName", "test" },
                        },
                    },
                },
            },
            new Aws.CodePipeline.Inputs.PipelineStageArgs
            {
                Name = "Deploy",
                Actions = new[]
                {
                    new Aws.CodePipeline.Inputs.PipelineStageActionArgs
                    {
                        Name = "Deploy",
                        Category = "Deploy",
                        Owner = "AWS",
                        Provider = "CloudFormation",
                        InputArtifacts = new[]
                        {
                            "build_output",
                        },
                        Version = "1",
                        Configuration = 
                        {
                            { "ActionMode", "REPLACE_ON_FAILURE" },
                            { "Capabilities", "CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM" },
                            { "OutputFileName", "CreateStackOutput.json" },
                            { "StackName", "MyStack" },
                            { "TemplatePath", "build_output::sam-templated.yaml" },
                        },
                    },
                },
            },
        },
    });

    var codepipelineBucketPab = new Aws.S3.BucketPublicAccessBlock("codepipeline_bucket_pab", new()
    {
        Bucket = codepipelineBucket.Id,
        BlockPublicAcls = true,
        BlockPublicPolicy = true,
        IgnorePublicAcls = true,
        RestrictPublicBuckets = true,
    });

    var codepipelinePolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "s3:GetObject",
                    "s3:GetObjectVersion",
                    "s3:GetBucketVersioning",
                    "s3:PutObjectAcl",
                    "s3:PutObject",
                },
                Resources = new[]
                {
                    codepipelineBucket.Arn,
                    $"{codepipelineBucket.Arn}/*",
                },
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "codestar-connections:UseConnection",
                },
                Resources = new[]
                {
                    example.Arn,
                },
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "codebuild:BatchGetBuilds",
                    "codebuild:StartBuild",
                },
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var codepipelinePolicyRolePolicy = new Aws.Iam.RolePolicy("codepipeline_policy", new()
    {
        Name = "codepipeline_policy",
        Role = codepipelineRole.Id,
        Policy = codepipelinePolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codepipeline"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codestarconnections"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kms"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := codestarconnections.NewConnection(ctx, "example", &codestarconnections.ConnectionArgs{
			Name:         pulumi.String("example-connection"),
			ProviderType: pulumi.String("GitHub"),
		})
		if err != nil {
			return err
		}
		codepipelineBucket, err := s3.NewBucketV2(ctx, "codepipeline_bucket", &s3.BucketV2Args{
			Bucket: pulumi.String("test-bucket"),
		})
		if err != nil {
			return err
		}
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"codepipeline.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		codepipelineRole, err := iam.NewRole(ctx, "codepipeline_role", &iam.RoleArgs{
			Name:             pulumi.String("test-role"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		s3kmskey, err := kms.LookupAlias(ctx, &kms.LookupAliasArgs{
			Name: "alias/myKmsKey",
		}, nil)
		if err != nil {
			return err
		}
		_, err = codepipeline.NewPipeline(ctx, "codepipeline", &codepipeline.PipelineArgs{
			Name:    pulumi.String("tf-test-pipeline"),
			RoleArn: codepipelineRole.Arn,
			ArtifactStores: codepipeline.PipelineArtifactStoreArray{
				&codepipeline.PipelineArtifactStoreArgs{
					Location: codepipelineBucket.Bucket,
					Type:     pulumi.String("S3"),
					EncryptionKey: &codepipeline.PipelineArtifactStoreEncryptionKeyArgs{
						Id:   pulumi.String(s3kmskey.Arn),
						Type: pulumi.String("KMS"),
					},
				},
			},
			Stages: codepipeline.PipelineStageArray{
				&codepipeline.PipelineStageArgs{
					Name: pulumi.String("Source"),
					Actions: codepipeline.PipelineStageActionArray{
						&codepipeline.PipelineStageActionArgs{
							Name:     pulumi.String("Source"),
							Category: pulumi.String("Source"),
							Owner:    pulumi.String("AWS"),
							Provider: pulumi.String("CodeStarSourceConnection"),
							Version:  pulumi.String("1"),
							OutputArtifacts: pulumi.StringArray{
								pulumi.String("source_output"),
							},
							Configuration: pulumi.StringMap{
								"ConnectionArn":    example.Arn,
								"FullRepositoryId": pulumi.String("my-organization/example"),
								"BranchName":       pulumi.String("main"),
							},
						},
					},
				},
				&codepipeline.PipelineStageArgs{
					Name: pulumi.String("Build"),
					Actions: codepipeline.PipelineStageActionArray{
						&codepipeline.PipelineStageActionArgs{
							Name:     pulumi.String("Build"),
							Category: pulumi.String("Build"),
							Owner:    pulumi.String("AWS"),
							Provider: pulumi.String("CodeBuild"),
							InputArtifacts: pulumi.StringArray{
								pulumi.String("source_output"),
							},
							OutputArtifacts: pulumi.StringArray{
								pulumi.String("build_output"),
							},
							Version: pulumi.String("1"),
							Configuration: pulumi.StringMap{
								"ProjectName": pulumi.String("test"),
							},
						},
					},
				},
				&codepipeline.PipelineStageArgs{
					Name: pulumi.String("Deploy"),
					Actions: codepipeline.PipelineStageActionArray{
						&codepipeline.PipelineStageActionArgs{
							Name:     pulumi.String("Deploy"),
							Category: pulumi.String("Deploy"),
							Owner:    pulumi.String("AWS"),
							Provider: pulumi.String("CloudFormation"),
							InputArtifacts: pulumi.StringArray{
								pulumi.String("build_output"),
							},
							Version: pulumi.String("1"),
							Configuration: pulumi.StringMap{
								"ActionMode":     pulumi.String("REPLACE_ON_FAILURE"),
								"Capabilities":   pulumi.String("CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM"),
								"OutputFileName": pulumi.String("CreateStackOutput.json"),
								"StackName":      pulumi.String("MyStack"),
								"TemplatePath":   pulumi.String("build_output::sam-templated.yaml"),
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = s3.NewBucketPublicAccessBlock(ctx, "codepipeline_bucket_pab", &s3.BucketPublicAccessBlockArgs{
			Bucket:                codepipelineBucket.ID(),
			BlockPublicAcls:       pulumi.Bool(true),
			BlockPublicPolicy:     pulumi.Bool(true),
			IgnorePublicAcls:      pulumi.Bool(true),
			RestrictPublicBuckets: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		codepipelinePolicy := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Effect: pulumi.String("Allow"),
					Actions: pulumi.StringArray{
						pulumi.String("s3:GetObject"),
						pulumi.String("s3:GetObjectVersion"),
						pulumi.String("s3:GetBucketVersioning"),
						pulumi.String("s3:PutObjectAcl"),
						pulumi.String("s3:PutObject"),
					},
					Resources: pulumi.StringArray{
						codepipelineBucket.Arn,
						codepipelineBucket.Arn.ApplyT(func(arn string) (string, error) {
							return fmt.Sprintf("%v/*", arn), nil
						}).(pulumi.StringOutput),
					},
				},
				&iam.GetPolicyDocumentStatementArgs{
					Effect: pulumi.String("Allow"),
					Actions: pulumi.StringArray{
						pulumi.String("codestar-connections:UseConnection"),
					},
					Resources: pulumi.StringArray{
						example.Arn,
					},
				},
				&iam.GetPolicyDocumentStatementArgs{
					Effect: pulumi.String("Allow"),
					Actions: pulumi.StringArray{
						pulumi.String("codebuild:BatchGetBuilds"),
						pulumi.String("codebuild:StartBuild"),
					},
					Resources: pulumi.StringArray{
						pulumi.String("*"),
					},
				},
			},
		}, nil)
		_, err = iam.NewRolePolicy(ctx, "codepipeline_policy", &iam.RolePolicyArgs{
			Name: pulumi.String("codepipeline_policy"),
			Role: codepipelineRole.ID(),
			Policy: pulumi.String(codepipelinePolicy.ApplyT(func(codepipelinePolicy iam.GetPolicyDocumentResult) (*string, error) {
				return &codepipelinePolicy.Json, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codestarconnections.Connection;
import com.pulumi.aws.codestarconnections.ConnectionArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.kms.KmsFunctions;
import com.pulumi.aws.kms.inputs.GetAliasArgs;
import com.pulumi.aws.codepipeline.Pipeline;
import com.pulumi.aws.codepipeline.PipelineArgs;
import com.pulumi.aws.codepipeline.inputs.PipelineArtifactStoreArgs;
import com.pulumi.aws.codepipeline.inputs.PipelineArtifactStoreEncryptionKeyArgs;
import com.pulumi.aws.codepipeline.inputs.PipelineStageArgs;
import com.pulumi.aws.s3.BucketPublicAccessBlock;
import com.pulumi.aws.s3.BucketPublicAccessBlockArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Connection("example", ConnectionArgs.builder()
            .name("example-connection")
            .providerType("GitHub")
            .build());

        var codepipelineBucket = new BucketV2("codepipelineBucket", BucketV2Args.builder()
            .bucket("test-bucket")
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("codepipeline.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var codepipelineRole = new Role("codepipelineRole", RoleArgs.builder()
            .name("test-role")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        final var s3kmskey = KmsFunctions.getAlias(GetAliasArgs.builder()
            .name("alias/myKmsKey")
            .build());

        var codepipeline = new Pipeline("codepipeline", PipelineArgs.builder()
            .name("tf-test-pipeline")
            .roleArn(codepipelineRole.arn())
            .artifactStores(PipelineArtifactStoreArgs.builder()
                .location(codepipelineBucket.bucket())
                .type("S3")
                .encryptionKey(PipelineArtifactStoreEncryptionKeyArgs.builder()
                    .id(s3kmskey.applyValue(getAliasResult -> getAliasResult.arn()))
                    .type("KMS")
                    .build())
                .build())
            .stages(            
                PipelineStageArgs.builder()
                    .name("Source")
                    .actions(PipelineStageActionArgs.builder()
                        .name("Source")
                        .category("Source")
                        .owner("AWS")
                        .provider("CodeStarSourceConnection")
                        .version("1")
                        .outputArtifacts("source_output")
                        .configuration(Map.ofEntries(
                            Map.entry("ConnectionArn", example.arn()),
                            Map.entry("FullRepositoryId", "my-organization/example"),
                            Map.entry("BranchName", "main")
                        ))
                        .build())
                    .build(),
                PipelineStageArgs.builder()
                    .name("Build")
                    .actions(PipelineStageActionArgs.builder()
                        .name("Build")
                        .category("Build")
                        .owner("AWS")
                        .provider("CodeBuild")
                        .inputArtifacts("source_output")
                        .outputArtifacts("build_output")
                        .version("1")
                        .configuration(Map.of("ProjectName", "test"))
                        .build())
                    .build(),
                PipelineStageArgs.builder()
                    .name("Deploy")
                    .actions(PipelineStageActionArgs.builder()
                        .name("Deploy")
                        .category("Deploy")
                        .owner("AWS")
                        .provider("CloudFormation")
                        .inputArtifacts("build_output")
                        .version("1")
                        .configuration(Map.ofEntries(
                            Map.entry("ActionMode", "REPLACE_ON_FAILURE"),
                            Map.entry("Capabilities", "CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM"),
                            Map.entry("OutputFileName", "CreateStackOutput.json"),
                            Map.entry("StackName", "MyStack"),
                            Map.entry("TemplatePath", "build_output::sam-templated.yaml")
                        ))
                        .build())
                    .build())
            .build());

        var codepipelineBucketPab = new BucketPublicAccessBlock("codepipelineBucketPab", BucketPublicAccessBlockArgs.builder()
            .bucket(codepipelineBucket.id())
            .blockPublicAcls(true)
            .blockPublicPolicy(true)
            .ignorePublicAcls(true)
            .restrictPublicBuckets(true)
            .build());

        final var codepipelinePolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(            
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .actions(                    
                        "s3:GetObject",
                        "s3:GetObjectVersion",
                        "s3:GetBucketVersioning",
                        "s3:PutObjectAcl",
                        "s3:PutObject")
                    .resources(                    
                        codepipelineBucket.arn(),
                        codepipelineBucket.arn().applyValue(arn -> String.format("%s/*", arn)))
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .actions("codestar-connections:UseConnection")
                    .resources(example.arn())
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .actions(                    
                        "codebuild:BatchGetBuilds",
                        "codebuild:StartBuild")
                    .resources("*")
                    .build())
            .build());

        var codepipelinePolicyRolePolicy = new RolePolicy("codepipelinePolicyRolePolicy", RolePolicyArgs.builder()
            .name("codepipeline_policy")
            .role(codepipelineRole.id())
            .policy(codepipelinePolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(codepipelinePolicy -> codepipelinePolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  codepipeline:
    type: aws:codepipeline:Pipeline
    properties:
      name: tf-test-pipeline
      roleArn: ${codepipelineRole.arn}
      artifactStores:
        - location: ${codepipelineBucket.bucket}
          type: S3
          encryptionKey:
            id: ${s3kmskey.arn}
            type: KMS
      stages:
        - name: Source
          actions:
            - name: Source
              category: Source
              owner: AWS
              provider: CodeStarSourceConnection
              version: '1'
              outputArtifacts:
                - source_output
              configuration:
                ConnectionArn: ${example.arn}
                FullRepositoryId: my-organization/example
                BranchName: main
        - name: Build
          actions:
            - name: Build
              category: Build
              owner: AWS
              provider: CodeBuild
              inputArtifacts:
                - source_output
              outputArtifacts:
                - build_output
              version: '1'
              configuration:
                ProjectName: test
        - name: Deploy
          actions:
            - name: Deploy
              category: Deploy
              owner: AWS
              provider: CloudFormation
              inputArtifacts:
                - build_output
              version: '1'
              configuration:
                ActionMode: REPLACE_ON_FAILURE
                Capabilities: CAPABILITY_AUTO_EXPAND,CAPABILITY_IAM
                OutputFileName: CreateStackOutput.json
                StackName: MyStack
                TemplatePath: build_output::sam-templated.yaml
  example:
    type: aws:codestarconnections:Connection
    properties:
      name: example-connection
      providerType: GitHub
  codepipelineBucket:
    type: aws:s3:BucketV2
    name: codepipeline_bucket
    properties:
      bucket: test-bucket
  codepipelineBucketPab:
    type: aws:s3:BucketPublicAccessBlock
    name: codepipeline_bucket_pab
    properties:
      bucket: ${codepipelineBucket.id}
      blockPublicAcls: true
      blockPublicPolicy: true
      ignorePublicAcls: true
      restrictPublicBuckets: true
  codepipelineRole:
    type: aws:iam:Role
    name: codepipeline_role
    properties:
      name: test-role
      assumeRolePolicy: ${assumeRole.json}
  codepipelinePolicyRolePolicy:
    type: aws:iam:RolePolicy
    name: codepipeline_policy
    properties:
      name: codepipeline_policy
      role: ${codepipelineRole.id}
      policy: ${codepipelinePolicy.json}
variables:
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - codepipeline.amazonaws.com
            actions:
              - sts:AssumeRole
  codepipelinePolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - s3:GetObject
              - s3:GetObjectVersion
              - s3:GetBucketVersioning
              - s3:PutObjectAcl
              - s3:PutObject
            resources:
              - ${codepipelineBucket.arn}
              - ${codepipelineBucket.arn}/*
          - effect: Allow
            actions:
              - codestar-connections:UseConnection
            resources:
              - ${example.arn}
          - effect: Allow
            actions:
              - codebuild:BatchGetBuilds
              - codebuild:StartBuild
            resources:
              - '*'
  s3kmskey:
    fn::invoke:
      function: aws:kms:getAlias
      arguments:
        name: alias/myKmsKey
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodePipelines using the name. For example:

```sh
$ pulumi import aws:codepipeline/pipeline:Pipeline foo example
```
�
artifactStoresi*g:e
c
codepipelinePipelineArtifactStore<aws:codepipeline/PipelineArtifactStore:PipelineArtifactStoreIOne or more artifact_store blocks. Artifact stores are documented below.
�
executionModeB" �The method that the pipeline will use to handle multiple executions. The default mode is `SUPERSEDED`. For value values, refer to the [AWS documentation](https://docs.aws.amazon.com/codepipeline/latest/APIReference/API_PipelineDeclaration.html#CodePipeline-Type-PipelineDeclaration-executionMode).

**Note:** `QUEUED` or `PARALLEL` mode can only be used with V2 pipelines.
(
nameB" The name of the pipeline.
g
pipelineTypeB" QType of the pipeline. Possible values are: `V1` and `V2`. Default value is `V1`.
�
roleArn" �A service role Amazon Resource Name (ARN) that grants AWS CodePipeline permission to make calls to AWS services on your behalf.
�
stagesQ*O:M
K
codepipelinePipelineStage,aws:codepipeline/PipelineStage:PipelineStage,A stage block. Stages are documented below.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
triggersYBW*U:S
Q
codepipelinePipelineTrigger0aws:codepipeline/PipelineTrigger:PipelineTriggerYA trigger block. Valid only when `pipeline_type` is `V2`. Triggers are documented below.
�
	variables\BZ*X:V
T
codepipelinePipelineVariable2aws:codepipeline/PipelineVariable:PipelineVariableiA pipeline-level variable block. Valid only when `pipeline_type` is `V2`. Variable are documented below.
"!
arn" The codepipeline ARN.
"�
artifactStoresi*g:e
c
codepipelinePipelineArtifactStore<aws:codepipeline/PipelineArtifactStore:PipelineArtifactStoreIOne or more artifact_store blocks. Artifact stores are documented below.
"�
executionModeB" �The method that the pipeline will use to handle multiple executions. The default mode is `SUPERSEDED`. For value values, refer to the [AWS documentation](https://docs.aws.amazon.com/codepipeline/latest/APIReference/API_PipelineDeclaration.html#CodePipeline-Type-PipelineDeclaration-executionMode).

**Note:** `QUEUED` or `PARALLEL` mode can only be used with V2 pipelines.
"&
name" The name of the pipeline.
"g
pipelineTypeB" QType of the pipeline. Possible values are: `V1` and `V2`. Default value is `V1`.
"�
roleArn" �A service role Amazon Resource Name (ARN) that grants AWS CodePipeline permission to make calls to AWS services on your behalf.
"�
stagesQ*O:M
K
codepipelinePipelineStage,aws:codepipeline/PipelineStage:PipelineStage,A stage block. Stages are documented below.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
triggersW*U:S
Q
codepipelinePipelineTrigger0aws:codepipeline/PipelineTrigger:PipelineTriggerYA trigger block. Valid only when `pipeline_type` is `V2`. Triggers are documented below.
"�
	variables\BZ*X:V
T
codepipelinePipelineVariable2aws:codepipeline/PipelineVariable:PipelineVariableiA pipeline-level variable block. Valid only when `pipeline_type` is `V2`. Variable are documented below.
*�"
9
codepipelineWebhook aws:codepipeline/webhook:Webhook�Provides a CodePipeline Webhook.

## Example Usage

<!--Start PulumiCodeChooser -->
```yaml
resources:
  bar:
    type: aws:codepipeline:Pipeline
    properties:
      name: tf-test-pipeline
      roleArn: ${barAwsIamRole.arn}
      artifactStores:
        - location: ${barAwsS3Bucket.bucket}
          type: S3
          encryptionKey:
            id: ${s3kmskey.arn}
            type: KMS
      stages:
        - name: Source
          actions:
            - name: Source
              category: Source
              owner: ThirdParty
              provider: GitHub
              version: '1'
              outputArtifacts:
                - test
              configuration:
                Owner: my-organization
                Repo: test
                Branch: master
        - name: Build
          actions:
            - name: Build
              category: Build
              owner: AWS
              provider: CodeBuild
              inputArtifacts:
                - test
              version: '1'
              configuration:
                ProjectName: test
  barWebhook:
    type: aws:codepipeline:Webhook
    name: bar
    properties:
      name: test-webhook-github-bar
      authentication: GITHUB_HMAC
      targetAction: Source
      targetPipeline: ${bar.name}
      authenticationConfiguration:
        secretToken: ${webhookSecret}
      filters:
        - jsonPath: $.ref
          matchEquals: refs/heads/{Branch}
  # Wire the CodePipeline webhook into a GitHub repository.
  barRepositoryWebhook:
    type: github:RepositoryWebhook
    name: bar
    properties:
      repository: ${repo.name}
      name: web
      configuration:
        url: ${barWebhook.url}
        contentType: json
        insecureSsl: true
        secret: ${webhookSecret}
      events:
        - push
variables:
  webhookSecret: super-secret
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodePipeline Webhooks using their ARN. For example:

```sh
$ pulumi import aws:codepipeline/webhook:Webhook example arn:aws:codepipeline:us-west-2:123456789012:webhook:example
```
l
authentication" VThe type of authentication  to use. One of `IP`, `GITHUB_HMAC`, or `UNAUTHENTICATED`.
�
authenticationConfiguration�B�:�
�
codepipeline"WebhookAuthenticationConfigurationVaws:codepipeline/WebhookAuthenticationConfiguration:WebhookAuthenticationConfigurationXAn `auth` block. Required for `IP` and `GITHUB_HMAC`. Auth blocks are documented below.
�
filtersQ*O:M
K
codepipelineWebhookFilter,aws:codepipeline/WebhookFilter:WebhookFilterAOne or more `filter` blocks. Filter blocks are documented below.
'
nameB" The name of the webhook.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
targetAction" �The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.
0
targetPipeline" The name of the pipeline.
"+
arn"  The CodePipeline webhook's ARN.
"l
authentication" VThe type of authentication  to use. One of `IP`, `GITHUB_HMAC`, or `UNAUTHENTICATED`.
"�
authenticationConfiguration�B�:�
�
codepipeline"WebhookAuthenticationConfigurationVaws:codepipeline/WebhookAuthenticationConfiguration:WebhookAuthenticationConfigurationXAn `auth` block. Required for `IP` and `GITHUB_HMAC`. Auth blocks are documented below.
"�
filtersQ*O:M
K
codepipelineWebhookFilter,aws:codepipeline/WebhookFilter:WebhookFilterAOne or more `filter` blocks. Filter blocks are documented below.
"%
name" The name of the webhook.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
targetAction" �The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.
"0
targetPipeline" The name of the pipeline.
"_
url" TThe CodePipeline webhook's URL. POST events to this endpoint to trigger the target.
*�]
P
codestarconnections
Connection-aws:codestarconnections/connection:Connection�NProvides a CodeStar Connection.

> **NOTE:** The `aws.codestarconnections.Connection` resource is created in the state `PENDING`. Authentication with the connection provider must be completed in the AWS Console. See the [AWS documentation](https://docs.aws.amazon.com/dtconsole/latest/userguide/connections-update.html) for details.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.codestarconnections.Connection("example", {
    name: "example-connection",
    providerType: "Bitbucket",
});
const examplePipeline = new aws.codepipeline.Pipeline("example", {
    artifactStores: [{}],
    stages: [
        {
            name: "Source",
            actions: [{
                name: "Source",
                category: "Source",
                owner: "AWS",
                provider: "CodeStarSourceConnection",
                version: "1",
                outputArtifacts: ["source_output"],
                configuration: {
                    ConnectionArn: example.arn,
                    FullRepositoryId: "my-organization/test",
                    BranchName: "main",
                },
            }],
        },
        {
            actions: [{}],
            name: "Build",
        },
        {
            actions: [{}],
            name: "Deploy",
        },
    ],
    name: "tf-test-pipeline",
    roleArn: codepipelineRole.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codestarconnections.Connection("example",
    name="example-connection",
    provider_type="Bitbucket")
example_pipeline = aws.codepipeline.Pipeline("example",
    artifact_stores=[{}],
    stages=[
        {
            "name": "Source",
            "actions": [{
                "name": "Source",
                "category": "Source",
                "owner": "AWS",
                "provider": "CodeStarSourceConnection",
                "version": "1",
                "output_artifacts": ["source_output"],
                "configuration": {
                    "ConnectionArn": example.arn,
                    "FullRepositoryId": "my-organization/test",
                    "BranchName": "main",
                },
            }],
        },
        {
            "actions": [{}],
            "name": "Build",
        },
        {
            "actions": [{}],
            "name": "Deploy",
        },
    ],
    name="tf-test-pipeline",
    role_arn=codepipeline_role["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CodeStarConnections.Connection("example", new()
    {
        Name = "example-connection",
        ProviderType = "Bitbucket",
    });

    var examplePipeline = new Aws.CodePipeline.Pipeline("example", new()
    {
        ArtifactStores = new[]
        {
            null,
        },
        Stages = new[]
        {
            new Aws.CodePipeline.Inputs.PipelineStageArgs
            {
                Name = "Source",
                Actions = new[]
                {
                    new Aws.CodePipeline.Inputs.PipelineStageActionArgs
                    {
                        Name = "Source",
                        Category = "Source",
                        Owner = "AWS",
                        Provider = "CodeStarSourceConnection",
                        Version = "1",
                        OutputArtifacts = new[]
                        {
                            "source_output",
                        },
                        Configuration = 
                        {
                            { "ConnectionArn", example.Arn },
                            { "FullRepositoryId", "my-organization/test" },
                            { "BranchName", "main" },
                        },
                    },
                },
            },
            new Aws.CodePipeline.Inputs.PipelineStageArgs
            {
                Actions = new[]
                {
                    null,
                },
                Name = "Build",
            },
            new Aws.CodePipeline.Inputs.PipelineStageArgs
            {
                Actions = new[]
                {
                    null,
                },
                Name = "Deploy",
            },
        },
        Name = "tf-test-pipeline",
        RoleArn = codepipelineRole.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codepipeline"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codestarconnections"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := codestarconnections.NewConnection(ctx, "example", &codestarconnections.ConnectionArgs{
			Name:         pulumi.String("example-connection"),
			ProviderType: pulumi.String("Bitbucket"),
		})
		if err != nil {
			return err
		}
		_, err = codepipeline.NewPipeline(ctx, "example", &codepipeline.PipelineArgs{
			ArtifactStores: codepipeline.PipelineArtifactStoreArray{
				&codepipeline.PipelineArtifactStoreArgs{},
			},
			Stages: codepipeline.PipelineStageArray{
				&codepipeline.PipelineStageArgs{
					Name: pulumi.String("Source"),
					Actions: codepipeline.PipelineStageActionArray{
						&codepipeline.PipelineStageActionArgs{
							Name:     pulumi.String("Source"),
							Category: pulumi.String("Source"),
							Owner:    pulumi.String("AWS"),
							Provider: pulumi.String("CodeStarSourceConnection"),
							Version:  pulumi.String("1"),
							OutputArtifacts: pulumi.StringArray{
								pulumi.String("source_output"),
							},
							Configuration: pulumi.StringMap{
								"ConnectionArn":    example.Arn,
								"FullRepositoryId": pulumi.String("my-organization/test"),
								"BranchName":       pulumi.String("main"),
							},
						},
					},
				},
				&codepipeline.PipelineStageArgs{
					Actions: codepipeline.PipelineStageActionArray{
						&codepipeline.PipelineStageActionArgs{},
					},
					Name: pulumi.String("Build"),
				},
				&codepipeline.PipelineStageArgs{
					Actions: codepipeline.PipelineStageActionArray{
						&codepipeline.PipelineStageActionArgs{},
					},
					Name: pulumi.String("Deploy"),
				},
			},
			Name:    pulumi.String("tf-test-pipeline"),
			RoleArn: pulumi.Any(codepipelineRole.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codestarconnections.Connection;
import com.pulumi.aws.codestarconnections.ConnectionArgs;
import com.pulumi.aws.codepipeline.Pipeline;
import com.pulumi.aws.codepipeline.PipelineArgs;
import com.pulumi.aws.codepipeline.inputs.PipelineArtifactStoreArgs;
import com.pulumi.aws.codepipeline.inputs.PipelineStageArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Connection("example", ConnectionArgs.builder()
            .name("example-connection")
            .providerType("Bitbucket")
            .build());

        var examplePipeline = new Pipeline("examplePipeline", PipelineArgs.builder()
            .artifactStores()
            .stages(            
                PipelineStageArgs.builder()
                    .name("Source")
                    .actions(PipelineStageActionArgs.builder()
                        .name("Source")
                        .category("Source")
                        .owner("AWS")
                        .provider("CodeStarSourceConnection")
                        .version("1")
                        .outputArtifacts("source_output")
                        .configuration(Map.ofEntries(
                            Map.entry("ConnectionArn", example.arn()),
                            Map.entry("FullRepositoryId", "my-organization/test"),
                            Map.entry("BranchName", "main")
                        ))
                        .build())
                    .build(),
                PipelineStageArgs.builder()
                    .actions()
                    .name("Build")
                    .build(),
                PipelineStageArgs.builder()
                    .actions()
                    .name("Deploy")
                    .build())
            .name("tf-test-pipeline")
            .roleArn(codepipelineRole.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:codestarconnections:Connection
    properties:
      name: example-connection
      providerType: Bitbucket
  examplePipeline:
    type: aws:codepipeline:Pipeline
    name: example
    properties:
      artifactStores:
        - {}
      stages:
        - name: Source
          actions:
            - name: Source
              category: Source
              owner: AWS
              provider: CodeStarSourceConnection
              version: '1'
              outputArtifacts:
                - source_output
              configuration:
                ConnectionArn: ${example.arn}
                FullRepositoryId: my-organization/test
                BranchName: main
        - actions:
            - {}
          name: Build
        - actions:
            - {}
          name: Deploy
      name: tf-test-pipeline
      roleArn: ${codepipelineRole.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodeStar connections using the ARN. For example:

```sh
$ pulumi import aws:codestarconnections/connection:Connection test-connection arn:aws:codestar-connections:us-west-1:0123456789:connection/79d4d357-a2ee-41e4-b350-2fe39ae59448
```
{
hostArnB" jThe Amazon Resource Name (ARN) of the host associated with the connection. Conflicts with `provider_type`
�
nameB" �The name of the connection to be created. The name must be unique in the calling AWS account. Changing `name` will create a new resource.
�
providerTypeB" �The name of the external provider where your third-party code repository is configured. Valid values are `Bitbucket`, `GitHub`, `GitHubEnterpriseServer`, `GitLab` or `GitLabSelfManaged`. Changing `provider_type` will create a new resource. Conflicts with `host_arn`
�
tagsB2" �Map of key-value resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"(
arn" The codestar connection ARN.
"p
connectionStatus" XThe codestar connection status. Possible values are `PENDING`, `AVAILABLE` and `ERROR`.
"{
hostArnB" jThe Amazon Resource Name (ARN) of the host associated with the connection. Conflicts with `provider_type`
"�
name" �The name of the connection to be created. The name must be unique in the calling AWS account. Changing `name` will create a new resource.
"�
providerType" �The name of the external provider where your third-party code repository is configured. Valid values are `Bitbucket`, `GitHub`, `GitHubEnterpriseServer`, `GitLab` or `GitLabSelfManaged`. Changing `provider_type` will create a new resource. Conflicts with `host_arn`
"�
tagsB2" �Map of key-value resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�#
>
codestarconnectionsHost!aws:codestarconnections/host:Host�Provides a CodeStar Host.

> **NOTE:** The `aws.codestarconnections.Host` resource is created in the state `PENDING`. Authentication with the host provider must be completed in the AWS Console. For more information visit [Set up a pending host](https://docs.aws.amazon.com/dtconsole/latest/userguide/connections-host-setup.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.codestarconnections.Host("example", {
    name: "example-host",
    providerEndpoint: "https://example.com",
    providerType: "GitHubEnterpriseServer",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codestarconnections.Host("example",
    name="example-host",
    provider_endpoint="https://example.com",
    provider_type="GitHubEnterpriseServer")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CodeStarConnections.Host("example", new()
    {
        Name = "example-host",
        ProviderEndpoint = "https://example.com",
        ProviderType = "GitHubEnterpriseServer",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codestarconnections"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := codestarconnections.NewHost(ctx, "example", &codestarconnections.HostArgs{
			Name:             pulumi.String("example-host"),
			ProviderEndpoint: pulumi.String("https://example.com"),
			ProviderType:     pulumi.String("GitHubEnterpriseServer"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codestarconnections.Host;
import com.pulumi.aws.codestarconnections.HostArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Host("example", HostArgs.builder()
            .name("example-host")
            .providerEndpoint("https://example.com")
            .providerType("GitHubEnterpriseServer")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:codestarconnections:Host
    properties:
      name: example-host
      providerEndpoint: https://example.com
      providerType: GitHubEnterpriseServer
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodeStar Host using the ARN. For example:

```sh
$ pulumi import aws:codestarconnections/host:Host example-host arn:aws:codestar-connections:us-west-1:0123456789:host/79d4d357-a2ee-41e4-b350-2fe39ae59448
```
f
nameB" XThe name of the host to be created. The name must be unique in the calling AWS account.
n
providerEndpoint" VThe endpoint of the infrastructure to be represented by the host after it is created.
l
providerType" XThe name of the external provider where your third-party code repository is configured.
�
vpcConfigurationtBr:p
n
codestarconnectionsHostVpcConfigurationAaws:codestarconnections/HostVpcConfiguration:HostVpcConfiguration�The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
""
arn" The CodeStar Host ARN.
"d
name" XThe name of the host to be created. The name must be unique in the calling AWS account.
"n
providerEndpoint" VThe endpoint of the infrastructure to be represented by the host after it is created.
"l
providerType" XThe name of the external provider where your third-party code repository is configured.
"�
status" �The CodeStar Host status. Possible values are `PENDING`, `AVAILABLE`, `VPC_CONFIG_DELETING`, `VPC_CONFIG_INITIALIZING`, and `VPC_CONFIG_FAILED_INITIALIZATION`.
"�
vpcConfigurationtBr:p
n
codestarconnectionsHostVpcConfigurationAaws:codestarconnections/HostVpcConfiguration:HostVpcConfiguration�The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
*�^
f
codestarnotificationsNotificationRule;aws:codestarnotifications/notificationRule:NotificationRule�JProvides a CodeStar Notifications Rule.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const code = new aws.codecommit.Repository("code", {repositoryName: "example-code-repo"});
const notif = new aws.sns.Topic("notif", {name: "notification"});
const notifAccess = notif.arn.apply(arn => aws.iam.getPolicyDocumentOutput({
    statements: [{
        actions: ["sns:Publish"],
        principals: [{
            type: "Service",
            identifiers: ["codestar-notifications.amazonaws.com"],
        }],
        resources: [arn],
    }],
}));
const _default = new aws.sns.TopicPolicy("default", {
    arn: notif.arn,
    policy: notifAccess.apply(notifAccess => notifAccess.json),
});
const commits = new aws.codestarnotifications.NotificationRule("commits", {
    detailType: "BASIC",
    eventTypeIds: ["codecommit-repository-comments-on-commits"],
    name: "example-code-repo-commits",
    resource: code.arn,
    targets: [{
        address: notif.arn,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

code = aws.codecommit.Repository("code", repository_name="example-code-repo")
notif = aws.sns.Topic("notif", name="notification")
notif_access = notif.arn.apply(lambda arn: aws.iam.get_policy_document_output(statements=[{
    "actions": ["sns:Publish"],
    "principals": [{
        "type": "Service",
        "identifiers": ["codestar-notifications.amazonaws.com"],
    }],
    "resources": [arn],
}]))
default = aws.sns.TopicPolicy("default",
    arn=notif.arn,
    policy=notif_access.json)
commits = aws.codestarnotifications.NotificationRule("commits",
    detail_type="BASIC",
    event_type_ids=["codecommit-repository-comments-on-commits"],
    name="example-code-repo-commits",
    resource=code.arn,
    targets=[{
        "address": notif.arn,
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var code = new Aws.CodeCommit.Repository("code", new()
    {
        RepositoryName = "example-code-repo",
    });

    var notif = new Aws.Sns.Topic("notif", new()
    {
        Name = "notification",
    });

    var notifAccess = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sns:Publish",
                },
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "codestar-notifications.amazonaws.com",
                        },
                    },
                },
                Resources = new[]
                {
                    notif.Arn,
                },
            },
        },
    });

    var @default = new Aws.Sns.TopicPolicy("default", new()
    {
        Arn = notif.Arn,
        Policy = notifAccess.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var commits = new Aws.CodeStarNotifications.NotificationRule("commits", new()
    {
        DetailType = "BASIC",
        EventTypeIds = new[]
        {
            "codecommit-repository-comments-on-commits",
        },
        Name = "example-code-repo-commits",
        Resource = code.Arn,
        Targets = new[]
        {
            new Aws.CodeStarNotifications.Inputs.NotificationRuleTargetArgs
            {
                Address = notif.Arn,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codecommit"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codestarnotifications"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
code, err := codecommit.NewRepository(ctx, "code", &codecommit.RepositoryArgs{
RepositoryName: pulumi.String("example-code-repo"),
})
if err != nil {
return err
}
notif, err := sns.NewTopic(ctx, "notif", &sns.TopicArgs{
Name: pulumi.String("notification"),
})
if err != nil {
return err
}
notifAccess := notif.Arn.ApplyT(func(arn string) (iam.GetPolicyDocumentResult, error) {
return iam.GetPolicyDocumentResult(interface{}(iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"sns:Publish",
},
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "Service",
Identifiers: []string{
"codestar-notifications.amazonaws.com",
},
},
},
Resources: interface{}{
arn,
},
},
},
}, nil))), nil
}).(iam.GetPolicyDocumentResultOutput)
_, err = sns.NewTopicPolicy(ctx, "default", &sns.TopicPolicyArgs{
Arn: notif.Arn,
Policy: pulumi.String(notifAccess.ApplyT(func(notifAccess iam.GetPolicyDocumentResult) (*string, error) {
return &notifAccess.Json, nil
}).(pulumi.StringPtrOutput)),
})
if err != nil {
return err
}
_, err = codestarnotifications.NewNotificationRule(ctx, "commits", &codestarnotifications.NotificationRuleArgs{
DetailType: pulumi.String("BASIC"),
EventTypeIds: pulumi.StringArray{
pulumi.String("codecommit-repository-comments-on-commits"),
},
Name: pulumi.String("example-code-repo-commits"),
Resource: code.Arn,
Targets: codestarnotifications.NotificationRuleTargetArray{
&codestarnotifications.NotificationRuleTargetArgs{
Address: notif.Arn,
},
},
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codecommit.Repository;
import com.pulumi.aws.codecommit.RepositoryArgs;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.sns.TopicPolicy;
import com.pulumi.aws.sns.TopicPolicyArgs;
import com.pulumi.aws.codestarnotifications.NotificationRule;
import com.pulumi.aws.codestarnotifications.NotificationRuleArgs;
import com.pulumi.aws.codestarnotifications.inputs.NotificationRuleTargetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var code = new Repository("code", RepositoryArgs.builder()
            .repositoryName("example-code-repo")
            .build());

        var notif = new Topic("notif", TopicArgs.builder()
            .name("notification")
            .build());

        final var notifAccess = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sns:Publish")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("codestar-notifications.amazonaws.com")
                    .build())
                .resources(notif.arn())
                .build())
            .build());

        var default_ = new TopicPolicy("default", TopicPolicyArgs.builder()
            .arn(notif.arn())
            .policy(notifAccess.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(notifAccess -> notifAccess.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

        var commits = new NotificationRule("commits", NotificationRuleArgs.builder()
            .detailType("BASIC")
            .eventTypeIds("codecommit-repository-comments-on-commits")
            .name("example-code-repo-commits")
            .resource(code.arn())
            .targets(NotificationRuleTargetArgs.builder()
                .address(notif.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  code:
    type: aws:codecommit:Repository
    properties:
      repositoryName: example-code-repo
  notif:
    type: aws:sns:Topic
    properties:
      name: notification
  default:
    type: aws:sns:TopicPolicy
    properties:
      arn: ${notif.arn}
      policy: ${notifAccess.json}
  commits:
    type: aws:codestarnotifications:NotificationRule
    properties:
      detailType: BASIC
      eventTypeIds:
        - codecommit-repository-comments-on-commits
      name: example-code-repo-commits
      resource: ${code.arn}
      targets:
        - address: ${notif.arn}
variables:
  notifAccess:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sns:Publish
            principals:
              - type: Service
                identifiers:
                  - codestar-notifications.amazonaws.com
            resources:
              - ${notif.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodeStar notification rule using the ARN. For example:

```sh
$ pulumi import aws:codestarnotifications/notificationRule:NotificationRule foo arn:aws:codestar-notifications:us-west-1:0123456789:notificationrule/2cdc68a3-8f7c-4893-b6a5-45b362bd4f2b
```
�

detailType" oThe level of detail to include in the notifications for this resource. Possible values are `BASIC` and `FULL`.
�
eventTypeIds*" �A list of event types associated with this notification rule.
For list of allowed events see [here](https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api).
-
nameB" The name of notification rule.
Q
resource" AThe ARN of the resource to associate with the notification rule.
y
statusB" iThe status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
targets�B~*|:z
x
codestarnotificationsNotificationRuleTargetGaws:codestarnotifications/NotificationRuleTarget:NotificationRuleTarget�Configuration blocks containing notification target information. Can be specified multiple times. At least one target must be specified on creation.
"/
arn" $The codestar notification rule ARN.
"�

detailType" oThe level of detail to include in the notifications for this resource. Possible values are `BASIC` and `FULL`.
"�
eventTypeIds*" �A list of event types associated with this notification rule.
For list of allowed events see [here](https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api).
"+
name" The name of notification rule.
"Q
resource" AThe ARN of the resource to associate with the notification rule.
"y
statusB" iThe status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
targets�B~*|:z
x
codestarnotificationsNotificationRuleTargetGaws:codestarnotifications/NotificationRuleTarget:NotificationRuleTarget�Configuration blocks containing notification target information. Can be specified multiple times. At least one target must be specified on creation.
*�`
>
cognitoIdentityPool%aws:cognito/identityPool:IdentityPool�KProvides an AWS Cognito Identity Pool.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const _default = new aws.iam.SamlProvider("default", {
    name: "my-saml-provider",
    samlMetadataDocument: std.file({
        input: "saml-metadata.xml",
    }).then(invoke => invoke.result),
});
const main = new aws.cognito.IdentityPool("main", {
    identityPoolName: "identity pool",
    allowUnauthenticatedIdentities: false,
    allowClassicFlow: false,
    cognitoIdentityProviders: [
        {
            clientId: "6lhlkkfbfb4q5kpp90urffae",
            providerName: "cognito-idp.us-east-1.amazonaws.com/us-east-1_Tv0493apJ",
            serverSideTokenCheck: false,
        },
        {
            clientId: "7kodkvfqfb4qfkp39eurffae",
            providerName: "cognito-idp.us-east-1.amazonaws.com/eu-west-1_Zr231apJu",
            serverSideTokenCheck: false,
        },
    ],
    supportedLoginProviders: {
        "graph.facebook.com": "7346241598935552",
        "accounts.google.com": "123456789012.apps.googleusercontent.com",
    },
    samlProviderArns: [_default.arn],
    openidConnectProviderArns: ["arn:aws:iam::123456789012:oidc-provider/id.example.com"],
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

default = aws.iam.SamlProvider("default",
    name="my-saml-provider",
    saml_metadata_document=std.file(input="saml-metadata.xml").result)
main = aws.cognito.IdentityPool("main",
    identity_pool_name="identity pool",
    allow_unauthenticated_identities=False,
    allow_classic_flow=False,
    cognito_identity_providers=[
        {
            "client_id": "6lhlkkfbfb4q5kpp90urffae",
            "provider_name": "cognito-idp.us-east-1.amazonaws.com/us-east-1_Tv0493apJ",
            "server_side_token_check": False,
        },
        {
            "client_id": "7kodkvfqfb4qfkp39eurffae",
            "provider_name": "cognito-idp.us-east-1.amazonaws.com/eu-west-1_Zr231apJu",
            "server_side_token_check": False,
        },
    ],
    supported_login_providers={
        "graph.facebook.com": "7346241598935552",
        "accounts.google.com": "123456789012.apps.googleusercontent.com",
    },
    saml_provider_arns=[default.arn],
    openid_connect_provider_arns=["arn:aws:iam::123456789012:oidc-provider/id.example.com"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Iam.SamlProvider("default", new()
    {
        Name = "my-saml-provider",
        SamlMetadataDocument = Std.File.Invoke(new()
        {
            Input = "saml-metadata.xml",
        }).Apply(invoke => invoke.Result),
    });

    var main = new Aws.Cognito.IdentityPool("main", new()
    {
        IdentityPoolName = "identity pool",
        AllowUnauthenticatedIdentities = false,
        AllowClassicFlow = false,
        CognitoIdentityProviders = new[]
        {
            new Aws.Cognito.Inputs.IdentityPoolCognitoIdentityProviderArgs
            {
                ClientId = "6lhlkkfbfb4q5kpp90urffae",
                ProviderName = "cognito-idp.us-east-1.amazonaws.com/us-east-1_Tv0493apJ",
                ServerSideTokenCheck = false,
            },
            new Aws.Cognito.Inputs.IdentityPoolCognitoIdentityProviderArgs
            {
                ClientId = "7kodkvfqfb4qfkp39eurffae",
                ProviderName = "cognito-idp.us-east-1.amazonaws.com/eu-west-1_Zr231apJu",
                ServerSideTokenCheck = false,
            },
        },
        SupportedLoginProviders = 
        {
            { "graph.facebook.com", "7346241598935552" },
            { "accounts.google.com", "123456789012.apps.googleusercontent.com" },
        },
        SamlProviderArns = new[]
        {
            @default.Arn,
        },
        OpenidConnectProviderArns = new[]
        {
            "arn:aws:iam::123456789012:oidc-provider/id.example.com",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "saml-metadata.xml",
		}, nil)
		if err != nil {
			return err
		}
		_, err = iam.NewSamlProvider(ctx, "default", &iam.SamlProviderArgs{
			Name:                 pulumi.String("my-saml-provider"),
			SamlMetadataDocument: pulumi.String(invokeFile.Result),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewIdentityPool(ctx, "main", &cognito.IdentityPoolArgs{
			IdentityPoolName:               pulumi.String("identity pool"),
			AllowUnauthenticatedIdentities: pulumi.Bool(false),
			AllowClassicFlow:               pulumi.Bool(false),
			CognitoIdentityProviders: cognito.IdentityPoolCognitoIdentityProviderArray{
				&cognito.IdentityPoolCognitoIdentityProviderArgs{
					ClientId:             pulumi.String("6lhlkkfbfb4q5kpp90urffae"),
					ProviderName:         pulumi.String("cognito-idp.us-east-1.amazonaws.com/us-east-1_Tv0493apJ"),
					ServerSideTokenCheck: pulumi.Bool(false),
				},
				&cognito.IdentityPoolCognitoIdentityProviderArgs{
					ClientId:             pulumi.String("7kodkvfqfb4qfkp39eurffae"),
					ProviderName:         pulumi.String("cognito-idp.us-east-1.amazonaws.com/eu-west-1_Zr231apJu"),
					ServerSideTokenCheck: pulumi.Bool(false),
				},
			},
			SupportedLoginProviders: pulumi.StringMap{
				"graph.facebook.com":  pulumi.String("7346241598935552"),
				"accounts.google.com": pulumi.String("123456789012.apps.googleusercontent.com"),
			},
			SamlProviderArns: pulumi.StringArray{
				_default.Arn,
			},
			OpenidConnectProviderArns: pulumi.StringArray{
				pulumi.String("arn:aws:iam::123456789012:oidc-provider/id.example.com"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.iam.SamlProvider;
import com.pulumi.aws.iam.SamlProviderArgs;
import com.pulumi.aws.cognito.IdentityPool;
import com.pulumi.aws.cognito.IdentityPoolArgs;
import com.pulumi.aws.cognito.inputs.IdentityPoolCognitoIdentityProviderArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new SamlProvider("default", SamlProviderArgs.builder()
            .name("my-saml-provider")
            .samlMetadataDocument(StdFunctions.file(FileArgs.builder()
                .input("saml-metadata.xml")
                .build()).result())
            .build());

        var main = new IdentityPool("main", IdentityPoolArgs.builder()
            .identityPoolName("identity pool")
            .allowUnauthenticatedIdentities(false)
            .allowClassicFlow(false)
            .cognitoIdentityProviders(            
                IdentityPoolCognitoIdentityProviderArgs.builder()
                    .clientId("6lhlkkfbfb4q5kpp90urffae")
                    .providerName("cognito-idp.us-east-1.amazonaws.com/us-east-1_Tv0493apJ")
                    .serverSideTokenCheck(false)
                    .build(),
                IdentityPoolCognitoIdentityProviderArgs.builder()
                    .clientId("7kodkvfqfb4qfkp39eurffae")
                    .providerName("cognito-idp.us-east-1.amazonaws.com/eu-west-1_Zr231apJu")
                    .serverSideTokenCheck(false)
                    .build())
            .supportedLoginProviders(Map.ofEntries(
                Map.entry("graph.facebook.com", "7346241598935552"),
                Map.entry("accounts.google.com", "123456789012.apps.googleusercontent.com")
            ))
            .samlProviderArns(default_.arn())
            .openidConnectProviderArns("arn:aws:iam::123456789012:oidc-provider/id.example.com")
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:iam:SamlProvider
    properties:
      name: my-saml-provider
      samlMetadataDocument:
        fn::invoke:
          function: std:file
          arguments:
            input: saml-metadata.xml
          return: result
  main:
    type: aws:cognito:IdentityPool
    properties:
      identityPoolName: identity pool
      allowUnauthenticatedIdentities: false
      allowClassicFlow: false
      cognitoIdentityProviders:
        - clientId: 6lhlkkfbfb4q5kpp90urffae
          providerName: cognito-idp.us-east-1.amazonaws.com/us-east-1_Tv0493apJ
          serverSideTokenCheck: false
        - clientId: 7kodkvfqfb4qfkp39eurffae
          providerName: cognito-idp.us-east-1.amazonaws.com/eu-west-1_Zr231apJu
          serverSideTokenCheck: false
      supportedLoginProviders:
        graph.facebook.com: '7346241598935552'
        accounts.google.com: 123456789012.apps.googleusercontent.com
      samlProviderArns:
        - ${default.arn}
      openidConnectProviderArns:
        - arn:aws:iam::123456789012:oidc-provider/id.example.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito Identity Pool using its ID. For example:

```sh
$ pulumi import aws:cognito/identityPool:IdentityPool mypool us-west-2:1a234567-8901-234b-5cde-f6789g01h2i3
```
k
allowClassicFlowB
 QEnables or disables the classic / basic authentication flow. Default is `false`.
j
allowUnauthenticatedIdentitiesB
 BWhether the identity pool supports unauthenticated logins or not.
�
cognitoIdentityProviders�B�*�:�
�
cognito#IdentityPoolCognitoIdentityProviderSaws:cognito/IdentityPoolCognitoIdentityProvider:IdentityPoolCognitoIdentityProviderEAn array of Amazon Cognito Identity user pools and their client IDs.
�
developerProviderNameB" �The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your
backend and the Cognito service to communicate about the developer provider.
8
identityPoolName"  The Cognito Identity Pool name.
K
openidConnectProviderArnsB*" &Set of OpendID Connect provider ARNs.
m
samlProviderArnsB*" QAn array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
_
supportedLoginProvidersB2" <Key-Value pairs mapping provider names to provider app IDs.
�
tagsB2" �A map of tags to assign to the Identity Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"k
allowClassicFlowB
 QEnables or disables the classic / basic authentication flow. Default is `false`.
"j
allowUnauthenticatedIdentitiesB
 BWhether the identity pool supports unauthenticated logins or not.
")
arn" The ARN of the identity pool.
"�
cognitoIdentityProviders�B�*�:�
�
cognito#IdentityPoolCognitoIdentityProviderSaws:cognito/IdentityPoolCognitoIdentityProvider:IdentityPoolCognitoIdentityProviderEAn array of Amazon Cognito Identity user pools and their client IDs.
"�
developerProviderNameB" �The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your
backend and the Cognito service to communicate about the developer provider.
"8
identityPoolName"  The Cognito Identity Pool name.
"K
openidConnectProviderArnsB*" &Set of OpendID Connect provider ARNs.
"m
samlProviderArnsB*" QAn array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
"_
supportedLoginProvidersB2" <Key-Value pairs mapping provider names to provider app IDs.
"�
tagsB2" �A map of tags to assign to the Identity Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�
z
cognito IdentityPoolProviderPrincipalTagMaws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag�Provides an AWS Cognito Identity Principal Mapping.

## Import

Using `pulumi import`, import Cognito Identity Pool Roles Attachment using the Identity Pool ID and provider name. For example:

```sh
$ pulumi import aws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag example us-west-2_abc123:CorpAD
```
+
identityPoolId" An identity pool ID.
?
identityProviderName" #The name of the identity provider.
<
principalTagsB2" #String to string map of variables.
M
useDefaultsB
 8use default (username and clientID) attribute mappings.
"+
identityPoolId" An identity pool ID.
"?
identityProviderName" #The name of the identity provider.
"<
principalTagsB2" #String to string map of variables.
"M
useDefaultsB
 8use default (username and clientID) attribute mappings.
*�

h
cognitoIdentityPoolRoleAttachmentAaws:cognito/identityPoolRoleAttachment:IdentityPoolRoleAttachment�Provides an AWS Cognito Identity Pool Roles Attachment.

## Import

Using `pulumi import`, import Cognito Identity Pool Roles Attachment using the Identity Pool ID. For example:

```sh
$ pulumi import aws:cognito/identityPoolRoleAttachment:IdentityPoolRoleAttachment example us-west-2:b64805ad-cb56-40ba-9ffc-f5d8207e6d42
```
G
identityPoolId" 1An identity pool ID in the format `REGION_GUID`.
�
roleMappings�B�*�:�
�
cognito%IdentityPoolRoleAttachmentRoleMappingWaws:cognito/IdentityPoolRoleAttachmentRoleMapping:IdentityPoolRoleAttachmentRoleMappingA List of Role Mapping.
�
roles2" �The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.
"G
identityPoolId" 1An identity pool ID in the format `REGION_GUID`.
"�
roleMappings�B�*�:�
�
cognito%IdentityPoolRoleAttachmentRoleMappingWaws:cognito/IdentityPoolRoleAttachmentRoleMapping:IdentityPoolRoleAttachmentRoleMappingA List of Role Mapping.
"�
roles2" �The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.
*�6
J
cognitoIdentityProvider-aws:cognito/identityProvider:IdentityProvider�*Provides a Cognito User Identity Provider resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cognito.UserPool("example", {
    name: "example-pool",
    autoVerifiedAttributes: ["email"],
});
const exampleProvider = new aws.cognito.IdentityProvider("example_provider", {
    userPoolId: example.id,
    providerName: "Google",
    providerType: "Google",
    providerDetails: {
        authorize_scopes: "email",
        client_id: "your client_id",
        client_secret: "your client_secret",
    },
    attributeMapping: {
        email: "email",
        username: "sub",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.UserPool("example",
    name="example-pool",
    auto_verified_attributes=["email"])
example_provider = aws.cognito.IdentityProvider("example_provider",
    user_pool_id=example.id,
    provider_name="Google",
    provider_type="Google",
    provider_details={
        "authorize_scopes": "email",
        "client_id": "your client_id",
        "client_secret": "your client_secret",
    },
    attribute_mapping={
        "email": "email",
        "username": "sub",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example-pool",
        AutoVerifiedAttributes = new[]
        {
            "email",
        },
    });

    var exampleProvider = new Aws.Cognito.IdentityProvider("example_provider", new()
    {
        UserPoolId = example.Id,
        ProviderName = "Google",
        ProviderType = "Google",
        ProviderDetails = 
        {
            { "authorize_scopes", "email" },
            { "client_id", "your client_id" },
            { "client_secret", "your client_secret" },
        },
        AttributeMapping = 
        {
            { "email", "email" },
            { "username", "sub" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example-pool"),
			AutoVerifiedAttributes: pulumi.StringArray{
				pulumi.String("email"),
			},
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewIdentityProvider(ctx, "example_provider", &cognito.IdentityProviderArgs{
			UserPoolId:   example.ID(),
			ProviderName: pulumi.String("Google"),
			ProviderType: pulumi.String("Google"),
			ProviderDetails: pulumi.StringMap{
				"authorize_scopes": pulumi.String("email"),
				"client_id":        pulumi.String("your client_id"),
				"client_secret":    pulumi.String("your client_secret"),
			},
			AttributeMapping: pulumi.StringMap{
				"email":    pulumi.String("email"),
				"username": pulumi.String("sub"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.IdentityProvider;
import com.pulumi.aws.cognito.IdentityProviderArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .name("example-pool")
            .autoVerifiedAttributes("email")
            .build());

        var exampleProvider = new IdentityProvider("exampleProvider", IdentityProviderArgs.builder()
            .userPoolId(example.id())
            .providerName("Google")
            .providerType("Google")
            .providerDetails(Map.ofEntries(
                Map.entry("authorize_scopes", "email"),
                Map.entry("client_id", "your client_id"),
                Map.entry("client_secret", "your client_secret")
            ))
            .attributeMapping(Map.ofEntries(
                Map.entry("email", "email"),
                Map.entry("username", "sub")
            ))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:UserPool
    properties:
      name: example-pool
      autoVerifiedAttributes:
        - email
  exampleProvider:
    type: aws:cognito:IdentityProvider
    name: example_provider
    properties:
      userPoolId: ${example.id}
      providerName: Google
      providerType: Google
      providerDetails:
        authorize_scopes: email
        client_id: your client_id
        client_secret: your client_secret
      attributeMapping:
        email: email
        username: sub
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_cognito_identity_provider` resources using their User Pool ID and Provider Name. For example:

```sh
$ pulumi import aws:cognito/identityProvider:IdentityProvider example us-west-2_abc123:CorpAD
```
�
attributeMappingB2" �The map of attribute mapping of user pool attributes. [AttributeMapping in AWS API documentation](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-AttributeMapping)
:
idpIdentifiersB*"  The list of identity providers.
K
providerDetails2" 2The map of identity details, such as access token
&
providerName" The provider name
�
providerType" �The provider type.  [See AWS API for valid values](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-ProviderType)
#

userPoolId" The user pool id
"�
attributeMapping2" �The map of attribute mapping of user pool attributes. [AttributeMapping in AWS API documentation](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-AttributeMapping)
":
idpIdentifiersB*"  The list of identity providers.
"K
providerDetails2" 2The map of identity details, such as access token
"&
providerName" The provider name
"�
providerType" �The provider type.  [See AWS API for valid values](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-ProviderType)
"#

userPoolId" The user pool id
*��
Y
cognitoManagedUserPoolClient7aws:cognito/managedUserPoolClient:ManagedUserPoolClient�{Use the `aws.cognito.UserPoolClient` resource to manage a Cognito User Pool Client.

**This resource is advanced** and has special caveats to consider before use. Please read this document completely before using the resource.

Use the `aws.cognito.ManagedUserPoolClient` resource to manage a Cognito User Pool Client that is automatically created by an AWS service. For instance, when [configuring an OpenSearch Domain to use Cognito authentication](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cognito-auth.html), the OpenSearch service creates the User Pool Client during setup and removes it when it is no longer required. As a result, the `aws.cognito.ManagedUserPoolClient` resource does not create or delete this resource, but instead assumes management of it.

Use the `aws.cognito.UserPoolClient` resource to manage Cognito User Pool Clients for normal use cases.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleUserPool = new aws.cognito.UserPool("example", {name: "example"});
const exampleIdentityPool = new aws.cognito.IdentityPool("example", {identityPoolName: "example"});
const current = aws.getPartition({});
const example = current.then(current => aws.iam.getPolicyDocument({
    statements: [{
        sid: "",
        actions: ["sts:AssumeRole"],
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: [`es.${current.dnsSuffix}`],
        }],
    }],
}));
const exampleRole = new aws.iam.Role("example", {
    name: "example-role",
    path: "/service-role/",
    assumeRolePolicy: example.then(example => example.json),
});
const exampleRolePolicyAttachment = new aws.iam.RolePolicyAttachment("example", {
    role: exampleRole.name,
    policyArn: current.then(current => `arn:${current.partition}:iam::aws:policy/AmazonESCognitoAccess`),
});
const exampleDomain = new aws.opensearch.Domain("example", {
    domainName: "example",
    cognitoOptions: {
        enabled: true,
        userPoolId: exampleUserPool.id,
        identityPoolId: exampleIdentityPool.id,
        roleArn: exampleRole.arn,
    },
    ebsOptions: {
        ebsEnabled: true,
        volumeSize: 10,
    },
}, {
    dependsOn: [
        exampleAwsCognitoUserPoolDomain,
        exampleRolePolicyAttachment,
    ],
});
const exampleManagedUserPoolClient = new aws.cognito.ManagedUserPoolClient("example", {
    namePrefix: "AmazonOpenSearchService-example",
    userPoolId: exampleUserPool.id,
}, {
    dependsOn: [exampleDomain],
});
```
```python
import pulumi
import pulumi_aws as aws

example_user_pool = aws.cognito.UserPool("example", name="example")
example_identity_pool = aws.cognito.IdentityPool("example", identity_pool_name="example")
current = aws.get_partition()
example = aws.iam.get_policy_document(statements=[{
    "sid": "",
    "actions": ["sts:AssumeRole"],
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": [f"es.{current.dns_suffix}"],
    }],
}])
example_role = aws.iam.Role("example",
    name="example-role",
    path="/service-role/",
    assume_role_policy=example.json)
example_role_policy_attachment = aws.iam.RolePolicyAttachment("example",
    role=example_role.name,
    policy_arn=f"arn:{current.partition}:iam::aws:policy/AmazonESCognitoAccess")
example_domain = aws.opensearch.Domain("example",
    domain_name="example",
    cognito_options={
        "enabled": True,
        "user_pool_id": example_user_pool.id,
        "identity_pool_id": example_identity_pool.id,
        "role_arn": example_role.arn,
    },
    ebs_options={
        "ebs_enabled": True,
        "volume_size": 10,
    },
    opts = pulumi.ResourceOptions(depends_on=[
            example_aws_cognito_user_pool_domain,
            example_role_policy_attachment,
        ]))
example_managed_user_pool_client = aws.cognito.ManagedUserPoolClient("example",
    name_prefix="AmazonOpenSearchService-example",
    user_pool_id=example_user_pool.id,
    opts = pulumi.ResourceOptions(depends_on=[example_domain]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleUserPool = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example",
    });

    var exampleIdentityPool = new Aws.Cognito.IdentityPool("example", new()
    {
        IdentityPoolName = "example",
    });

    var current = Aws.GetPartition.Invoke();

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "",
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            $"es.{current.Apply(getPartitionResult => getPartitionResult.DnsSuffix)}",
                        },
                    },
                },
            },
        },
    });

    var exampleRole = new Aws.Iam.Role("example", new()
    {
        Name = "example-role",
        Path = "/service-role/",
        AssumeRolePolicy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleRolePolicyAttachment = new Aws.Iam.RolePolicyAttachment("example", new()
    {
        Role = exampleRole.Name,
        PolicyArn = $"arn:{current.Apply(getPartitionResult => getPartitionResult.Partition)}:iam::aws:policy/AmazonESCognitoAccess",
    });

    var exampleDomain = new Aws.OpenSearch.Domain("example", new()
    {
        DomainName = "example",
        CognitoOptions = new Aws.OpenSearch.Inputs.DomainCognitoOptionsArgs
        {
            Enabled = true,
            UserPoolId = exampleUserPool.Id,
            IdentityPoolId = exampleIdentityPool.Id,
            RoleArn = exampleRole.Arn,
        },
        EbsOptions = new Aws.OpenSearch.Inputs.DomainEbsOptionsArgs
        {
            EbsEnabled = true,
            VolumeSize = 10,
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsCognitoUserPoolDomain,
            exampleRolePolicyAttachment,
        },
    });

    var exampleManagedUserPoolClient = new Aws.Cognito.ManagedUserPoolClient("example", new()
    {
        NamePrefix = "AmazonOpenSearchService-example",
        UserPoolId = exampleUserPool.Id,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleDomain,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/opensearch"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleUserPool, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleIdentityPool, err := cognito.NewIdentityPool(ctx, "example", &cognito.IdentityPoolArgs{
			IdentityPoolName: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		current, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		example, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid: pulumi.StringRef(""),
					Actions: []string{
						"sts:AssumeRole",
					},
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								fmt.Sprintf("es.%v", current.DnsSuffix),
							},
						},
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			Name:             pulumi.String("example-role"),
			Path:             pulumi.String("/service-role/"),
			AssumeRolePolicy: pulumi.String(example.Json),
		})
		if err != nil {
			return err
		}
		exampleRolePolicyAttachment, err := iam.NewRolePolicyAttachment(ctx, "example", &iam.RolePolicyAttachmentArgs{
			Role:      exampleRole.Name,
			PolicyArn: pulumi.Sprintf("arn:%v:iam::aws:policy/AmazonESCognitoAccess", current.Partition),
		})
		if err != nil {
			return err
		}
		exampleDomain, err := opensearch.NewDomain(ctx, "example", &opensearch.DomainArgs{
			DomainName: pulumi.String("example"),
			CognitoOptions: &opensearch.DomainCognitoOptionsArgs{
				Enabled:        pulumi.Bool(true),
				UserPoolId:     exampleUserPool.ID(),
				IdentityPoolId: exampleIdentityPool.ID(),
				RoleArn:        exampleRole.Arn,
			},
			EbsOptions: &opensearch.DomainEbsOptionsArgs{
				EbsEnabled: pulumi.Bool(true),
				VolumeSize: pulumi.Int(10),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsCognitoUserPoolDomain,
			exampleRolePolicyAttachment,
		}))
		if err != nil {
			return err
		}
		_, err = cognito.NewManagedUserPoolClient(ctx, "example", &cognito.ManagedUserPoolClientArgs{
			NamePrefix: pulumi.String("AmazonOpenSearchService-example"),
			UserPoolId: exampleUserPool.ID(),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleDomain,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.IdentityPool;
import com.pulumi.aws.cognito.IdentityPoolArgs;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.RolePolicyAttachment;
import com.pulumi.aws.iam.RolePolicyAttachmentArgs;
import com.pulumi.aws.opensearch.Domain;
import com.pulumi.aws.opensearch.DomainArgs;
import com.pulumi.aws.opensearch.inputs.DomainCognitoOptionsArgs;
import com.pulumi.aws.opensearch.inputs.DomainEbsOptionsArgs;
import com.pulumi.aws.cognito.ManagedUserPoolClient;
import com.pulumi.aws.cognito.ManagedUserPoolClientArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleUserPool = new UserPool("exampleUserPool", UserPoolArgs.builder()
            .name("example")
            .build());

        var exampleIdentityPool = new IdentityPool("exampleIdentityPool", IdentityPoolArgs.builder()
            .identityPoolName("example")
            .build());

        final var current = AwsFunctions.getPartition();

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("")
                .actions("sts:AssumeRole")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers(String.format("es.%s", current.applyValue(getPartitionResult -> getPartitionResult.dnsSuffix())))
                    .build())
                .build())
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("example-role")
            .path("/service-role/")
            .assumeRolePolicy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var exampleRolePolicyAttachment = new RolePolicyAttachment("exampleRolePolicyAttachment", RolePolicyAttachmentArgs.builder()
            .role(exampleRole.name())
            .policyArn(String.format("arn:%s:iam::aws:policy/AmazonESCognitoAccess", current.applyValue(getPartitionResult -> getPartitionResult.partition())))
            .build());

        var exampleDomain = new Domain("exampleDomain", DomainArgs.builder()
            .domainName("example")
            .cognitoOptions(DomainCognitoOptionsArgs.builder()
                .enabled(true)
                .userPoolId(exampleUserPool.id())
                .identityPoolId(exampleIdentityPool.id())
                .roleArn(exampleRole.arn())
                .build())
            .ebsOptions(DomainEbsOptionsArgs.builder()
                .ebsEnabled(true)
                .volumeSize(10)
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    exampleAwsCognitoUserPoolDomain,
                    exampleRolePolicyAttachment)
                .build());

        var exampleManagedUserPoolClient = new ManagedUserPoolClient("exampleManagedUserPoolClient", ManagedUserPoolClientArgs.builder()
            .namePrefix("AmazonOpenSearchService-example")
            .userPoolId(exampleUserPool.id())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleDomain)
                .build());

    }
}
```
```yaml
resources:
  exampleManagedUserPoolClient:
    type: aws:cognito:ManagedUserPoolClient
    name: example
    properties:
      namePrefix: AmazonOpenSearchService-example
      userPoolId: ${exampleUserPool.id}
    options:
      dependsOn:
        - ${exampleDomain}
  exampleUserPool:
    type: aws:cognito:UserPool
    name: example
    properties:
      name: example
  exampleIdentityPool:
    type: aws:cognito:IdentityPool
    name: example
    properties:
      identityPoolName: example
  exampleDomain:
    type: aws:opensearch:Domain
    name: example
    properties:
      domainName: example
      cognitoOptions:
        enabled: true
        userPoolId: ${exampleUserPool.id}
        identityPoolId: ${exampleIdentityPool.id}
        roleArn: ${exampleRole.arn}
      ebsOptions:
        ebsEnabled: true
        volumeSize: 10
    options:
      dependsOn:
        - ${exampleAwsCognitoUserPoolDomain}
        - ${exampleRolePolicyAttachment}
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: example-role
      path: /service-role/
      assumeRolePolicy: ${example.json}
  exampleRolePolicyAttachment:
    type: aws:iam:RolePolicyAttachment
    name: example
    properties:
      role: ${exampleRole.name}
      policyArn: arn:${current.partition}:iam::aws:policy/AmazonESCognitoAccess
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: ""
            actions:
              - sts:AssumeRole
            effect: Allow
            principals:
              - type: Service
                identifiers:
                  - es.${current.dnsSuffix}
  current:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito User Pool Clients using the `id` of the Cognito User Pool and the `id` of the Cognito User Pool Client. For example:

```sh
$ pulumi import aws:cognito/managedUserPoolClient:ManagedUserPoolClient client us-west-2_abc123/3ho4ek12345678909nh3fmhpko
```
�
accessTokenValidityB �Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
�
allowedOauthFlowsB*" �List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
�
allowedOauthFlowsUserPoolClientB
 �Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
�
allowedOauthScopesB*" �List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
�
analyticsConfiguration�B�:�
�
cognito+ManagedUserPoolClientAnalyticsConfigurationcaws:cognito/ManagedUserPoolClientAnalyticsConfiguration:ManagedUserPoolClientAnalyticsConfigurationoConfiguration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
�
authSessionValidityB �Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
�
callbackUrlsB*" �List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
d
defaultRedirectUriB" HDefault redirect URI and must be included in the list of callback URLs.
k
(enablePropagateAdditionalUserContextDataB
 9Enables the propagation of additional user context data.
E
enableTokenRevocationB
 &Enables or disables token revocation.
�
explicitAuthFlowsB*" �List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
�
idTokenValidityB �Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
�

logoutUrlsB*" �List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
�
namePatternB" sRegular expression that matches the name of the desired User Pool Client. It must only match one User Pool Client.
�

namePrefixB" �String that matches the beginning of the name of the desired User Pool Client. It must match only one User Pool Client.

The following arguments are optional:
�
preventUserExistenceErrorsB" �Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
b
readAttributesB*" HList of user pool attributes that the application client can read from.
�
refreshTokenValidityB �Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
�
supportedIdentityProvidersB*" �List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
�
tokenValidityUnits�B�:�
�
cognito'ManagedUserPoolClientTokenValidityUnits[aws:cognito/ManagedUserPoolClientTokenValidityUnits:ManagedUserPoolClientTokenValidityUnitseConfiguration block for representing the validity times in units. See details below. Detailed below.
8

userPoolId" &User pool that the client belongs to.
b
writeAttributesB*" GList of user pool attributes that the application client can write to.
"�
accessTokenValidity �Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
"�
allowedOauthFlows*" �List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
"�
allowedOauthFlowsUserPoolClient
 �Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
"�
allowedOauthScopes*" �List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
"�
analyticsConfiguration�B�:�
�
cognito+ManagedUserPoolClientAnalyticsConfigurationcaws:cognito/ManagedUserPoolClientAnalyticsConfiguration:ManagedUserPoolClientAnalyticsConfigurationoConfiguration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
"�
authSessionValidity �Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
"�
callbackUrls*" �List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
";
clientSecret" 'Client secret of the user pool client.
"b
defaultRedirectUri" HDefault redirect URI and must be included in the list of callback URLs.
"i
(enablePropagateAdditionalUserContextData
 9Enables the propagation of additional user context data.
"C
enableTokenRevocation
 &Enables or disables token revocation.
"�
explicitAuthFlows*" �List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
"�
idTokenValidity �Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
"�

logoutUrls*" �List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
"*
name" Name of the user pool client.
"�
namePatternB" sRegular expression that matches the name of the desired User Pool Client. It must only match one User Pool Client.
"�

namePrefixB" �String that matches the beginning of the name of the desired User Pool Client. It must match only one User Pool Client.

The following arguments are optional:
"�
preventUserExistenceErrors" �Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
"`
readAttributes*" HList of user pool attributes that the application client can read from.
"�
refreshTokenValidity �Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
"�
supportedIdentityProviders*" �List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
"�
tokenValidityUnits�B�:�
�
cognito'ManagedUserPoolClientTokenValidityUnits[aws:cognito/ManagedUserPoolClientTokenValidityUnits:ManagedUserPoolClientTokenValidityUnitseConfiguration block for representing the validity times in units. See details below. Detailed below.
"8

userPoolId" &User pool that the client belongs to.
"`
writeAttributes*" GList of user pool attributes that the application client can write to.
*�=
D
cognitoResourceServer)aws:cognito/resourceServer:ResourceServer�7Provides a Cognito Resource Server.

## Example Usage

### Create a basic resource server

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const pool = new aws.cognito.UserPool("pool", {name: "pool"});
const resource = new aws.cognito.ResourceServer("resource", {
    identifier: "https://example.com",
    name: "example",
    userPoolId: pool.id,
});
```
```python
import pulumi
import pulumi_aws as aws

pool = aws.cognito.UserPool("pool", name="pool")
resource = aws.cognito.ResourceServer("resource",
    identifier="https://example.com",
    name="example",
    user_pool_id=pool.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var pool = new Aws.Cognito.UserPool("pool", new()
    {
        Name = "pool",
    });

    var resource = new Aws.Cognito.ResourceServer("resource", new()
    {
        Identifier = "https://example.com",
        Name = "example",
        UserPoolId = pool.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		pool, err := cognito.NewUserPool(ctx, "pool", &cognito.UserPoolArgs{
			Name: pulumi.String("pool"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewResourceServer(ctx, "resource", &cognito.ResourceServerArgs{
			Identifier: pulumi.String("https://example.com"),
			Name:       pulumi.String("example"),
			UserPoolId: pool.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.ResourceServer;
import com.pulumi.aws.cognito.ResourceServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pool = new UserPool("pool", UserPoolArgs.builder()
            .name("pool")
            .build());

        var resource = new ResourceServer("resource", ResourceServerArgs.builder()
            .identifier("https://example.com")
            .name("example")
            .userPoolId(pool.id())
            .build());

    }
}
```
```yaml
resources:
  pool:
    type: aws:cognito:UserPool
    properties:
      name: pool
  resource:
    type: aws:cognito:ResourceServer
    properties:
      identifier: https://example.com
      name: example
      userPoolId: ${pool.id}
```
<!--End PulumiCodeChooser -->

### Create a resource server with sample-scope

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const pool = new aws.cognito.UserPool("pool", {name: "pool"});
const resource = new aws.cognito.ResourceServer("resource", {
    identifier: "https://example.com",
    name: "example",
    scopes: [{
        scopeName: "sample-scope",
        scopeDescription: "a Sample Scope Description",
    }],
    userPoolId: pool.id,
});
```
```python
import pulumi
import pulumi_aws as aws

pool = aws.cognito.UserPool("pool", name="pool")
resource = aws.cognito.ResourceServer("resource",
    identifier="https://example.com",
    name="example",
    scopes=[{
        "scope_name": "sample-scope",
        "scope_description": "a Sample Scope Description",
    }],
    user_pool_id=pool.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var pool = new Aws.Cognito.UserPool("pool", new()
    {
        Name = "pool",
    });

    var resource = new Aws.Cognito.ResourceServer("resource", new()
    {
        Identifier = "https://example.com",
        Name = "example",
        Scopes = new[]
        {
            new Aws.Cognito.Inputs.ResourceServerScopeArgs
            {
                ScopeName = "sample-scope",
                ScopeDescription = "a Sample Scope Description",
            },
        },
        UserPoolId = pool.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		pool, err := cognito.NewUserPool(ctx, "pool", &cognito.UserPoolArgs{
			Name: pulumi.String("pool"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewResourceServer(ctx, "resource", &cognito.ResourceServerArgs{
			Identifier: pulumi.String("https://example.com"),
			Name:       pulumi.String("example"),
			Scopes: cognito.ResourceServerScopeArray{
				&cognito.ResourceServerScopeArgs{
					ScopeName:        pulumi.String("sample-scope"),
					ScopeDescription: pulumi.String("a Sample Scope Description"),
				},
			},
			UserPoolId: pool.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.ResourceServer;
import com.pulumi.aws.cognito.ResourceServerArgs;
import com.pulumi.aws.cognito.inputs.ResourceServerScopeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pool = new UserPool("pool", UserPoolArgs.builder()
            .name("pool")
            .build());

        var resource = new ResourceServer("resource", ResourceServerArgs.builder()
            .identifier("https://example.com")
            .name("example")
            .scopes(ResourceServerScopeArgs.builder()
                .scopeName("sample-scope")
                .scopeDescription("a Sample Scope Description")
                .build())
            .userPoolId(pool.id())
            .build());

    }
}
```
```yaml
resources:
  pool:
    type: aws:cognito:UserPool
    properties:
      name: pool
  resource:
    type: aws:cognito:ResourceServer
    properties:
      identifier: https://example.com
      name: example
      scopes:
        - scopeName: sample-scope
          scopeDescription: a Sample Scope Description
      userPoolId: ${pool.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_cognito_resource_server` using their User Pool ID and Identifier. For example:

```sh
$ pulumi import aws:cognito/resourceServer:ResourceServer example "us-west-2_abc123|https://example.com"
```
9

identifier" 'An identifier for the resource server.
.
nameB"  A name for the resource server.
�
scopes[BY*W:U
S
cognitoResourceServerScope3aws:cognito/ResourceServerScope:ResourceServerScopeA list of Authorization Scope.
3

userPoolId" !User pool the client belongs to.
"9

identifier" 'An identifier for the resource server.
",
name"  A name for the resource server.
"x
scopeIdentifiers*" ^A list of all scopes configured for this resource server in the format identifier/scope_name.
"�
scopes[BY*W:U
S
cognitoResourceServerScope3aws:cognito/ResourceServerScope:ResourceServerScopeA list of Authorization Scope.
"3

userPoolId" !User pool the client belongs to.
*�,
M
cognitoRiskConfiguration/aws:cognito/riskConfiguration:RiskConfiguration�Provides a Cognito Risk Configuration resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cognito.RiskConfiguration("example", {
    userPoolId: exampleAwsCognitoUserPool.id,
    riskExceptionConfiguration: {
        blockedIpRangeLists: ["10.10.10.10/32"],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.RiskConfiguration("example",
    user_pool_id=example_aws_cognito_user_pool["id"],
    risk_exception_configuration={
        "blocked_ip_range_lists": ["10.10.10.10/32"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.RiskConfiguration("example", new()
    {
        UserPoolId = exampleAwsCognitoUserPool.Id,
        RiskExceptionConfiguration = new Aws.Cognito.Inputs.RiskConfigurationRiskExceptionConfigurationArgs
        {
            BlockedIpRangeLists = new[]
            {
                "10.10.10.10/32",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.NewRiskConfiguration(ctx, "example", &cognito.RiskConfigurationArgs{
			UserPoolId: pulumi.Any(exampleAwsCognitoUserPool.Id),
			RiskExceptionConfiguration: &cognito.RiskConfigurationRiskExceptionConfigurationArgs{
				BlockedIpRangeLists: pulumi.StringArray{
					pulumi.String("10.10.10.10/32"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.RiskConfiguration;
import com.pulumi.aws.cognito.RiskConfigurationArgs;
import com.pulumi.aws.cognito.inputs.RiskConfigurationRiskExceptionConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new RiskConfiguration("example", RiskConfigurationArgs.builder()
            .userPoolId(exampleAwsCognitoUserPool.id())
            .riskExceptionConfiguration(RiskConfigurationRiskExceptionConfigurationArgs.builder()
                .blockedIpRangeLists("10.10.10.10/32")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:RiskConfiguration
    properties:
      userPoolId: ${exampleAwsCognitoUserPool.id}
      riskExceptionConfiguration:
        blockedIpRangeLists:
          - 10.10.10.10/32
```
<!--End PulumiCodeChooser -->

## Import

Import using the user pool ID and Client ID separated by a `:`:

__Using `pulumi import` to import__ Cognito Risk Configurations using the user pool ID or the user pool ID and Client Id separated by a `:`. For example:

Import using the user pool ID:

```sh
$ pulumi import aws:cognito/riskConfiguration:RiskConfiguration main example
```
Import using the user pool ID and Client ID separated by a `:`:

```sh
$ pulumi import aws:cognito/riskConfiguration:RiskConfiguration main example:example
```
�
 accountTakeoverRiskConfiguration�B�:�
�
cognito1RiskConfigurationAccountTakeoverRiskConfigurationoaws:cognito/RiskConfigurationAccountTakeoverRiskConfiguration:RiskConfigurationAccountTakeoverRiskConfiguration<The account takeover risk configuration. See details below.
�
clientIdB" �The app client ID. When the client ID is not provided, the same risk configuration is applied to all the clients in the User Pool.
�
'compromisedCredentialsRiskConfiguration�B�:�
�
cognito8RiskConfigurationCompromisedCredentialsRiskConfiguration}aws:cognito/RiskConfigurationCompromisedCredentialsRiskConfiguration:RiskConfigurationCompromisedCredentialsRiskConfigurationCThe compromised credentials risk configuration. See details below.
�
riskExceptionConfiguration�B�:�
�
cognito+RiskConfigurationRiskExceptionConfigurationcaws:cognito/RiskConfigurationRiskExceptionConfiguration:RiskConfigurationRiskExceptionConfigurationDThe configuration to override the risk decision. See details below.
$

userPoolId" The user pool ID.
"�
 accountTakeoverRiskConfiguration�B�:�
�
cognito1RiskConfigurationAccountTakeoverRiskConfigurationoaws:cognito/RiskConfigurationAccountTakeoverRiskConfiguration:RiskConfigurationAccountTakeoverRiskConfiguration<The account takeover risk configuration. See details below.
"�
clientIdB" �The app client ID. When the client ID is not provided, the same risk configuration is applied to all the clients in the User Pool.
"�
'compromisedCredentialsRiskConfiguration�B�:�
�
cognito8RiskConfigurationCompromisedCredentialsRiskConfiguration}aws:cognito/RiskConfigurationCompromisedCredentialsRiskConfiguration:RiskConfigurationCompromisedCredentialsRiskConfigurationCThe compromised credentials risk configuration. See details below.
"�
riskExceptionConfiguration�B�:�
�
cognito+RiskConfigurationRiskExceptionConfigurationcaws:cognito/RiskConfigurationRiskExceptionConfiguration:RiskConfigurationRiskExceptionConfigurationDThe configuration to override the risk decision. See details below.
"$

userPoolId" The user pool ID.
*؇
&
cognitoUseraws:cognito/user:User�NProvides a Cognito User Resource.

## Example Usage

### Basic configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cognito.UserPool("example", {name: "MyExamplePool"});
const exampleUser = new aws.cognito.User("example", {
    userPoolId: example.id,
    username: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.UserPool("example", name="MyExamplePool")
example_user = aws.cognito.User("example",
    user_pool_id=example.id,
    username="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        Name = "MyExamplePool",
    });

    var exampleUser = new Aws.Cognito.User("example", new()
    {
        UserPoolId = example.Id,
        Username = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("MyExamplePool"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUser(ctx, "example", &cognito.UserArgs{
			UserPoolId: example.ID(),
			Username:   pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.User;
import com.pulumi.aws.cognito.UserArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .name("MyExamplePool")
            .build());

        var exampleUser = new User("exampleUser", UserArgs.builder()
            .userPoolId(example.id())
            .username("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:UserPool
    properties:
      name: MyExamplePool
  exampleUser:
    type: aws:cognito:User
    name: example
    properties:
      userPoolId: ${example.id}
      username: example
```
<!--End PulumiCodeChooser -->

### Setting user attributes

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cognito.UserPool("example", {
    name: "mypool",
    schemas: [
        {
            name: "example",
            attributeDataType: "Boolean",
            mutable: false,
            required: false,
            developerOnlyAttribute: false,
        },
        {
            name: "foo",
            attributeDataType: "String",
            mutable: false,
            required: false,
            developerOnlyAttribute: false,
            stringAttributeConstraints: {},
        },
    ],
});
const exampleUser = new aws.cognito.User("example", {
    userPoolId: example.id,
    username: "example",
    attributes: {
        example: "true",
        foo: "bar",
        email: "no-reply@example.com",
        email_verified: "true",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.UserPool("example",
    name="mypool",
    schemas=[
        {
            "name": "example",
            "attribute_data_type": "Boolean",
            "mutable": False,
            "required": False,
            "developer_only_attribute": False,
        },
        {
            "name": "foo",
            "attribute_data_type": "String",
            "mutable": False,
            "required": False,
            "developer_only_attribute": False,
            "string_attribute_constraints": {},
        },
    ])
example_user = aws.cognito.User("example",
    user_pool_id=example.id,
    username="example",
    attributes={
        "example": "true",
        "foo": "bar",
        "email": "no-reply@example.com",
        "email_verified": "true",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        Name = "mypool",
        Schemas = new[]
        {
            new Aws.Cognito.Inputs.UserPoolSchemaArgs
            {
                Name = "example",
                AttributeDataType = "Boolean",
                Mutable = false,
                Required = false,
                DeveloperOnlyAttribute = false,
            },
            new Aws.Cognito.Inputs.UserPoolSchemaArgs
            {
                Name = "foo",
                AttributeDataType = "String",
                Mutable = false,
                Required = false,
                DeveloperOnlyAttribute = false,
                StringAttributeConstraints = null,
            },
        },
    });

    var exampleUser = new Aws.Cognito.User("example", new()
    {
        UserPoolId = example.Id,
        Username = "example",
        Attributes = 
        {
            { "example", "true" },
            { "foo", "bar" },
            { "email", "no-reply@example.com" },
            { "email_verified", "true" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("mypool"),
			Schemas: cognito.UserPoolSchemaArray{
				&cognito.UserPoolSchemaArgs{
					Name:                   pulumi.String("example"),
					AttributeDataType:      pulumi.String("Boolean"),
					Mutable:                pulumi.Bool(false),
					Required:               pulumi.Bool(false),
					DeveloperOnlyAttribute: pulumi.Bool(false),
				},
				&cognito.UserPoolSchemaArgs{
					Name:                       pulumi.String("foo"),
					AttributeDataType:          pulumi.String("String"),
					Mutable:                    pulumi.Bool(false),
					Required:                   pulumi.Bool(false),
					DeveloperOnlyAttribute:     pulumi.Bool(false),
					StringAttributeConstraints: &cognito.UserPoolSchemaStringAttributeConstraintsArgs{},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUser(ctx, "example", &cognito.UserArgs{
			UserPoolId: example.ID(),
			Username:   pulumi.String("example"),
			Attributes: pulumi.StringMap{
				"example":        pulumi.String("true"),
				"foo":            pulumi.String("bar"),
				"email":          pulumi.String("no-reply@example.com"),
				"email_verified": pulumi.String("true"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.inputs.UserPoolSchemaArgs;
import com.pulumi.aws.cognito.inputs.UserPoolSchemaStringAttributeConstraintsArgs;
import com.pulumi.aws.cognito.User;
import com.pulumi.aws.cognito.UserArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .name("mypool")
            .schemas(            
                UserPoolSchemaArgs.builder()
                    .name("example")
                    .attributeDataType("Boolean")
                    .mutable(false)
                    .required(false)
                    .developerOnlyAttribute(false)
                    .build(),
                UserPoolSchemaArgs.builder()
                    .name("foo")
                    .attributeDataType("String")
                    .mutable(false)
                    .required(false)
                    .developerOnlyAttribute(false)
                    .stringAttributeConstraints()
                    .build())
            .build());

        var exampleUser = new User("exampleUser", UserArgs.builder()
            .userPoolId(example.id())
            .username("example")
            .attributes(Map.ofEntries(
                Map.entry("example", true),
                Map.entry("foo", "bar"),
                Map.entry("email", "no-reply@example.com"),
                Map.entry("email_verified", true)
            ))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:UserPool
    properties:
      name: mypool
      schemas:
        - name: example
          attributeDataType: Boolean
          mutable: false
          required: false
          developerOnlyAttribute: false
        - name: foo
          attributeDataType: String
          mutable: false
          required: false
          developerOnlyAttribute: false
          stringAttributeConstraints: {}
  exampleUser:
    type: aws:cognito:User
    name: example
    properties:
      userPoolId: ${example.id}
      username: example
      attributes:
        example: true
        foo: bar
        email: no-reply@example.com
        email_verified: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito User using the `user_pool_id`/`name` attributes concatenated. For example:

```sh
$ pulumi import aws:cognito/user:User user us-east-1_vG78M4goG/user
```
g

attributesB2" QA map that contains user attributes and attribute values to be set for the user.
�
clientMetadataB2" �A map of custom key-value pairs that you can provide as input for any custom workflows that user creation triggers. Amazon Cognito does not store the `client_metadata` value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).
�
desiredDeliveryMediumsB*" �A list of mediums to the welcome message will be sent through. Allowed values are `EMAIL` and `SMS`. If it's provided, make sure you have also specified `email` attribute for the `EMAIL` medium and `phone_number` for the `SMS`. More than one value can be specified. Amazon Cognito does not store the `desired_delivery_mediums` value. Defaults to `["SMS"]`.
�
enabledB
 �Specifies whether the user should be enabled after creation. The welcome message will be sent regardless of the `enabled` value. The behavior can be changed with `message_action` argument. Defaults to `true`.
�
forceAliasCreationB
 �If this parameter is set to True and the `phone_number` or `email` address specified in the `attributes` parameter already exists as an alias with a different user, Amazon Cognito will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias. Amazon Cognito does not store the `force_alias_creation` value. Defaults to `false`.
�
messageActionB" �Set to `RESEND` to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to `SUPPRESS` to suppress sending the message. Only one value can be specified. Amazon Cognito does not store the `message_action` value.
�
passwordB" �The user's permanent password. This password must conform to the password policy specified by user pool the user belongs to. The welcome message always contains only `temporary_password` value. You can suppress sending the welcome message with the `message_action` argument. Amazon Cognito does not store the `password` value. Conflicts with `temporary_password`.
U
temporaryPasswordB" :The user's temporary password. Conflicts with `password`.
U

userPoolId" CThe user pool ID for the user pool where the user will be created.
�
username" �The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username cannot be changed.

The following arguments are optional:
�
validationDataB2" �The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. Amazon Cognito does not store the `validation_data` value. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).

> **NOTE:** Clearing `password` or `temporary_password` does not reset user's password in Cognito.
"g

attributesB2" QA map that contains user attributes and attribute values to be set for the user.
"�
clientMetadataB2" �A map of custom key-value pairs that you can provide as input for any custom workflows that user creation triggers. Amazon Cognito does not store the `client_metadata` value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).
"
creationDate" "�
desiredDeliveryMediumsB*" �A list of mediums to the welcome message will be sent through. Allowed values are `EMAIL` and `SMS`. If it's provided, make sure you have also specified `email` attribute for the `EMAIL` medium and `phone_number` for the `SMS`. More than one value can be specified. Amazon Cognito does not store the `desired_delivery_mediums` value. Defaults to `["SMS"]`.
"�
enabledB
 �Specifies whether the user should be enabled after creation. The welcome message will be sent regardless of the `enabled` value. The behavior can be changed with `message_action` argument. Defaults to `true`.
"�
forceAliasCreationB
 �If this parameter is set to True and the `phone_number` or `email` address specified in the `attributes` parameter already exists as an alias with a different user, Amazon Cognito will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias. Amazon Cognito does not store the `force_alias_creation` value. Defaults to `false`.
"
lastModifiedDate" "�
messageActionB" �Set to `RESEND` to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to `SUPPRESS` to suppress sending the message. Only one value can be specified. Amazon Cognito does not store the `message_action` value.
"
mfaSettingLists*" "�
passwordB" �The user's permanent password. This password must conform to the password policy specified by user pool the user belongs to. The welcome message always contains only `temporary_password` value. You can suppress sending the welcome message with the `message_action` argument. Amazon Cognito does not store the `password` value. Conflicts with `temporary_password`.
"
preferredMfaSetting" "#
status" current user status.
"F
sub" ;unique user id that is never reassignable to another user.
"U
temporaryPasswordB" :The user's temporary password. Conflicts with `password`.
"U

userPoolId" CThe user pool ID for the user pool where the user will be created.
"�
username" �The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username cannot be changed.

The following arguments are optional:
"�
validationDataB2" �The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. Amazon Cognito does not store the `validation_data` value. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).

> **NOTE:** Clearing `password` or `temporary_password` does not reset user's password in Cognito.
*�W
5
cognito	UserGroupaws:cognito/userGroup:UserGroup�SProvides a Cognito User Group resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = new aws.cognito.UserPool("main", {name: "identity pool"});
const groupRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Federated",
            identifiers: ["cognito-identity.amazonaws.com"],
        }],
        actions: ["sts:AssumeRoleWithWebIdentity"],
        conditions: [
            {
                test: "StringEquals",
                variable: "cognito-identity.amazonaws.com:aud",
                values: ["us-east-1:12345678-dead-beef-cafe-123456790ab"],
            },
            {
                test: "ForAnyValue:StringLike",
                variable: "cognito-identity.amazonaws.com:amr",
                values: ["authenticated"],
            },
        ],
    }],
});
const groupRoleRole = new aws.iam.Role("group_role", {
    name: "user-group-role",
    assumeRolePolicy: groupRole.then(groupRole => groupRole.json),
});
const mainUserGroup = new aws.cognito.UserGroup("main", {
    name: "user-group",
    userPoolId: main.id,
    description: "Managed by Pulumi",
    precedence: 42,
    roleArn: groupRoleRole.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.cognito.UserPool("main", name="identity pool")
group_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Federated",
        "identifiers": ["cognito-identity.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRoleWithWebIdentity"],
    "conditions": [
        {
            "test": "StringEquals",
            "variable": "cognito-identity.amazonaws.com:aud",
            "values": ["us-east-1:12345678-dead-beef-cafe-123456790ab"],
        },
        {
            "test": "ForAnyValue:StringLike",
            "variable": "cognito-identity.amazonaws.com:amr",
            "values": ["authenticated"],
        },
    ],
}])
group_role_role = aws.iam.Role("group_role",
    name="user-group-role",
    assume_role_policy=group_role.json)
main_user_group = aws.cognito.UserGroup("main",
    name="user-group",
    user_pool_id=main.id,
    description="Managed by Pulumi",
    precedence=42,
    role_arn=group_role_role.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = new Aws.Cognito.UserPool("main", new()
    {
        Name = "identity pool",
    });

    var groupRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Federated",
                        Identifiers = new[]
                        {
                            "cognito-identity.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRoleWithWebIdentity",
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "StringEquals",
                        Variable = "cognito-identity.amazonaws.com:aud",
                        Values = new[]
                        {
                            "us-east-1:12345678-dead-beef-cafe-123456790ab",
                        },
                    },
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "ForAnyValue:StringLike",
                        Variable = "cognito-identity.amazonaws.com:amr",
                        Values = new[]
                        {
                            "authenticated",
                        },
                    },
                },
            },
        },
    });

    var groupRoleRole = new Aws.Iam.Role("group_role", new()
    {
        Name = "user-group-role",
        AssumeRolePolicy = groupRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var mainUserGroup = new Aws.Cognito.UserGroup("main", new()
    {
        Name = "user-group",
        UserPoolId = main.Id,
        Description = "Managed by Pulumi",
        Precedence = 42,
        RoleArn = groupRoleRole.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		main, err := cognito.NewUserPool(ctx, "main", &cognito.UserPoolArgs{
			Name: pulumi.String("identity pool"),
		})
		if err != nil {
			return err
		}
		groupRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Federated",
							Identifiers: []string{
								"cognito-identity.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRoleWithWebIdentity",
					},
					Conditions: []iam.GetPolicyDocumentStatementCondition{
						{
							Test:     "StringEquals",
							Variable: "cognito-identity.amazonaws.com:aud",
							Values: []string{
								"us-east-1:12345678-dead-beef-cafe-123456790ab",
							},
						},
						{
							Test:     "ForAnyValue:StringLike",
							Variable: "cognito-identity.amazonaws.com:amr",
							Values: []string{
								"authenticated",
							},
						},
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		groupRoleRole, err := iam.NewRole(ctx, "group_role", &iam.RoleArgs{
			Name:             pulumi.String("user-group-role"),
			AssumeRolePolicy: pulumi.String(groupRole.Json),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUserGroup(ctx, "main", &cognito.UserGroupArgs{
			Name:        pulumi.String("user-group"),
			UserPoolId:  main.ID(),
			Description: pulumi.String("Managed by Pulumi"),
			Precedence:  pulumi.Int(42),
			RoleArn:     groupRoleRole.Arn,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.cognito.UserGroup;
import com.pulumi.aws.cognito.UserGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var main = new UserPool("main", UserPoolArgs.builder()
            .name("identity pool")
            .build());

        final var groupRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Federated")
                    .identifiers("cognito-identity.amazonaws.com")
                    .build())
                .actions("sts:AssumeRoleWithWebIdentity")
                .conditions(                
                    GetPolicyDocumentStatementConditionArgs.builder()
                        .test("StringEquals")
                        .variable("cognito-identity.amazonaws.com:aud")
                        .values("us-east-1:12345678-dead-beef-cafe-123456790ab")
                        .build(),
                    GetPolicyDocumentStatementConditionArgs.builder()
                        .test("ForAnyValue:StringLike")
                        .variable("cognito-identity.amazonaws.com:amr")
                        .values("authenticated")
                        .build())
                .build())
            .build());

        var groupRoleRole = new Role("groupRoleRole", RoleArgs.builder()
            .name("user-group-role")
            .assumeRolePolicy(groupRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var mainUserGroup = new UserGroup("mainUserGroup", UserGroupArgs.builder()
            .name("user-group")
            .userPoolId(main.id())
            .description("Managed by Pulumi")
            .precedence(42)
            .roleArn(groupRoleRole.arn())
            .build());

    }
}
```
```yaml
resources:
  main:
    type: aws:cognito:UserPool
    properties:
      name: identity pool
  groupRoleRole:
    type: aws:iam:Role
    name: group_role
    properties:
      name: user-group-role
      assumeRolePolicy: ${groupRole.json}
  mainUserGroup:
    type: aws:cognito:UserGroup
    name: main
    properties:
      name: user-group
      userPoolId: ${main.id}
      description: Managed by Pulumi
      precedence: 42
      roleArn: ${groupRoleRole.arn}
variables:
  groupRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Federated
                identifiers:
                  - cognito-identity.amazonaws.com
            actions:
              - sts:AssumeRoleWithWebIdentity
            conditions:
              - test: StringEquals
                variable: cognito-identity.amazonaws.com:aud
                values:
                  - us-east-1:12345678-dead-beef-cafe-123456790ab
              - test: ForAnyValue:StringLike
                variable: cognito-identity.amazonaws.com:amr
                values:
                  - authenticated
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito User Groups using the `user_pool_id`/`name` attributes concatenated. For example:

```sh
$ pulumi import aws:cognito/userGroup:UserGroup group us-east-1_vG78M4goG/user-group
```
8
descriptionB" #The description of the user group.
*
nameB" The name of the user group.
6

precedenceB "The precedence of the user group.
O
roleArnB" >The ARN of the IAM role to be associated with the user group.
$

userPoolId" The user pool ID.
"8
descriptionB" #The description of the user group.
"(
name" The name of the user group.
"6

precedenceB "The precedence of the user group.
"O
roleArnB" >The ARN of the IAM role to be associated with the user group.
"$

userPoolId" The user pool ID.
*�6
;
cognitoUserInGroup#aws:cognito/userInGroup:UserInGroup�2Adds the specified user to the specified group.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cognito.UserPool("example", {
    name: "example",
    passwordPolicy: {
        temporaryPasswordValidityDays: 7,
        minimumLength: 6,
        requireUppercase: false,
        requireSymbols: false,
        requireNumbers: false,
    },
});
const exampleUser = new aws.cognito.User("example", {
    userPoolId: example.id,
    username: "example",
});
const exampleUserGroup = new aws.cognito.UserGroup("example", {
    userPoolId: example.id,
    name: "example",
});
const exampleUserInGroup = new aws.cognito.UserInGroup("example", {
    userPoolId: example.id,
    groupName: exampleUserGroup.name,
    username: exampleUser.username,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.UserPool("example",
    name="example",
    password_policy={
        "temporary_password_validity_days": 7,
        "minimum_length": 6,
        "require_uppercase": False,
        "require_symbols": False,
        "require_numbers": False,
    })
example_user = aws.cognito.User("example",
    user_pool_id=example.id,
    username="example")
example_user_group = aws.cognito.UserGroup("example",
    user_pool_id=example.id,
    name="example")
example_user_in_group = aws.cognito.UserInGroup("example",
    user_pool_id=example.id,
    group_name=example_user_group.name,
    username=example_user.username)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example",
        PasswordPolicy = new Aws.Cognito.Inputs.UserPoolPasswordPolicyArgs
        {
            TemporaryPasswordValidityDays = 7,
            MinimumLength = 6,
            RequireUppercase = false,
            RequireSymbols = false,
            RequireNumbers = false,
        },
    });

    var exampleUser = new Aws.Cognito.User("example", new()
    {
        UserPoolId = example.Id,
        Username = "example",
    });

    var exampleUserGroup = new Aws.Cognito.UserGroup("example", new()
    {
        UserPoolId = example.Id,
        Name = "example",
    });

    var exampleUserInGroup = new Aws.Cognito.UserInGroup("example", new()
    {
        UserPoolId = example.Id,
        GroupName = exampleUserGroup.Name,
        Username = exampleUser.Username,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example"),
			PasswordPolicy: &cognito.UserPoolPasswordPolicyArgs{
				TemporaryPasswordValidityDays: pulumi.Int(7),
				MinimumLength:                 pulumi.Int(6),
				RequireUppercase:              pulumi.Bool(false),
				RequireSymbols:                pulumi.Bool(false),
				RequireNumbers:                pulumi.Bool(false),
			},
		})
		if err != nil {
			return err
		}
		exampleUser, err := cognito.NewUser(ctx, "example", &cognito.UserArgs{
			UserPoolId: example.ID(),
			Username:   pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleUserGroup, err := cognito.NewUserGroup(ctx, "example", &cognito.UserGroupArgs{
			UserPoolId: example.ID(),
			Name:       pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUserInGroup(ctx, "example", &cognito.UserInGroupArgs{
			UserPoolId: example.ID(),
			GroupName:  exampleUserGroup.Name,
			Username:   exampleUser.Username,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.inputs.UserPoolPasswordPolicyArgs;
import com.pulumi.aws.cognito.User;
import com.pulumi.aws.cognito.UserArgs;
import com.pulumi.aws.cognito.UserGroup;
import com.pulumi.aws.cognito.UserGroupArgs;
import com.pulumi.aws.cognito.UserInGroup;
import com.pulumi.aws.cognito.UserInGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .name("example")
            .passwordPolicy(UserPoolPasswordPolicyArgs.builder()
                .temporaryPasswordValidityDays(7)
                .minimumLength(6)
                .requireUppercase(false)
                .requireSymbols(false)
                .requireNumbers(false)
                .build())
            .build());

        var exampleUser = new User("exampleUser", UserArgs.builder()
            .userPoolId(example.id())
            .username("example")
            .build());

        var exampleUserGroup = new UserGroup("exampleUserGroup", UserGroupArgs.builder()
            .userPoolId(example.id())
            .name("example")
            .build());

        var exampleUserInGroup = new UserInGroup("exampleUserInGroup", UserInGroupArgs.builder()
            .userPoolId(example.id())
            .groupName(exampleUserGroup.name())
            .username(exampleUser.username())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:UserPool
    properties:
      name: example
      passwordPolicy:
        temporaryPasswordValidityDays: 7
        minimumLength: 6
        requireUppercase: false
        requireSymbols: false
        requireNumbers: false
  exampleUser:
    type: aws:cognito:User
    name: example
    properties:
      userPoolId: ${example.id}
      username: example
  exampleUserGroup:
    type: aws:cognito:UserGroup
    name: example
    properties:
      userPoolId: ${example.id}
      name: example
  exampleUserInGroup:
    type: aws:cognito:UserInGroup
    name: example
    properties:
      userPoolId: ${example.id}
      groupName: ${exampleUserGroup.name}
      username: ${exampleUser.username}
```
<!--End PulumiCodeChooser -->
I
	groupName" 8The name of the group to which the user is to be added.
:

userPoolId" (The user pool ID of the user and group.
C
username" 3The username of the user to be added to the group.
"I
	groupName" 8The name of the group to which the user is to be added.
":

userPoolId" (The user pool ID of the user and group.
"C
username" 3The username of the user to be added to the group.
*߲
2
cognitoUserPoolaws:cognito/userPool:UserPool�OProvides a Cognito User Pool resource.

## Example Usage

### Basic configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const pool = new aws.cognito.UserPool("pool", {name: "mypool"});
```
```python
import pulumi
import pulumi_aws as aws

pool = aws.cognito.UserPool("pool", name="mypool")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var pool = new Aws.Cognito.UserPool("pool", new()
    {
        Name = "mypool",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.NewUserPool(ctx, "pool", &cognito.UserPoolArgs{
			Name: pulumi.String("mypool"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pool = new UserPool("pool", UserPoolArgs.builder()
            .name("mypool")
            .build());

    }
}
```
```yaml
resources:
  pool:
    type: aws:cognito:UserPool
    properties:
      name: mypool
```
<!--End PulumiCodeChooser -->

### Enabling SMS and Software Token Multi-Factor Authentication

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cognito.UserPool("example", {
    mfaConfiguration: "ON",
    smsAuthenticationMessage: "Your code is {####}",
    smsConfiguration: {
        externalId: "example",
        snsCallerArn: exampleAwsIamRole.arn,
        snsRegion: "us-east-1",
    },
    softwareTokenMfaConfiguration: {
        enabled: true,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.UserPool("example",
    mfa_configuration="ON",
    sms_authentication_message="Your code is {####}",
    sms_configuration={
        "external_id": "example",
        "sns_caller_arn": example_aws_iam_role["arn"],
        "sns_region": "us-east-1",
    },
    software_token_mfa_configuration={
        "enabled": True,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        MfaConfiguration = "ON",
        SmsAuthenticationMessage = "Your code is {####}",
        SmsConfiguration = new Aws.Cognito.Inputs.UserPoolSmsConfigurationArgs
        {
            ExternalId = "example",
            SnsCallerArn = exampleAwsIamRole.Arn,
            SnsRegion = "us-east-1",
        },
        SoftwareTokenMfaConfiguration = new Aws.Cognito.Inputs.UserPoolSoftwareTokenMfaConfigurationArgs
        {
            Enabled = true,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			MfaConfiguration:         pulumi.String("ON"),
			SmsAuthenticationMessage: pulumi.String("Your code is {####}"),
			SmsConfiguration: &cognito.UserPoolSmsConfigurationArgs{
				ExternalId:   pulumi.String("example"),
				SnsCallerArn: pulumi.Any(exampleAwsIamRole.Arn),
				SnsRegion:    pulumi.String("us-east-1"),
			},
			SoftwareTokenMfaConfiguration: &cognito.UserPoolSoftwareTokenMfaConfigurationArgs{
				Enabled: pulumi.Bool(true),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.inputs.UserPoolSmsConfigurationArgs;
import com.pulumi.aws.cognito.inputs.UserPoolSoftwareTokenMfaConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .mfaConfiguration("ON")
            .smsAuthenticationMessage("Your code is {####}")
            .smsConfiguration(UserPoolSmsConfigurationArgs.builder()
                .externalId("example")
                .snsCallerArn(exampleAwsIamRole.arn())
                .snsRegion("us-east-1")
                .build())
            .softwareTokenMfaConfiguration(UserPoolSoftwareTokenMfaConfigurationArgs.builder()
                .enabled(true)
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:UserPool
    properties:
      mfaConfiguration: ON
      smsAuthenticationMessage: Your code is {####}
      smsConfiguration:
        externalId: example
        snsCallerArn: ${exampleAwsIamRole.arn}
        snsRegion: us-east-1
      softwareTokenMfaConfiguration:
        enabled: true
```
<!--End PulumiCodeChooser -->

### Using Account Recovery Setting

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.cognito.UserPool("test", {
    name: "mypool",
    accountRecoverySetting: {
        recoveryMechanisms: [
            {
                name: "verified_email",
                priority: 1,
            },
            {
                name: "verified_phone_number",
                priority: 2,
            },
        ],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.cognito.UserPool("test",
    name="mypool",
    account_recovery_setting={
        "recovery_mechanisms": [
            {
                "name": "verified_email",
                "priority": 1,
            },
            {
                "name": "verified_phone_number",
                "priority": 2,
            },
        ],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Cognito.UserPool("test", new()
    {
        Name = "mypool",
        AccountRecoverySetting = new Aws.Cognito.Inputs.UserPoolAccountRecoverySettingArgs
        {
            RecoveryMechanisms = new[]
            {
                new Aws.Cognito.Inputs.UserPoolAccountRecoverySettingRecoveryMechanismArgs
                {
                    Name = "verified_email",
                    Priority = 1,
                },
                new Aws.Cognito.Inputs.UserPoolAccountRecoverySettingRecoveryMechanismArgs
                {
                    Name = "verified_phone_number",
                    Priority = 2,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.NewUserPool(ctx, "test", &cognito.UserPoolArgs{
			Name: pulumi.String("mypool"),
			AccountRecoverySetting: &cognito.UserPoolAccountRecoverySettingArgs{
				RecoveryMechanisms: cognito.UserPoolAccountRecoverySettingRecoveryMechanismArray{
					&cognito.UserPoolAccountRecoverySettingRecoveryMechanismArgs{
						Name:     pulumi.String("verified_email"),
						Priority: pulumi.Int(1),
					},
					&cognito.UserPoolAccountRecoverySettingRecoveryMechanismArgs{
						Name:     pulumi.String("verified_phone_number"),
						Priority: pulumi.Int(2),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.inputs.UserPoolAccountRecoverySettingArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new UserPool("test", UserPoolArgs.builder()
            .name("mypool")
            .accountRecoverySetting(UserPoolAccountRecoverySettingArgs.builder()
                .recoveryMechanisms(                
                    UserPoolAccountRecoverySettingRecoveryMechanismArgs.builder()
                        .name("verified_email")
                        .priority(1)
                        .build(),
                    UserPoolAccountRecoverySettingRecoveryMechanismArgs.builder()
                        .name("verified_phone_number")
                        .priority(2)
                        .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:cognito:UserPool
    properties:
      name: mypool
      accountRecoverySetting:
        recoveryMechanisms:
          - name: verified_email
            priority: 1
          - name: verified_phone_number
            priority: 2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito User Pools using the `id`. For example:

```sh
$ pulumi import aws:cognito/userPool:UserPool pool us-west-2_abc123
```
�
accountRecoverySettingzBx:v
t
cognitoUserPoolAccountRecoverySettingIaws:cognito/UserPoolAccountRecoverySetting:UserPoolAccountRecoverySetting�Configuration block to define which verified available method a user can use to recover their forgotten password. Detailed below.
�
adminCreateUserConfigwBu:s
q
cognitoUserPoolAdminCreateUserConfigGaws:cognito/UserPoolAdminCreateUserConfig:UserPoolAdminCreateUserConfigEConfiguration block for creating a new user profile. Detailed below.
�
aliasAttributesB*" �Attributes supported as an alias for this user pool. Valid values: `phone_number`, `email`, or `preferred_username`. Conflicts with `username_attributes`.
i
autoVerifiedAttributesB*" GAttributes to be auto-verified. Valid values: `email`, `phone_number`.
�
deletionProtectionB" �When active, DeletionProtection prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature. Valid values are `ACTIVE` and `INACTIVE`, Default value is `INACTIVE`.
�
deviceConfigurationqBo:m
k
cognitoUserPoolDeviceConfigurationCaws:cognito/UserPoolDeviceConfiguration:UserPoolDeviceConfigurationIConfiguration block for the user pool's device tracking. Detailed below.
�
emailConfigurationnBl:j
h
cognitoUserPoolEmailConfigurationAaws:cognito/UserPoolEmailConfiguration:UserPoolEmailConfiguration;Configuration block for configuring email. Detailed below.
�
emailVerificationMessageB" �String representing the email verification message. Conflicts with `verification_message_template` configuration block `email_message` argument.
�
emailVerificationSubjectB" �String representing the email verification subject. Conflicts with `verification_message_template` configuration block `email_subject` argument.
�
lambdaConfig\BZ:X
V
cognitoUserPoolLambdaConfig5aws:cognito/UserPoolLambdaConfig:UserPoolLambdaConfig_Configuration block for the AWS Lambda triggers associated with the user pool. Detailed below.
�
mfaConfigurationB" �Multi-Factor Authentication (MFA) configuration for the User Pool. Defaults of `OFF`. Valid values are `OFF` (MFA Tokens are not required), `ON` (MFA is required for all users to sign in; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured), or `OPTIONAL` (MFA Will be required only for individual users who have MFA Enabled; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured).
L
nameB" >Name of the user pool.

The following arguments are optional:
�
passwordPolicybB`:^
\
cognitoUserPoolPasswordPolicy9aws:cognito/UserPoolPasswordPolicy:UserPoolPasswordPolicyYConfiguration block for information about the user pool password policy. Detailed below.
�
schemasLBJ*H:F
D
cognitoUserPoolSchema)aws:cognito/UserPoolSchema:UserPoolSchema�Configuration block for the schema attributes of a user pool. Detailed below. Schema attributes from the [standard attribute set](https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#cognito-user-pools-standard-attributes) only need to be specified if they are different from the default configuration. Attributes can be added, but not modified or removed. Maximum of 50 attributes.
�
smsAuthenticationMessageB" �String representing the SMS authentication message. The Message must contain the `{####}` placeholder, which will be replaced with the code.
�
smsConfigurationhBf:d
b
cognitoUserPoolSmsConfiguration=aws:cognito/UserPoolSmsConfiguration:UserPoolSmsConfiguration�Configuration block for Short Message Service (SMS) settings. Detailed below. These settings apply to SMS user verification and SMS Multi-Factor Authentication (MFA). Due to Cognito API restrictions, the SMS configuration cannot be removed without recreating the Cognito User Pool. For user data safety, this resource will ignore the removal of this configuration by disabling drift detection. To force resource recreation after this configuration has been applied, see the `taint` command.
�
smsVerificationMessageB" �String representing the SMS verification message. Conflicts with `verification_message_template` configuration block `sms_message` argument.
�
softwareTokenMfaConfiguration�B�:�
�
cognito%UserPoolSoftwareTokenMfaConfigurationWaws:cognito/UserPoolSoftwareTokenMfaConfiguration:UserPoolSoftwareTokenMfaConfigurationbConfiguration block for software token Mult-Factor Authentication (MFA) settings. Detailed below.
�
tagsB2" �Map of tags to assign to the User Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
userAttributeUpdateSettings�B�:�
�
cognito#UserPoolUserAttributeUpdateSettingsSaws:cognito/UserPoolUserAttributeUpdateSettings:UserPoolUserAttributeUpdateSettingsHConfiguration block for user attribute update settings. Detailed below.
�
userPoolAddOnsbB`:^
\
cognitoUserPoolUserPoolAddOns9aws:cognito/UserPoolUserPoolAddOns:UserPoolUserPoolAddOnsoConfiguration block for user pool add-ons to enable user pool advanced security mode features. Detailed below.
�
usernameAttributesB*" �Whether email addresses or phone numbers can be specified as usernames when a user signs up. Conflicts with `alias_attributes`.
�
usernameConfigurationwBu:s
q
cognitoUserPoolUsernameConfigurationGaws:cognito/UserPoolUsernameConfiguration:UserPoolUsernameConfiguration@Configuration block for username configuration. Detailed below.
�
verificationMessageTemplate�B�:�
�
cognito#UserPoolVerificationMessageTemplateSaws:cognito/UserPoolVerificationMessageTemplate:UserPoolVerificationMessageTemplateHConfiguration block for verification message templates. Detailed below.
"�
accountRecoverySettingzBx:v
t
cognitoUserPoolAccountRecoverySettingIaws:cognito/UserPoolAccountRecoverySetting:UserPoolAccountRecoverySetting�Configuration block to define which verified available method a user can use to recover their forgotten password. Detailed below.
"�
adminCreateUserConfigu:s
q
cognitoUserPoolAdminCreateUserConfigGaws:cognito/UserPoolAdminCreateUserConfig:UserPoolAdminCreateUserConfigEConfiguration block for creating a new user profile. Detailed below.
"�
aliasAttributesB*" �Attributes supported as an alias for this user pool. Valid values: `phone_number`, `email`, or `preferred_username`. Conflicts with `username_attributes`.
"!
arn" ARN of the user pool.
"i
autoVerifiedAttributesB*" GAttributes to be auto-verified. Valid values: `email`, `phone_number`.
"4
creationDate"  Date the user pool was created.
"�
customDomain" �A custom domain name that you provide to Amazon Cognito. This parameter applies only if you use a custom domain to host the sign-up and sign-in pages for your application. For example: `auth.example.com`.
"�
deletionProtectionB" �When active, DeletionProtection prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature. Valid values are `ACTIVE` and `INACTIVE`, Default value is `INACTIVE`.
"�
deviceConfigurationqBo:m
k
cognitoUserPoolDeviceConfigurationCaws:cognito/UserPoolDeviceConfiguration:UserPoolDeviceConfigurationIConfiguration block for the user pool's device tracking. Detailed below.
"X
domain" JHolds the domain prefix if the user pool has a domain associated with it.
"�
emailConfigurationnBl:j
h
cognitoUserPoolEmailConfigurationAaws:cognito/UserPoolEmailConfiguration:UserPoolEmailConfiguration;Configuration block for configuring email. Detailed below.
"�
emailVerificationMessage" �String representing the email verification message. Conflicts with `verification_message_template` configuration block `email_message` argument.
"�
emailVerificationSubject" �String representing the email verification subject. Conflicts with `verification_message_template` configuration block `email_subject` argument.
"n
endpoint" ^Endpoint name of the user pool. Example format: `cognito-idp.REGION.amazonaws.com/xxxx_yyyyy`
"M
estimatedNumberOfUsers /A number estimating the size of the user pool.
"�
lambdaConfig\BZ:X
V
cognitoUserPoolLambdaConfig5aws:cognito/UserPoolLambdaConfig:UserPoolLambdaConfig_Configuration block for the AWS Lambda triggers associated with the user pool. Detailed below.
">
lastModifiedDate" &Date the user pool was last modified.
"�
mfaConfigurationB" �Multi-Factor Authentication (MFA) configuration for the User Pool. Defaults of `OFF`. Valid values are `OFF` (MFA Tokens are not required), `ON` (MFA is required for all users to sign in; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured), or `OPTIONAL` (MFA Will be required only for individual users who have MFA Enabled; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured).
"J
name" >Name of the user pool.

The following arguments are optional:
"�
passwordPolicy`:^
\
cognitoUserPoolPasswordPolicy9aws:cognito/UserPoolPasswordPolicy:UserPoolPasswordPolicyYConfiguration block for information about the user pool password policy. Detailed below.
"�
schemasLBJ*H:F
D
cognitoUserPoolSchema)aws:cognito/UserPoolSchema:UserPoolSchema�Configuration block for the schema attributes of a user pool. Detailed below. Schema attributes from the [standard attribute set](https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#cognito-user-pools-standard-attributes) only need to be specified if they are different from the default configuration. Attributes can be added, but not modified or removed. Maximum of 50 attributes.
"�
smsAuthenticationMessageB" �String representing the SMS authentication message. The Message must contain the `{####}` placeholder, which will be replaced with the code.
"�
smsConfigurationf:d
b
cognitoUserPoolSmsConfiguration=aws:cognito/UserPoolSmsConfiguration:UserPoolSmsConfiguration�Configuration block for Short Message Service (SMS) settings. Detailed below. These settings apply to SMS user verification and SMS Multi-Factor Authentication (MFA). Due to Cognito API restrictions, the SMS configuration cannot be removed without recreating the Cognito User Pool. For user data safety, this resource will ignore the removal of this configuration by disabling drift detection. To force resource recreation after this configuration has been applied, see the `taint` command.
"�
smsVerificationMessage" �String representing the SMS verification message. Conflicts with `verification_message_template` configuration block `sms_message` argument.
"�
softwareTokenMfaConfiguration�B�:�
�
cognito%UserPoolSoftwareTokenMfaConfigurationWaws:cognito/UserPoolSoftwareTokenMfaConfiguration:UserPoolSoftwareTokenMfaConfigurationbConfiguration block for software token Mult-Factor Authentication (MFA) settings. Detailed below.
"�
tagsB2" �Map of tags to assign to the User Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
userAttributeUpdateSettings�B�:�
�
cognito#UserPoolUserAttributeUpdateSettingsSaws:cognito/UserPoolUserAttributeUpdateSettings:UserPoolUserAttributeUpdateSettingsHConfiguration block for user attribute update settings. Detailed below.
"�
userPoolAddOnsbB`:^
\
cognitoUserPoolUserPoolAddOns9aws:cognito/UserPoolUserPoolAddOns:UserPoolUserPoolAddOnsoConfiguration block for user pool add-ons to enable user pool advanced security mode features. Detailed below.
"�
usernameAttributesB*" �Whether email addresses or phone numbers can be specified as usernames when a user signs up. Conflicts with `alias_attributes`.
"�
usernameConfigurationwBu:s
q
cognitoUserPoolUsernameConfigurationGaws:cognito/UserPoolUsernameConfiguration:UserPoolUsernameConfiguration@Configuration block for username configuration. Detailed below.
"�
verificationMessageTemplate�:�
�
cognito#UserPoolVerificationMessageTemplateSaws:cognito/UserPoolVerificationMessageTemplate:UserPoolVerificationMessageTemplateHConfiguration block for verification message templates. Detailed below.
*�
D
cognitoUserPoolClient)aws:cognito/userPoolClient:UserPoolClient��Provides a Cognito User Pool Client resource.

To manage a User Pool Client created by another service, such as when [configuring an OpenSearch Domain to use Cognito authentication](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cognito-auth.html),
use the `aws.cognito.ManagedUserPoolClient` resource instead.

## Example Usage

### Create a basic user pool client

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const pool = new aws.cognito.UserPool("pool", {name: "pool"});
const client = new aws.cognito.UserPoolClient("client", {
    name: "client",
    userPoolId: pool.id,
});
```
```python
import pulumi
import pulumi_aws as aws

pool = aws.cognito.UserPool("pool", name="pool")
client = aws.cognito.UserPoolClient("client",
    name="client",
    user_pool_id=pool.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var pool = new Aws.Cognito.UserPool("pool", new()
    {
        Name = "pool",
    });

    var client = new Aws.Cognito.UserPoolClient("client", new()
    {
        Name = "client",
        UserPoolId = pool.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		pool, err := cognito.NewUserPool(ctx, "pool", &cognito.UserPoolArgs{
			Name: pulumi.String("pool"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUserPoolClient(ctx, "client", &cognito.UserPoolClientArgs{
			Name:       pulumi.String("client"),
			UserPoolId: pool.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolClient;
import com.pulumi.aws.cognito.UserPoolClientArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pool = new UserPool("pool", UserPoolArgs.builder()
            .name("pool")
            .build());

        var client = new UserPoolClient("client", UserPoolClientArgs.builder()
            .name("client")
            .userPoolId(pool.id())
            .build());

    }
}
```
```yaml
resources:
  client:
    type: aws:cognito:UserPoolClient
    properties:
      name: client
      userPoolId: ${pool.id}
  pool:
    type: aws:cognito:UserPool
    properties:
      name: pool
```
<!--End PulumiCodeChooser -->

### Create a user pool client with no SRP authentication

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const pool = new aws.cognito.UserPool("pool", {name: "pool"});
const client = new aws.cognito.UserPoolClient("client", {
    name: "client",
    userPoolId: pool.id,
    generateSecret: true,
    explicitAuthFlows: ["ADMIN_NO_SRP_AUTH"],
});
```
```python
import pulumi
import pulumi_aws as aws

pool = aws.cognito.UserPool("pool", name="pool")
client = aws.cognito.UserPoolClient("client",
    name="client",
    user_pool_id=pool.id,
    generate_secret=True,
    explicit_auth_flows=["ADMIN_NO_SRP_AUTH"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var pool = new Aws.Cognito.UserPool("pool", new()
    {
        Name = "pool",
    });

    var client = new Aws.Cognito.UserPoolClient("client", new()
    {
        Name = "client",
        UserPoolId = pool.Id,
        GenerateSecret = true,
        ExplicitAuthFlows = new[]
        {
            "ADMIN_NO_SRP_AUTH",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		pool, err := cognito.NewUserPool(ctx, "pool", &cognito.UserPoolArgs{
			Name: pulumi.String("pool"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUserPoolClient(ctx, "client", &cognito.UserPoolClientArgs{
			Name:           pulumi.String("client"),
			UserPoolId:     pool.ID(),
			GenerateSecret: pulumi.Bool(true),
			ExplicitAuthFlows: pulumi.StringArray{
				pulumi.String("ADMIN_NO_SRP_AUTH"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolClient;
import com.pulumi.aws.cognito.UserPoolClientArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pool = new UserPool("pool", UserPoolArgs.builder()
            .name("pool")
            .build());

        var client = new UserPoolClient("client", UserPoolClientArgs.builder()
            .name("client")
            .userPoolId(pool.id())
            .generateSecret(true)
            .explicitAuthFlows("ADMIN_NO_SRP_AUTH")
            .build());

    }
}
```
```yaml
resources:
  client:
    type: aws:cognito:UserPoolClient
    properties:
      name: client
      userPoolId: ${pool.id}
      generateSecret: true
      explicitAuthFlows:
        - ADMIN_NO_SRP_AUTH
  pool:
    type: aws:cognito:UserPool
    properties:
      name: pool
```
<!--End PulumiCodeChooser -->

### Create a user pool client with pinpoint analytics

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testUserPool = new aws.cognito.UserPool("test", {name: "pool"});
const testApp = new aws.pinpoint.App("test", {name: "pinpoint"});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["cognito-idp.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const testRole = new aws.iam.Role("test", {
    name: "role",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const testUserPoolClient = new aws.cognito.UserPoolClient("test", {
    name: "pool_client",
    userPoolId: testUserPool.id,
    analyticsConfiguration: {
        applicationId: testApp.applicationId,
        externalId: "some_id",
        roleArn: testRole.arn,
        userDataShared: true,
    },
});
const current = aws.getCallerIdentity({});
const test = aws.iam.getPolicyDocumentOutput({
    statements: [{
        effect: "Allow",
        actions: [
            "mobiletargeting:UpdateEndpoint",
            "mobiletargeting:PutEvents",
        ],
        resources: [pulumi.all([current, testApp.applicationId]).apply(([current, applicationId]) => `arn:aws:mobiletargeting:*:${current.accountId}:apps/${applicationId}*`)],
    }],
});
const testRolePolicy = new aws.iam.RolePolicy("test", {
    name: "role_policy",
    role: testRole.id,
    policy: test.apply(test => test.json),
});
```
```python
import pulumi
import pulumi_aws as aws

test_user_pool = aws.cognito.UserPool("test", name="pool")
test_app = aws.pinpoint.App("test", name="pinpoint")
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["cognito-idp.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
test_role = aws.iam.Role("test",
    name="role",
    assume_role_policy=assume_role.json)
test_user_pool_client = aws.cognito.UserPoolClient("test",
    name="pool_client",
    user_pool_id=test_user_pool.id,
    analytics_configuration={
        "application_id": test_app.application_id,
        "external_id": "some_id",
        "role_arn": test_role.arn,
        "user_data_shared": True,
    })
current = aws.get_caller_identity()
test = aws.iam.get_policy_document_output(statements=[{
    "effect": "Allow",
    "actions": [
        "mobiletargeting:UpdateEndpoint",
        "mobiletargeting:PutEvents",
    ],
    "resources": [test_app.application_id.apply(lambda application_id: f"arn:aws:mobiletargeting:*:{current.account_id}:apps/{application_id}*")],
}])
test_role_policy = aws.iam.RolePolicy("test",
    name="role_policy",
    role=test_role.id,
    policy=test.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testUserPool = new Aws.Cognito.UserPool("test", new()
    {
        Name = "pool",
    });

    var testApp = new Aws.Pinpoint.App("test", new()
    {
        Name = "pinpoint",
    });

    var assumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "cognito-idp.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var testRole = new Aws.Iam.Role("test", new()
    {
        Name = "role",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var testUserPoolClient = new Aws.Cognito.UserPoolClient("test", new()
    {
        Name = "pool_client",
        UserPoolId = testUserPool.Id,
        AnalyticsConfiguration = new Aws.Cognito.Inputs.UserPoolClientAnalyticsConfigurationArgs
        {
            ApplicationId = testApp.ApplicationId,
            ExternalId = "some_id",
            RoleArn = testRole.Arn,
            UserDataShared = true,
        },
    });

    var current = Aws.GetCallerIdentity.Invoke();

    var test = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "mobiletargeting:UpdateEndpoint",
                    "mobiletargeting:PutEvents",
                },
                Resources = new[]
                {
                    $"arn:aws:mobiletargeting:*:{current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:apps/{testApp.ApplicationId}*",
                },
            },
        },
    });

    var testRolePolicy = new Aws.Iam.RolePolicy("test", new()
    {
        Name = "role_policy",
        Role = testRole.Id,
        Policy = test.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/pinpoint"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		testUserPool, err := cognito.NewUserPool(ctx, "test", &cognito.UserPoolArgs{
			Name: pulumi.String("pool"),
		})
		if err != nil {
			return err
		}
		testApp, err := pinpoint.NewApp(ctx, "test", &pinpoint.AppArgs{
			Name: pulumi.String("pinpoint"),
		})
		if err != nil {
			return err
		}
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"cognito-idp.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		testRole, err := iam.NewRole(ctx, "test", &iam.RoleArgs{
			Name:             pulumi.String("role"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUserPoolClient(ctx, "test", &cognito.UserPoolClientArgs{
			Name:       pulumi.String("pool_client"),
			UserPoolId: testUserPool.ID(),
			AnalyticsConfiguration: &cognito.UserPoolClientAnalyticsConfigurationArgs{
				ApplicationId:  testApp.ApplicationId,
				ExternalId:     pulumi.String("some_id"),
				RoleArn:        testRole.Arn,
				UserDataShared: pulumi.Bool(true),
			},
		})
		if err != nil {
			return err
		}
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		test := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Effect: pulumi.String("Allow"),
					Actions: pulumi.StringArray{
						pulumi.String("mobiletargeting:UpdateEndpoint"),
						pulumi.String("mobiletargeting:PutEvents"),
					},
					Resources: pulumi.StringArray{
						testApp.ApplicationId.ApplyT(func(applicationId string) (string, error) {
							return fmt.Sprintf("arn:aws:mobiletargeting:*:%v:apps/%v*", current.AccountId, applicationId), nil
						}).(pulumi.StringOutput),
					},
				},
			},
		}, nil)
		_, err = iam.NewRolePolicy(ctx, "test", &iam.RolePolicyArgs{
			Name: pulumi.String("role_policy"),
			Role: testRole.ID(),
			Policy: pulumi.String(test.ApplyT(func(test iam.GetPolicyDocumentResult) (*string, error) {
				return &test.Json, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.pinpoint.App;
import com.pulumi.aws.pinpoint.AppArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.cognito.UserPoolClient;
import com.pulumi.aws.cognito.UserPoolClientArgs;
import com.pulumi.aws.cognito.inputs.UserPoolClientAnalyticsConfigurationArgs;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var testUserPool = new UserPool("testUserPool", UserPoolArgs.builder()
            .name("pool")
            .build());

        var testApp = new App("testApp", AppArgs.builder()
            .name("pinpoint")
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("cognito-idp.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var testRole = new Role("testRole", RoleArgs.builder()
            .name("role")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var testUserPoolClient = new UserPoolClient("testUserPoolClient", UserPoolClientArgs.builder()
            .name("pool_client")
            .userPoolId(testUserPool.id())
            .analyticsConfiguration(UserPoolClientAnalyticsConfigurationArgs.builder()
                .applicationId(testApp.applicationId())
                .externalId("some_id")
                .roleArn(testRole.arn())
                .userDataShared(true)
                .build())
            .build());

        final var current = AwsFunctions.getCallerIdentity();

        final var test = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions(                
                    "mobiletargeting:UpdateEndpoint",
                    "mobiletargeting:PutEvents")
                .resources(testApp.applicationId().applyValue(applicationId -> String.format("arn:aws:mobiletargeting:*:%s:apps/%s*", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()),applicationId)))
                .build())
            .build());

        var testRolePolicy = new RolePolicy("testRolePolicy", RolePolicyArgs.builder()
            .name("role_policy")
            .role(testRole.id())
            .policy(test.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(test -> test.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  testUserPoolClient:
    type: aws:cognito:UserPoolClient
    name: test
    properties:
      name: pool_client
      userPoolId: ${testUserPool.id}
      analyticsConfiguration:
        applicationId: ${testApp.applicationId}
        externalId: some_id
        roleArn: ${testRole.arn}
        userDataShared: true
  testUserPool:
    type: aws:cognito:UserPool
    name: test
    properties:
      name: pool
  testApp:
    type: aws:pinpoint:App
    name: test
    properties:
      name: pinpoint
  testRole:
    type: aws:iam:Role
    name: test
    properties:
      name: role
      assumeRolePolicy: ${assumeRole.json}
  testRolePolicy:
    type: aws:iam:RolePolicy
    name: test
    properties:
      name: role_policy
      role: ${testRole.id}
      policy: ${test.json}
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - cognito-idp.amazonaws.com
            actions:
              - sts:AssumeRole
  test:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - mobiletargeting:UpdateEndpoint
              - mobiletargeting:PutEvents
            resources:
              - arn:aws:mobiletargeting:*:${current.accountId}:apps/${testApp.applicationId}*
```
<!--End PulumiCodeChooser -->

### Create a user pool client with Cognito as the identity provider

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const pool = new aws.cognito.UserPool("pool", {name: "pool"});
const userpoolClient = new aws.cognito.UserPoolClient("userpool_client", {
    name: "client",
    userPoolId: pool.id,
    callbackUrls: ["https://example.com"],
    allowedOauthFlowsUserPoolClient: true,
    allowedOauthFlows: [
        "code",
        "implicit",
    ],
    allowedOauthScopes: [
        "email",
        "openid",
    ],
    supportedIdentityProviders: ["COGNITO"],
});
```
```python
import pulumi
import pulumi_aws as aws

pool = aws.cognito.UserPool("pool", name="pool")
userpool_client = aws.cognito.UserPoolClient("userpool_client",
    name="client",
    user_pool_id=pool.id,
    callback_urls=["https://example.com"],
    allowed_oauth_flows_user_pool_client=True,
    allowed_oauth_flows=[
        "code",
        "implicit",
    ],
    allowed_oauth_scopes=[
        "email",
        "openid",
    ],
    supported_identity_providers=["COGNITO"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var pool = new Aws.Cognito.UserPool("pool", new()
    {
        Name = "pool",
    });

    var userpoolClient = new Aws.Cognito.UserPoolClient("userpool_client", new()
    {
        Name = "client",
        UserPoolId = pool.Id,
        CallbackUrls = new[]
        {
            "https://example.com",
        },
        AllowedOauthFlowsUserPoolClient = true,
        AllowedOauthFlows = new[]
        {
            "code",
            "implicit",
        },
        AllowedOauthScopes = new[]
        {
            "email",
            "openid",
        },
        SupportedIdentityProviders = new[]
        {
            "COGNITO",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		pool, err := cognito.NewUserPool(ctx, "pool", &cognito.UserPoolArgs{
			Name: pulumi.String("pool"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUserPoolClient(ctx, "userpool_client", &cognito.UserPoolClientArgs{
			Name:       pulumi.String("client"),
			UserPoolId: pool.ID(),
			CallbackUrls: pulumi.StringArray{
				pulumi.String("https://example.com"),
			},
			AllowedOauthFlowsUserPoolClient: pulumi.Bool(true),
			AllowedOauthFlows: pulumi.StringArray{
				pulumi.String("code"),
				pulumi.String("implicit"),
			},
			AllowedOauthScopes: pulumi.StringArray{
				pulumi.String("email"),
				pulumi.String("openid"),
			},
			SupportedIdentityProviders: pulumi.StringArray{
				pulumi.String("COGNITO"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolClient;
import com.pulumi.aws.cognito.UserPoolClientArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pool = new UserPool("pool", UserPoolArgs.builder()
            .name("pool")
            .build());

        var userpoolClient = new UserPoolClient("userpoolClient", UserPoolClientArgs.builder()
            .name("client")
            .userPoolId(pool.id())
            .callbackUrls("https://example.com")
            .allowedOauthFlowsUserPoolClient(true)
            .allowedOauthFlows(            
                "code",
                "implicit")
            .allowedOauthScopes(            
                "email",
                "openid")
            .supportedIdentityProviders("COGNITO")
            .build());

    }
}
```
```yaml
resources:
  userpoolClient:
    type: aws:cognito:UserPoolClient
    name: userpool_client
    properties:
      name: client
      userPoolId: ${pool.id}
      callbackUrls:
        - https://example.com
      allowedOauthFlowsUserPoolClient: true
      allowedOauthFlows:
        - code
        - implicit
      allowedOauthScopes:
        - email
        - openid
      supportedIdentityProviders:
        - COGNITO
  pool:
    type: aws:cognito:UserPool
    properties:
      name: pool
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito User Pool Clients using the `id` of the Cognito User Pool, and the `id` of the Cognito User Pool Client. For example:

```sh
$ pulumi import aws:cognito/userPoolClient:UserPoolClient client us-west-2_abc123/3ho4ek12345678909nh3fmhpko
```
�
accessTokenValidityB �Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
�
allowedOauthFlowsB*" �List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
�
allowedOauthFlowsUserPoolClientB
 �Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
�
allowedOauthScopesB*" �List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
�
analyticsConfiguration�B�:�
�
cognito$UserPoolClientAnalyticsConfigurationUaws:cognito/UserPoolClientAnalyticsConfiguration:UserPoolClientAnalyticsConfigurationoConfiguration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
�
authSessionValidityB �Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
�
callbackUrlsB*" �List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
d
defaultRedirectUriB" HDefault redirect URI and must be included in the list of callback URLs.
k
(enablePropagateAdditionalUserContextDataB
 9Enables the propagation of additional user context data.
E
enableTokenRevocationB
 &Enables or disables token revocation.
�
explicitAuthFlowsB*" �List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
c
generateSecretB
 KBoolean flag indicating whether an application secret should be generated.
�
idTokenValidityB �Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
�

logoutUrlsB*" �List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
.
nameB"  Name of the application client.
�
preventUserExistenceErrorsB" �Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
b
readAttributesB*" HList of user pool attributes that the application client can read from.
�
refreshTokenValidityB �Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
�
supportedIdentityProvidersB*" �List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
�
tokenValidityUnits�B~:|
z
cognito UserPoolClientTokenValidityUnitsMaws:cognito/UserPoolClientTokenValidityUnits:UserPoolClientTokenValidityUnitseConfiguration block for representing the validity times in units. See details below. Detailed below.
Z

userPoolId" HUser pool the client belongs to.

The following arguments are optional:
b
writeAttributesB*" GList of user pool attributes that the application client can write to.
"�
accessTokenValidity �Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
"�
allowedOauthFlows*" �List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
"�
allowedOauthFlowsUserPoolClient
 �Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
"�
allowedOauthScopes*" �List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
"�
analyticsConfiguration�B�:�
�
cognito$UserPoolClientAnalyticsConfigurationUaws:cognito/UserPoolClientAnalyticsConfiguration:UserPoolClientAnalyticsConfigurationoConfiguration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
"�
authSessionValidity �Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
"�
callbackUrls*" �List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
";
clientSecret" 'Client secret of the user pool client.
"b
defaultRedirectUri" HDefault redirect URI and must be included in the list of callback URLs.
"i
(enablePropagateAdditionalUserContextData
 9Enables the propagation of additional user context data.
"C
enableTokenRevocation
 &Enables or disables token revocation.
"�
explicitAuthFlows*" �List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
"c
generateSecretB
 KBoolean flag indicating whether an application secret should be generated.
"�
idTokenValidity �Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
"�

logoutUrls*" �List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
",
name"  Name of the application client.
"�
preventUserExistenceErrors" �Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
"`
readAttributes*" HList of user pool attributes that the application client can read from.
"�
refreshTokenValidity �Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
"�
supportedIdentityProviders*" �List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
"�
tokenValidityUnits�B~:|
z
cognito UserPoolClientTokenValidityUnitsMaws:cognito/UserPoolClientTokenValidityUnits:UserPoolClientTokenValidityUnitseConfiguration block for representing the validity times in units. See details below. Detailed below.
"Z

userPoolId" HUser pool the client belongs to.

The following arguments are optional:
"`
writeAttributes*" GList of user pool attributes that the application client can write to.
*�S
D
cognitoUserPoolDomain)aws:cognito/userPoolDomain:UserPoolDomain�HProvides a Cognito User Pool Domain resource.

## Example Usage

### Amazon Cognito domain

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cognito.UserPool("example", {name: "example-pool"});
const main = new aws.cognito.UserPoolDomain("main", {
    domain: "example-domain",
    userPoolId: example.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.UserPool("example", name="example-pool")
main = aws.cognito.UserPoolDomain("main",
    domain="example-domain",
    user_pool_id=example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example-pool",
    });

    var main = new Aws.Cognito.UserPoolDomain("main", new()
    {
        Domain = "example-domain",
        UserPoolId = example.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example-pool"),
		})
		if err != nil {
			return err
		}
		_, err = cognito.NewUserPoolDomain(ctx, "main", &cognito.UserPoolDomainArgs{
			Domain:     pulumi.String("example-domain"),
			UserPoolId: example.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolDomain;
import com.pulumi.aws.cognito.UserPoolDomainArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .name("example-pool")
            .build());

        var main = new UserPoolDomain("main", UserPoolDomainArgs.builder()
            .domain("example-domain")
            .userPoolId(example.id())
            .build());

    }
}
```
```yaml
resources:
  main:
    type: aws:cognito:UserPoolDomain
    properties:
      domain: example-domain
      userPoolId: ${example.id}
  example:
    type: aws:cognito:UserPool
    properties:
      name: example-pool
```
<!--End PulumiCodeChooser -->

### Custom Cognito domain

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleUserPool = new aws.cognito.UserPool("example", {name: "example-pool"});
const main = new aws.cognito.UserPoolDomain("main", {
    domain: "auth.example.com",
    certificateArn: cert.arn,
    userPoolId: exampleUserPool.id,
});
const example = aws.route53.getZone({
    name: "example.com",
});
const auth_cognito_A = new aws.route53.Record("auth-cognito-A", {
    name: main.domain,
    type: aws.route53.RecordType.A,
    zoneId: example.then(example => example.zoneId),
    aliases: [{
        evaluateTargetHealth: false,
        name: main.cloudfrontDistribution,
        zoneId: main.cloudfrontDistributionZoneId,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example_user_pool = aws.cognito.UserPool("example", name="example-pool")
main = aws.cognito.UserPoolDomain("main",
    domain="auth.example.com",
    certificate_arn=cert["arn"],
    user_pool_id=example_user_pool.id)
example = aws.route53.get_zone(name="example.com")
auth_cognito__a = aws.route53.Record("auth-cognito-A",
    name=main.domain,
    type=aws.route53.RecordType.A,
    zone_id=example.zone_id,
    aliases=[{
        "evaluate_target_health": False,
        "name": main.cloudfront_distribution,
        "zone_id": main.cloudfront_distribution_zone_id,
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleUserPool = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example-pool",
    });

    var main = new Aws.Cognito.UserPoolDomain("main", new()
    {
        Domain = "auth.example.com",
        CertificateArn = cert.Arn,
        UserPoolId = exampleUserPool.Id,
    });

    var example = Aws.Route53.GetZone.Invoke(new()
    {
        Name = "example.com",
    });

    var auth_cognito_A = new Aws.Route53.Record("auth-cognito-A", new()
    {
        Name = main.Domain,
        Type = Aws.Route53.RecordType.A,
        ZoneId = example.Apply(getZoneResult => getZoneResult.ZoneId),
        Aliases = new[]
        {
            new Aws.Route53.Inputs.RecordAliasArgs
            {
                EvaluateTargetHealth = false,
                Name = main.CloudfrontDistribution,
                ZoneId = main.CloudfrontDistributionZoneId,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/route53"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleUserPool, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example-pool"),
		})
		if err != nil {
			return err
		}
		main, err := cognito.NewUserPoolDomain(ctx, "main", &cognito.UserPoolDomainArgs{
			Domain:         pulumi.String("auth.example.com"),
			CertificateArn: pulumi.Any(cert.Arn),
			UserPoolId:     exampleUserPool.ID(),
		})
		if err != nil {
			return err
		}
		example, err := route53.LookupZone(ctx, &route53.LookupZoneArgs{
			Name: pulumi.StringRef("example.com"),
		}, nil)
		if err != nil {
			return err
		}
		_, err = route53.NewRecord(ctx, "auth-cognito-A", &route53.RecordArgs{
			Name:   main.Domain,
			Type:   pulumi.String(route53.RecordTypeA),
			ZoneId: pulumi.String(example.ZoneId),
			Aliases: route53.RecordAliasArray{
				&route53.RecordAliasArgs{
					EvaluateTargetHealth: pulumi.Bool(false),
					Name:                 main.CloudfrontDistribution,
					ZoneId:               main.CloudfrontDistributionZoneId,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolDomain;
import com.pulumi.aws.cognito.UserPoolDomainArgs;
import com.pulumi.aws.route53.Route53Functions;
import com.pulumi.aws.route53.inputs.GetZoneArgs;
import com.pulumi.aws.route53.Record;
import com.pulumi.aws.route53.RecordArgs;
import com.pulumi.aws.route53.inputs.RecordAliasArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleUserPool = new UserPool("exampleUserPool", UserPoolArgs.builder()
            .name("example-pool")
            .build());

        var main = new UserPoolDomain("main", UserPoolDomainArgs.builder()
            .domain("auth.example.com")
            .certificateArn(cert.arn())
            .userPoolId(exampleUserPool.id())
            .build());

        final var example = Route53Functions.getZone(GetZoneArgs.builder()
            .name("example.com")
            .build());

        var auth_cognito_A = new Record("auth-cognito-A", RecordArgs.builder()
            .name(main.domain())
            .type("A")
            .zoneId(example.applyValue(getZoneResult -> getZoneResult.zoneId()))
            .aliases(RecordAliasArgs.builder()
                .evaluateTargetHealth(false)
                .name(main.cloudfrontDistribution())
                .zoneId(main.cloudfrontDistributionZoneId())
                .build())
            .build());

    }
}
```
```yaml
resources:
  main:
    type: aws:cognito:UserPoolDomain
    properties:
      domain: auth.example.com
      certificateArn: ${cert.arn}
      userPoolId: ${exampleUserPool.id}
  exampleUserPool:
    type: aws:cognito:UserPool
    name: example
    properties:
      name: example-pool
  auth-cognito-A:
    type: aws:route53:Record
    properties:
      name: ${main.domain}
      type: A
      zoneId: ${example.zoneId}
      aliases:
        - evaluateTargetHealth: false
          name: ${main.cloudfrontDistribution}
          zoneId: ${main.cloudfrontDistributionZoneId}
variables:
  example:
    fn::invoke:
      function: aws:route53:getZone
      arguments:
        name: example.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito User Pool Domains using the `domain`. For example:

```sh
$ pulumi import aws:cognito/userPoolDomain:UserPoolDomain main auth.example.org
```
_
certificateArnB" GThe ARN of an ISSUED ACM certificate in us-east-1 for a custom domain.
�
domain" �For custom domains, this is the fully-qualified domain name, such as auth.example.com. For Amazon Cognito prefix domains, this is the prefix alone, such as auth.
$

userPoolId" The user pool ID.
"@
awsAccountId" ,The AWS account ID for the user pool owner.
"_
certificateArnB" GThe ARN of an ISSUED ACM certificate in us-east-1 for a custom domain.
"�
cloudfrontDistribution" �The Amazon CloudFront endpoint (e.g. `dpp0gtxikpq3y.cloudfront.net`) that you use as the target of the alias that you set up with your Domain Name Service (DNS) provider.
"�
cloudfrontDistributionArn" dThe URL of the CloudFront distribution. This is required to generate the ALIAS `aws.route53.Record`
"`
cloudfrontDistributionZoneId" <The Route 53 hosted zone ID of the CloudFront distribution.
"�
domain" �For custom domains, this is the fully-qualified domain name, such as auth.example.com. For Amazon Cognito prefix domains, this is the prefix alone, such as auth.
"Q
s3Bucket" AThe S3 bucket where the static files for this domain are stored.
"$

userPoolId" The user pool ID.
" 
version" The app version.
*�n
_
cognitoUserPoolUICustomization;aws:cognito/userPoolUICustomization:UserPoolUICustomization�`Provides a Cognito User Pool UI Customization resource.

> **Note:** To use this resource, the user pool must have a domain associated with it. For more information, see the Amazon Cognito Developer Guide on [Customizing the Built-in Sign-In and Sign-up Webpages](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-app-ui-customization.html).

## Example Usage

### UI customization settings for a single client

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.cognito.UserPool("example", {name: "example"});
const exampleUserPoolDomain = new aws.cognito.UserPoolDomain("example", {
    domain: "example",
    userPoolId: example.id,
});
const exampleUserPoolClient = new aws.cognito.UserPoolClient("example", {
    name: "example",
    userPoolId: example.id,
});
const exampleUserPoolUICustomization = new aws.cognito.UserPoolUICustomization("example", {
    clientId: exampleUserPoolClient.id,
    css: ".label-customizable {font-weight: 400;}",
    imageFile: std.filebase64({
        input: "logo.png",
    }).then(invoke => invoke.result),
    userPoolId: exampleUserPoolDomain.userPoolId,
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.cognito.UserPool("example", name="example")
example_user_pool_domain = aws.cognito.UserPoolDomain("example",
    domain="example",
    user_pool_id=example.id)
example_user_pool_client = aws.cognito.UserPoolClient("example",
    name="example",
    user_pool_id=example.id)
example_user_pool_ui_customization = aws.cognito.UserPoolUICustomization("example",
    client_id=example_user_pool_client.id,
    css=".label-customizable {font-weight: 400;}",
    image_file=std.filebase64(input="logo.png").result,
    user_pool_id=example_user_pool_domain.user_pool_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example",
    });

    var exampleUserPoolDomain = new Aws.Cognito.UserPoolDomain("example", new()
    {
        Domain = "example",
        UserPoolId = example.Id,
    });

    var exampleUserPoolClient = new Aws.Cognito.UserPoolClient("example", new()
    {
        Name = "example",
        UserPoolId = example.Id,
    });

    var exampleUserPoolUICustomization = new Aws.Cognito.UserPoolUICustomization("example", new()
    {
        ClientId = exampleUserPoolClient.Id,
        Css = ".label-customizable {font-weight: 400;}",
        ImageFile = Std.Filebase64.Invoke(new()
        {
            Input = "logo.png",
        }).Apply(invoke => invoke.Result),
        UserPoolId = exampleUserPoolDomain.UserPoolId,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleUserPoolDomain, err := cognito.NewUserPoolDomain(ctx, "example", &cognito.UserPoolDomainArgs{
			Domain:     pulumi.String("example"),
			UserPoolId: example.ID(),
		})
		if err != nil {
			return err
		}
		exampleUserPoolClient, err := cognito.NewUserPoolClient(ctx, "example", &cognito.UserPoolClientArgs{
			Name:       pulumi.String("example"),
			UserPoolId: example.ID(),
		})
		if err != nil {
			return err
		}
		invokeFilebase64, err := std.Filebase64(ctx, &std.Filebase64Args{
			Input: "logo.png",
		}, nil)
		if err != nil {
			return err
		}
		_, err = cognito.NewUserPoolUICustomization(ctx, "example", &cognito.UserPoolUICustomizationArgs{
			ClientId:   exampleUserPoolClient.ID(),
			Css:        pulumi.String(".label-customizable {font-weight: 400;}"),
			ImageFile:  pulumi.String(invokeFilebase64.Result),
			UserPoolId: exampleUserPoolDomain.UserPoolId,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolDomain;
import com.pulumi.aws.cognito.UserPoolDomainArgs;
import com.pulumi.aws.cognito.UserPoolClient;
import com.pulumi.aws.cognito.UserPoolClientArgs;
import com.pulumi.aws.cognito.UserPoolUICustomization;
import com.pulumi.aws.cognito.UserPoolUICustomizationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .name("example")
            .build());

        var exampleUserPoolDomain = new UserPoolDomain("exampleUserPoolDomain", UserPoolDomainArgs.builder()
            .domain("example")
            .userPoolId(example.id())
            .build());

        var exampleUserPoolClient = new UserPoolClient("exampleUserPoolClient", UserPoolClientArgs.builder()
            .name("example")
            .userPoolId(example.id())
            .build());

        var exampleUserPoolUICustomization = new UserPoolUICustomization("exampleUserPoolUICustomization", UserPoolUICustomizationArgs.builder()
            .clientId(exampleUserPoolClient.id())
            .css(".label-customizable {font-weight: 400;}")
            .imageFile(StdFunctions.filebase64(Filebase64Args.builder()
                .input("logo.png")
                .build()).result())
            .userPoolId(exampleUserPoolDomain.userPoolId())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:UserPool
    properties:
      name: example
  exampleUserPoolDomain:
    type: aws:cognito:UserPoolDomain
    name: example
    properties:
      domain: example
      userPoolId: ${example.id}
  exampleUserPoolClient:
    type: aws:cognito:UserPoolClient
    name: example
    properties:
      name: example
      userPoolId: ${example.id}
  exampleUserPoolUICustomization:
    type: aws:cognito:UserPoolUICustomization
    name: example
    properties:
      clientId: ${exampleUserPoolClient.id}
      css: '.label-customizable {font-weight: 400;}'
      imageFile:
        fn::invoke:
          function: std:filebase64
          arguments:
            input: logo.png
          return: result
      userPoolId: ${exampleUserPoolDomain.userPoolId}
```
<!--End PulumiCodeChooser -->

### UI customization settings for all clients

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.cognito.UserPool("example", {name: "example"});
const exampleUserPoolDomain = new aws.cognito.UserPoolDomain("example", {
    domain: "example",
    userPoolId: example.id,
});
const exampleUserPoolUICustomization = new aws.cognito.UserPoolUICustomization("example", {
    css: ".label-customizable {font-weight: 400;}",
    imageFile: std.filebase64({
        input: "logo.png",
    }).then(invoke => invoke.result),
    userPoolId: exampleUserPoolDomain.userPoolId,
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.cognito.UserPool("example", name="example")
example_user_pool_domain = aws.cognito.UserPoolDomain("example",
    domain="example",
    user_pool_id=example.id)
example_user_pool_ui_customization = aws.cognito.UserPoolUICustomization("example",
    css=".label-customizable {font-weight: 400;}",
    image_file=std.filebase64(input="logo.png").result,
    user_pool_id=example_user_pool_domain.user_pool_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example",
    });

    var exampleUserPoolDomain = new Aws.Cognito.UserPoolDomain("example", new()
    {
        Domain = "example",
        UserPoolId = example.Id,
    });

    var exampleUserPoolUICustomization = new Aws.Cognito.UserPoolUICustomization("example", new()
    {
        Css = ".label-customizable {font-weight: 400;}",
        ImageFile = Std.Filebase64.Invoke(new()
        {
            Input = "logo.png",
        }).Apply(invoke => invoke.Result),
        UserPoolId = exampleUserPoolDomain.UserPoolId,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleUserPoolDomain, err := cognito.NewUserPoolDomain(ctx, "example", &cognito.UserPoolDomainArgs{
			Domain:     pulumi.String("example"),
			UserPoolId: example.ID(),
		})
		if err != nil {
			return err
		}
		invokeFilebase64, err := std.Filebase64(ctx, &std.Filebase64Args{
			Input: "logo.png",
		}, nil)
		if err != nil {
			return err
		}
		_, err = cognito.NewUserPoolUICustomization(ctx, "example", &cognito.UserPoolUICustomizationArgs{
			Css:        pulumi.String(".label-customizable {font-weight: 400;}"),
			ImageFile:  pulumi.String(invokeFilebase64.Result),
			UserPoolId: exampleUserPoolDomain.UserPoolId,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolDomain;
import com.pulumi.aws.cognito.UserPoolDomainArgs;
import com.pulumi.aws.cognito.UserPoolUICustomization;
import com.pulumi.aws.cognito.UserPoolUICustomizationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserPool("example", UserPoolArgs.builder()
            .name("example")
            .build());

        var exampleUserPoolDomain = new UserPoolDomain("exampleUserPoolDomain", UserPoolDomainArgs.builder()
            .domain("example")
            .userPoolId(example.id())
            .build());

        var exampleUserPoolUICustomization = new UserPoolUICustomization("exampleUserPoolUICustomization", UserPoolUICustomizationArgs.builder()
            .css(".label-customizable {font-weight: 400;}")
            .imageFile(StdFunctions.filebase64(Filebase64Args.builder()
                .input("logo.png")
                .build()).result())
            .userPoolId(exampleUserPoolDomain.userPoolId())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cognito:UserPool
    properties:
      name: example
  exampleUserPoolDomain:
    type: aws:cognito:UserPoolDomain
    name: example
    properties:
      domain: example
      userPoolId: ${example.id}
  exampleUserPoolUICustomization:
    type: aws:cognito:UserPoolUICustomization
    name: example
    properties:
      css: '.label-customizable {font-weight: 400;}'
      imageFile:
        fn::invoke:
          function: std:filebase64
          arguments:
            input: logo.png
          return: result
      userPoolId: ${exampleUserPoolDomain.userPoolId}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cognito User Pool UI Customizations using the `user_pool_id` and `client_id` separated by `,`. For example:

```sh
$ pulumi import aws:cognito/userPoolUICustomization:UserPoolUICustomization example us-west-2_ZCTarbt5C,12bu4fuk3mlgqa2rtrujgp6egq
```
�
clientIdB" �The client ID for the client app. Defaults to `ALL`. If `ALL` is specified, the `css` and/or `image_file` settings will be used for every client that has no UI customization set previously.
~
cssB" qThe CSS values in the UI customization, provided as a String. At least one of `css` or `image_file` is required.
�
	imageFileB" �The uploaded logo image for the UI customization, provided as a base64-encoded String. Drift detection is not possible for this argument. At least one of `css` or `image_file` is required.
6

userPoolId" $The user pool ID for the user pool.
"�
clientIdB" �The client ID for the client app. Defaults to `ALL`. If `ALL` is specified, the `css` and/or `image_file` settings will be used for every client that has no UI customization set previously.
"�
creationDate" qThe creation date in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) for the UI customization.
"~
cssB" qThe CSS values in the UI customization, provided as a String. At least one of `css` or `image_file` is required.
"*

cssVersion" The CSS version number.
"�
	imageFileB" �The uploaded logo image for the UI customization, provided as a base64-encoded String. Drift detection is not possible for this argument. At least one of `css` or `image_file` is required.
"=
imageUrl" -The logo image URL for the UI customization.
"�
lastModifiedDate" vThe last-modified date in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) for the UI customization.
"6

userPoolId" $The user pool ID for the user pool.
*�P
V

comprehendDocumentClassifier4aws:comprehend/documentClassifier:DocumentClassifier�'Resource for managing an AWS Comprehend Document Classifier.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const documents = new aws.s3.BucketObjectv2("documents", {});
const example = new aws.comprehend.DocumentClassifier("example", {
    name: "example",
    dataAccessRoleArn: exampleAwsIamRole.arn,
    languageCode: "en",
    inputDataConfig: {
        s3Uri: pulumi.interpolate`s3://${test.bucket}/${documents.id}`,
    },
}, {
    dependsOn: [exampleAwsIamRolePolicy],
});
const entities = new aws.s3.BucketObjectv2("entities", {});
```
```python
import pulumi
import pulumi_aws as aws

documents = aws.s3.BucketObjectv2("documents")
example = aws.comprehend.DocumentClassifier("example",
    name="example",
    data_access_role_arn=example_aws_iam_role["arn"],
    language_code="en",
    input_data_config={
        "s3_uri": documents.id.apply(lambda id: f"s3://{test['bucket']}/{id}"),
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_iam_role_policy]))
entities = aws.s3.BucketObjectv2("entities")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var documents = new Aws.S3.BucketObjectv2("documents");

    var example = new Aws.Comprehend.DocumentClassifier("example", new()
    {
        Name = "example",
        DataAccessRoleArn = exampleAwsIamRole.Arn,
        LanguageCode = "en",
        InputDataConfig = new Aws.Comprehend.Inputs.DocumentClassifierInputDataConfigArgs
        {
            S3Uri = documents.Id.Apply(id => $"s3://{test.Bucket}/{id}"),
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsIamRolePolicy,
        },
    });

    var entities = new Aws.S3.BucketObjectv2("entities");

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/comprehend"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		documents, err := s3.NewBucketObjectv2(ctx, "documents", nil)
		if err != nil {
			return err
		}
		_, err = comprehend.NewDocumentClassifier(ctx, "example", &comprehend.DocumentClassifierArgs{
			Name:              pulumi.String("example"),
			DataAccessRoleArn: pulumi.Any(exampleAwsIamRole.Arn),
			LanguageCode:      pulumi.String("en"),
			InputDataConfig: &comprehend.DocumentClassifierInputDataConfigArgs{
				S3Uri: documents.ID().ApplyT(func(id string) (string, error) {
					return fmt.Sprintf("s3://%v/%v", test.Bucket, id), nil
				}).(pulumi.StringOutput),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsIamRolePolicy,
		}))
		if err != nil {
			return err
		}
		_, err = s3.NewBucketObjectv2(ctx, "entities", nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.comprehend.DocumentClassifier;
import com.pulumi.aws.comprehend.DocumentClassifierArgs;
import com.pulumi.aws.comprehend.inputs.DocumentClassifierInputDataConfigArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var documents = new BucketObjectv2("documents");

        var example = new DocumentClassifier("example", DocumentClassifierArgs.builder()
            .name("example")
            .dataAccessRoleArn(exampleAwsIamRole.arn())
            .languageCode("en")
            .inputDataConfig(DocumentClassifierInputDataConfigArgs.builder()
                .s3Uri(documents.id().applyValue(id -> String.format("s3://%s/%s", test.bucket(),id)))
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsIamRolePolicy)
                .build());

        var entities = new BucketObjectv2("entities");

    }
}
```
```yaml
resources:
  example:
    type: aws:comprehend:DocumentClassifier
    properties:
      name: example
      dataAccessRoleArn: ${exampleAwsIamRole.arn}
      languageCode: en
      inputDataConfig:
        s3Uri: s3://${test.bucket}/${documents.id}
    options:
      dependsOn:
        - ${exampleAwsIamRolePolicy}
  documents:
    type: aws:s3:BucketObjectv2
  entities:
    type: aws:s3:BucketObjectv2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Comprehend Document Classifier using the ARN. For example:

```sh
$ pulumi import aws:comprehend/documentClassifier:DocumentClassifier example arn:aws:comprehend:us-west-2:123456789012:document_classifier/example
```
p
dataAccessRoleArn" WThe ARN for an IAM Role which allows Comprehend to read the training and testing data.
�
inputDataConfig�:�
�

comprehend!DocumentClassifierInputDataConfigRaws:comprehend/DocumentClassifierInputDataConfig:DocumentClassifierInputDataConfigpConfiguration for the training and testing data.
See the `input_data_config` Configuration Block section below.
m
languageCode" YTwo-letter language code for the language.
One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
�
modeB" �The document classification mode.
One of `MULTI_CLASS` or `MULTI_LABEL`.
`MULTI_CLASS` is also known as "Single Label" in the AWS Console.
s
modelKmsKeyIdB" \KMS Key used to encrypt trained Document Classifiers.
Can be a KMS Key ID or a KMS Key ARN.
�
nameB" �Name for the Document Classifier.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).

The following arguments are optional:
�
outputDataConfig�B�:�
�

comprehend"DocumentClassifierOutputDataConfigTaws:comprehend/DocumentClassifierOutputDataConfig:DocumentClassifierOutputDataConfigrConfiguration for the output results of training.
See the `output_data_config` Configuration Block section below.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
�
versionNameB" �Name for the version of the Document Classifier.
Each version must have a unique name within the Document Classifier.
If omitted, the provider will assign a random, unique version name.
If explicitly set to `""`, no version name will be set.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name_prefix`.
�
versionNamePrefixB" �Creates a unique version name beginning with the specified prefix.
Has a maximum length of 37 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name`.
}
volumeKmsKeyIdB" eKMS Key used to encrypt storage volumes during job processing.
Can be a KMS Key ID or a KMS Key ARN.
�
	vpcConfigwBu:s
q

comprehendDocumentClassifierVpcConfigFaws:comprehend/DocumentClassifierVpcConfig:DocumentClassifierVpcConfig�Configuration parameters for VPC to contain Document Classifier resources.
See the `vpc_config` Configuration Block section below.
"3
arn" (ARN of the Document Classifier version.
"p
dataAccessRoleArn" WThe ARN for an IAM Role which allows Comprehend to read the training and testing data.
"�
inputDataConfig�:�
�

comprehend!DocumentClassifierInputDataConfigRaws:comprehend/DocumentClassifierInputDataConfig:DocumentClassifierInputDataConfigpConfiguration for the training and testing data.
See the `input_data_config` Configuration Block section below.
"m
languageCode" YTwo-letter language code for the language.
One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
"�
modeB" �The document classification mode.
One of `MULTI_CLASS` or `MULTI_LABEL`.
`MULTI_CLASS` is also known as "Single Label" in the AWS Console.
"s
modelKmsKeyIdB" \KMS Key used to encrypt trained Document Classifiers.
Can be a KMS Key ID or a KMS Key ARN.
"�
name" �Name for the Document Classifier.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).

The following arguments are optional:
"�
outputDataConfig�:�
�

comprehend"DocumentClassifierOutputDataConfigTaws:comprehend/DocumentClassifierOutputDataConfig:DocumentClassifierOutputDataConfigrConfiguration for the output results of training.
See the `output_data_config` Configuration Block section below.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
versionName" �Name for the version of the Document Classifier.
Each version must have a unique name within the Document Classifier.
If omitted, the provider will assign a random, unique version name.
If explicitly set to `""`, no version name will be set.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name_prefix`.
"�
versionNamePrefix" �Creates a unique version name beginning with the specified prefix.
Has a maximum length of 37 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name`.
"}
volumeKmsKeyIdB" eKMS Key used to encrypt storage volumes during job processing.
Can be a KMS Key ID or a KMS Key ARN.
"�
	vpcConfigwBu:s
q

comprehendDocumentClassifierVpcConfigFaws:comprehend/DocumentClassifierVpcConfig:DocumentClassifierVpcConfig�Configuration parameters for VPC to contain Document Classifier resources.
See the `vpc_config` Configuration Block section below.
*�a
P

comprehendEntityRecognizer0aws:comprehend/entityRecognizer:EntityRecognizer�@Resource for managing an AWS Comprehend Entity Recognizer.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const documents = new aws.s3.BucketObjectv2("documents", {});
const entities = new aws.s3.BucketObjectv2("entities", {});
const example = new aws.comprehend.EntityRecognizer("example", {
    name: "example",
    dataAccessRoleArn: exampleAwsIamRole.arn,
    languageCode: "en",
    inputDataConfig: {
        entityTypes: [
            {
                type: "ENTITY_1",
            },
            {
                type: "ENTITY_2",
            },
        ],
        documents: {
            s3Uri: pulumi.interpolate`s3://${documentsAwsS3Bucket.bucket}/${documents.id}`,
        },
        entityList: {
            s3Uri: pulumi.interpolate`s3://${entitiesAwsS3Bucket.bucket}/${entities.id}`,
        },
    },
}, {
    dependsOn: [exampleAwsIamRolePolicy],
});
```
```python
import pulumi
import pulumi_aws as aws

documents = aws.s3.BucketObjectv2("documents")
entities = aws.s3.BucketObjectv2("entities")
example = aws.comprehend.EntityRecognizer("example",
    name="example",
    data_access_role_arn=example_aws_iam_role["arn"],
    language_code="en",
    input_data_config={
        "entity_types": [
            {
                "type": "ENTITY_1",
            },
            {
                "type": "ENTITY_2",
            },
        ],
        "documents": {
            "s3_uri": documents.id.apply(lambda id: f"s3://{documents_aws_s3_bucket['bucket']}/{id}"),
        },
        "entity_list": {
            "s3_uri": entities.id.apply(lambda id: f"s3://{entities_aws_s3_bucket['bucket']}/{id}"),
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_iam_role_policy]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var documents = new Aws.S3.BucketObjectv2("documents");

    var entities = new Aws.S3.BucketObjectv2("entities");

    var example = new Aws.Comprehend.EntityRecognizer("example", new()
    {
        Name = "example",
        DataAccessRoleArn = exampleAwsIamRole.Arn,
        LanguageCode = "en",
        InputDataConfig = new Aws.Comprehend.Inputs.EntityRecognizerInputDataConfigArgs
        {
            EntityTypes = new[]
            {
                new Aws.Comprehend.Inputs.EntityRecognizerInputDataConfigEntityTypeArgs
                {
                    Type = "ENTITY_1",
                },
                new Aws.Comprehend.Inputs.EntityRecognizerInputDataConfigEntityTypeArgs
                {
                    Type = "ENTITY_2",
                },
            },
            Documents = new Aws.Comprehend.Inputs.EntityRecognizerInputDataConfigDocumentsArgs
            {
                S3Uri = documents.Id.Apply(id => $"s3://{documentsAwsS3Bucket.Bucket}/{id}"),
            },
            EntityList = new Aws.Comprehend.Inputs.EntityRecognizerInputDataConfigEntityListArgs
            {
                S3Uri = entities.Id.Apply(id => $"s3://{entitiesAwsS3Bucket.Bucket}/{id}"),
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsIamRolePolicy,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/comprehend"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		documents, err := s3.NewBucketObjectv2(ctx, "documents", nil)
		if err != nil {
			return err
		}
		entities, err := s3.NewBucketObjectv2(ctx, "entities", nil)
		if err != nil {
			return err
		}
		_, err = comprehend.NewEntityRecognizer(ctx, "example", &comprehend.EntityRecognizerArgs{
			Name:              pulumi.String("example"),
			DataAccessRoleArn: pulumi.Any(exampleAwsIamRole.Arn),
			LanguageCode:      pulumi.String("en"),
			InputDataConfig: &comprehend.EntityRecognizerInputDataConfigArgs{
				EntityTypes: comprehend.EntityRecognizerInputDataConfigEntityTypeArray{
					&comprehend.EntityRecognizerInputDataConfigEntityTypeArgs{
						Type: pulumi.String("ENTITY_1"),
					},
					&comprehend.EntityRecognizerInputDataConfigEntityTypeArgs{
						Type: pulumi.String("ENTITY_2"),
					},
				},
				Documents: &comprehend.EntityRecognizerInputDataConfigDocumentsArgs{
					S3Uri: documents.ID().ApplyT(func(id string) (string, error) {
						return fmt.Sprintf("s3://%v/%v", documentsAwsS3Bucket.Bucket, id), nil
					}).(pulumi.StringOutput),
				},
				EntityList: &comprehend.EntityRecognizerInputDataConfigEntityListArgs{
					S3Uri: entities.ID().ApplyT(func(id string) (string, error) {
						return fmt.Sprintf("s3://%v/%v", entitiesAwsS3Bucket.Bucket, id), nil
					}).(pulumi.StringOutput),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsIamRolePolicy,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.comprehend.EntityRecognizer;
import com.pulumi.aws.comprehend.EntityRecognizerArgs;
import com.pulumi.aws.comprehend.inputs.EntityRecognizerInputDataConfigArgs;
import com.pulumi.aws.comprehend.inputs.EntityRecognizerInputDataConfigDocumentsArgs;
import com.pulumi.aws.comprehend.inputs.EntityRecognizerInputDataConfigEntityListArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var documents = new BucketObjectv2("documents");

        var entities = new BucketObjectv2("entities");

        var example = new EntityRecognizer("example", EntityRecognizerArgs.builder()
            .name("example")
            .dataAccessRoleArn(exampleAwsIamRole.arn())
            .languageCode("en")
            .inputDataConfig(EntityRecognizerInputDataConfigArgs.builder()
                .entityTypes(                
                    EntityRecognizerInputDataConfigEntityTypeArgs.builder()
                        .type("ENTITY_1")
                        .build(),
                    EntityRecognizerInputDataConfigEntityTypeArgs.builder()
                        .type("ENTITY_2")
                        .build())
                .documents(EntityRecognizerInputDataConfigDocumentsArgs.builder()
                    .s3Uri(documents.id().applyValue(id -> String.format("s3://%s/%s", documentsAwsS3Bucket.bucket(),id)))
                    .build())
                .entityList(EntityRecognizerInputDataConfigEntityListArgs.builder()
                    .s3Uri(entities.id().applyValue(id -> String.format("s3://%s/%s", entitiesAwsS3Bucket.bucket(),id)))
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsIamRolePolicy)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:comprehend:EntityRecognizer
    properties:
      name: example
      dataAccessRoleArn: ${exampleAwsIamRole.arn}
      languageCode: en
      inputDataConfig:
        entityTypes:
          - type: ENTITY_1
          - type: ENTITY_2
        documents:
          s3Uri: s3://${documentsAwsS3Bucket.bucket}/${documents.id}
        entityList:
          s3Uri: s3://${entitiesAwsS3Bucket.bucket}/${entities.id}
    options:
      dependsOn:
        - ${exampleAwsIamRolePolicy}
  documents:
    type: aws:s3:BucketObjectv2
  entities:
    type: aws:s3:BucketObjectv2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Comprehend Entity Recognizer using the ARN. For example:

```sh
$ pulumi import aws:comprehend/entityRecognizer:EntityRecognizer example arn:aws:comprehend:us-west-2:123456789012:entity-recognizer/example
```
p
dataAccessRoleArn" WThe ARN for an IAM Role which allows Comprehend to read the training and testing data.
�
inputDataConfig�:
}

comprehendEntityRecognizerInputDataConfigNaws:comprehend/EntityRecognizerInputDataConfig:EntityRecognizerInputDataConfigpConfiguration for the training and testing data.
See the `input_data_config` Configuration Block section below.
m
languageCode" YTwo-letter language code for the language.
One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
^
modelKmsKeyIdB" GThe ID or ARN of a KMS Key used to encrypt trained Entity Recognizers.
�
nameB" �Name for the Entity Recognizer.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).

The following arguments are optional:
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
�
versionNameB" �Name for the version of the Entity Recognizer.
Each version must have a unique name within the Entity Recognizer.
If omitted, the provider will assign a random, unique version name.
If explicitly set to `""`, no version name will be set.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name_prefix`.
�
versionNamePrefixB" �Creates a unique version name beginning with the specified prefix.
Has a maximum length of 37 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name`.
f
volumeKmsKeyIdB" NID or ARN of a KMS Key used to encrypt storage volumes during job processing.
�
	vpcConfigqBo:m
k

comprehendEntityRecognizerVpcConfigBaws:comprehend/EntityRecognizerVpcConfig:EntityRecognizerVpcConfig�Configuration parameters for VPC to contain Entity Recognizer resources.
See the `vpc_config` Configuration Block section below.
"1
arn" &ARN of the Entity Recognizer version.
"p
dataAccessRoleArn" WThe ARN for an IAM Role which allows Comprehend to read the training and testing data.
"�
inputDataConfig�:
}

comprehendEntityRecognizerInputDataConfigNaws:comprehend/EntityRecognizerInputDataConfig:EntityRecognizerInputDataConfigpConfiguration for the training and testing data.
See the `input_data_config` Configuration Block section below.
"m
languageCode" YTwo-letter language code for the language.
One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
"^
modelKmsKeyIdB" GThe ID or ARN of a KMS Key used to encrypt trained Entity Recognizers.
"�
name" �Name for the Entity Recognizer.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).

The following arguments are optional:
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
versionName" �Name for the version of the Entity Recognizer.
Each version must have a unique name within the Entity Recognizer.
If omitted, the provider will assign a random, unique version name.
If explicitly set to `""`, no version name will be set.
Has a maximum length of 63 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name_prefix`.
"�
versionNamePrefix" �Creates a unique version name beginning with the specified prefix.
Has a maximum length of 37 characters.
Can contain upper- and lower-case letters, numbers, and hypen (`-`).
Conflicts with `version_name`.
"f
volumeKmsKeyIdB" NID or ARN of a KMS Key used to encrypt storage volumes during job processing.
"�
	vpcConfigqBo:m
k

comprehendEntityRecognizerVpcConfigBaws:comprehend/EntityRecognizerVpcConfig:EntityRecognizerVpcConfig�Configuration parameters for VPC to contain Entity Recognizer resources.
See the `vpc_config` Configuration Block section below.
*�
\
computeoptimizerEnrollmentStatus6aws:computeoptimizer/enrollmentStatus:EnrollmentStatus�Manages AWS Compute Optimizer enrollment status.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.computeoptimizer.EnrollmentStatus("example", {status: "Active"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.computeoptimizer.EnrollmentStatus("example", status="Active")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ComputeOptimizer.EnrollmentStatus("example", new()
    {
        Status = "Active",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/computeoptimizer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := computeoptimizer.NewEnrollmentStatus(ctx, "example", &computeoptimizer.EnrollmentStatusArgs{
			Status: pulumi.String("Active"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.computeoptimizer.EnrollmentStatus;
import com.pulumi.aws.computeoptimizer.EnrollmentStatusArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new EnrollmentStatus("example", EnrollmentStatusArgs.builder()
            .status("Active")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:computeoptimizer:EnrollmentStatus
    properties:
      status: Active
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import enrollment status using the account ID. For example:

```sh
$ pulumi import aws:computeoptimizer/enrollmentStatus:EnrollmentStatus example 123456789012
```
�
includeMemberAccountsB
 �Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
X
status" JThe enrollment status of the account. Valid values: `Active`, `Inactive`.
�
timeoutszBx:v
t
computeoptimizerEnrollmentStatusTimeoutsFaws:computeoptimizer/EnrollmentStatusTimeouts:EnrollmentStatusTimeouts"�
includeMemberAccounts
 �Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
"�
numberOfMemberAccountsOptedIn �The count of organization member accounts that are opted in to the service, if your account is an organization management account.
"X
status" JThe enrollment status of the account. Valid values: `Active`, `Inactive`.
"�
timeoutszBx:v
t
computeoptimizerEnrollmentStatusTimeoutsFaws:computeoptimizer/EnrollmentStatusTimeouts:EnrollmentStatusTimeouts*�f
w
computeoptimizerRecommendationPreferencesHaws:computeoptimizer/recommendationPreferences:RecommendationPreferences�DManages AWS Compute Optimizer recommendation preferences.

## Example Usage

### Lookback Period Preference

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.computeoptimizer.RecommendationPreferences("example", {
    resourceType: "Ec2Instance",
    scope: {
        name: "AccountId",
        value: "123456789012",
    },
    lookBackPeriod: "DAYS_32",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.computeoptimizer.RecommendationPreferences("example",
    resource_type="Ec2Instance",
    scope={
        "name": "AccountId",
        "value": "123456789012",
    },
    look_back_period="DAYS_32")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ComputeOptimizer.RecommendationPreferences("example", new()
    {
        ResourceType = "Ec2Instance",
        Scope = new Aws.ComputeOptimizer.Inputs.RecommendationPreferencesScopeArgs
        {
            Name = "AccountId",
            Value = "123456789012",
        },
        LookBackPeriod = "DAYS_32",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/computeoptimizer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := computeoptimizer.NewRecommendationPreferences(ctx, "example", &computeoptimizer.RecommendationPreferencesArgs{
			ResourceType: pulumi.String("Ec2Instance"),
			Scope: &computeoptimizer.RecommendationPreferencesScopeArgs{
				Name:  pulumi.String("AccountId"),
				Value: pulumi.String("123456789012"),
			},
			LookBackPeriod: pulumi.String("DAYS_32"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.computeoptimizer.RecommendationPreferences;
import com.pulumi.aws.computeoptimizer.RecommendationPreferencesArgs;
import com.pulumi.aws.computeoptimizer.inputs.RecommendationPreferencesScopeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new RecommendationPreferences("example", RecommendationPreferencesArgs.builder()
            .resourceType("Ec2Instance")
            .scope(RecommendationPreferencesScopeArgs.builder()
                .name("AccountId")
                .value("123456789012")
                .build())
            .lookBackPeriod("DAYS_32")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:computeoptimizer:RecommendationPreferences
    properties:
      resourceType: Ec2Instance
      scope:
        name: AccountId
        value: '123456789012'
      lookBackPeriod: DAYS_32
```
<!--End PulumiCodeChooser -->

### Multiple Preferences

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.computeoptimizer.RecommendationPreferences("example", {
    resourceType: "Ec2Instance",
    scope: {
        name: "AccountId",
        value: "123456789012",
    },
    enhancedInfrastructureMetrics: "Active",
    externalMetricsPreference: {
        source: "Datadog",
    },
    preferredResources: [{
        includeLists: [
            "m5.xlarge",
            "r5",
        ],
        name: "Ec2InstanceTypes",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.computeoptimizer.RecommendationPreferences("example",
    resource_type="Ec2Instance",
    scope={
        "name": "AccountId",
        "value": "123456789012",
    },
    enhanced_infrastructure_metrics="Active",
    external_metrics_preference={
        "source": "Datadog",
    },
    preferred_resources=[{
        "include_lists": [
            "m5.xlarge",
            "r5",
        ],
        "name": "Ec2InstanceTypes",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ComputeOptimizer.RecommendationPreferences("example", new()
    {
        ResourceType = "Ec2Instance",
        Scope = new Aws.ComputeOptimizer.Inputs.RecommendationPreferencesScopeArgs
        {
            Name = "AccountId",
            Value = "123456789012",
        },
        EnhancedInfrastructureMetrics = "Active",
        ExternalMetricsPreference = new Aws.ComputeOptimizer.Inputs.RecommendationPreferencesExternalMetricsPreferenceArgs
        {
            Source = "Datadog",
        },
        PreferredResources = new[]
        {
            new Aws.ComputeOptimizer.Inputs.RecommendationPreferencesPreferredResourceArgs
            {
                IncludeLists = new[]
                {
                    "m5.xlarge",
                    "r5",
                },
                Name = "Ec2InstanceTypes",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/computeoptimizer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := computeoptimizer.NewRecommendationPreferences(ctx, "example", &computeoptimizer.RecommendationPreferencesArgs{
			ResourceType: pulumi.String("Ec2Instance"),
			Scope: &computeoptimizer.RecommendationPreferencesScopeArgs{
				Name:  pulumi.String("AccountId"),
				Value: pulumi.String("123456789012"),
			},
			EnhancedInfrastructureMetrics: pulumi.String("Active"),
			ExternalMetricsPreference: &computeoptimizer.RecommendationPreferencesExternalMetricsPreferenceArgs{
				Source: pulumi.String("Datadog"),
			},
			PreferredResources: computeoptimizer.RecommendationPreferencesPreferredResourceArray{
				&computeoptimizer.RecommendationPreferencesPreferredResourceArgs{
					IncludeLists: pulumi.StringArray{
						pulumi.String("m5.xlarge"),
						pulumi.String("r5"),
					},
					Name: pulumi.String("Ec2InstanceTypes"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.computeoptimizer.RecommendationPreferences;
import com.pulumi.aws.computeoptimizer.RecommendationPreferencesArgs;
import com.pulumi.aws.computeoptimizer.inputs.RecommendationPreferencesScopeArgs;
import com.pulumi.aws.computeoptimizer.inputs.RecommendationPreferencesExternalMetricsPreferenceArgs;
import com.pulumi.aws.computeoptimizer.inputs.RecommendationPreferencesPreferredResourceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new RecommendationPreferences("example", RecommendationPreferencesArgs.builder()
            .resourceType("Ec2Instance")
            .scope(RecommendationPreferencesScopeArgs.builder()
                .name("AccountId")
                .value("123456789012")
                .build())
            .enhancedInfrastructureMetrics("Active")
            .externalMetricsPreference(RecommendationPreferencesExternalMetricsPreferenceArgs.builder()
                .source("Datadog")
                .build())
            .preferredResources(RecommendationPreferencesPreferredResourceArgs.builder()
                .includeLists(                
                    "m5.xlarge",
                    "r5")
                .name("Ec2InstanceTypes")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:computeoptimizer:RecommendationPreferences
    properties:
      resourceType: Ec2Instance
      scope:
        name: AccountId
        value: '123456789012'
      enhancedInfrastructureMetrics: Active
      externalMetricsPreference:
        source: Datadog
      preferredResources:
        - includeLists:
            - m5.xlarge
            - r5
          name: Ec2InstanceTypes
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import recommendation preferences using the resource type, scope name and scope value. For example:

```sh
$ pulumi import aws:computeoptimizer/recommendationPreferences:RecommendationPreferences example Ec2Instance,AccountId,123456789012
```
�
enhancedInfrastructureMetricsB" qThe status of the enhanced infrastructure metrics recommendation preference. Valid values: `Active`, `Inactive`.
�
externalMetricsPreference�B�:�
�
computeoptimizer2RecommendationPreferencesExternalMetricsPreferencezaws:computeoptimizer/RecommendationPreferencesExternalMetricsPreference:RecommendationPreferencesExternalMetricsPreferencegThe provider of the external metrics recommendation preference. See External Metrics Preference below.
�
inferredWorkloadTypesB" iThe status of the inferred workload types recommendation preference. Valid values: `Active`, `Inactive`.
�
lookBackPeriodB" �The preference to control the number of days the utilization metrics of the AWS resource are analyzed. Valid values: `DAYS_14`, `DAYS_32`, `DAYS_93`.
�
preferredResources�B�*�:�
�
computeoptimizer*RecommendationPreferencesPreferredResourcejaws:computeoptimizer/RecommendationPreferencesPreferredResource:RecommendationPreferencesPreferredResource�The preference to control which resource type values are considered when generating rightsizing recommendations. See Preferred Resources below.
�
resourceType" ~The target resource type of the recommendation preferences. Valid values: `Ec2Instance`, `AutoScalingGroup`, `RdsDBInstance`.
�
savingsEstimationModeB" iThe status of the savings estimation mode preference. Valid values: `AfterDiscounts`, `BeforeDiscounts`.
�
scope�B�:�
�
computeoptimizerRecommendationPreferencesScopeRaws:computeoptimizer/RecommendationPreferencesScope:RecommendationPreferencesScope>The scope of the recommendation preferences. See Scope below.
�
utilizationPreferences�B�*�:�
�
computeoptimizer.RecommendationPreferencesUtilizationPreferenceraws:computeoptimizer/RecommendationPreferencesUtilizationPreference:RecommendationPreferencesUtilizationPreference�The preference to control the resource’s CPU utilization threshold, CPU utilization headroom, and memory utilization headroom. See Utilization Preferences below.
"�
enhancedInfrastructureMetricsB" qThe status of the enhanced infrastructure metrics recommendation preference. Valid values: `Active`, `Inactive`.
"�
externalMetricsPreference�B�:�
�
computeoptimizer2RecommendationPreferencesExternalMetricsPreferencezaws:computeoptimizer/RecommendationPreferencesExternalMetricsPreference:RecommendationPreferencesExternalMetricsPreferencegThe provider of the external metrics recommendation preference. See External Metrics Preference below.
"�
inferredWorkloadTypesB" iThe status of the inferred workload types recommendation preference. Valid values: `Active`, `Inactive`.
"�
lookBackPeriod" �The preference to control the number of days the utilization metrics of the AWS resource are analyzed. Valid values: `DAYS_14`, `DAYS_32`, `DAYS_93`.
"�
preferredResources�B�*�:�
�
computeoptimizer*RecommendationPreferencesPreferredResourcejaws:computeoptimizer/RecommendationPreferencesPreferredResource:RecommendationPreferencesPreferredResource�The preference to control which resource type values are considered when generating rightsizing recommendations. See Preferred Resources below.
"�
resourceType" ~The target resource type of the recommendation preferences. Valid values: `Ec2Instance`, `AutoScalingGroup`, `RdsDBInstance`.
"�
savingsEstimationModeB" iThe status of the savings estimation mode preference. Valid values: `AfterDiscounts`, `BeforeDiscounts`.
"�
scope�B�:�
�
computeoptimizerRecommendationPreferencesScopeRaws:computeoptimizer/RecommendationPreferencesScope:RecommendationPreferencesScope>The scope of the recommendation preferences. See Scope below.
"�
utilizationPreferences�B�*�:�
�
computeoptimizer.RecommendationPreferencesUtilizationPreferenceraws:computeoptimizer/RecommendationPreferencesUtilizationPreference:RecommendationPreferencesUtilizationPreference�The preference to control the resource’s CPU utilization threshold, CPU utilization headroom, and memory utilization headroom. See Utilization Preferences below.
*�r
D
connectBotAssociation)aws:connect/botAssociation:BotAssociation�mAllows the specified Amazon Connect instance to access the specified Amazon Lex (V1) bot. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html) and [Add an Amazon Lex bot](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-lex.html).

> **NOTE:** This resource only currently supports Amazon Lex (V1) Associations.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.BotAssociation("example", {
    instanceId: exampleAwsConnectInstance.id,
    lexBot: {
        lexRegion: "us-west-2",
        name: "Test",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.BotAssociation("example",
    instance_id=example_aws_connect_instance["id"],
    lex_bot={
        "lex_region": "us-west-2",
        "name": "Test",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.BotAssociation("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        LexBot = new Aws.Connect.Inputs.BotAssociationLexBotArgs
        {
            LexRegion = "us-west-2",
            Name = "Test",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewBotAssociation(ctx, "example", &connect.BotAssociationArgs{
			InstanceId: pulumi.Any(exampleAwsConnectInstance.Id),
			LexBot: &connect.BotAssociationLexBotArgs{
				LexRegion: pulumi.String("us-west-2"),
				Name:      pulumi.String("Test"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.BotAssociation;
import com.pulumi.aws.connect.BotAssociationArgs;
import com.pulumi.aws.connect.inputs.BotAssociationLexBotArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new BotAssociation("example", BotAssociationArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .lexBot(BotAssociationLexBotArgs.builder()
                .lexRegion("us-west-2")
                .name("Test")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:BotAssociation
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      lexBot:
        lexRegion: us-west-2
        name: Test
```
<!--End PulumiCodeChooser -->

### Including a sample Lex bot

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const example = new aws.lex.Intent("example", {
    createVersion: true,
    name: "connect_lex_intent",
    fulfillmentActivity: {
        type: "ReturnIntent",
    },
    sampleUtterances: ["I would like to pick up flowers."],
});
const exampleBot = new aws.lex.Bot("example", {
    abortStatement: {
        messages: [{
            content: "Sorry, I am not able to assist at this time.",
            contentType: "PlainText",
        }],
    },
    clarificationPrompt: {
        maxAttempts: 2,
        messages: [{
            content: "I didn't understand you, what would you like to do?",
            contentType: "PlainText",
        }],
    },
    intents: [{
        intentName: example.name,
        intentVersion: "1",
    }],
    childDirected: false,
    name: "connect_lex_bot",
    processBehavior: "BUILD",
});
const exampleBotAssociation = new aws.connect.BotAssociation("example", {
    instanceId: exampleAwsConnectInstance.id,
    lexBot: {
        lexRegion: current.then(current => current.name),
        name: exampleBot.name,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
example = aws.lex.Intent("example",
    create_version=True,
    name="connect_lex_intent",
    fulfillment_activity={
        "type": "ReturnIntent",
    },
    sample_utterances=["I would like to pick up flowers."])
example_bot = aws.lex.Bot("example",
    abort_statement={
        "messages": [{
            "content": "Sorry, I am not able to assist at this time.",
            "content_type": "PlainText",
        }],
    },
    clarification_prompt={
        "max_attempts": 2,
        "messages": [{
            "content": "I didn't understand you, what would you like to do?",
            "content_type": "PlainText",
        }],
    },
    intents=[{
        "intent_name": example.name,
        "intent_version": "1",
    }],
    child_directed=False,
    name="connect_lex_bot",
    process_behavior="BUILD")
example_bot_association = aws.connect.BotAssociation("example",
    instance_id=example_aws_connect_instance["id"],
    lex_bot={
        "lex_region": current.name,
        "name": example_bot.name,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var example = new Aws.Lex.Intent("example", new()
    {
        CreateVersion = true,
        Name = "connect_lex_intent",
        FulfillmentActivity = new Aws.Lex.Inputs.IntentFulfillmentActivityArgs
        {
            Type = "ReturnIntent",
        },
        SampleUtterances = new[]
        {
            "I would like to pick up flowers.",
        },
    });

    var exampleBot = new Aws.Lex.Bot("example", new()
    {
        AbortStatement = new Aws.Lex.Inputs.BotAbortStatementArgs
        {
            Messages = new[]
            {
                new Aws.Lex.Inputs.BotAbortStatementMessageArgs
                {
                    Content = "Sorry, I am not able to assist at this time.",
                    ContentType = "PlainText",
                },
            },
        },
        ClarificationPrompt = new Aws.Lex.Inputs.BotClarificationPromptArgs
        {
            MaxAttempts = 2,
            Messages = new[]
            {
                new Aws.Lex.Inputs.BotClarificationPromptMessageArgs
                {
                    Content = "I didn't understand you, what would you like to do?",
                    ContentType = "PlainText",
                },
            },
        },
        Intents = new[]
        {
            new Aws.Lex.Inputs.BotIntentArgs
            {
                IntentName = example.Name,
                IntentVersion = "1",
            },
        },
        ChildDirected = false,
        Name = "connect_lex_bot",
        ProcessBehavior = "BUILD",
    });

    var exampleBotAssociation = new Aws.Connect.BotAssociation("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        LexBot = new Aws.Connect.Inputs.BotAssociationLexBotArgs
        {
            LexRegion = current.Apply(getRegionResult => getRegionResult.Name),
            Name = exampleBot.Name,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/lex"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		example, err := lex.NewIntent(ctx, "example", &lex.IntentArgs{
			CreateVersion: pulumi.Bool(true),
			Name:          pulumi.String("connect_lex_intent"),
			FulfillmentActivity: &lex.IntentFulfillmentActivityArgs{
				Type: pulumi.String("ReturnIntent"),
			},
			SampleUtterances: pulumi.StringArray{
				pulumi.String("I would like to pick up flowers."),
			},
		})
		if err != nil {
			return err
		}
		exampleBot, err := lex.NewBot(ctx, "example", &lex.BotArgs{
			AbortStatement: &lex.BotAbortStatementArgs{
				Messages: lex.BotAbortStatementMessageArray{
					&lex.BotAbortStatementMessageArgs{
						Content:     pulumi.String("Sorry, I am not able to assist at this time."),
						ContentType: pulumi.String("PlainText"),
					},
				},
			},
			ClarificationPrompt: &lex.BotClarificationPromptArgs{
				MaxAttempts: pulumi.Int(2),
				Messages: lex.BotClarificationPromptMessageArray{
					&lex.BotClarificationPromptMessageArgs{
						Content:     pulumi.String("I didn't understand you, what would you like to do?"),
						ContentType: pulumi.String("PlainText"),
					},
				},
			},
			Intents: lex.BotIntentArray{
				&lex.BotIntentArgs{
					IntentName:    example.Name,
					IntentVersion: pulumi.String("1"),
				},
			},
			ChildDirected:   pulumi.Bool(false),
			Name:            pulumi.String("connect_lex_bot"),
			ProcessBehavior: pulumi.String("BUILD"),
		})
		if err != nil {
			return err
		}
		_, err = connect.NewBotAssociation(ctx, "example", &connect.BotAssociationArgs{
			InstanceId: pulumi.Any(exampleAwsConnectInstance.Id),
			LexBot: &connect.BotAssociationLexBotArgs{
				LexRegion: pulumi.String(current.Name),
				Name:      exampleBot.Name,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.lex.Intent;
import com.pulumi.aws.lex.IntentArgs;
import com.pulumi.aws.lex.inputs.IntentFulfillmentActivityArgs;
import com.pulumi.aws.lex.Bot;
import com.pulumi.aws.lex.BotArgs;
import com.pulumi.aws.lex.inputs.BotAbortStatementArgs;
import com.pulumi.aws.lex.inputs.BotClarificationPromptArgs;
import com.pulumi.aws.lex.inputs.BotIntentArgs;
import com.pulumi.aws.connect.BotAssociation;
import com.pulumi.aws.connect.BotAssociationArgs;
import com.pulumi.aws.connect.inputs.BotAssociationLexBotArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

        var example = new Intent("example", IntentArgs.builder()
            .createVersion(true)
            .name("connect_lex_intent")
            .fulfillmentActivity(IntentFulfillmentActivityArgs.builder()
                .type("ReturnIntent")
                .build())
            .sampleUtterances("I would like to pick up flowers.")
            .build());

        var exampleBot = new Bot("exampleBot", BotArgs.builder()
            .abortStatement(BotAbortStatementArgs.builder()
                .messages(BotAbortStatementMessageArgs.builder()
                    .content("Sorry, I am not able to assist at this time.")
                    .contentType("PlainText")
                    .build())
                .build())
            .clarificationPrompt(BotClarificationPromptArgs.builder()
                .maxAttempts(2)
                .messages(BotClarificationPromptMessageArgs.builder()
                    .content("I didn't understand you, what would you like to do?")
                    .contentType("PlainText")
                    .build())
                .build())
            .intents(BotIntentArgs.builder()
                .intentName(example.name())
                .intentVersion("1")
                .build())
            .childDirected(false)
            .name("connect_lex_bot")
            .processBehavior("BUILD")
            .build());

        var exampleBotAssociation = new BotAssociation("exampleBotAssociation", BotAssociationArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .lexBot(BotAssociationLexBotArgs.builder()
                .lexRegion(current.applyValue(getRegionResult -> getRegionResult.name()))
                .name(exampleBot.name())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:lex:Intent
    properties:
      createVersion: true
      name: connect_lex_intent
      fulfillmentActivity:
        type: ReturnIntent
      sampleUtterances:
        - I would like to pick up flowers.
  exampleBot:
    type: aws:lex:Bot
    name: example
    properties:
      abortStatement:
        messages:
          - content: Sorry, I am not able to assist at this time.
            contentType: PlainText
      clarificationPrompt:
        maxAttempts: 2
        messages:
          - content: I didn't understand you, what would you like to do?
            contentType: PlainText
      intents:
        - intentName: ${example.name}
          intentVersion: '1'
      childDirected: false
      name: connect_lex_bot
      processBehavior: BUILD
  exampleBotAssociation:
    type: aws:connect:BotAssociation
    name: example
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      lexBot:
        lexRegion: ${current.name}
        name: ${exampleBot.name}
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_connect_bot_association` using the Amazon Connect instance ID, Lex (V1) bot name, and Lex (V1) bot region separated by colons (`:`). For example:

```sh
$ pulumi import aws:connect/botAssociation:BotAssociation example aaaaaaaa-bbbb-cccc-dddd-111111111111:Example:us-west-2
```
y

instanceId" gThe identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
�
lexBotZ:X
V
connectBotAssociationLexBot5aws:connect/BotAssociationLexBot:BotAssociationLexBotEConfiguration information of an Amazon Lex (V1) bot. Detailed below.
"y

instanceId" gThe identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
"�
lexBotZ:X
V
connectBotAssociationLexBot5aws:connect/BotAssociationLexBot:BotAssociationLexBotEConfiguration information of an Amazon Lex (V1) bot. Detailed below.
*ђ
;
connectContactFlow#aws:connect/contactFlow:ContactFlow�~Provides an Amazon Connect Contact Flow resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

This resource embeds or references Contact Flows specified in Amazon Connect Contact Flow Language. For more information see
[Amazon Connect Flow language](https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html)

!> **WARN:** Contact Flows exported from the Console [Contact Flow import/export](https://docs.aws.amazon.com/connect/latest/adminguide/contact-flow-import-export.html) are not in the Amazon Connect Contact Flow Language and can not be used with this resource. Instead, the recommendation is to use the AWS CLI [`describe-contact-flow`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/connect/describe-contact-flow.html).
See example below which uses `jq` to extract the `Content` attribute and saves it to a local file.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.ContactFlow("test", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Test",
    description: "Test Contact Flow Description",
    type: "CONTACT_FLOW",
    content: JSON.stringify({
        Version: "2019-10-30",
        StartAction: "12345678-1234-1234-1234-123456789012",
        Actions: [
            {
                Identifier: "12345678-1234-1234-1234-123456789012",
                Type: "MessageParticipant",
                Transitions: {
                    NextAction: "abcdef-abcd-abcd-abcd-abcdefghijkl",
                    Errors: [],
                    Conditions: [],
                },
                Parameters: {
                    Text: "Thanks for calling the sample flow!",
                },
            },
            {
                Identifier: "abcdef-abcd-abcd-abcd-abcdefghijkl",
                Type: "DisconnectParticipant",
                Transitions: {},
                Parameters: {},
            },
        ],
    }),
    tags: {
        Name: "Test Contact Flow",
        Application: "Example",
        Method: "Create",
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

test = aws.connect.ContactFlow("test",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Test",
    description="Test Contact Flow Description",
    type="CONTACT_FLOW",
    content=json.dumps({
        "Version": "2019-10-30",
        "StartAction": "12345678-1234-1234-1234-123456789012",
        "Actions": [
            {
                "Identifier": "12345678-1234-1234-1234-123456789012",
                "Type": "MessageParticipant",
                "Transitions": {
                    "NextAction": "abcdef-abcd-abcd-abcd-abcdefghijkl",
                    "Errors": [],
                    "Conditions": [],
                },
                "Parameters": {
                    "Text": "Thanks for calling the sample flow!",
                },
            },
            {
                "Identifier": "abcdef-abcd-abcd-abcd-abcdefghijkl",
                "Type": "DisconnectParticipant",
                "Transitions": {},
                "Parameters": {},
            },
        ],
    }),
    tags={
        "Name": "Test Contact Flow",
        "Application": "Example",
        "Method": "Create",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.ContactFlow("test", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Test",
        Description = "Test Contact Flow Description",
        Type = "CONTACT_FLOW",
        Content = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2019-10-30",
            ["StartAction"] = "12345678-1234-1234-1234-123456789012",
            ["Actions"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Identifier"] = "12345678-1234-1234-1234-123456789012",
                    ["Type"] = "MessageParticipant",
                    ["Transitions"] = new Dictionary<string, object?>
                    {
                        ["NextAction"] = "abcdef-abcd-abcd-abcd-abcdefghijkl",
                        ["Errors"] = new[]
                        {
                        },
                        ["Conditions"] = new[]
                        {
                        },
                    },
                    ["Parameters"] = new Dictionary<string, object?>
                    {
                        ["Text"] = "Thanks for calling the sample flow!",
                    },
                },
                new Dictionary<string, object?>
                {
                    ["Identifier"] = "abcdef-abcd-abcd-abcd-abcdefghijkl",
                    ["Type"] = "DisconnectParticipant",
                    ["Transitions"] = new Dictionary<string, object?>
                    {
                    },
                    ["Parameters"] = new Dictionary<string, object?>
                    {
                    },
                },
            },
        }),
        Tags = 
        {
            { "Name", "Test Contact Flow" },
            { "Application", "Example" },
            { "Method", "Create" },
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version":     "2019-10-30",
			"StartAction": "12345678-1234-1234-1234-123456789012",
			"Actions": []interface{}{
				map[string]interface{}{
					"Identifier": "12345678-1234-1234-1234-123456789012",
					"Type":       "MessageParticipant",
					"Transitions": map[string]interface{}{
						"NextAction": "abcdef-abcd-abcd-abcd-abcdefghijkl",
						"Errors":     []interface{}{},
						"Conditions": []interface{}{},
					},
					"Parameters": map[string]interface{}{
						"Text": "Thanks for calling the sample flow!",
					},
				},
				map[string]interface{}{
					"Identifier":  "abcdef-abcd-abcd-abcd-abcdefghijkl",
					"Type":        "DisconnectParticipant",
					"Transitions": map[string]interface{}{},
					"Parameters":  map[string]interface{}{},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = connect.NewContactFlow(ctx, "test", &connect.ContactFlowArgs{
			InstanceId:  pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:        pulumi.String("Test"),
			Description: pulumi.String("Test Contact Flow Description"),
			Type:        pulumi.String("CONTACT_FLOW"),
			Content:     pulumi.String(json0),
			Tags: pulumi.StringMap{
				"Name":        pulumi.String("Test Contact Flow"),
				"Application": pulumi.String("Example"),
				"Method":      pulumi.String("Create"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ContactFlow;
import com.pulumi.aws.connect.ContactFlowArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new ContactFlow("test", ContactFlowArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Test")
            .description("Test Contact Flow Description")
            .type("CONTACT_FLOW")
            .content(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2019-10-30"),
                    jsonProperty("StartAction", "12345678-1234-1234-1234-123456789012"),
                    jsonProperty("Actions", jsonArray(
                        jsonObject(
                            jsonProperty("Identifier", "12345678-1234-1234-1234-123456789012"),
                            jsonProperty("Type", "MessageParticipant"),
                            jsonProperty("Transitions", jsonObject(
                                jsonProperty("NextAction", "abcdef-abcd-abcd-abcd-abcdefghijkl"),
                                jsonProperty("Errors", jsonArray(
                                )),
                                jsonProperty("Conditions", jsonArray(
                                ))
                            )),
                            jsonProperty("Parameters", jsonObject(
                                jsonProperty("Text", "Thanks for calling the sample flow!")
                            ))
                        ), 
                        jsonObject(
                            jsonProperty("Identifier", "abcdef-abcd-abcd-abcd-abcdefghijkl"),
                            jsonProperty("Type", "DisconnectParticipant"),
                            jsonProperty("Transitions", jsonObject(

                            )),
                            jsonProperty("Parameters", jsonObject(

                            ))
                        )
                    ))
                )))
            .tags(Map.ofEntries(
                Map.entry("Name", "Test Contact Flow"),
                Map.entry("Application", "Example"),
                Map.entry("Method", "Create")
            ))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:ContactFlow
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Test
      description: Test Contact Flow Description
      type: CONTACT_FLOW
      content:
        fn::toJSON:
          Version: 2019-10-30
          StartAction: 12345678-1234-1234-1234-123456789012
          Actions:
            - Identifier: 12345678-1234-1234-1234-123456789012
              Type: MessageParticipant
              Transitions:
                NextAction: abcdef-abcd-abcd-abcd-abcdefghijkl
                Errors: []
                Conditions: []
              Parameters:
                Text: Thanks for calling the sample flow!
            - Identifier: abcdef-abcd-abcd-abcd-abcdefghijkl
              Type: DisconnectParticipant
              Transitions: {}
              Parameters: {}
      tags:
        Name: Test Contact Flow
        Application: Example
        Method: Create
```
<!--End PulumiCodeChooser -->

### With External Content

Use the AWS CLI to extract Contact Flow Content:

```console
% aws connect describe-contact-flow --instance-id 1b3c5d8-1b3c-1b3c-1b3c-1b3c5d81b3c5 --contact-flow-id c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5 --region us-west-2 | jq '.ContactFlow.Content | fromjson' > contact_flow.json
```

Use the generated file as input:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const test = new aws.connect.ContactFlow("test", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Test",
    description: "Test Contact Flow Description",
    type: "CONTACT_FLOW",
    filename: "contact_flow.json",
    contentHash: std.filebase64sha256({
        input: "contact_flow.json",
    }).then(invoke => invoke.result),
    tags: {
        Name: "Test Contact Flow",
        Application: "Example",
        Method: "Create",
    },
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

test = aws.connect.ContactFlow("test",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Test",
    description="Test Contact Flow Description",
    type="CONTACT_FLOW",
    filename="contact_flow.json",
    content_hash=std.filebase64sha256(input="contact_flow.json").result,
    tags={
        "Name": "Test Contact Flow",
        "Application": "Example",
        "Method": "Create",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.ContactFlow("test", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Test",
        Description = "Test Contact Flow Description",
        Type = "CONTACT_FLOW",
        Filename = "contact_flow.json",
        ContentHash = Std.Filebase64sha256.Invoke(new()
        {
            Input = "contact_flow.json",
        }).Apply(invoke => invoke.Result),
        Tags = 
        {
            { "Name", "Test Contact Flow" },
            { "Application", "Example" },
            { "Method", "Create" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFilebase64sha256, err := std.Filebase64sha256(ctx, &std.Filebase64sha256Args{
			Input: "contact_flow.json",
		}, nil)
		if err != nil {
			return err
		}
		_, err = connect.NewContactFlow(ctx, "test", &connect.ContactFlowArgs{
			InstanceId:  pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:        pulumi.String("Test"),
			Description: pulumi.String("Test Contact Flow Description"),
			Type:        pulumi.String("CONTACT_FLOW"),
			Filename:    pulumi.String("contact_flow.json"),
			ContentHash: pulumi.String(invokeFilebase64sha256.Result),
			Tags: pulumi.StringMap{
				"Name":        pulumi.String("Test Contact Flow"),
				"Application": pulumi.String("Example"),
				"Method":      pulumi.String("Create"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ContactFlow;
import com.pulumi.aws.connect.ContactFlowArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new ContactFlow("test", ContactFlowArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Test")
            .description("Test Contact Flow Description")
            .type("CONTACT_FLOW")
            .filename("contact_flow.json")
            .contentHash(StdFunctions.filebase64sha256(Filebase64sha256Args.builder()
                .input("contact_flow.json")
                .build()).result())
            .tags(Map.ofEntries(
                Map.entry("Name", "Test Contact Flow"),
                Map.entry("Application", "Example"),
                Map.entry("Method", "Create")
            ))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:ContactFlow
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Test
      description: Test Contact Flow Description
      type: CONTACT_FLOW
      filename: contact_flow.json
      contentHash:
        fn::invoke:
          function: std:filebase64sha256
          arguments:
            input: contact_flow.json
          return: result
      tags:
        Name: Test Contact Flow
        Application: Example
        Method: Create
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Contact Flows using the `instance_id` and `contact_flow_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/contactFlow:ContactFlow example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
�
contentB" �Specifies the content of the Contact Flow, provided as a JSON string, written in Amazon Connect Contact Flow Language. If defined, the `filename` argument cannot be used.
�
contentHashB" {Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the Contact Flow source specified with `filename`.
D
descriptionB" /Specifies the description of the Contact Flow.
m
filenameB" [The path to the Contact Flow source within the local filesystem. Conflicts with `content`.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
6
nameB" (Specifies the name of the Contact Flow.
�
tagsB2" �Tags to apply to the Contact Flow. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
typeB" �Specifies the type of the Contact Flow. Defaults to `CONTACT_FLOW`. Allowed Values are: `CONTACT_FLOW`, `CUSTOMER_QUEUE`, `CUSTOMER_HOLD`, `CUSTOMER_WHISPER`, `AGENT_HOLD`, `AGENT_WHISPER`, `OUTBOUND_WHISPER`, `AGENT_TRANSFER`, `QUEUE_TRANSFER`.
"?
arn" 4The Amazon Resource Name (ARN) of the Contact Flow.
"9
contactFlowId" $The identifier of the Contact Flow.
"�
content" �Specifies the content of the Contact Flow, provided as a JSON string, written in Amazon Connect Contact Flow Language. If defined, the `filename` argument cannot be used.
"�
contentHashB" {Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the Contact Flow source specified with `filename`.
"D
descriptionB" /Specifies the description of the Contact Flow.
"m
filenameB" [The path to the Contact Flow source within the local filesystem. Conflicts with `content`.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"4
name" (Specifies the name of the Contact Flow.
"�
tagsB2" �Tags to apply to the Contact Flow. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
typeB" �Specifies the type of the Contact Flow. Defaults to `CONTACT_FLOW`. Allowed Values are: `CONTACT_FLOW`, `CUSTOMER_QUEUE`, `CUSTOMER_HOLD`, `CUSTOMER_WHISPER`, `AGENT_HOLD`, `AGENT_WHISPER`, `OUTBOUND_WHISPER`, `AGENT_TRANSFER`, `QUEUE_TRANSFER`.
*��
M
connectContactFlowModule/aws:connect/contactFlowModule:ContactFlowModule��Provides an Amazon Connect Contact Flow Module resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

This resource embeds or references Contact Flows Modules specified in Amazon Connect Contact Flow Language. For more information see
[Amazon Connect Flow language](https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html)

!> **WARN:** Contact Flow Modules exported from the Console [See Contact Flow import/export which is the same for Contact Flow Modules](https://docs.aws.amazon.com/connect/latest/adminguide/contact-flow-import-export.html) are not in the Amazon Connect Contact Flow Language and can not be used with this resource. Instead, the recommendation is to use the AWS CLI [`describe-contact-flow-module`](https://docs.aws.amazon.com/cli/latest/reference/connect/describe-contact-flow-module.html).
See example below which uses `jq` to extract the `Content` attribute and saves it to a local file.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.ContactFlowModule("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
    description: "Example Contact Flow Module Description",
    content: JSON.stringify({
        Version: "2019-10-30",
        StartAction: "12345678-1234-1234-1234-123456789012",
        Actions: [
            {
                Identifier: "12345678-1234-1234-1234-123456789012",
                Parameters: {
                    Text: "Hello contact flow module",
                },
                Transitions: {
                    NextAction: "abcdef-abcd-abcd-abcd-abcdefghijkl",
                    Errors: [],
                    Conditions: [],
                },
                Type: "MessageParticipant",
            },
            {
                Identifier: "abcdef-abcd-abcd-abcd-abcdefghijkl",
                Type: "DisconnectParticipant",
                Parameters: {},
                Transitions: {},
            },
        ],
        Settings: {
            InputParameters: [],
            OutputParameters: [],
            Transitions: [
                {
                    DisplayName: "Success",
                    ReferenceName: "Success",
                    Description: "",
                },
                {
                    DisplayName: "Error",
                    ReferenceName: "Error",
                    Description: "",
                },
            ],
        },
    }),
    tags: {
        Name: "Example Contact Flow Module",
        Application: "Example",
        Method: "Create",
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.connect.ContactFlowModule("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example",
    description="Example Contact Flow Module Description",
    content=json.dumps({
        "Version": "2019-10-30",
        "StartAction": "12345678-1234-1234-1234-123456789012",
        "Actions": [
            {
                "Identifier": "12345678-1234-1234-1234-123456789012",
                "Parameters": {
                    "Text": "Hello contact flow module",
                },
                "Transitions": {
                    "NextAction": "abcdef-abcd-abcd-abcd-abcdefghijkl",
                    "Errors": [],
                    "Conditions": [],
                },
                "Type": "MessageParticipant",
            },
            {
                "Identifier": "abcdef-abcd-abcd-abcd-abcdefghijkl",
                "Type": "DisconnectParticipant",
                "Parameters": {},
                "Transitions": {},
            },
        ],
        "Settings": {
            "InputParameters": [],
            "OutputParameters": [],
            "Transitions": [
                {
                    "DisplayName": "Success",
                    "ReferenceName": "Success",
                    "Description": "",
                },
                {
                    "DisplayName": "Error",
                    "ReferenceName": "Error",
                    "Description": "",
                },
            ],
        },
    }),
    tags={
        "Name": "Example Contact Flow Module",
        "Application": "Example",
        "Method": "Create",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.ContactFlowModule("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
        Description = "Example Contact Flow Module Description",
        Content = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2019-10-30",
            ["StartAction"] = "12345678-1234-1234-1234-123456789012",
            ["Actions"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Identifier"] = "12345678-1234-1234-1234-123456789012",
                    ["Parameters"] = new Dictionary<string, object?>
                    {
                        ["Text"] = "Hello contact flow module",
                    },
                    ["Transitions"] = new Dictionary<string, object?>
                    {
                        ["NextAction"] = "abcdef-abcd-abcd-abcd-abcdefghijkl",
                        ["Errors"] = new[]
                        {
                        },
                        ["Conditions"] = new[]
                        {
                        },
                    },
                    ["Type"] = "MessageParticipant",
                },
                new Dictionary<string, object?>
                {
                    ["Identifier"] = "abcdef-abcd-abcd-abcd-abcdefghijkl",
                    ["Type"] = "DisconnectParticipant",
                    ["Parameters"] = new Dictionary<string, object?>
                    {
                    },
                    ["Transitions"] = new Dictionary<string, object?>
                    {
                    },
                },
            },
            ["Settings"] = new Dictionary<string, object?>
            {
                ["InputParameters"] = new[]
                {
                },
                ["OutputParameters"] = new[]
                {
                },
                ["Transitions"] = new[]
                {
                    new Dictionary<string, object?>
                    {
                        ["DisplayName"] = "Success",
                        ["ReferenceName"] = "Success",
                        ["Description"] = "",
                    },
                    new Dictionary<string, object?>
                    {
                        ["DisplayName"] = "Error",
                        ["ReferenceName"] = "Error",
                        ["Description"] = "",
                    },
                },
            },
        }),
        Tags = 
        {
            { "Name", "Example Contact Flow Module" },
            { "Application", "Example" },
            { "Method", "Create" },
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version":     "2019-10-30",
			"StartAction": "12345678-1234-1234-1234-123456789012",
			"Actions": []interface{}{
				map[string]interface{}{
					"Identifier": "12345678-1234-1234-1234-123456789012",
					"Parameters": map[string]interface{}{
						"Text": "Hello contact flow module",
					},
					"Transitions": map[string]interface{}{
						"NextAction": "abcdef-abcd-abcd-abcd-abcdefghijkl",
						"Errors":     []interface{}{},
						"Conditions": []interface{}{},
					},
					"Type": "MessageParticipant",
				},
				map[string]interface{}{
					"Identifier":  "abcdef-abcd-abcd-abcd-abcdefghijkl",
					"Type":        "DisconnectParticipant",
					"Parameters":  map[string]interface{}{},
					"Transitions": map[string]interface{}{},
				},
			},
			"Settings": map[string]interface{}{
				"InputParameters":  []interface{}{},
				"OutputParameters": []interface{}{},
				"Transitions": []map[string]interface{}{
					map[string]interface{}{
						"DisplayName":   "Success",
						"ReferenceName": "Success",
						"Description":   "",
					},
					map[string]interface{}{
						"DisplayName":   "Error",
						"ReferenceName": "Error",
						"Description":   "",
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = connect.NewContactFlowModule(ctx, "example", &connect.ContactFlowModuleArgs{
			InstanceId:  pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:        pulumi.String("Example"),
			Description: pulumi.String("Example Contact Flow Module Description"),
			Content:     pulumi.String(json0),
			Tags: pulumi.StringMap{
				"Name":        pulumi.String("Example Contact Flow Module"),
				"Application": pulumi.String("Example"),
				"Method":      pulumi.String("Create"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ContactFlowModule;
import com.pulumi.aws.connect.ContactFlowModuleArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactFlowModule("example", ContactFlowModuleArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .description("Example Contact Flow Module Description")
            .content(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2019-10-30"),
                    jsonProperty("StartAction", "12345678-1234-1234-1234-123456789012"),
                    jsonProperty("Actions", jsonArray(
                        jsonObject(
                            jsonProperty("Identifier", "12345678-1234-1234-1234-123456789012"),
                            jsonProperty("Parameters", jsonObject(
                                jsonProperty("Text", "Hello contact flow module")
                            )),
                            jsonProperty("Transitions", jsonObject(
                                jsonProperty("NextAction", "abcdef-abcd-abcd-abcd-abcdefghijkl"),
                                jsonProperty("Errors", jsonArray(
                                )),
                                jsonProperty("Conditions", jsonArray(
                                ))
                            )),
                            jsonProperty("Type", "MessageParticipant")
                        ), 
                        jsonObject(
                            jsonProperty("Identifier", "abcdef-abcd-abcd-abcd-abcdefghijkl"),
                            jsonProperty("Type", "DisconnectParticipant"),
                            jsonProperty("Parameters", jsonObject(

                            )),
                            jsonProperty("Transitions", jsonObject(

                            ))
                        )
                    )),
                    jsonProperty("Settings", jsonObject(
                        jsonProperty("InputParameters", jsonArray(
                        )),
                        jsonProperty("OutputParameters", jsonArray(
                        )),
                        jsonProperty("Transitions", jsonArray(
                            jsonObject(
                                jsonProperty("DisplayName", "Success"),
                                jsonProperty("ReferenceName", "Success"),
                                jsonProperty("Description", "")
                            ), 
                            jsonObject(
                                jsonProperty("DisplayName", "Error"),
                                jsonProperty("ReferenceName", "Error"),
                                jsonProperty("Description", "")
                            )
                        ))
                    ))
                )))
            .tags(Map.ofEntries(
                Map.entry("Name", "Example Contact Flow Module"),
                Map.entry("Application", "Example"),
                Map.entry("Method", "Create")
            ))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:ContactFlowModule
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Example
      description: Example Contact Flow Module Description
      content:
        fn::toJSON:
          Version: 2019-10-30
          StartAction: 12345678-1234-1234-1234-123456789012
          Actions:
            - Identifier: 12345678-1234-1234-1234-123456789012
              Parameters:
                Text: Hello contact flow module
              Transitions:
                NextAction: abcdef-abcd-abcd-abcd-abcdefghijkl
                Errors: []
                Conditions: []
              Type: MessageParticipant
            - Identifier: abcdef-abcd-abcd-abcd-abcdefghijkl
              Type: DisconnectParticipant
              Parameters: {}
              Transitions: {}
          Settings:
            InputParameters: []
            OutputParameters: []
            Transitions:
              - DisplayName: Success
                ReferenceName: Success
                Description: ""
              - DisplayName: Error
                ReferenceName: Error
                Description: ""
      tags:
        Name: Example Contact Flow Module
        Application: Example
        Method: Create
```
<!--End PulumiCodeChooser -->

### With External Content

Use the AWS CLI to extract Contact Flow Content:

```console
% aws connect describe-contact-flow-module --instance-id 1b3c5d8-1b3c-1b3c-1b3c-1b3c5d81b3c5 --contact-flow-module-id c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5 --region us-west-2 | jq '.ContactFlowModule.Content | fromjson' > contact_flow_module.json
```

Use the generated file as input:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.connect.ContactFlowModule("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
    description: "Example Contact Flow Module Description",
    filename: "contact_flow_module.json",
    contentHash: std.filebase64sha256({
        input: "contact_flow_module.json",
    }).then(invoke => invoke.result),
    tags: {
        Name: "Example Contact Flow Module",
        Application: "Example",
        Method: "Create",
    },
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.connect.ContactFlowModule("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example",
    description="Example Contact Flow Module Description",
    filename="contact_flow_module.json",
    content_hash=std.filebase64sha256(input="contact_flow_module.json").result,
    tags={
        "Name": "Example Contact Flow Module",
        "Application": "Example",
        "Method": "Create",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.ContactFlowModule("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
        Description = "Example Contact Flow Module Description",
        Filename = "contact_flow_module.json",
        ContentHash = Std.Filebase64sha256.Invoke(new()
        {
            Input = "contact_flow_module.json",
        }).Apply(invoke => invoke.Result),
        Tags = 
        {
            { "Name", "Example Contact Flow Module" },
            { "Application", "Example" },
            { "Method", "Create" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFilebase64sha256, err := std.Filebase64sha256(ctx, &std.Filebase64sha256Args{
			Input: "contact_flow_module.json",
		}, nil)
		if err != nil {
			return err
		}
		_, err = connect.NewContactFlowModule(ctx, "example", &connect.ContactFlowModuleArgs{
			InstanceId:  pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:        pulumi.String("Example"),
			Description: pulumi.String("Example Contact Flow Module Description"),
			Filename:    pulumi.String("contact_flow_module.json"),
			ContentHash: pulumi.String(invokeFilebase64sha256.Result),
			Tags: pulumi.StringMap{
				"Name":        pulumi.String("Example Contact Flow Module"),
				"Application": pulumi.String("Example"),
				"Method":      pulumi.String("Create"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ContactFlowModule;
import com.pulumi.aws.connect.ContactFlowModuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactFlowModule("example", ContactFlowModuleArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .description("Example Contact Flow Module Description")
            .filename("contact_flow_module.json")
            .contentHash(StdFunctions.filebase64sha256(Filebase64sha256Args.builder()
                .input("contact_flow_module.json")
                .build()).result())
            .tags(Map.ofEntries(
                Map.entry("Name", "Example Contact Flow Module"),
                Map.entry("Application", "Example"),
                Map.entry("Method", "Create")
            ))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:ContactFlowModule
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Example
      description: Example Contact Flow Module Description
      filename: contact_flow_module.json
      contentHash:
        fn::invoke:
          function: std:filebase64sha256
          arguments:
            input: contact_flow_module.json
          return: result
      tags:
        Name: Example Contact Flow Module
        Application: Example
        Method: Create
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Contact Flow Modules using the `instance_id` and `contact_flow_module_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/contactFlowModule:ContactFlowModule example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
�
contentB" �Specifies the content of the Contact Flow Module, provided as a JSON string, written in Amazon Connect Contact Flow Language. If defined, the `filename` argument cannot be used.
�
contentHashB" �Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the Contact Flow Module source specified with `filename`.
K
descriptionB" 6Specifies the description of the Contact Flow Module.
t
filenameB" bThe path to the Contact Flow Module source within the local filesystem. Conflicts with `content`.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
=
nameB" /Specifies the name of the Contact Flow Module.
�
tagsB2" �Tags to apply to the Contact Flow Module. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"F
arn" ;The Amazon Resource Name (ARN) of the Contact Flow Module.
"F
contactFlowModuleId" +The identifier of the Contact Flow Module.
"�
content" �Specifies the content of the Contact Flow Module, provided as a JSON string, written in Amazon Connect Contact Flow Language. If defined, the `filename` argument cannot be used.
"�
contentHashB" �Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the Contact Flow Module source specified with `filename`.
"K
descriptionB" 6Specifies the description of the Contact Flow Module.
"t
filenameB" bThe path to the Contact Flow Module source within the local filesystem. Conflicts with `content`.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
";
name" /Specifies the name of the Contact Flow Module.
"�
tagsB2" �Tags to apply to the Contact Flow Module. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�L
J
connectHoursOfOperation-aws:connect/hoursOfOperation:HoursOfOperation�=Provides an Amazon Connect Hours of Operation resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.HoursOfOperation("test", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Office Hours",
    description: "Monday office hours",
    timeZone: "EST",
    configs: [
        {
            day: "MONDAY",
            endTime: {
                hours: 23,
                minutes: 8,
            },
            startTime: {
                hours: 8,
                minutes: 0,
            },
        },
        {
            day: "TUESDAY",
            endTime: {
                hours: 21,
                minutes: 0,
            },
            startTime: {
                hours: 9,
                minutes: 0,
            },
        },
    ],
    tags: {
        Name: "Example Hours of Operation",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.HoursOfOperation("test",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Office Hours",
    description="Monday office hours",
    time_zone="EST",
    configs=[
        {
            "day": "MONDAY",
            "end_time": {
                "hours": 23,
                "minutes": 8,
            },
            "start_time": {
                "hours": 8,
                "minutes": 0,
            },
        },
        {
            "day": "TUESDAY",
            "end_time": {
                "hours": 21,
                "minutes": 0,
            },
            "start_time": {
                "hours": 9,
                "minutes": 0,
            },
        },
    ],
    tags={
        "Name": "Example Hours of Operation",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.HoursOfOperation("test", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Office Hours",
        Description = "Monday office hours",
        TimeZone = "EST",
        Configs = new[]
        {
            new Aws.Connect.Inputs.HoursOfOperationConfigArgs
            {
                Day = "MONDAY",
                EndTime = new Aws.Connect.Inputs.HoursOfOperationConfigEndTimeArgs
                {
                    Hours = 23,
                    Minutes = 8,
                },
                StartTime = new Aws.Connect.Inputs.HoursOfOperationConfigStartTimeArgs
                {
                    Hours = 8,
                    Minutes = 0,
                },
            },
            new Aws.Connect.Inputs.HoursOfOperationConfigArgs
            {
                Day = "TUESDAY",
                EndTime = new Aws.Connect.Inputs.HoursOfOperationConfigEndTimeArgs
                {
                    Hours = 21,
                    Minutes = 0,
                },
                StartTime = new Aws.Connect.Inputs.HoursOfOperationConfigStartTimeArgs
                {
                    Hours = 9,
                    Minutes = 0,
                },
            },
        },
        Tags = 
        {
            { "Name", "Example Hours of Operation" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewHoursOfOperation(ctx, "test", &connect.HoursOfOperationArgs{
			InstanceId:  pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:        pulumi.String("Office Hours"),
			Description: pulumi.String("Monday office hours"),
			TimeZone:    pulumi.String("EST"),
			Configs: connect.HoursOfOperationConfigArray{
				&connect.HoursOfOperationConfigArgs{
					Day: pulumi.String("MONDAY"),
					EndTime: &connect.HoursOfOperationConfigEndTimeArgs{
						Hours:   pulumi.Int(23),
						Minutes: pulumi.Int(8),
					},
					StartTime: &connect.HoursOfOperationConfigStartTimeArgs{
						Hours:   pulumi.Int(8),
						Minutes: pulumi.Int(0),
					},
				},
				&connect.HoursOfOperationConfigArgs{
					Day: pulumi.String("TUESDAY"),
					EndTime: &connect.HoursOfOperationConfigEndTimeArgs{
						Hours:   pulumi.Int(21),
						Minutes: pulumi.Int(0),
					},
					StartTime: &connect.HoursOfOperationConfigStartTimeArgs{
						Hours:   pulumi.Int(9),
						Minutes: pulumi.Int(0),
					},
				},
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Hours of Operation"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.HoursOfOperation;
import com.pulumi.aws.connect.HoursOfOperationArgs;
import com.pulumi.aws.connect.inputs.HoursOfOperationConfigArgs;
import com.pulumi.aws.connect.inputs.HoursOfOperationConfigEndTimeArgs;
import com.pulumi.aws.connect.inputs.HoursOfOperationConfigStartTimeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new HoursOfOperation("test", HoursOfOperationArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Office Hours")
            .description("Monday office hours")
            .timeZone("EST")
            .configs(            
                HoursOfOperationConfigArgs.builder()
                    .day("MONDAY")
                    .endTime(HoursOfOperationConfigEndTimeArgs.builder()
                        .hours(23)
                        .minutes(8)
                        .build())
                    .startTime(HoursOfOperationConfigStartTimeArgs.builder()
                        .hours(8)
                        .minutes(0)
                        .build())
                    .build(),
                HoursOfOperationConfigArgs.builder()
                    .day("TUESDAY")
                    .endTime(HoursOfOperationConfigEndTimeArgs.builder()
                        .hours(21)
                        .minutes(0)
                        .build())
                    .startTime(HoursOfOperationConfigStartTimeArgs.builder()
                        .hours(9)
                        .minutes(0)
                        .build())
                    .build())
            .tags(Map.of("Name", "Example Hours of Operation"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:HoursOfOperation
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Office Hours
      description: Monday office hours
      timeZone: EST
      configs:
        - day: MONDAY
          endTime:
            hours: 23
            minutes: 8
          startTime:
            hours: 8
            minutes: 0
        - day: TUESDAY
          endTime:
            hours: 21
            minutes: 0
          startTime:
            hours: 9
            minutes: 0
      tags:
        Name: Example Hours of Operation
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Hours of Operations using the `instance_id` and `hours_of_operation_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/hoursOfOperation:HoursOfOperation example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
�
configsb*`:^
\
connectHoursOfOperationConfig9aws:connect/HoursOfOperationConfig:HoursOfOperationConfig�One or more config blocks which define the configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below.
J
descriptionB" 5Specifies the description of the Hours of Operation.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
<
nameB" .Specifies the name of the Hours of Operation.
�
tagsB2" �Tags to apply to the Hours of Operation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
C
timeZone" 3Specifies the time zone of the Hours of Operation.
"E
arn" :The Amazon Resource Name (ARN) of the Hours of Operation.
"�
configsb*`:^
\
connectHoursOfOperationConfig9aws:connect/HoursOfOperationConfig:HoursOfOperationConfig�One or more config blocks which define the configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below.
"J
descriptionB" 5Specifies the description of the Hours of Operation.
"E
hoursOfOperationId" +The identifier for the hours of operation.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
":
name" .Specifies the name of the Hours of Operation.
"�
tagsB2" �Tags to apply to the Hours of Operation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"C
timeZone" 3Specifies the time zone of the Hours of Operation.
*�`
2
connectInstanceaws:connect/instance:Instance�GProvides an Amazon Connect instance resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

!> **WARN:** Amazon Connect enforces a limit of [100 combined instance creation and deletions every 30 days](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#feature-limits). For example, if you create 80 instances and delete 20 of them, you must wait 30 days to create or delete another instance. Use care when creating or deleting instances.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.Instance("test", {
    identityManagementType: "CONNECT_MANAGED",
    inboundCallsEnabled: true,
    instanceAlias: "friendly-name-connect",
    outboundCallsEnabled: true,
    tags: {
        hello: "world",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.Instance("test",
    identity_management_type="CONNECT_MANAGED",
    inbound_calls_enabled=True,
    instance_alias="friendly-name-connect",
    outbound_calls_enabled=True,
    tags={
        "hello": "world",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.Instance("test", new()
    {
        IdentityManagementType = "CONNECT_MANAGED",
        InboundCallsEnabled = true,
        InstanceAlias = "friendly-name-connect",
        OutboundCallsEnabled = true,
        Tags = 
        {
            { "hello", "world" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstance(ctx, "test", &connect.InstanceArgs{
			IdentityManagementType: pulumi.String("CONNECT_MANAGED"),
			InboundCallsEnabled:    pulumi.Bool(true),
			InstanceAlias:          pulumi.String("friendly-name-connect"),
			OutboundCallsEnabled:   pulumi.Bool(true),
			Tags: pulumi.StringMap{
				"hello": pulumi.String("world"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.Instance;
import com.pulumi.aws.connect.InstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Instance("test", InstanceArgs.builder()
            .identityManagementType("CONNECT_MANAGED")
            .inboundCallsEnabled(true)
            .instanceAlias("friendly-name-connect")
            .outboundCallsEnabled(true)
            .tags(Map.of("hello", "world"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:Instance
    properties:
      identityManagementType: CONNECT_MANAGED
      inboundCallsEnabled: true
      instanceAlias: friendly-name-connect
      outboundCallsEnabled: true
      tags:
        hello: world
```
<!--End PulumiCodeChooser -->


### With Existing Active Directory

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.Instance("test", {
    directoryId: testAwsDirectoryServiceDirectory.id,
    identityManagementType: "EXISTING_DIRECTORY",
    inboundCallsEnabled: true,
    instanceAlias: "friendly-name-connect",
    outboundCallsEnabled: true,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.Instance("test",
    directory_id=test_aws_directory_service_directory["id"],
    identity_management_type="EXISTING_DIRECTORY",
    inbound_calls_enabled=True,
    instance_alias="friendly-name-connect",
    outbound_calls_enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.Instance("test", new()
    {
        DirectoryId = testAwsDirectoryServiceDirectory.Id,
        IdentityManagementType = "EXISTING_DIRECTORY",
        InboundCallsEnabled = true,
        InstanceAlias = "friendly-name-connect",
        OutboundCallsEnabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstance(ctx, "test", &connect.InstanceArgs{
			DirectoryId:            pulumi.Any(testAwsDirectoryServiceDirectory.Id),
			IdentityManagementType: pulumi.String("EXISTING_DIRECTORY"),
			InboundCallsEnabled:    pulumi.Bool(true),
			InstanceAlias:          pulumi.String("friendly-name-connect"),
			OutboundCallsEnabled:   pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.Instance;
import com.pulumi.aws.connect.InstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Instance("test", InstanceArgs.builder()
            .directoryId(testAwsDirectoryServiceDirectory.id())
            .identityManagementType("EXISTING_DIRECTORY")
            .inboundCallsEnabled(true)
            .instanceAlias("friendly-name-connect")
            .outboundCallsEnabled(true)
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:Instance
    properties:
      directoryId: ${testAwsDirectoryServiceDirectory.id}
      identityManagementType: EXISTING_DIRECTORY
      inboundCallsEnabled: true
      instanceAlias: friendly-name-connect
      outboundCallsEnabled: true
```
<!--End PulumiCodeChooser -->


### With SAML

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.Instance("test", {
    identityManagementType: "SAML",
    inboundCallsEnabled: true,
    instanceAlias: "friendly-name-connect",
    outboundCallsEnabled: true,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.Instance("test",
    identity_management_type="SAML",
    inbound_calls_enabled=True,
    instance_alias="friendly-name-connect",
    outbound_calls_enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.Instance("test", new()
    {
        IdentityManagementType = "SAML",
        InboundCallsEnabled = true,
        InstanceAlias = "friendly-name-connect",
        OutboundCallsEnabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstance(ctx, "test", &connect.InstanceArgs{
			IdentityManagementType: pulumi.String("SAML"),
			InboundCallsEnabled:    pulumi.Bool(true),
			InstanceAlias:          pulumi.String("friendly-name-connect"),
			OutboundCallsEnabled:   pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.Instance;
import com.pulumi.aws.connect.InstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Instance("test", InstanceArgs.builder()
            .identityManagementType("SAML")
            .inboundCallsEnabled(true)
            .instanceAlias("friendly-name-connect")
            .outboundCallsEnabled(true)
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:Instance
    properties:
      identityManagementType: SAML
      inboundCallsEnabled: true
      instanceAlias: friendly-name-connect
      outboundCallsEnabled: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Connect instances using the `id`. For example:

```sh
$ pulumi import aws:connect/instance:Instance example f1288a1f-6193-445a-b47e-af739b2
```
q
autoResolveBestVoicesEnabledB
 KSpecifies whether auto resolve best voices is enabled. Defaults to `true`.
f
contactFlowLogsEnabledB
 FSpecifies whether contact flow logs are enabled. Defaults to `false`.
[
contactLensEnabledB
 ?Specifies whether contact lens is enabled. Defaults to `true`.
k
directoryIdB" VThe identifier for the directory if identity_management_type is `EXISTING_DIRECTORY`.
�
earlyMediaEnabledB
 oSpecifies whether early media for outbound calls is enabled . Defaults to `true` if outbound calls is enabled.
�
identityManagementType" �Specifies the identity management type attached to the instance. Allowed Values are: `SAML`, `CONNECT_MANAGED`, `EXISTING_DIRECTORY`.
H
inboundCallsEnabled
 -Specifies whether inbound calls are enabled.
e
instanceAliasB" NSpecifies the name of the instance. Required if `directory_id` not specified.
u
multiPartyConferenceEnabledB
 PSpecifies whether multi-party calls/conference is enabled. Defaults to `false`.
J
outboundCallsEnabled
 .Specifies whether outbound calls are enabled.
�
tagsB2" �Tags to apply to the Instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
<!-- * `use_custom_tts_voices` - (Optional) Whether use custom tts voices is enabled. Defaults to `false` -->
"7
arn" ,Amazon Resource Name (ARN) of the instance.
"q
autoResolveBestVoicesEnabledB
 KSpecifies whether auto resolve best voices is enabled. Defaults to `true`.
"f
contactFlowLogsEnabledB
 FSpecifies whether contact flow logs are enabled. Defaults to `false`.
"[
contactLensEnabledB
 ?Specifies whether contact lens is enabled. Defaults to `true`.
"2
createdTime" When the instance was created.
"k
directoryIdB" VThe identifier for the directory if identity_management_type is `EXISTING_DIRECTORY`.
"�
earlyMediaEnabledB
 oSpecifies whether early media for outbound calls is enabled . Defaults to `true` if outbound calls is enabled.
"�
identityManagementType" �Specifies the identity management type attached to the instance. Allowed Values are: `SAML`, `CONNECT_MANAGED`, `EXISTING_DIRECTORY`.
"H
inboundCallsEnabled
 -Specifies whether inbound calls are enabled.
"e
instanceAliasB" NSpecifies the name of the instance. Required if `directory_id` not specified.
"u
multiPartyConferenceEnabledB
 PSpecifies whether multi-party calls/conference is enabled. Defaults to `false`.
"J
outboundCallsEnabled
 .Specifies whether outbound calls are enabled.
"5
serviceRole" "The service role of the instance.
")
status" The state of the instance.
"�
tagsB2" �Tags to apply to the Instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
<!-- * `use_custom_tts_voices` - (Optional) Whether use custom tts voices is enabled. Defaults to `false` -->
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
Y
connectInstanceStorageConfig7aws:connect/instanceStorageConfig:InstanceStorageConfig��Provides an Amazon Connect Instance Storage Config resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

### Storage Config Kinesis Firehose Config

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.InstanceStorageConfig("example", {
    instanceId: exampleAwsConnectInstance.id,
    resourceType: "CONTACT_TRACE_RECORDS",
    storageConfig: {
        kinesisFirehoseConfig: {
            firehoseArn: exampleAwsKinesisFirehoseDeliveryStream.arn,
        },
        storageType: "KINESIS_FIREHOSE",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.InstanceStorageConfig("example",
    instance_id=example_aws_connect_instance["id"],
    resource_type="CONTACT_TRACE_RECORDS",
    storage_config={
        "kinesis_firehose_config": {
            "firehose_arn": example_aws_kinesis_firehose_delivery_stream["arn"],
        },
        "storage_type": "KINESIS_FIREHOSE",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.InstanceStorageConfig("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        ResourceType = "CONTACT_TRACE_RECORDS",
        StorageConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigArgs
        {
            KinesisFirehoseConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigKinesisFirehoseConfigArgs
            {
                FirehoseArn = exampleAwsKinesisFirehoseDeliveryStream.Arn,
            },
            StorageType = "KINESIS_FIREHOSE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstanceStorageConfig(ctx, "example", &connect.InstanceStorageConfigArgs{
			InstanceId:   pulumi.Any(exampleAwsConnectInstance.Id),
			ResourceType: pulumi.String("CONTACT_TRACE_RECORDS"),
			StorageConfig: &connect.InstanceStorageConfigStorageConfigArgs{
				KinesisFirehoseConfig: &connect.InstanceStorageConfigStorageConfigKinesisFirehoseConfigArgs{
					FirehoseArn: pulumi.Any(exampleAwsKinesisFirehoseDeliveryStream.Arn),
				},
				StorageType: pulumi.String("KINESIS_FIREHOSE"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.InstanceStorageConfig;
import com.pulumi.aws.connect.InstanceStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigKinesisFirehoseConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceStorageConfig("example", InstanceStorageConfigArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .resourceType("CONTACT_TRACE_RECORDS")
            .storageConfig(InstanceStorageConfigStorageConfigArgs.builder()
                .kinesisFirehoseConfig(InstanceStorageConfigStorageConfigKinesisFirehoseConfigArgs.builder()
                    .firehoseArn(exampleAwsKinesisFirehoseDeliveryStream.arn())
                    .build())
                .storageType("KINESIS_FIREHOSE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:InstanceStorageConfig
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      resourceType: CONTACT_TRACE_RECORDS
      storageConfig:
        kinesisFirehoseConfig:
          firehoseArn: ${exampleAwsKinesisFirehoseDeliveryStream.arn}
        storageType: KINESIS_FIREHOSE
```
<!--End PulumiCodeChooser -->

### Storage Config Kinesis Stream Config

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.InstanceStorageConfig("example", {
    instanceId: exampleAwsConnectInstance.id,
    resourceType: "CONTACT_TRACE_RECORDS",
    storageConfig: {
        kinesisStreamConfig: {
            streamArn: exampleAwsKinesisStream.arn,
        },
        storageType: "KINESIS_STREAM",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.InstanceStorageConfig("example",
    instance_id=example_aws_connect_instance["id"],
    resource_type="CONTACT_TRACE_RECORDS",
    storage_config={
        "kinesis_stream_config": {
            "stream_arn": example_aws_kinesis_stream["arn"],
        },
        "storage_type": "KINESIS_STREAM",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.InstanceStorageConfig("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        ResourceType = "CONTACT_TRACE_RECORDS",
        StorageConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigArgs
        {
            KinesisStreamConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigKinesisStreamConfigArgs
            {
                StreamArn = exampleAwsKinesisStream.Arn,
            },
            StorageType = "KINESIS_STREAM",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstanceStorageConfig(ctx, "example", &connect.InstanceStorageConfigArgs{
			InstanceId:   pulumi.Any(exampleAwsConnectInstance.Id),
			ResourceType: pulumi.String("CONTACT_TRACE_RECORDS"),
			StorageConfig: &connect.InstanceStorageConfigStorageConfigArgs{
				KinesisStreamConfig: &connect.InstanceStorageConfigStorageConfigKinesisStreamConfigArgs{
					StreamArn: pulumi.Any(exampleAwsKinesisStream.Arn),
				},
				StorageType: pulumi.String("KINESIS_STREAM"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.InstanceStorageConfig;
import com.pulumi.aws.connect.InstanceStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigKinesisStreamConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceStorageConfig("example", InstanceStorageConfigArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .resourceType("CONTACT_TRACE_RECORDS")
            .storageConfig(InstanceStorageConfigStorageConfigArgs.builder()
                .kinesisStreamConfig(InstanceStorageConfigStorageConfigKinesisStreamConfigArgs.builder()
                    .streamArn(exampleAwsKinesisStream.arn())
                    .build())
                .storageType("KINESIS_STREAM")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:InstanceStorageConfig
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      resourceType: CONTACT_TRACE_RECORDS
      storageConfig:
        kinesisStreamConfig:
          streamArn: ${exampleAwsKinesisStream.arn}
        storageType: KINESIS_STREAM
```
<!--End PulumiCodeChooser -->

### Storage Config Kinesis Video Stream Config

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.InstanceStorageConfig("example", {
    instanceId: exampleAwsConnectInstance.id,
    resourceType: "MEDIA_STREAMS",
    storageConfig: {
        kinesisVideoStreamConfig: {
            prefix: "example",
            retentionPeriodHours: 3,
            encryptionConfig: {
                encryptionType: "KMS",
                keyId: exampleAwsKmsKey.arn,
            },
        },
        storageType: "KINESIS_VIDEO_STREAM",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.InstanceStorageConfig("example",
    instance_id=example_aws_connect_instance["id"],
    resource_type="MEDIA_STREAMS",
    storage_config={
        "kinesis_video_stream_config": {
            "prefix": "example",
            "retention_period_hours": 3,
            "encryption_config": {
                "encryption_type": "KMS",
                "key_id": example_aws_kms_key["arn"],
            },
        },
        "storage_type": "KINESIS_VIDEO_STREAM",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.InstanceStorageConfig("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        ResourceType = "MEDIA_STREAMS",
        StorageConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigArgs
        {
            KinesisVideoStreamConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigKinesisVideoStreamConfigArgs
            {
                Prefix = "example",
                RetentionPeriodHours = 3,
                EncryptionConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfigArgs
                {
                    EncryptionType = "KMS",
                    KeyId = exampleAwsKmsKey.Arn,
                },
            },
            StorageType = "KINESIS_VIDEO_STREAM",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstanceStorageConfig(ctx, "example", &connect.InstanceStorageConfigArgs{
			InstanceId:   pulumi.Any(exampleAwsConnectInstance.Id),
			ResourceType: pulumi.String("MEDIA_STREAMS"),
			StorageConfig: &connect.InstanceStorageConfigStorageConfigArgs{
				KinesisVideoStreamConfig: &connect.InstanceStorageConfigStorageConfigKinesisVideoStreamConfigArgs{
					Prefix:               pulumi.String("example"),
					RetentionPeriodHours: pulumi.Int(3),
					EncryptionConfig: &connect.InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfigArgs{
						EncryptionType: pulumi.String("KMS"),
						KeyId:          pulumi.Any(exampleAwsKmsKey.Arn),
					},
				},
				StorageType: pulumi.String("KINESIS_VIDEO_STREAM"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.InstanceStorageConfig;
import com.pulumi.aws.connect.InstanceStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigKinesisVideoStreamConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceStorageConfig("example", InstanceStorageConfigArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .resourceType("MEDIA_STREAMS")
            .storageConfig(InstanceStorageConfigStorageConfigArgs.builder()
                .kinesisVideoStreamConfig(InstanceStorageConfigStorageConfigKinesisVideoStreamConfigArgs.builder()
                    .prefix("example")
                    .retentionPeriodHours(3)
                    .encryptionConfig(InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfigArgs.builder()
                        .encryptionType("KMS")
                        .keyId(exampleAwsKmsKey.arn())
                        .build())
                    .build())
                .storageType("KINESIS_VIDEO_STREAM")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:InstanceStorageConfig
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      resourceType: MEDIA_STREAMS
      storageConfig:
        kinesisVideoStreamConfig:
          prefix: example
          retentionPeriodHours: 3
          encryptionConfig:
            encryptionType: KMS
            keyId: ${exampleAwsKmsKey.arn}
        storageType: KINESIS_VIDEO_STREAM
```
<!--End PulumiCodeChooser -->

### Storage Config S3 Config

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.InstanceStorageConfig("example", {
    instanceId: exampleAwsConnectInstance.id,
    resourceType: "CHAT_TRANSCRIPTS",
    storageConfig: {
        s3Config: {
            bucketName: exampleAwsS3Bucket.id,
            bucketPrefix: "example",
        },
        storageType: "S3",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.InstanceStorageConfig("example",
    instance_id=example_aws_connect_instance["id"],
    resource_type="CHAT_TRANSCRIPTS",
    storage_config={
        "s3_config": {
            "bucket_name": example_aws_s3_bucket["id"],
            "bucket_prefix": "example",
        },
        "storage_type": "S3",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.InstanceStorageConfig("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        ResourceType = "CHAT_TRANSCRIPTS",
        StorageConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigArgs
        {
            S3Config = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigS3ConfigArgs
            {
                BucketName = exampleAwsS3Bucket.Id,
                BucketPrefix = "example",
            },
            StorageType = "S3",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstanceStorageConfig(ctx, "example", &connect.InstanceStorageConfigArgs{
			InstanceId:   pulumi.Any(exampleAwsConnectInstance.Id),
			ResourceType: pulumi.String("CHAT_TRANSCRIPTS"),
			StorageConfig: &connect.InstanceStorageConfigStorageConfigArgs{
				S3Config: &connect.InstanceStorageConfigStorageConfigS3ConfigArgs{
					BucketName:   pulumi.Any(exampleAwsS3Bucket.Id),
					BucketPrefix: pulumi.String("example"),
				},
				StorageType: pulumi.String("S3"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.InstanceStorageConfig;
import com.pulumi.aws.connect.InstanceStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigS3ConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceStorageConfig("example", InstanceStorageConfigArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .resourceType("CHAT_TRANSCRIPTS")
            .storageConfig(InstanceStorageConfigStorageConfigArgs.builder()
                .s3Config(InstanceStorageConfigStorageConfigS3ConfigArgs.builder()
                    .bucketName(exampleAwsS3Bucket.id())
                    .bucketPrefix("example")
                    .build())
                .storageType("S3")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:InstanceStorageConfig
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      resourceType: CHAT_TRANSCRIPTS
      storageConfig:
        s3Config:
          bucketName: ${exampleAwsS3Bucket.id}
          bucketPrefix: example
        storageType: S3
```
<!--End PulumiCodeChooser -->

### Storage Config S3 Config with Encryption Config

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.InstanceStorageConfig("example", {
    instanceId: exampleAwsConnectInstance.id,
    resourceType: "CHAT_TRANSCRIPTS",
    storageConfig: {
        s3Config: {
            bucketName: exampleAwsS3Bucket.id,
            bucketPrefix: "example",
            encryptionConfig: {
                encryptionType: "KMS",
                keyId: exampleAwsKmsKey.arn,
            },
        },
        storageType: "S3",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.InstanceStorageConfig("example",
    instance_id=example_aws_connect_instance["id"],
    resource_type="CHAT_TRANSCRIPTS",
    storage_config={
        "s3_config": {
            "bucket_name": example_aws_s3_bucket["id"],
            "bucket_prefix": "example",
            "encryption_config": {
                "encryption_type": "KMS",
                "key_id": example_aws_kms_key["arn"],
            },
        },
        "storage_type": "S3",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.InstanceStorageConfig("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        ResourceType = "CHAT_TRANSCRIPTS",
        StorageConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigArgs
        {
            S3Config = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigS3ConfigArgs
            {
                BucketName = exampleAwsS3Bucket.Id,
                BucketPrefix = "example",
                EncryptionConfig = new Aws.Connect.Inputs.InstanceStorageConfigStorageConfigS3ConfigEncryptionConfigArgs
                {
                    EncryptionType = "KMS",
                    KeyId = exampleAwsKmsKey.Arn,
                },
            },
            StorageType = "S3",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewInstanceStorageConfig(ctx, "example", &connect.InstanceStorageConfigArgs{
			InstanceId:   pulumi.Any(exampleAwsConnectInstance.Id),
			ResourceType: pulumi.String("CHAT_TRANSCRIPTS"),
			StorageConfig: &connect.InstanceStorageConfigStorageConfigArgs{
				S3Config: &connect.InstanceStorageConfigStorageConfigS3ConfigArgs{
					BucketName:   pulumi.Any(exampleAwsS3Bucket.Id),
					BucketPrefix: pulumi.String("example"),
					EncryptionConfig: &connect.InstanceStorageConfigStorageConfigS3ConfigEncryptionConfigArgs{
						EncryptionType: pulumi.String("KMS"),
						KeyId:          pulumi.Any(exampleAwsKmsKey.Arn),
					},
				},
				StorageType: pulumi.String("S3"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.InstanceStorageConfig;
import com.pulumi.aws.connect.InstanceStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigS3ConfigArgs;
import com.pulumi.aws.connect.inputs.InstanceStorageConfigStorageConfigS3ConfigEncryptionConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceStorageConfig("example", InstanceStorageConfigArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .resourceType("CHAT_TRANSCRIPTS")
            .storageConfig(InstanceStorageConfigStorageConfigArgs.builder()
                .s3Config(InstanceStorageConfigStorageConfigS3ConfigArgs.builder()
                    .bucketName(exampleAwsS3Bucket.id())
                    .bucketPrefix("example")
                    .encryptionConfig(InstanceStorageConfigStorageConfigS3ConfigEncryptionConfigArgs.builder()
                        .encryptionType("KMS")
                        .keyId(exampleAwsKmsKey.arn())
                        .build())
                    .build())
                .storageType("S3")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:InstanceStorageConfig
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      resourceType: CHAT_TRANSCRIPTS
      storageConfig:
        s3Config:
          bucketName: ${exampleAwsS3Bucket.id}
          bucketPrefix: example
          encryptionConfig:
            encryptionType: KMS
            keyId: ${exampleAwsKmsKey.arn}
        storageType: S3
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Instance Storage Configs using the `instance_id`, `association_id`, and `resource_type` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/instanceStorageConfig:InstanceStorageConfig example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5:CHAT_TRANSCRIPTS
```
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
�
resourceType" �A valid resource type. Valid Values: `AGENT_EVENTS` | `ATTACHMENTS` | `CALL_RECORDINGS` | `CHAT_TRANSCRIPTS` | `CONTACT_EVALUATIONS` | `CONTACT_TRACE_RECORDS` | `MEDIA_STREAMS` | `REAL_TIME_CONTACT_ANALYSIS_SEGMENTS` | `SCHEDULED_REPORTS` | `SCREEN_RECORDINGS`.
�
storageConfig�:�
�
connect"InstanceStorageConfigStorageConfigQaws:connect/InstanceStorageConfigStorageConfig:InstanceStorageConfigStorageConfigXSpecifies the storage configuration options for the Connect Instance. Documented below.
"�
associationId" }The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"�
resourceType" �A valid resource type. Valid Values: `AGENT_EVENTS` | `ATTACHMENTS` | `CALL_RECORDINGS` | `CHAT_TRANSCRIPTS` | `CONTACT_EVALUATIONS` | `CONTACT_TRACE_RECORDS` | `MEDIA_STREAMS` | `REAL_TIME_CONTACT_ANALYSIS_SEGMENTS` | `SCHEDULED_REPORTS` | `SCREEN_RECORDINGS`.
"�
storageConfig�:�
�
connect"InstanceStorageConfigStorageConfigQaws:connect/InstanceStorageConfigStorageConfig:InstanceStorageConfigStorageConfigXSpecifies the storage configuration options for the Connect Instance. Documented below.
*�
e
connectLambdaFunctionAssociation?aws:connect/lambdaFunctionAssociation:LambdaFunctionAssociation�Provides an Amazon Connect Lambda Function Association. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html) and [Invoke AWS Lambda functions](https://docs.aws.amazon.com/connect/latest/adminguide/connect-lambda-functions.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.LambdaFunctionAssociation("example", {
    functionArn: exampleAwsLambdaFunction.arn,
    instanceId: exampleAwsConnectInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.LambdaFunctionAssociation("example",
    function_arn=example_aws_lambda_function["arn"],
    instance_id=example_aws_connect_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.LambdaFunctionAssociation("example", new()
    {
        FunctionArn = exampleAwsLambdaFunction.Arn,
        InstanceId = exampleAwsConnectInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewLambdaFunctionAssociation(ctx, "example", &connect.LambdaFunctionAssociationArgs{
			FunctionArn: pulumi.Any(exampleAwsLambdaFunction.Arn),
			InstanceId:  pulumi.Any(exampleAwsConnectInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.LambdaFunctionAssociation;
import com.pulumi.aws.connect.LambdaFunctionAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new LambdaFunctionAssociation("example", LambdaFunctionAssociationArgs.builder()
            .functionArn(exampleAwsLambdaFunction.arn())
            .instanceId(exampleAwsConnectInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:LambdaFunctionAssociation
    properties:
      functionArn: ${exampleAwsLambdaFunction.arn}
      instanceId: ${exampleAwsConnectInstance.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_connect_lambda_function_association` using the `instance_id` and `function_arn` separated by a comma (`,`). For example:

```sh
$ pulumi import aws:connect/lambdaFunctionAssociation:LambdaFunctionAssociation example aaaaaaaa-bbbb-cccc-dddd-111111111111,arn:aws:lambda:us-west-2:123456789123:function:example
```
o
functionArn" \Amazon Resource Name (ARN) of the Lambda Function, omitting any version or alias qualifier.
y

instanceId" gThe identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
"o
functionArn" \Amazon Resource Name (ARN) of the Lambda Function, omitting any version or alias qualifier.
"y

instanceId" gThe identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
*�P
;
connectPhoneNumber#aws:connect/phoneNumber:PhoneNumber�=Provides an Amazon Connect Phone Number resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.PhoneNumber("example", {
    targetArn: exampleAwsConnectInstance.arn,
    countryCode: "US",
    type: "DID",
    tags: {
        hello: "world",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.PhoneNumber("example",
    target_arn=example_aws_connect_instance["arn"],
    country_code="US",
    type="DID",
    tags={
        "hello": "world",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.PhoneNumber("example", new()
    {
        TargetArn = exampleAwsConnectInstance.Arn,
        CountryCode = "US",
        Type = "DID",
        Tags = 
        {
            { "hello", "world" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewPhoneNumber(ctx, "example", &connect.PhoneNumberArgs{
			TargetArn:   pulumi.Any(exampleAwsConnectInstance.Arn),
			CountryCode: pulumi.String("US"),
			Type:        pulumi.String("DID"),
			Tags: pulumi.StringMap{
				"hello": pulumi.String("world"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.PhoneNumber;
import com.pulumi.aws.connect.PhoneNumberArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PhoneNumber("example", PhoneNumberArgs.builder()
            .targetArn(exampleAwsConnectInstance.arn())
            .countryCode("US")
            .type("DID")
            .tags(Map.of("hello", "world"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:PhoneNumber
    properties:
      targetArn: ${exampleAwsConnectInstance.arn}
      countryCode: US
      type: DID
      tags:
        hello: world
```
<!--End PulumiCodeChooser -->

### Description

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.PhoneNumber("example", {
    targetArn: exampleAwsConnectInstance.arn,
    countryCode: "US",
    type: "DID",
    description: "example description",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.PhoneNumber("example",
    target_arn=example_aws_connect_instance["arn"],
    country_code="US",
    type="DID",
    description="example description")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.PhoneNumber("example", new()
    {
        TargetArn = exampleAwsConnectInstance.Arn,
        CountryCode = "US",
        Type = "DID",
        Description = "example description",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewPhoneNumber(ctx, "example", &connect.PhoneNumberArgs{
			TargetArn:   pulumi.Any(exampleAwsConnectInstance.Arn),
			CountryCode: pulumi.String("US"),
			Type:        pulumi.String("DID"),
			Description: pulumi.String("example description"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.PhoneNumber;
import com.pulumi.aws.connect.PhoneNumberArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PhoneNumber("example", PhoneNumberArgs.builder()
            .targetArn(exampleAwsConnectInstance.arn())
            .countryCode("US")
            .type("DID")
            .description("example description")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:PhoneNumber
    properties:
      targetArn: ${exampleAwsConnectInstance.arn}
      countryCode: US
      type: DID
      description: example description
```
<!--End PulumiCodeChooser -->

### Prefix to filter phone numbers

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.PhoneNumber("example", {
    targetArn: exampleAwsConnectInstance.arn,
    countryCode: "US",
    type: "DID",
    prefix: "+18005",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.PhoneNumber("example",
    target_arn=example_aws_connect_instance["arn"],
    country_code="US",
    type="DID",
    prefix="+18005")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.PhoneNumber("example", new()
    {
        TargetArn = exampleAwsConnectInstance.Arn,
        CountryCode = "US",
        Type = "DID",
        Prefix = "+18005",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewPhoneNumber(ctx, "example", &connect.PhoneNumberArgs{
			TargetArn:   pulumi.Any(exampleAwsConnectInstance.Arn),
			CountryCode: pulumi.String("US"),
			Type:        pulumi.String("DID"),
			Prefix:      pulumi.String("+18005"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.PhoneNumber;
import com.pulumi.aws.connect.PhoneNumberArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PhoneNumber("example", PhoneNumberArgs.builder()
            .targetArn(exampleAwsConnectInstance.arn())
            .countryCode("US")
            .type("DID")
            .prefix("+18005")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:PhoneNumber
    properties:
      targetArn: ${exampleAwsConnectInstance.arn}
      countryCode: US
      type: DID
      prefix: '+18005'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Phone Numbers using its `id`. For example:

```sh
$ pulumi import aws:connect/phoneNumber:PhoneNumber example 12345678-abcd-1234-efgh-9876543210ab
```
�
countryCode" �The ISO country code. For a list of Valid values, refer to [PhoneNumberCountryCode](https://docs.aws.amazon.com/connect/latest/APIReference/API_SearchAvailablePhoneNumbers.html#connect-SearchAvailablePhoneNumbers-request-PhoneNumberCountryCode).
:
descriptionB" %The description of the phone number.
�
prefixB" �The prefix of the phone number that is used to filter available phone numbers. If provided, it must contain `+` as part of the country code. Do not specify this argument when importing the resource.
�
tagsB2" �Tags to apply to the Phone Number. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
p
	targetArn" _The Amazon Resource Name (ARN) for Amazon Connect instances that phone numbers are claimed to.
I
type" =The type of phone number. Valid Values: `TOLL_FREE` | `DID`.
"(
arn" The ARN of the phone number.
"�
countryCode" �The ISO country code. For a list of Valid values, refer to [PhoneNumberCountryCode](https://docs.aws.amazon.com/connect/latest/APIReference/API_SearchAvailablePhoneNumbers.html#connect-SearchAvailablePhoneNumbers-request-PhoneNumberCountryCode).
":
descriptionB" %The description of the phone number.
"
phoneNumber" lThe phone number. Phone numbers are formatted `[+] [country code] [subscriber number including area code]`.
"�
prefixB" �The prefix of the phone number that is used to filter available phone numbers. If provided, it must contain `+` as part of the country code. Do not specify this argument when importing the resource.
"�
statusesS*Q:O
M
connectPhoneNumberStatus/aws:connect/PhoneNumberStatus:PhoneNumberStatusTThe status of the phone number. Valid Values: `CLAIMED` | `IN_PROGRESS` | `FAILED`.
"�
tagsB2" �Tags to apply to the Phone Number. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"p
	targetArn" _The Amazon Resource Name (ARN) for Amazon Connect instances that phone numbers are claimed to.
"I
type" =The type of phone number. Valid Values: `TOLL_FREE` | `DID`.
*�s
)
connectQueueaws:connect/queue:Queue�_Provides an Amazon Connect Queue resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.Queue("test", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example Name",
    description: "Example Description",
    hoursOfOperationId: "12345678-1234-1234-1234-123456789012",
    tags: {
        Name: "Example Queue",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.Queue("test",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example Name",
    description="Example Description",
    hours_of_operation_id="12345678-1234-1234-1234-123456789012",
    tags={
        "Name": "Example Queue",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.Queue("test", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example Name",
        Description = "Example Description",
        HoursOfOperationId = "12345678-1234-1234-1234-123456789012",
        Tags = 
        {
            { "Name", "Example Queue" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewQueue(ctx, "test", &connect.QueueArgs{
			InstanceId:         pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:               pulumi.String("Example Name"),
			Description:        pulumi.String("Example Description"),
			HoursOfOperationId: pulumi.String("12345678-1234-1234-1234-123456789012"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Queue"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.Queue;
import com.pulumi.aws.connect.QueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Queue("test", QueueArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example Name")
            .description("Example Description")
            .hoursOfOperationId("12345678-1234-1234-1234-123456789012")
            .tags(Map.of("Name", "Example Queue"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:Queue
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Example Name
      description: Example Description
      hoursOfOperationId: 12345678-1234-1234-1234-123456789012
      tags:
        Name: Example Queue
```
<!--End PulumiCodeChooser -->

### With Quick Connect IDs

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.Queue("test", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example Name",
    description: "Example Description",
    hoursOfOperationId: "12345678-1234-1234-1234-123456789012",
    quickConnectIds: ["12345678-abcd-1234-abcd-123456789012"],
    tags: {
        Name: "Example Queue with Quick Connect IDs",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.Queue("test",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example Name",
    description="Example Description",
    hours_of_operation_id="12345678-1234-1234-1234-123456789012",
    quick_connect_ids=["12345678-abcd-1234-abcd-123456789012"],
    tags={
        "Name": "Example Queue with Quick Connect IDs",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.Queue("test", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example Name",
        Description = "Example Description",
        HoursOfOperationId = "12345678-1234-1234-1234-123456789012",
        QuickConnectIds = new[]
        {
            "12345678-abcd-1234-abcd-123456789012",
        },
        Tags = 
        {
            { "Name", "Example Queue with Quick Connect IDs" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewQueue(ctx, "test", &connect.QueueArgs{
			InstanceId:         pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:               pulumi.String("Example Name"),
			Description:        pulumi.String("Example Description"),
			HoursOfOperationId: pulumi.String("12345678-1234-1234-1234-123456789012"),
			QuickConnectIds: pulumi.StringArray{
				pulumi.String("12345678-abcd-1234-abcd-123456789012"),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Queue with Quick Connect IDs"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.Queue;
import com.pulumi.aws.connect.QueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Queue("test", QueueArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example Name")
            .description("Example Description")
            .hoursOfOperationId("12345678-1234-1234-1234-123456789012")
            .quickConnectIds("12345678-abcd-1234-abcd-123456789012")
            .tags(Map.of("Name", "Example Queue with Quick Connect IDs"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:Queue
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Example Name
      description: Example Description
      hoursOfOperationId: 12345678-1234-1234-1234-123456789012
      quickConnectIds:
        - 12345678-abcd-1234-abcd-123456789012
      tags:
        Name: Example Queue with Quick Connect IDs
```
<!--End PulumiCodeChooser -->

### With Outbound Caller Config

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.Queue("test", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example Name",
    description: "Example Description",
    hoursOfOperationId: "12345678-1234-1234-1234-123456789012",
    outboundCallerConfig: {
        outboundCallerIdName: "example",
        outboundCallerIdNumberId: "12345678-abcd-1234-abcd-123456789012",
        outboundFlowId: "87654321-defg-1234-defg-987654321234",
    },
    tags: {
        Name: "Example Queue with Outbound Caller Config",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.Queue("test",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example Name",
    description="Example Description",
    hours_of_operation_id="12345678-1234-1234-1234-123456789012",
    outbound_caller_config={
        "outbound_caller_id_name": "example",
        "outbound_caller_id_number_id": "12345678-abcd-1234-abcd-123456789012",
        "outbound_flow_id": "87654321-defg-1234-defg-987654321234",
    },
    tags={
        "Name": "Example Queue with Outbound Caller Config",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.Queue("test", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example Name",
        Description = "Example Description",
        HoursOfOperationId = "12345678-1234-1234-1234-123456789012",
        OutboundCallerConfig = new Aws.Connect.Inputs.QueueOutboundCallerConfigArgs
        {
            OutboundCallerIdName = "example",
            OutboundCallerIdNumberId = "12345678-abcd-1234-abcd-123456789012",
            OutboundFlowId = "87654321-defg-1234-defg-987654321234",
        },
        Tags = 
        {
            { "Name", "Example Queue with Outbound Caller Config" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewQueue(ctx, "test", &connect.QueueArgs{
			InstanceId:         pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:               pulumi.String("Example Name"),
			Description:        pulumi.String("Example Description"),
			HoursOfOperationId: pulumi.String("12345678-1234-1234-1234-123456789012"),
			OutboundCallerConfig: &connect.QueueOutboundCallerConfigArgs{
				OutboundCallerIdName:     pulumi.String("example"),
				OutboundCallerIdNumberId: pulumi.String("12345678-abcd-1234-abcd-123456789012"),
				OutboundFlowId:           pulumi.String("87654321-defg-1234-defg-987654321234"),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Queue with Outbound Caller Config"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.Queue;
import com.pulumi.aws.connect.QueueArgs;
import com.pulumi.aws.connect.inputs.QueueOutboundCallerConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Queue("test", QueueArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example Name")
            .description("Example Description")
            .hoursOfOperationId("12345678-1234-1234-1234-123456789012")
            .outboundCallerConfig(QueueOutboundCallerConfigArgs.builder()
                .outboundCallerIdName("example")
                .outboundCallerIdNumberId("12345678-abcd-1234-abcd-123456789012")
                .outboundFlowId("87654321-defg-1234-defg-987654321234")
                .build())
            .tags(Map.of("Name", "Example Queue with Outbound Caller Config"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:Queue
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Example Name
      description: Example Description
      hoursOfOperationId: 12345678-1234-1234-1234-123456789012
      outboundCallerConfig:
        outboundCallerIdName: example
        outboundCallerIdNumberId: 12345678-abcd-1234-abcd-123456789012
        outboundFlowId: 87654321-defg-1234-defg-987654321234
      tags:
        Name: Example Queue with Outbound Caller Config
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Queues using the `instance_id` and `queue_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/queue:Queue example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
=
descriptionB" (Specifies the description of the Queue.
N
hoursOfOperationId" 4Specifies the identifier of the Hours of Operation.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
�
maxContactsB tSpecifies the maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
/
nameB" !Specifies the name of the Queue.
�
outboundCallerConfigkBi:g
e
connectQueueOutboundCallerConfig?aws:connect/QueueOutboundCallerConfig:QueueOutboundCallerConfig�A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
�
quickConnectIdsB*" xSpecifies a list of quick connects ids that determine the quick connects available to agents who are working the queue.
`
statusB" PSpecifies the description of the Queue. Valid values are `ENABLED`, `DISABLED`.
�
tagsB2" �Tags to apply to the Queue. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"8
arn" -The Amazon Resource Name (ARN) of the Queue.
"=
descriptionB" (Specifies the description of the Queue.
"N
hoursOfOperationId" 4Specifies the identifier of the Hours of Operation.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"�
maxContactsB tSpecifies the maximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
"-
name" !Specifies the name of the Queue.
"�
outboundCallerConfigkBi:g
e
connectQueueOutboundCallerConfig?aws:connect/QueueOutboundCallerConfig:QueueOutboundCallerConfig�A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
"-
queueId" The identifier for the Queue.
"�
quickConnectIdsB*" xSpecifies a list of quick connects ids that determine the quick connects available to agents who are working the queue.
"^
status" PSpecifies the description of the Queue. Valid values are `ENABLED`, `DISABLED`.
"�
tagsB2" �Tags to apply to the Queue. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�4
>
connectQuickConnect%aws:connect/quickConnect:QuickConnect�&Provides an Amazon Connect Quick Connect resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.connect.QuickConnect("test", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example Name",
    description: "quick connect phone number",
    quickConnectConfig: {
        quickConnectType: "PHONE_NUMBER",
        phoneConfigs: [{
            phoneNumber: "+12345678912",
        }],
    },
    tags: {
        Name: "Example Quick Connect",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.QuickConnect("test",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example Name",
    description="quick connect phone number",
    quick_connect_config={
        "quick_connect_type": "PHONE_NUMBER",
        "phone_configs": [{
            "phone_number": "+12345678912",
        }],
    },
    tags={
        "Name": "Example Quick Connect",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Connect.QuickConnect("test", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example Name",
        Description = "quick connect phone number",
        QuickConnectConfig = new Aws.Connect.Inputs.QuickConnectQuickConnectConfigArgs
        {
            QuickConnectType = "PHONE_NUMBER",
            PhoneConfigs = new[]
            {
                new Aws.Connect.Inputs.QuickConnectQuickConnectConfigPhoneConfigArgs
                {
                    PhoneNumber = "+12345678912",
                },
            },
        },
        Tags = 
        {
            { "Name", "Example Quick Connect" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewQuickConnect(ctx, "test", &connect.QuickConnectArgs{
			InstanceId:  pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:        pulumi.String("Example Name"),
			Description: pulumi.String("quick connect phone number"),
			QuickConnectConfig: &connect.QuickConnectQuickConnectConfigArgs{
				QuickConnectType: pulumi.String("PHONE_NUMBER"),
				PhoneConfigs: connect.QuickConnectQuickConnectConfigPhoneConfigArray{
					&connect.QuickConnectQuickConnectConfigPhoneConfigArgs{
						PhoneNumber: pulumi.String("+12345678912"),
					},
				},
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Quick Connect"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.QuickConnect;
import com.pulumi.aws.connect.QuickConnectArgs;
import com.pulumi.aws.connect.inputs.QuickConnectQuickConnectConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new QuickConnect("test", QuickConnectArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example Name")
            .description("quick connect phone number")
            .quickConnectConfig(QuickConnectQuickConnectConfigArgs.builder()
                .quickConnectType("PHONE_NUMBER")
                .phoneConfigs(QuickConnectQuickConnectConfigPhoneConfigArgs.builder()
                    .phoneNumber("+12345678912")
                    .build())
                .build())
            .tags(Map.of("Name", "Example Quick Connect"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:connect:QuickConnect
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: Example Name
      description: quick connect phone number
      quickConnectConfig:
        quickConnectType: PHONE_NUMBER
        phoneConfigs:
          - phoneNumber: '+12345678912'
      tags:
        Name: Example Quick Connect
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Quick Connects using the `instance_id` and `quick_connect_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/quickConnect:QuickConnect example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
E
descriptionB" 0Specifies the description of the Quick Connect.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
7
nameB" )Specifies the name of the Quick Connect.
�
quickConnectConfigx:v
t
connectQuickConnectQuickConnectConfigIaws:connect/QuickConnectQuickConnectConfig:QuickConnectQuickConnectConfig�A block that defines the configuration information for the Quick Connect: `quick_connect_type` and one of `phone_config`, `queue_config`, `user_config` . The Quick Connect Config block is documented below.
�
tagsB2" �Tags to apply to the Quick Connect. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"@
arn" 5The Amazon Resource Name (ARN) of the Quick Connect.
"E
descriptionB" 0Specifies the description of the Quick Connect.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"5
name" )Specifies the name of the Quick Connect.
"�
quickConnectConfigx:v
t
connectQuickConnectQuickConnectConfigIaws:connect/QuickConnectQuickConnectConfig:QuickConnectQuickConnectConfig�A block that defines the configuration information for the Quick Connect: `quick_connect_type` and one of `phone_config`, `queue_config`, `user_config` . The Quick Connect Config block is documented below.
"<
quickConnectId" &The identifier for the Quick Connect.
"�
tagsB2" �Tags to apply to the Quick Connect. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�E
D
connectRoutingProfile)aws:connect/routingProfile:RoutingProfile�/Provides an Amazon Connect Routing Profile resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.RoutingProfile("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "example",
    defaultOutboundQueueId: "12345678-1234-1234-1234-123456789012",
    description: "example description",
    mediaConcurrencies: [{
        channel: "VOICE",
        concurrency: 1,
    }],
    queueConfigs: [{
        channel: "VOICE",
        delay: 2,
        priority: 1,
        queueId: "12345678-1234-1234-1234-123456789012",
    }],
    tags: {
        Name: "Example Routing Profile",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.RoutingProfile("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="example",
    default_outbound_queue_id="12345678-1234-1234-1234-123456789012",
    description="example description",
    media_concurrencies=[{
        "channel": "VOICE",
        "concurrency": 1,
    }],
    queue_configs=[{
        "channel": "VOICE",
        "delay": 2,
        "priority": 1,
        "queue_id": "12345678-1234-1234-1234-123456789012",
    }],
    tags={
        "Name": "Example Routing Profile",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.RoutingProfile("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "example",
        DefaultOutboundQueueId = "12345678-1234-1234-1234-123456789012",
        Description = "example description",
        MediaConcurrencies = new[]
        {
            new Aws.Connect.Inputs.RoutingProfileMediaConcurrencyArgs
            {
                Channel = "VOICE",
                Concurrency = 1,
            },
        },
        QueueConfigs = new[]
        {
            new Aws.Connect.Inputs.RoutingProfileQueueConfigArgs
            {
                Channel = "VOICE",
                Delay = 2,
                Priority = 1,
                QueueId = "12345678-1234-1234-1234-123456789012",
            },
        },
        Tags = 
        {
            { "Name", "Example Routing Profile" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewRoutingProfile(ctx, "example", &connect.RoutingProfileArgs{
			InstanceId:             pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:                   pulumi.String("example"),
			DefaultOutboundQueueId: pulumi.String("12345678-1234-1234-1234-123456789012"),
			Description:            pulumi.String("example description"),
			MediaConcurrencies: connect.RoutingProfileMediaConcurrencyArray{
				&connect.RoutingProfileMediaConcurrencyArgs{
					Channel:     pulumi.String("VOICE"),
					Concurrency: pulumi.Int(1),
				},
			},
			QueueConfigs: connect.RoutingProfileQueueConfigArray{
				&connect.RoutingProfileQueueConfigArgs{
					Channel:  pulumi.String("VOICE"),
					Delay:    pulumi.Int(2),
					Priority: pulumi.Int(1),
					QueueId:  pulumi.String("12345678-1234-1234-1234-123456789012"),
				},
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Routing Profile"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.RoutingProfile;
import com.pulumi.aws.connect.RoutingProfileArgs;
import com.pulumi.aws.connect.inputs.RoutingProfileMediaConcurrencyArgs;
import com.pulumi.aws.connect.inputs.RoutingProfileQueueConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new RoutingProfile("example", RoutingProfileArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("example")
            .defaultOutboundQueueId("12345678-1234-1234-1234-123456789012")
            .description("example description")
            .mediaConcurrencies(RoutingProfileMediaConcurrencyArgs.builder()
                .channel("VOICE")
                .concurrency(1)
                .build())
            .queueConfigs(RoutingProfileQueueConfigArgs.builder()
                .channel("VOICE")
                .delay(2)
                .priority(1)
                .queueId("12345678-1234-1234-1234-123456789012")
                .build())
            .tags(Map.of("Name", "Example Routing Profile"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:RoutingProfile
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: example
      defaultOutboundQueueId: 12345678-1234-1234-1234-123456789012
      description: example description
      mediaConcurrencies:
        - channel: VOICE
          concurrency: 1
      queueConfigs:
        - channel: VOICE
          delay: 2
          priority: 1
          queueId: 12345678-1234-1234-1234-123456789012
      tags:
        Name: Example Routing Profile
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Routing Profiles using the `instance_id` and `routing_profile_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/routingProfile:RoutingProfile example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
\
defaultOutboundQueueId" >Specifies the default outbound queue for the Routing Profile.
E
description" 2Specifies the description of the Routing Profile.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
�
mediaConcurrenciesz*x:v
t
connectRoutingProfileMediaConcurrencyIaws:connect/RoutingProfileMediaConcurrency:RoutingProfileMediaConcurrency�One or more `media_concurrencies` blocks that specify the channels that agents can handle in the Contact Control Panel (CCP) for this Routing Profile. The `media_concurrencies` block is documented below.
9
nameB" +Specifies the name of the Routing Profile.
�
queueConfigsmBk*i:g
e
connectRoutingProfileQueueConfig?aws:connect/RoutingProfileQueueConfig:RoutingProfileQueueConfig�One or more `queue_configs` blocks that specify the inbound queues associated with the routing profile. If no queue is added, the agent only can make outbound calls. The `queue_configs` block is documented below.
�
tagsB2" �Tags to apply to the Routing Profile. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"B
arn" 7The Amazon Resource Name (ARN) of the Routing Profile.
"\
defaultOutboundQueueId" >Specifies the default outbound queue for the Routing Profile.
"E
description" 2Specifies the description of the Routing Profile.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"�
mediaConcurrenciesz*x:v
t
connectRoutingProfileMediaConcurrencyIaws:connect/RoutingProfileMediaConcurrency:RoutingProfileMediaConcurrency�One or more `media_concurrencies` blocks that specify the channels that agents can handle in the Contact Control Panel (CCP) for this Routing Profile. The `media_concurrencies` block is documented below.
"7
name" +Specifies the name of the Routing Profile.
"�
queueConfigsmBk*i:g
e
connectRoutingProfileQueueConfig?aws:connect/RoutingProfileQueueConfig:RoutingProfileQueueConfig�One or more `queue_configs` blocks that specify the inbound queues associated with the routing profile. If no queue is added, the agent only can make outbound calls. The `queue_configs` block is documented below.
"@
routingProfileId" (The identifier for the Routing Profile.
"�
tagsB2" �Tags to apply to the Routing Profile. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�*
G
connectSecurityProfile+aws:connect/securityProfile:SecurityProfile�Provides an Amazon Connect Security Profile resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.SecurityProfile("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "example",
    description: "example description",
    permissions: [
        "BasicAgentAccess",
        "OutboundCallAccess",
    ],
    tags: {
        Name: "Example Security Profile",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.SecurityProfile("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="example",
    description="example description",
    permissions=[
        "BasicAgentAccess",
        "OutboundCallAccess",
    ],
    tags={
        "Name": "Example Security Profile",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.SecurityProfile("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "example",
        Description = "example description",
        Permissions = new[]
        {
            "BasicAgentAccess",
            "OutboundCallAccess",
        },
        Tags = 
        {
            { "Name", "Example Security Profile" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewSecurityProfile(ctx, "example", &connect.SecurityProfileArgs{
			InstanceId:  pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:        pulumi.String("example"),
			Description: pulumi.String("example description"),
			Permissions: pulumi.StringArray{
				pulumi.String("BasicAgentAccess"),
				pulumi.String("OutboundCallAccess"),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Security Profile"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.SecurityProfile;
import com.pulumi.aws.connect.SecurityProfileArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new SecurityProfile("example", SecurityProfileArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("example")
            .description("example description")
            .permissions(            
                "BasicAgentAccess",
                "OutboundCallAccess")
            .tags(Map.of("Name", "Example Security Profile"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:SecurityProfile
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: example
      description: example description
      permissions:
        - BasicAgentAccess
        - OutboundCallAccess
      tags:
        Name: Example Security Profile
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Security Profiles using the `instance_id` and `security_profile_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/securityProfile:SecurityProfile example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
H
descriptionB" 3Specifies the description of the Security Profile.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
:
nameB" ,Specifies the name of the Security Profile.
Y
permissionsB*" BSpecifies a list of permissions assigned to the security profile.
�
tagsB2" �Tags to apply to the Security Profile. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"C
arn" 8The Amazon Resource Name (ARN) of the Security Profile.
"H
descriptionB" 3Specifies the description of the Security Profile.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"8
name" ,Specifies the name of the Security Profile.
"]
organizationResourceId" ?The organization resource identifier for the security profile.
"Y
permissionsB*" BSpecifies a list of permissions assigned to the security profile.
"B
securityProfileId" )The identifier for the Security Profile.
"�
tagsB2" �Tags to apply to the Security Profile. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
&
connectUseraws:connect/user:User��Provides an Amazon Connect User resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.User("example", {
    instanceId: exampleAwsConnectInstance.id,
    name: "example",
    password: "Password123",
    routingProfileId: exampleAwsConnectRoutingProfile.routingProfileId,
    securityProfileIds: [exampleAwsConnectSecurityProfile.securityProfileId],
    identityInfo: {
        firstName: "example",
        lastName: "example2",
    },
    phoneConfig: {
        afterContactWorkTimeLimit: 0,
        phoneType: "SOFT_PHONE",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.User("example",
    instance_id=example_aws_connect_instance["id"],
    name="example",
    password="Password123",
    routing_profile_id=example_aws_connect_routing_profile["routingProfileId"],
    security_profile_ids=[example_aws_connect_security_profile["securityProfileId"]],
    identity_info={
        "first_name": "example",
        "last_name": "example2",
    },
    phone_config={
        "after_contact_work_time_limit": 0,
        "phone_type": "SOFT_PHONE",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.User("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        Name = "example",
        Password = "Password123",
        RoutingProfileId = exampleAwsConnectRoutingProfile.RoutingProfileId,
        SecurityProfileIds = new[]
        {
            exampleAwsConnectSecurityProfile.SecurityProfileId,
        },
        IdentityInfo = new Aws.Connect.Inputs.UserIdentityInfoArgs
        {
            FirstName = "example",
            LastName = "example2",
        },
        PhoneConfig = new Aws.Connect.Inputs.UserPhoneConfigArgs
        {
            AfterContactWorkTimeLimit = 0,
            PhoneType = "SOFT_PHONE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUser(ctx, "example", &connect.UserArgs{
			InstanceId:       pulumi.Any(exampleAwsConnectInstance.Id),
			Name:             pulumi.String("example"),
			Password:         pulumi.String("Password123"),
			RoutingProfileId: pulumi.Any(exampleAwsConnectRoutingProfile.RoutingProfileId),
			SecurityProfileIds: pulumi.StringArray{
				exampleAwsConnectSecurityProfile.SecurityProfileId,
			},
			IdentityInfo: &connect.UserIdentityInfoArgs{
				FirstName: pulumi.String("example"),
				LastName:  pulumi.String("example2"),
			},
			PhoneConfig: &connect.UserPhoneConfigArgs{
				AfterContactWorkTimeLimit: pulumi.Int(0),
				PhoneType:                 pulumi.String("SOFT_PHONE"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.User;
import com.pulumi.aws.connect.UserArgs;
import com.pulumi.aws.connect.inputs.UserIdentityInfoArgs;
import com.pulumi.aws.connect.inputs.UserPhoneConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new User("example", UserArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .name("example")
            .password("Password123")
            .routingProfileId(exampleAwsConnectRoutingProfile.routingProfileId())
            .securityProfileIds(exampleAwsConnectSecurityProfile.securityProfileId())
            .identityInfo(UserIdentityInfoArgs.builder()
                .firstName("example")
                .lastName("example2")
                .build())
            .phoneConfig(UserPhoneConfigArgs.builder()
                .afterContactWorkTimeLimit(0)
                .phoneType("SOFT_PHONE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:User
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      name: example
      password: Password123
      routingProfileId: ${exampleAwsConnectRoutingProfile.routingProfileId}
      securityProfileIds:
        - ${exampleAwsConnectSecurityProfile.securityProfileId}
      identityInfo:
        firstName: example
        lastName: example2
      phoneConfig:
        afterContactWorkTimeLimit: 0
        phoneType: SOFT_PHONE
```
<!--End PulumiCodeChooser -->

### With hierarchy_group_id

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.User("example", {
    instanceId: exampleAwsConnectInstance.id,
    name: "example",
    password: "Password123",
    routingProfileId: exampleAwsConnectRoutingProfile.routingProfileId,
    hierarchyGroupId: exampleAwsConnectUserHierarchyGroup.hierarchyGroupId,
    securityProfileIds: [exampleAwsConnectSecurityProfile.securityProfileId],
    identityInfo: {
        firstName: "example",
        lastName: "example2",
    },
    phoneConfig: {
        afterContactWorkTimeLimit: 0,
        phoneType: "SOFT_PHONE",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.User("example",
    instance_id=example_aws_connect_instance["id"],
    name="example",
    password="Password123",
    routing_profile_id=example_aws_connect_routing_profile["routingProfileId"],
    hierarchy_group_id=example_aws_connect_user_hierarchy_group["hierarchyGroupId"],
    security_profile_ids=[example_aws_connect_security_profile["securityProfileId"]],
    identity_info={
        "first_name": "example",
        "last_name": "example2",
    },
    phone_config={
        "after_contact_work_time_limit": 0,
        "phone_type": "SOFT_PHONE",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.User("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        Name = "example",
        Password = "Password123",
        RoutingProfileId = exampleAwsConnectRoutingProfile.RoutingProfileId,
        HierarchyGroupId = exampleAwsConnectUserHierarchyGroup.HierarchyGroupId,
        SecurityProfileIds = new[]
        {
            exampleAwsConnectSecurityProfile.SecurityProfileId,
        },
        IdentityInfo = new Aws.Connect.Inputs.UserIdentityInfoArgs
        {
            FirstName = "example",
            LastName = "example2",
        },
        PhoneConfig = new Aws.Connect.Inputs.UserPhoneConfigArgs
        {
            AfterContactWorkTimeLimit = 0,
            PhoneType = "SOFT_PHONE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUser(ctx, "example", &connect.UserArgs{
			InstanceId:       pulumi.Any(exampleAwsConnectInstance.Id),
			Name:             pulumi.String("example"),
			Password:         pulumi.String("Password123"),
			RoutingProfileId: pulumi.Any(exampleAwsConnectRoutingProfile.RoutingProfileId),
			HierarchyGroupId: pulumi.Any(exampleAwsConnectUserHierarchyGroup.HierarchyGroupId),
			SecurityProfileIds: pulumi.StringArray{
				exampleAwsConnectSecurityProfile.SecurityProfileId,
			},
			IdentityInfo: &connect.UserIdentityInfoArgs{
				FirstName: pulumi.String("example"),
				LastName:  pulumi.String("example2"),
			},
			PhoneConfig: &connect.UserPhoneConfigArgs{
				AfterContactWorkTimeLimit: pulumi.Int(0),
				PhoneType:                 pulumi.String("SOFT_PHONE"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.User;
import com.pulumi.aws.connect.UserArgs;
import com.pulumi.aws.connect.inputs.UserIdentityInfoArgs;
import com.pulumi.aws.connect.inputs.UserPhoneConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new User("example", UserArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .name("example")
            .password("Password123")
            .routingProfileId(exampleAwsConnectRoutingProfile.routingProfileId())
            .hierarchyGroupId(exampleAwsConnectUserHierarchyGroup.hierarchyGroupId())
            .securityProfileIds(exampleAwsConnectSecurityProfile.securityProfileId())
            .identityInfo(UserIdentityInfoArgs.builder()
                .firstName("example")
                .lastName("example2")
                .build())
            .phoneConfig(UserPhoneConfigArgs.builder()
                .afterContactWorkTimeLimit(0)
                .phoneType("SOFT_PHONE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:User
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      name: example
      password: Password123
      routingProfileId: ${exampleAwsConnectRoutingProfile.routingProfileId}
      hierarchyGroupId: ${exampleAwsConnectUserHierarchyGroup.hierarchyGroupId}
      securityProfileIds:
        - ${exampleAwsConnectSecurityProfile.securityProfileId}
      identityInfo:
        firstName: example
        lastName: example2
      phoneConfig:
        afterContactWorkTimeLimit: 0
        phoneType: SOFT_PHONE
```
<!--End PulumiCodeChooser -->

### With identity_info filled

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.User("example", {
    instanceId: exampleAwsConnectInstance.id,
    name: "example",
    password: "Password123",
    routingProfileId: exampleAwsConnectRoutingProfile.routingProfileId,
    securityProfileIds: [exampleAwsConnectSecurityProfile.securityProfileId],
    identityInfo: {
        email: "example@example.com",
        firstName: "example",
        lastName: "example2",
    },
    phoneConfig: {
        afterContactWorkTimeLimit: 0,
        phoneType: "SOFT_PHONE",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.User("example",
    instance_id=example_aws_connect_instance["id"],
    name="example",
    password="Password123",
    routing_profile_id=example_aws_connect_routing_profile["routingProfileId"],
    security_profile_ids=[example_aws_connect_security_profile["securityProfileId"]],
    identity_info={
        "email": "example@example.com",
        "first_name": "example",
        "last_name": "example2",
    },
    phone_config={
        "after_contact_work_time_limit": 0,
        "phone_type": "SOFT_PHONE",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.User("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        Name = "example",
        Password = "Password123",
        RoutingProfileId = exampleAwsConnectRoutingProfile.RoutingProfileId,
        SecurityProfileIds = new[]
        {
            exampleAwsConnectSecurityProfile.SecurityProfileId,
        },
        IdentityInfo = new Aws.Connect.Inputs.UserIdentityInfoArgs
        {
            Email = "example@example.com",
            FirstName = "example",
            LastName = "example2",
        },
        PhoneConfig = new Aws.Connect.Inputs.UserPhoneConfigArgs
        {
            AfterContactWorkTimeLimit = 0,
            PhoneType = "SOFT_PHONE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUser(ctx, "example", &connect.UserArgs{
			InstanceId:       pulumi.Any(exampleAwsConnectInstance.Id),
			Name:             pulumi.String("example"),
			Password:         pulumi.String("Password123"),
			RoutingProfileId: pulumi.Any(exampleAwsConnectRoutingProfile.RoutingProfileId),
			SecurityProfileIds: pulumi.StringArray{
				exampleAwsConnectSecurityProfile.SecurityProfileId,
			},
			IdentityInfo: &connect.UserIdentityInfoArgs{
				Email:     pulumi.String("example@example.com"),
				FirstName: pulumi.String("example"),
				LastName:  pulumi.String("example2"),
			},
			PhoneConfig: &connect.UserPhoneConfigArgs{
				AfterContactWorkTimeLimit: pulumi.Int(0),
				PhoneType:                 pulumi.String("SOFT_PHONE"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.User;
import com.pulumi.aws.connect.UserArgs;
import com.pulumi.aws.connect.inputs.UserIdentityInfoArgs;
import com.pulumi.aws.connect.inputs.UserPhoneConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new User("example", UserArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .name("example")
            .password("Password123")
            .routingProfileId(exampleAwsConnectRoutingProfile.routingProfileId())
            .securityProfileIds(exampleAwsConnectSecurityProfile.securityProfileId())
            .identityInfo(UserIdentityInfoArgs.builder()
                .email("example@example.com")
                .firstName("example")
                .lastName("example2")
                .build())
            .phoneConfig(UserPhoneConfigArgs.builder()
                .afterContactWorkTimeLimit(0)
                .phoneType("SOFT_PHONE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:User
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      name: example
      password: Password123
      routingProfileId: ${exampleAwsConnectRoutingProfile.routingProfileId}
      securityProfileIds:
        - ${exampleAwsConnectSecurityProfile.securityProfileId}
      identityInfo:
        email: example@example.com
        firstName: example
        lastName: example2
      phoneConfig:
        afterContactWorkTimeLimit: 0
        phoneType: SOFT_PHONE
```
<!--End PulumiCodeChooser -->

### With phone_config phone type as desk phone

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.User("example", {
    instanceId: exampleAwsConnectInstance.id,
    name: "example",
    password: "Password123",
    routingProfileId: exampleAwsConnectRoutingProfile.routingProfileId,
    securityProfileIds: [exampleAwsConnectSecurityProfile.securityProfileId],
    phoneConfig: {
        afterContactWorkTimeLimit: 0,
        phoneType: "SOFT_PHONE",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.User("example",
    instance_id=example_aws_connect_instance["id"],
    name="example",
    password="Password123",
    routing_profile_id=example_aws_connect_routing_profile["routingProfileId"],
    security_profile_ids=[example_aws_connect_security_profile["securityProfileId"]],
    phone_config={
        "after_contact_work_time_limit": 0,
        "phone_type": "SOFT_PHONE",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.User("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        Name = "example",
        Password = "Password123",
        RoutingProfileId = exampleAwsConnectRoutingProfile.RoutingProfileId,
        SecurityProfileIds = new[]
        {
            exampleAwsConnectSecurityProfile.SecurityProfileId,
        },
        PhoneConfig = new Aws.Connect.Inputs.UserPhoneConfigArgs
        {
            AfterContactWorkTimeLimit = 0,
            PhoneType = "SOFT_PHONE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUser(ctx, "example", &connect.UserArgs{
			InstanceId:       pulumi.Any(exampleAwsConnectInstance.Id),
			Name:             pulumi.String("example"),
			Password:         pulumi.String("Password123"),
			RoutingProfileId: pulumi.Any(exampleAwsConnectRoutingProfile.RoutingProfileId),
			SecurityProfileIds: pulumi.StringArray{
				exampleAwsConnectSecurityProfile.SecurityProfileId,
			},
			PhoneConfig: &connect.UserPhoneConfigArgs{
				AfterContactWorkTimeLimit: pulumi.Int(0),
				PhoneType:                 pulumi.String("SOFT_PHONE"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.User;
import com.pulumi.aws.connect.UserArgs;
import com.pulumi.aws.connect.inputs.UserPhoneConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new User("example", UserArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .name("example")
            .password("Password123")
            .routingProfileId(exampleAwsConnectRoutingProfile.routingProfileId())
            .securityProfileIds(exampleAwsConnectSecurityProfile.securityProfileId())
            .phoneConfig(UserPhoneConfigArgs.builder()
                .afterContactWorkTimeLimit(0)
                .phoneType("SOFT_PHONE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:User
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      name: example
      password: Password123
      routingProfileId: ${exampleAwsConnectRoutingProfile.routingProfileId}
      securityProfileIds:
        - ${exampleAwsConnectSecurityProfile.securityProfileId}
      phoneConfig:
        afterContactWorkTimeLimit: 0
        phoneType: SOFT_PHONE
```
<!--End PulumiCodeChooser -->

### With multiple Security profile ids specified in security_profile_ids

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.User("example", {
    instanceId: exampleAwsConnectInstance.id,
    name: "example",
    password: "Password123",
    routingProfileId: exampleAwsConnectRoutingProfile.routingProfileId,
    securityProfileIds: [
        exampleAwsConnectSecurityProfile.securityProfileId,
        example2.securityProfileId,
    ],
    phoneConfig: {
        afterContactWorkTimeLimit: 0,
        autoAccept: false,
        deskPhoneNumber: "+112345678912",
        phoneType: "DESK_PHONE",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.User("example",
    instance_id=example_aws_connect_instance["id"],
    name="example",
    password="Password123",
    routing_profile_id=example_aws_connect_routing_profile["routingProfileId"],
    security_profile_ids=[
        example_aws_connect_security_profile["securityProfileId"],
        example2["securityProfileId"],
    ],
    phone_config={
        "after_contact_work_time_limit": 0,
        "auto_accept": False,
        "desk_phone_number": "+112345678912",
        "phone_type": "DESK_PHONE",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.User("example", new()
    {
        InstanceId = exampleAwsConnectInstance.Id,
        Name = "example",
        Password = "Password123",
        RoutingProfileId = exampleAwsConnectRoutingProfile.RoutingProfileId,
        SecurityProfileIds = new[]
        {
            exampleAwsConnectSecurityProfile.SecurityProfileId,
            example2.SecurityProfileId,
        },
        PhoneConfig = new Aws.Connect.Inputs.UserPhoneConfigArgs
        {
            AfterContactWorkTimeLimit = 0,
            AutoAccept = false,
            DeskPhoneNumber = "+112345678912",
            PhoneType = "DESK_PHONE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUser(ctx, "example", &connect.UserArgs{
			InstanceId:       pulumi.Any(exampleAwsConnectInstance.Id),
			Name:             pulumi.String("example"),
			Password:         pulumi.String("Password123"),
			RoutingProfileId: pulumi.Any(exampleAwsConnectRoutingProfile.RoutingProfileId),
			SecurityProfileIds: pulumi.StringArray{
				exampleAwsConnectSecurityProfile.SecurityProfileId,
				example2.SecurityProfileId,
			},
			PhoneConfig: &connect.UserPhoneConfigArgs{
				AfterContactWorkTimeLimit: pulumi.Int(0),
				AutoAccept:                pulumi.Bool(false),
				DeskPhoneNumber:           pulumi.String("+112345678912"),
				PhoneType:                 pulumi.String("DESK_PHONE"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.User;
import com.pulumi.aws.connect.UserArgs;
import com.pulumi.aws.connect.inputs.UserPhoneConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new User("example", UserArgs.builder()
            .instanceId(exampleAwsConnectInstance.id())
            .name("example")
            .password("Password123")
            .routingProfileId(exampleAwsConnectRoutingProfile.routingProfileId())
            .securityProfileIds(            
                exampleAwsConnectSecurityProfile.securityProfileId(),
                example2.securityProfileId())
            .phoneConfig(UserPhoneConfigArgs.builder()
                .afterContactWorkTimeLimit(0)
                .autoAccept(false)
                .deskPhoneNumber("+112345678912")
                .phoneType("DESK_PHONE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:User
    properties:
      instanceId: ${exampleAwsConnectInstance.id}
      name: example
      password: Password123
      routingProfileId: ${exampleAwsConnectRoutingProfile.routingProfileId}
      securityProfileIds:
        - ${exampleAwsConnectSecurityProfile.securityProfileId}
        - ${example2.securityProfileId}
      phoneConfig:
        afterContactWorkTimeLimit: 0
        autoAccept: false
        deskPhoneNumber: '+112345678912'
        phoneType: DESK_PHONE
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Users using the `instance_id` and `user_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/user:User example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
�
directoryUserIdB" �The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory. This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.
N
hierarchyGroupIdB" 4The identifier of the hierarchy group for the user.
�
identityInfoPBN:L
J
connectUserIdentityInfo-aws:connect/UserIdentityInfo:UserIdentityInfoTA block that contains information about the identity of the user. Documented below.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
�
nameB" �The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from `[a-zA-Z0-9_-.\@]+`.
�
passwordB" �The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.
�
phoneConfigK:I
G
connectUserPhoneConfig+aws:connect/UserPhoneConfig:UserPhoneConfig[A block that contains information about the phone settings for the user. Documented below.
L
routingProfileId" 4The identifier of the routing profile for the user.
�
securityProfileIds*" �A list of identifiers for the security profiles for the user. Specify a minimum of 1 and maximum of 10 security profile ids. For more information, see [Best Practices for Security Profiles](https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html) in the Amazon Connect Administrator Guide.
�
tagsB2" �Tags to apply to the user. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"7
arn" ,The Amazon Resource Name (ARN) of the user.
"�
directoryUserId" �The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory. This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.
"N
hierarchyGroupIdB" 4The identifier of the hierarchy group for the user.
"�
identityInfoPBN:L
J
connectUserIdentityInfo-aws:connect/UserIdentityInfo:UserIdentityInfoTA block that contains information about the identity of the user. Documented below.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"�
name" �The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from `[a-zA-Z0-9_-.\@]+`.
"�
passwordB" �The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.
"�
phoneConfigK:I
G
connectUserPhoneConfig+aws:connect/UserPhoneConfig:UserPhoneConfig[A block that contains information about the phone settings for the user. Documented below.
"L
routingProfileId" 4The identifier of the routing profile for the user.
"�
securityProfileIds*" �A list of identifiers for the security profiles for the user. Specify a minimum of 1 and maximum of 10 security profile ids. For more information, see [Best Practices for Security Profiles](https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html) in the Amazon Connect Administrator Guide.
"�
tagsB2" �Tags to apply to the user. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"+
userId" The identifier for the user.
*�I
P
connectUserHierarchyGroup1aws:connect/userHierarchyGroup:UserHierarchyGroup�<Provides an Amazon Connect User Hierarchy Group resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

> **NOTE:** The User Hierarchy Structure must be created before creating a User Hierarchy Group.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.UserHierarchyGroup("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "example",
    tags: {
        Name: "Example User Hierarchy Group",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.UserHierarchyGroup("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="example",
    tags={
        "Name": "Example User Hierarchy Group",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.UserHierarchyGroup("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "example",
        Tags = 
        {
            { "Name", "Example User Hierarchy Group" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUserHierarchyGroup(ctx, "example", &connect.UserHierarchyGroupArgs{
			InstanceId: pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:       pulumi.String("example"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example User Hierarchy Group"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.UserHierarchyGroup;
import com.pulumi.aws.connect.UserHierarchyGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserHierarchyGroup("example", UserHierarchyGroupArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("example")
            .tags(Map.of("Name", "Example User Hierarchy Group"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:UserHierarchyGroup
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: example
      tags:
        Name: Example User Hierarchy Group
```
<!--End PulumiCodeChooser -->

### With a parent group

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const parent = new aws.connect.UserHierarchyGroup("parent", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "parent",
    tags: {
        Name: "Example User Hierarchy Group Parent",
    },
});
const child = new aws.connect.UserHierarchyGroup("child", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "child",
    parentGroupId: parent.hierarchyGroupId,
    tags: {
        Name: "Example User Hierarchy Group Child",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

parent = aws.connect.UserHierarchyGroup("parent",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="parent",
    tags={
        "Name": "Example User Hierarchy Group Parent",
    })
child = aws.connect.UserHierarchyGroup("child",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="child",
    parent_group_id=parent.hierarchy_group_id,
    tags={
        "Name": "Example User Hierarchy Group Child",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var parent = new Aws.Connect.UserHierarchyGroup("parent", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "parent",
        Tags = 
        {
            { "Name", "Example User Hierarchy Group Parent" },
        },
    });

    var child = new Aws.Connect.UserHierarchyGroup("child", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "child",
        ParentGroupId = parent.HierarchyGroupId,
        Tags = 
        {
            { "Name", "Example User Hierarchy Group Child" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		parent, err := connect.NewUserHierarchyGroup(ctx, "parent", &connect.UserHierarchyGroupArgs{
			InstanceId: pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:       pulumi.String("parent"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example User Hierarchy Group Parent"),
			},
		})
		if err != nil {
			return err
		}
		_, err = connect.NewUserHierarchyGroup(ctx, "child", &connect.UserHierarchyGroupArgs{
			InstanceId:    pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:          pulumi.String("child"),
			ParentGroupId: parent.HierarchyGroupId,
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example User Hierarchy Group Child"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.UserHierarchyGroup;
import com.pulumi.aws.connect.UserHierarchyGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var parent = new UserHierarchyGroup("parent", UserHierarchyGroupArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("parent")
            .tags(Map.of("Name", "Example User Hierarchy Group Parent"))
            .build());

        var child = new UserHierarchyGroup("child", UserHierarchyGroupArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("child")
            .parentGroupId(parent.hierarchyGroupId())
            .tags(Map.of("Name", "Example User Hierarchy Group Child"))
            .build());

    }
}
```
```yaml
resources:
  parent:
    type: aws:connect:UserHierarchyGroup
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: parent
      tags:
        Name: Example User Hierarchy Group Parent
  child:
    type: aws:connect:UserHierarchyGroup
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: child
      parentGroupId: ${parent.hierarchyGroupId}
      tags:
        Name: Example User Hierarchy Group Child
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect User Hierarchy Groups using the `instance_id` and `hierarchy_group_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/userHierarchyGroup:UserHierarchyGroup example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
Z
nameB" LThe name of the user hierarchy group. Must not be more than 100 characters.
�
parentGroupIdB" zThe identifier for the parent hierarchy group. The user hierarchy is created at level one if the parent group ID is null.
�
tagsB2" �Tags to apply to the hierarchy group. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"B
arn" 7The Amazon Resource Name (ARN) of the hierarchy group.
"@
hierarchyGroupId" (The identifier for the hierarchy group.
"�
hierarchyPaths}*{:y
w
connectUserHierarchyGroupHierarchyPathKaws:connect/UserHierarchyGroupHierarchyPath:UserHierarchyGroupHierarchyPath{A block that contains information about the levels in the hierarchy group. The `hierarchy_path` block is documented below.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"C
levelId" 4The identifier of the level in the hierarchy group.
"X
name" LThe name of the user hierarchy group. Must not be more than 100 characters.
"�
parentGroupIdB" zThe identifier for the parent hierarchy group. The user hierarchy is created at level one if the parent group ID is null.
"�
tagsB2" �Tags to apply to the hierarchy group. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�S
\
connectUserHierarchyStructure9aws:connect/userHierarchyStructure:UserHierarchyStructure�MProvides an Amazon Connect User Hierarchy Structure resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.UserHierarchyStructure("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hierarchyStructure: {
        levelOne: {
            name: "levelone",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.UserHierarchyStructure("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hierarchy_structure={
        "level_one": {
            "name": "levelone",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.UserHierarchyStructure("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        HierarchyStructure = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureArgs
        {
            LevelOne = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureLevelOneArgs
            {
                Name = "levelone",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUserHierarchyStructure(ctx, "example", &connect.UserHierarchyStructureArgs{
			InstanceId: pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			HierarchyStructure: &connect.UserHierarchyStructureHierarchyStructureArgs{
				LevelOne: &connect.UserHierarchyStructureHierarchyStructureLevelOneArgs{
					Name: pulumi.String("levelone"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.UserHierarchyStructure;
import com.pulumi.aws.connect.UserHierarchyStructureArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureLevelOneArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserHierarchyStructure("example", UserHierarchyStructureArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .hierarchyStructure(UserHierarchyStructureHierarchyStructureArgs.builder()
                .levelOne(UserHierarchyStructureHierarchyStructureLevelOneArgs.builder()
                    .name("levelone")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:UserHierarchyStructure
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      hierarchyStructure:
        levelOne:
          name: levelone
```
<!--End PulumiCodeChooser -->

### With Five Levels

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.UserHierarchyStructure("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hierarchyStructure: {
        levelOne: {
            name: "levelone",
        },
        levelTwo: {
            name: "leveltwo",
        },
        levelThree: {
            name: "levelthree",
        },
        levelFour: {
            name: "levelfour",
        },
        levelFive: {
            name: "levelfive",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.UserHierarchyStructure("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hierarchy_structure={
        "level_one": {
            "name": "levelone",
        },
        "level_two": {
            "name": "leveltwo",
        },
        "level_three": {
            "name": "levelthree",
        },
        "level_four": {
            "name": "levelfour",
        },
        "level_five": {
            "name": "levelfive",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.UserHierarchyStructure("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        HierarchyStructure = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureArgs
        {
            LevelOne = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureLevelOneArgs
            {
                Name = "levelone",
            },
            LevelTwo = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureLevelTwoArgs
            {
                Name = "leveltwo",
            },
            LevelThree = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureLevelThreeArgs
            {
                Name = "levelthree",
            },
            LevelFour = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureLevelFourArgs
            {
                Name = "levelfour",
            },
            LevelFive = new Aws.Connect.Inputs.UserHierarchyStructureHierarchyStructureLevelFiveArgs
            {
                Name = "levelfive",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewUserHierarchyStructure(ctx, "example", &connect.UserHierarchyStructureArgs{
			InstanceId: pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			HierarchyStructure: &connect.UserHierarchyStructureHierarchyStructureArgs{
				LevelOne: &connect.UserHierarchyStructureHierarchyStructureLevelOneArgs{
					Name: pulumi.String("levelone"),
				},
				LevelTwo: &connect.UserHierarchyStructureHierarchyStructureLevelTwoArgs{
					Name: pulumi.String("leveltwo"),
				},
				LevelThree: &connect.UserHierarchyStructureHierarchyStructureLevelThreeArgs{
					Name: pulumi.String("levelthree"),
				},
				LevelFour: &connect.UserHierarchyStructureHierarchyStructureLevelFourArgs{
					Name: pulumi.String("levelfour"),
				},
				LevelFive: &connect.UserHierarchyStructureHierarchyStructureLevelFiveArgs{
					Name: pulumi.String("levelfive"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.UserHierarchyStructure;
import com.pulumi.aws.connect.UserHierarchyStructureArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureLevelOneArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureLevelTwoArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureLevelThreeArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureLevelFourArgs;
import com.pulumi.aws.connect.inputs.UserHierarchyStructureHierarchyStructureLevelFiveArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new UserHierarchyStructure("example", UserHierarchyStructureArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .hierarchyStructure(UserHierarchyStructureHierarchyStructureArgs.builder()
                .levelOne(UserHierarchyStructureHierarchyStructureLevelOneArgs.builder()
                    .name("levelone")
                    .build())
                .levelTwo(UserHierarchyStructureHierarchyStructureLevelTwoArgs.builder()
                    .name("leveltwo")
                    .build())
                .levelThree(UserHierarchyStructureHierarchyStructureLevelThreeArgs.builder()
                    .name("levelthree")
                    .build())
                .levelFour(UserHierarchyStructureHierarchyStructureLevelFourArgs.builder()
                    .name("levelfour")
                    .build())
                .levelFive(UserHierarchyStructureHierarchyStructureLevelFiveArgs.builder()
                    .name("levelfive")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:UserHierarchyStructure
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      hierarchyStructure:
        levelOne:
          name: levelone
        levelTwo:
          name: leveltwo
        levelThree:
          name: levelthree
        levelFour:
          name: levelfour
        levelFive:
          name: levelfive
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect User Hierarchy Structures using the `instance_id`. For example:

```sh
$ pulumi import aws:connect/userHierarchyStructure:UserHierarchyStructure example f1288a1f-6193-445a-b47e-af739b2
```
�
hierarchyStructure�:�
�
connect(UserHierarchyStructureHierarchyStructure]aws:connect/UserHierarchyStructureHierarchyStructure:UserHierarchyStructureHierarchyStructurelA block that defines the hierarchy structure's levels. The `hierarchy_structure` block is documented below.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"�
hierarchyStructure�:�
�
connect(UserHierarchyStructureHierarchyStructure]aws:connect/UserHierarchyStructureHierarchyStructure:UserHierarchyStructureHierarchyStructurelA block that defines the hierarchy structure's levels. The `hierarchy_structure` block is documented below.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
*�8
8
connect
Vocabulary!aws:connect/vocabulary:Vocabulary�Provides an Amazon Connect Vocabulary resource. For more information see
[Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.connect.Vocabulary("example", {
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "example",
    content: `Phrase\x09IPA\x09SoundsLike\x09DisplayAs
Los-Angeles\x09\x09\x09Los Angeles
F.B.I.\x09ɛ f b i aɪ\x09\x09FBI
Etienne\x09\x09eh-tee-en\x09`,
    languageCode: "en-US",
    tags: {
        Key1: "Value1",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.Vocabulary("example",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="example",
    content="""Phrase\x09IPA\x09SoundsLike\x09DisplayAs
Los-Angeles\x09\x09\x09Los Angeles
F.B.I.\x09ɛ f b i aɪ\x09\x09FBI
Etienne\x09\x09eh-tee-en\x09""",
    language_code="en-US",
    tags={
        "Key1": "Value1",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Connect.Vocabulary("example", new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "example",
        Content = @"Phrase	IPA	SoundsLike	DisplayAs
Los-Angeles			Los Angeles
F.B.I.	ɛ f b i aɪ		FBI
Etienne		eh-tee-en	",
        LanguageCode = "en-US",
        Tags = 
        {
            { "Key1", "Value1" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.NewVocabulary(ctx, "example", &connect.VocabularyArgs{
			InstanceId:   pulumi.String("aaaaaaaa-bbbb-cccc-dddd-111111111111"),
			Name:         pulumi.String("example"),
			Content:      pulumi.String("Phrase	IPA	SoundsLike	DisplayAs\nLos-Angeles			Los Angeles\nF.B.I.	ɛ f b i aɪ		FBI\nEtienne		eh-tee-en	"),
			LanguageCode: pulumi.String("en-US"),
			Tags: pulumi.StringMap{
				"Key1": pulumi.String("Value1"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.Vocabulary;
import com.pulumi.aws.connect.VocabularyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Vocabulary("example", VocabularyArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("example")
            .content("""
Phrase	IPA	SoundsLike	DisplayAs
Los-Angeles			Los Angeles
F.B.I.	ɛ f b i aɪ		FBI
Etienne		eh-tee-en	            """)
            .languageCode("en-US")
            .tags(Map.of("Key1", "Value1"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:connect:Vocabulary
    properties:
      instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
      name: example
      content: |-
        Phrase	IPA	SoundsLike	DisplayAs
        Los-Angeles			Los Angeles
        F.B.I.	ɛ f b i aɪ		FBI
        Etienne		eh-tee-en	
      languageCode: en-US
      tags:
        Key1: Value1
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon Connect Vocabularies using the `instance_id` and `vocabulary_id` separated by a colon (`:`). For example:

```sh
$ pulumi import aws:connect/vocabulary:Vocabulary example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
```
�
content" �The content of the custom vocabulary in plain-text format with a table of values. Each row in the table represents a word or a phrase, described with Phrase, IPA, SoundsLike, and DisplayAs fields. Separate the fields with TAB characters. For more information, see [Create a custom vocabulary using a table](https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table). Minimum length of `1`. Maximum length of `60000`.
S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
�
languageCode" �The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see [What is Amazon Transcribe?](https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html). Valid Values are `ar-AE`, `de-CH`, `de-DE`, `en-AB`, `en-AU`, `en-GB`, `en-IE`, `en-IN`, `en-US`, `en-WL`, `es-ES`, `es-US`, `fr-CA`, `fr-FR`, `hi-IN`, `it-IT`, `ja-JP`, `ko-KR`, `pt-BR`, `pt-PT`, `zh-CN`.
\
nameB" NA unique name of the custom vocabulary. Must not be more than 140 characters.
�
tagsB2" �Tags to apply to the vocabulary. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"=
arn" 2The Amazon Resource Name (ARN) of the vocabulary.
"�
content" �The content of the custom vocabulary in plain-text format with a table of values. Each row in the table represents a word or a phrase, described with Phrase, IPA, SoundsLike, and DisplayAs fields. Separate the fields with TAB characters. For more information, see [Create a custom vocabulary using a table](https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table). Minimum length of `1`. Maximum length of `60000`.
"K
failureReason" 6The reason why the custom vocabulary was not created.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"�
languageCode" �The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see [What is Amazon Transcribe?](https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html). Valid Values are `ar-AE`, `de-CH`, `de-DE`, `en-AB`, `en-AU`, `en-GB`, `en-IE`, `en-IN`, `en-US`, `en-WL`, `es-ES`, `es-US`, `fr-CA`, `fr-FR`, `hi-IN`, `it-IT`, `ja-JP`, `ko-KR`, `pt-BR`, `pt-PT`, `zh-CN`.
"T
lastModifiedTime" <The timestamp when the custom vocabulary was last modified.
"Z
name" NA unique name of the custom vocabulary. Must not be more than 140 characters.
"�
state" �The current state of the custom vocabulary. Valid values are `CREATION_IN_PROGRESS`, `ACTIVE`, `CREATION_FAILED`, `DELETE_IN_PROGRESS`.
"�
tagsB2" �Tags to apply to the vocabulary. If configured with a provider
`default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"=
vocabularyId" )The identifier of the custom vocabulary.
2�
!getArnaws:index/getArn:getArn�Parses an ARN into its constituent parts.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const dbInstance = aws.getArn({
    arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
});
```
```python
import pulumi
import pulumi_aws as aws

db_instance = aws.get_arn(arn="arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var dbInstance = Aws.GetArn.Invoke(new()
    {
        Arn = "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetArn(ctx, &aws.GetArnArgs{
			Arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetArnArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var dbInstance = AwsFunctions.getArn(GetArnArgs.builder()
            .arn("arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
            .build());

    }
}
```
```yaml
variables:
  dbInstance:
    fn::invoke:
      function: aws:getArn
      arguments:
        arn: arn:aws:rds:eu-west-1:123456789012:db:mysql-db
```
<!--End PulumiCodeChooser -->

arn" ARN to parse.


idB" "�
account" �The [ID](https://docs.aws.amazon.com/general/latest/gr/acct-identifiers.html) of the AWS account that owns the resource, without the hyphens.
"	
arn" "
id" "4
	partition" #Partition that the resource is in.
"�
region" �Region the resource resides in.
Note that the ARNs for some resources do not require a region, so this component might be omitted.
"�
resource" �Content of this part of the ARN varies by service.
It often includes an indicator of the type of resource—for example, an IAM user or Amazon RDS database —followed by a slash (/) or a colon (:), followed by the resource name itself.
"�
service" �The [service namespace](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces) that identifies the AWS product.
2�[
HgetAvailabilityZone1aws:index/getAvailabilityZone:getAvailabilityZone�J`aws.getAvailabilityZone` provides details about a specific availability zone (AZ)
in the current region.

This can be used both to validate an availability zone given in a variable
and to split the AZ name into its component parts of an AWS region and an
AZ identifier letter. The latter may be useful e.g., for implementing a
consistent subnet numbering scheme across several regions by mapping both
the region and the subnet letter to network numbers.

This is different from the `aws.getAvailabilityZones` (plural) data source,
which provides a list of the available zones.

## Example Usage

The following example shows how this data source might be used to derive
VPC and subnet CIDR prefixes systematically for an availability zone.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const config = new pulumi.Config();
const regionNumber = config.getObject("regionNumber") || {
    "ap-northeast-1": 5,
    "eu-central-1": 4,
    "us-east-1": 1,
    "us-west-1": 2,
    "us-west-2": 3,
};
const azNumber = config.getObject("azNumber") || {
    a: 1,
    b: 2,
    c: 3,
    d: 4,
    e: 5,
    f: 6,
};
// Retrieve the AZ where we want to create network resources
// This must be in the region selected on the AWS provider.
const example = aws.getAvailabilityZone({
    name: "eu-central-1a",
});
// Create a VPC for the region associated with the AZ
const exampleVpc = new aws.ec2.Vpc("example", {cidrBlock: example.then(example => std.cidrsubnet({
    input: "10.0.0.0/8",
    newbits: 4,
    netnum: regionNumber[example.region],
})).then(invoke => invoke.result)});
// Create a subnet for the AZ within the regional VPC
const exampleSubnet = new aws.ec2.Subnet("example", {
    vpcId: exampleVpc.id,
    cidrBlock: pulumi.all([exampleVpc.cidrBlock, example]).apply(([cidrBlock, example]) => std.cidrsubnetOutput({
        input: cidrBlock,
        newbits: 4,
        netnum: azNumber[example.nameSuffix],
    })).apply(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

config = pulumi.Config()
region_number = config.get_object("regionNumber")
if region_number is None:
    region_number = {
        "ap-northeast-1": 5,
        "eu-central-1": 4,
        "us-east-1": 1,
        "us-west-1": 2,
        "us-west-2": 3,
    }
az_number = config.get_object("azNumber")
if az_number is None:
    az_number = {
        "a": 1,
        "b": 2,
        "c": 3,
        "d": 4,
        "e": 5,
        "f": 6,
    }
# Retrieve the AZ where we want to create network resources
# This must be in the region selected on the AWS provider.
example = aws.get_availability_zone(name="eu-central-1a")
# Create a VPC for the region associated with the AZ
example_vpc = aws.ec2.Vpc("example", cidr_block=std.cidrsubnet(input="10.0.0.0/8",
    newbits=4,
    netnum=region_number[example.region]).result)
# Create a subnet for the AZ within the regional VPC
example_subnet = aws.ec2.Subnet("example",
    vpc_id=example_vpc.id,
    cidr_block=example_vpc.cidr_block.apply(lambda cidr_block: std.cidrsubnet_output(input=cidr_block,
        newbits=4,
        netnum=az_number[example.name_suffix])).apply(lambda invoke: invoke.result))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var regionNumber = config.GetObject<dynamic>("regionNumber") ?? 
    {
        { "ap-northeast-1", 5 },
        { "eu-central-1", 4 },
        { "us-east-1", 1 },
        { "us-west-1", 2 },
        { "us-west-2", 3 },
    };
    var azNumber = config.GetObject<dynamic>("azNumber") ?? 
    {
        { "a", 1 },
        { "b", 2 },
        { "c", 3 },
        { "d", 4 },
        { "e", 5 },
        { "f", 6 },
    };
    // Retrieve the AZ where we want to create network resources
    // This must be in the region selected on the AWS provider.
    var example = Aws.GetAvailabilityZone.Invoke(new()
    {
        Name = "eu-central-1a",
    });

    // Create a VPC for the region associated with the AZ
    var exampleVpc = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = Std.Cidrsubnet.Invoke(new()
        {
            Input = "10.0.0.0/8",
            Newbits = 4,
            Netnum = regionNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.Region)],
        }).Apply(invoke => invoke.Result),
    });

    // Create a subnet for the AZ within the regional VPC
    var exampleSubnet = new Aws.Ec2.Subnet("example", new()
    {
        VpcId = exampleVpc.Id,
        CidrBlock = Output.Tuple(exampleVpc.CidrBlock, example).Apply(values =>
        {
            var cidrBlock = values.Item1;
            var example = values.Item2;
            return Std.Cidrsubnet.Invoke(new()
            {
                Input = cidrBlock,
                Newbits = 4,
                Netnum = azNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.NameSuffix)],
            });
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		regionNumber := map[string]interface{}{
			"ap-northeast-1": 5,
			"eu-central-1":   4,
			"us-east-1":      1,
			"us-west-1":      2,
			"us-west-2":      3,
		}
		if param := cfg.GetObject("regionNumber"); param != nil {
			regionNumber = param
		}
		azNumber := map[string]interface{}{
			"a": 1,
			"b": 2,
			"c": 3,
			"d": 4,
			"e": 5,
			"f": 6,
		}
		if param := cfg.GetObject("azNumber"); param != nil {
			azNumber = param
		}
		// Retrieve the AZ where we want to create network resources
		// This must be in the region selected on the AWS provider.
		example, err := aws.GetAvailabilityZone(ctx, &aws.GetAvailabilityZoneArgs{
			Name: pulumi.StringRef("eu-central-1a"),
		}, nil)
		if err != nil {
			return err
		}
		invokeCidrsubnet, err := std.Cidrsubnet(ctx, &std.CidrsubnetArgs{
			Input:   "10.0.0.0/8",
			Newbits: 4,
			Netnum:  regionNumber[example.Region],
		}, nil)
		if err != nil {
			return err
		}
		// Create a VPC for the region associated with the AZ
		exampleVpc, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock: pulumi.String(invokeCidrsubnet.Result),
		})
		if err != nil {
			return err
		}
		// Create a subnet for the AZ within the regional VPC
		_, err = ec2.NewSubnet(ctx, "example", &ec2.SubnetArgs{
			VpcId: exampleVpc.ID(),
			CidrBlock: pulumi.String(exampleVpc.CidrBlock.ApplyT(func(cidrBlock string) (std.CidrsubnetResult, error) {
				return std.CidrsubnetResult(interface{}(std.CidrsubnetOutput(ctx, std.CidrsubnetOutputArgs{
					Input:   cidrBlock,
					Newbits: 4,
					Netnum:  pulumi.Int(azNumber[example.NameSuffix]),
				}, nil))), nil
			}).(std.CidrsubnetResultOutput).ApplyT(func(invoke std.CidrsubnetResult) (*string, error) {
				return invoke.Result, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZoneArgs;
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var config = ctx.config();
        final var regionNumber = config.get("regionNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        final var azNumber = config.get("azNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        // Retrieve the AZ where we want to create network resources
        // This must be in the region selected on the AWS provider.
        final var example = AwsFunctions.getAvailabilityZone(GetAvailabilityZoneArgs.builder()
            .name("eu-central-1a")
            .build());

        // Create a VPC for the region associated with the AZ
        var exampleVpc = new Vpc("exampleVpc", VpcArgs.builder()
            .cidrBlock(StdFunctions.cidrsubnet(CidrsubnetArgs.builder()
                .input("10.0.0.0/8")
                .newbits(4)
                .netnum(regionNumber[example.applyValue(getAvailabilityZoneResult -> getAvailabilityZoneResult.region())])
                .build()).result())
            .build());

        // Create a subnet for the AZ within the regional VPC
        var exampleSubnet = new Subnet("exampleSubnet", SubnetArgs.builder()
            .vpcId(exampleVpc.id())
            .cidrBlock(exampleVpc.cidrBlock().applyValue(cidrBlock -> StdFunctions.cidrsubnet()).applyValue(invoke -> invoke.result()))
            .build());

    }
}
```
<!--End PulumiCodeChooser -->
�
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
�
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter6Configuration block(s) for filtering. Detailed below.
<
nameB" .Full name of the availability zone to select.
z
stateB" kSpecific availability zone state to require. May be any of `"available"`, `"information"` or `"impaired"`.
<
zoneIdB" ,Zone ID of the availability zone to select.
"
allAvailabilityZonesB
 "m
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter"�
	groupName" �For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "�

nameSuffix" �Part of the AZ name that appears after the region name, uniquely identifying the AZ within its region.
For Availability Zones this is usually a single letter, for example `a` for the `us-west-2a` zone.
For Local and Wavelength Zones this is a longer string, for example `wl1-sfo-wlz-1` for the `us-west-2-wl1-sfo-wlz-1` zone.
"Y
networkBorderGroup" ?The name of the location from which the address is advertised.
"�
optInStatus" �For Availability Zones, this always has the value of `opt-in-not-required`. For Local Zones, this is the opt in status. The possible values are `opted-in` and `not-opted-in`.
"�
parentZoneId" sID of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"�
parentZoneName" uName of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"�
region" �Region where the selected availability zone resides. This is always the region selected on the provider, since this data source searches only within that region.
"
state" "
zoneId" "c
zoneType" SType of zone. Values are `availability-zone`, `local-zone`, and `wavelength-zone`.
2�\
KgetAvailabilityZones3aws:index/getAvailabilityZones:getAvailabilityZones�PThe Availability Zones data source allows access to the list of AWS
Availability Zones which can be accessed by an AWS account within the region
configured in the provider.

This is different from the `aws.getAvailabilityZone` (singular) data source,
which provides some details about a specific availability zone.

> When [Local Zones](https://aws.amazon.com/about-aws/global-infrastructure/localzones/) are enabled in a region, by default the API and this data source include both Local Zones and Availability Zones. To return only Availability Zones, see the example section below.

## Example Usage

### By State

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// Declare the data source
const available = aws.getAvailabilityZones({
    state: "available",
});
// e.g., Create subnets in the first two available availability zones
const primary = new aws.ec2.Subnet("primary", {availabilityZone: available.then(available => available.names?.[0])});
const secondary = new aws.ec2.Subnet("secondary", {availabilityZone: available.then(available => available.names?.[1])});
```
```python
import pulumi
import pulumi_aws as aws

# Declare the data source
available = aws.get_availability_zones(state="available")
# e.g., Create subnets in the first two available availability zones
primary = aws.ec2.Subnet("primary", availability_zone=available.names[0])
secondary = aws.ec2.Subnet("secondary", availability_zone=available.names[1])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // Declare the data source
    var available = Aws.GetAvailabilityZones.Invoke(new()
    {
        State = "available",
    });

    // e.g., Create subnets in the first two available availability zones
    var primary = new Aws.Ec2.Subnet("primary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[0]),
    });

    var secondary = new Aws.Ec2.Subnet("secondary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[1]),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// Declare the data source
		available, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			State: pulumi.StringRef("available"),
		}, nil)
		if err != nil {
			return err
		}
		// e.g., Create subnets in the first two available availability zones
		_, err = ec2.NewSubnet(ctx, "primary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[0]),
		})
		if err != nil {
			return err
		}
		_, err = ec2.NewSubnet(ctx, "secondary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[1]),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // Declare the data source
        final var available = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .state("available")
            .build());

        // e.g., Create subnets in the first two available availability zones
        var primary = new Subnet("primary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[0]))
            .build());

        var secondary = new Subnet("secondary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[1]))
            .build());

    }
}
```
```yaml
resources:
  # e.g., Create subnets in the first two available availability zones
  primary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[0]}
  secondary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[1]}
variables:
  # Declare the data source
  available:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        state: available
```
<!--End PulumiCodeChooser -->

### By Filter

All Local Zones (regardless of opt-in status):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    allAvailabilityZones: true,
    filters: [{
        name: "opt-in-status",
        values: [
            "not-opted-in",
            "opted-in",
        ],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(all_availability_zones=True,
    filters=[{
        "name": "opt-in-status",
        "values": [
            "not-opted-in",
            "opted-in",
        ],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        AllAvailabilityZones = true,
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                    "opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			AllAvailabilityZones: pulumi.BoolRef(true),
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
						"opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .allAvailabilityZones(true)
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values(                
                    "not-opted-in",
                    "opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        allAvailabilityZones: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
              - opted-in
```
<!--End PulumiCodeChooser -->

Only Availability Zones (no Local Zones):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    filters: [{
        name: "opt-in-status",
        values: ["opt-in-not-required"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(filters=[{
    "name": "opt-in-status",
    "values": ["opt-in-not-required"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "opt-in-not-required",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"opt-in-not-required",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values("opt-in-not-required")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        filters:
          - name: opt-in-status
            values:
              - opt-in-not-required
```
<!--End PulumiCodeChooser -->
�
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
D
excludeNamesB*" ,List of Availability Zone names to exclude.
D
excludeZoneIdsB*" *List of Availability Zone IDs to exclude.
�
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter6Configuration block(s) for filtering. Detailed below.
�
stateB" �Allows to filter list of Availability Zones based on their
current state. Can be either `"available"`, `"information"`, `"impaired"` or
`"unavailable"`. By default the list includes a complete set of Availability Zones
to which the underlying AWS account has access, regardless of their state.
"
allAvailabilityZonesB
 "
excludeNamesB*" "
excludeZoneIdsB*" "p
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter"�

groupNames*" �A set of the Availability Zone Group names. For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"M
names*" >List of the Availability Zone names available to the account.
"
stateB" "M
zoneIds*" <List of the Availability Zone IDs available to the account.
2�[
WgetBillingServiceAccount;aws:index/getBillingServiceAccount:getBillingServiceAccount�YUse this data source to get the Account ID of the [AWS Billing and Cost Management Service Account](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-getting-started.html#step-2) for the purpose of permitting in S3 bucket policy.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.getBillingServiceAccount({});
const billingLogs = new aws.s3.BucketV2("billing_logs", {bucket: "my-billing-tf-test-bucket"});
const billingLogsAcl = new aws.s3.BucketAclV2("billing_logs_acl", {
    bucket: billingLogs.id,
    acl: "private",
});
const allowBillingLogging = pulumi.all([main, billingLogs.arn, main, billingLogs.arn]).apply(([main, billingLogsArn, main1, billingLogsArn1]) => aws.iam.getPolicyDocumentOutput({
    statements: [
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main.arn],
            }],
            actions: [
                "s3:GetBucketAcl",
                "s3:GetBucketPolicy",
            ],
            resources: [billingLogsArn],
        },
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main1.arn],
            }],
            actions: ["s3:PutObject"],
            resources: [`${billingLogsArn1}/*`],
        },
    ],
}));
const allowBillingLoggingBucketPolicy = new aws.s3.BucketPolicy("allow_billing_logging", {
    bucket: billingLogs.id,
    policy: allowBillingLogging.apply(allowBillingLogging => allowBillingLogging.json),
});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.get_billing_service_account()
billing_logs = aws.s3.BucketV2("billing_logs", bucket="my-billing-tf-test-bucket")
billing_logs_acl = aws.s3.BucketAclV2("billing_logs_acl",
    bucket=billing_logs.id,
    acl="private")
allow_billing_logging = pulumi.Output.all(
    billingLogsArn=billing_logs.arn,
    billingLogsArn1=billing_logs.arn
).apply(lambda resolved_outputs: aws.iam.get_policy_document_output(statements=[
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": [
            "s3:GetBucketAcl",
            "s3:GetBucketPolicy",
        ],
        "resources": [resolved_outputs['billingLogsArn']],
    },
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": ["s3:PutObject"],
        "resources": [f"{resolved_outputs['billingLogsArn1']}/*"],
    },
]))

allow_billing_logging_bucket_policy = aws.s3.BucketPolicy("allow_billing_logging",
    bucket=billing_logs.id,
    policy=allow_billing_logging.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.GetBillingServiceAccount.Invoke();

    var billingLogs = new Aws.S3.BucketV2("billing_logs", new()
    {
        Bucket = "my-billing-tf-test-bucket",
    });

    var billingLogsAcl = new Aws.S3.BucketAclV2("billing_logs_acl", new()
    {
        Bucket = billingLogs.Id,
        Acl = "private",
    });

    var allowBillingLogging = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:GetBucketAcl",
                    "s3:GetBucketPolicy",
                },
                Resources = new[]
                {
                    billingLogs.Arn,
                },
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:PutObject",
                },
                Resources = new[]
                {
                    $"{billingLogs.Arn}/*",
                },
            },
        },
    });

    var allowBillingLoggingBucketPolicy = new Aws.S3.BucketPolicy("allow_billing_logging", new()
    {
        Bucket = billingLogs.Id,
        Policy = allowBillingLogging.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
main, err := aws.GetBillingServiceAccount(ctx, &aws.GetBillingServiceAccountArgs{
}, nil);
if err != nil {
return err
}
billingLogs, err := s3.NewBucketV2(ctx, "billing_logs", &s3.BucketV2Args{
Bucket: pulumi.String("my-billing-tf-test-bucket"),
})
if err != nil {
return err
}
_, err = s3.NewBucketAclV2(ctx, "billing_logs_acl", &s3.BucketAclV2Args{
Bucket: billingLogs.ID(),
Acl: pulumi.String("private"),
})
if err != nil {
return err
}
allowBillingLogging := pulumi.All(billingLogs.Arn,billingLogs.Arn).ApplyT(func(_args []interface{}) (iam.GetPolicyDocumentResult, error) {
billingLogsArn := _args[0].(string)
billingLogsArn1 := _args[1].(string)
return iam.GetPolicyDocumentResult(interface{}(iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:GetBucketAcl",
"s3:GetBucketPolicy",
},
Resources: []string{
billingLogsArn,
},
},
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:PutObject",
},
Resources: []string{
fmt.Sprintf("%v/*", billingLogsArn1),
},
},
},
}, nil))), nil
}).(iam.GetPolicyDocumentResultOutput)
_, err = s3.NewBucketPolicy(ctx, "allow_billing_logging", &s3.BucketPolicyArgs{
Bucket: billingLogs.ID(),
Policy: pulumi.String(allowBillingLogging.ApplyT(func(allowBillingLogging iam.GetPolicyDocumentResult) (*string, error) {
return &allowBillingLogging.Json, nil
}).(pulumi.StringPtrOutput)),
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetBillingServiceAccountArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketAclV2;
import com.pulumi.aws.s3.BucketAclV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.s3.BucketPolicy;
import com.pulumi.aws.s3.BucketPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var main = AwsFunctions.getBillingServiceAccount();

        var billingLogs = new BucketV2("billingLogs", BucketV2Args.builder()
            .bucket("my-billing-tf-test-bucket")
            .build());

        var billingLogsAcl = new BucketAclV2("billingLogsAcl", BucketAclV2Args.builder()
            .bucket(billingLogs.id())
            .acl("private")
            .build());

        final var allowBillingLogging = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(            
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions(                    
                        "s3:GetBucketAcl",
                        "s3:GetBucketPolicy")
                    .resources(billingLogs.arn())
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions("s3:PutObject")
                    .resources(billingLogs.arn().applyValue(arn -> String.format("%s/*", arn)))
                    .build())
            .build());

        var allowBillingLoggingBucketPolicy = new BucketPolicy("allowBillingLoggingBucketPolicy", BucketPolicyArgs.builder()
            .bucket(billingLogs.id())
            .policy(allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(allowBillingLogging -> allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  billingLogs:
    type: aws:s3:BucketV2
    name: billing_logs
    properties:
      bucket: my-billing-tf-test-bucket
  billingLogsAcl:
    type: aws:s3:BucketAclV2
    name: billing_logs_acl
    properties:
      bucket: ${billingLogs.id}
      acl: private
  allowBillingLoggingBucketPolicy:
    type: aws:s3:BucketPolicy
    name: allow_billing_logging
    properties:
      bucket: ${billingLogs.id}
      policy: ${allowBillingLogging.json}
variables:
  main:
    fn::invoke:
      function: aws:getBillingServiceAccount
      arguments: {}
  allowBillingLogging:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:GetBucketAcl
              - s3:GetBucketPolicy
            resources:
              - ${billingLogs.arn}
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:PutObject
            resources:
              - ${billingLogs.arn}/*
```
<!--End PulumiCodeChooser -->
3
idB" 'ID of the AWS billing service account.
"3
arn" (ARN of the AWS billing service account.
"1
id" 'ID of the AWS billing service account.
2�
BgetCallerIdentity-aws:index/getCallerIdentity:getCallerIdentity�Use this data source to get the access to the effective Account ID, User ID, and ARN in
which this provider is authorized.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
export const accountId = current.then(current => current.accountId);
export const callerArn = current.then(current => current.arn);
export const callerUser = current.then(current => current.userId);
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_caller_identity()
pulumi.export("accountId", current.account_id)
pulumi.export("callerArn", current.arn)
pulumi.export("callerUser", current.user_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    return new Dictionary<string, object?>
    {
        ["accountId"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
        ["callerArn"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.Arn),
        ["callerUser"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.UserId),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		ctx.Export("accountId", current.AccountId)
		ctx.Export("callerArn", current.Arn)
		ctx.Export("callerUser", current.UserId)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getCallerIdentity();

        ctx.export("accountId", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()));
        ctx.export("callerArn", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.arn()));
        ctx.export("callerUser", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.userId()));
    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
outputs:
  accountId: ${current.accountId}
  callerArn: ${current.arn}
  callerUser: ${current.userId}
```
<!--End PulumiCodeChooser -->
W
idB" KAccount ID number of the account that owns or contains the calling entity.
"`
	accountId" OAWS Account ID number of the account that owns or contains the calling entity.
"3
arn" (ARN associated with the calling entity.
"U
id" KAccount ID number of the account that owns or contains the calling entity.
"7
userId" )Unique identifier of the calling entity.
2�
9getDefaultTags'aws:index/getDefaultTags:getDefaultTags�Use this data source to get the default tags configured on the provider.

With this data source, you can apply default tags to resources not _directly_ managed by a resource, such as the instances underneath an Auto Scaling group or the volumes created for an EC2 instance.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetDefaultTags(ctx, &aws.GetDefaultTagsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetDefaultTagsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getDefaultTags();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getDefaultTags
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Dynamically Apply Default Tags to Auto Scaling Group

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
const exampleGroup = new aws.autoscaling.Group("example", {tags: .map(entry => ({
    key: entry.key,
    value: entry.value,
    propagateAtLaunch: true,
}))});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
example_group = aws.autoscaling.Group("example", tags=[{
    "key": entry["key"],
    "value": entry["value"],
    "propagate_at_launch": True,
} for entry in [{"key": k, "value": v} for k, v in example.tags]])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

    var exampleGroup = new Aws.AutoScaling.Group("example", new()
    {
        Tags = ,
    });

});
```
<!--End PulumiCodeChooser -->


idB" "
id" ":
tags2" ,Key-value mapping of provider default tags.
2�;
0getIpRanges!aws:index/getIpRanges:getIpRanges�1Use this data source to get the IP ranges of various AWS products and services. For more information about the contents of this data source and required JSON syntax if referencing a custom URL, see the [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const europeanEc2 = aws.getIpRanges({
    regions: [
        "eu-west-1",
        "eu-central-1",
    ],
    services: ["ec2"],
});
const fromEurope = new aws.ec2.SecurityGroup("from_europe", {
    name: "from_europe",
    ingress: [{
        fromPort: 443,
        toPort: 443,
        protocol: "tcp",
        cidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.cidrBlocks),
        ipv6CidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.ipv6CidrBlocks),
    }],
    tags: {
        CreateDate: europeanEc2.then(europeanEc2 => europeanEc2.createDate),
        SyncToken: europeanEc2.then(europeanEc2 => europeanEc2.syncToken),
    },
});
```
```python
import pulumi
import pulumi_aws as aws

european_ec2 = aws.get_ip_ranges(regions=[
        "eu-west-1",
        "eu-central-1",
    ],
    services=["ec2"])
from_europe = aws.ec2.SecurityGroup("from_europe",
    name="from_europe",
    ingress=[{
        "from_port": 443,
        "to_port": 443,
        "protocol": "tcp",
        "cidr_blocks": european_ec2.cidr_blocks,
        "ipv6_cidr_blocks": european_ec2.ipv6_cidr_blocks,
    }],
    tags={
        "CreateDate": european_ec2.create_date,
        "SyncToken": european_ec2.sync_token,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var europeanEc2 = Aws.GetIpRanges.Invoke(new()
    {
        Regions = new[]
        {
            "eu-west-1",
            "eu-central-1",
        },
        Services = new[]
        {
            "ec2",
        },
    });

    var fromEurope = new Aws.Ec2.SecurityGroup("from_europe", new()
    {
        Name = "from_europe",
        Ingress = new[]
        {
            new Aws.Ec2.Inputs.SecurityGroupIngressArgs
            {
                FromPort = 443,
                ToPort = 443,
                Protocol = "tcp",
                CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CidrBlocks),
                Ipv6CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.Ipv6CidrBlocks),
            },
        },
        Tags = 
        {
            { "CreateDate", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CreateDate) },
            { "SyncToken", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.SyncToken) },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		europeanEc2, err := aws.GetIpRanges(ctx, &aws.GetIpRangesArgs{
			Regions: []string{
				"eu-west-1",
				"eu-central-1",
			},
			Services: []string{
				"ec2",
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = ec2.NewSecurityGroup(ctx, "from_europe", &ec2.SecurityGroupArgs{
			Name: pulumi.String("from_europe"),
			Ingress: ec2.SecurityGroupIngressArray{
				&ec2.SecurityGroupIngressArgs{
					FromPort:       pulumi.Int(443),
					ToPort:         pulumi.Int(443),
					Protocol:       pulumi.String("tcp"),
					CidrBlocks:     interface{}(europeanEc2.CidrBlocks),
					Ipv6CidrBlocks: interface{}(europeanEc2.Ipv6CidrBlocks),
				},
			},
			Tags: pulumi.StringMap{
				"CreateDate": pulumi.String(europeanEc2.CreateDate),
				"SyncToken":  pulumi.Int(europeanEc2.SyncToken),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetIpRangesArgs;
import com.pulumi.aws.ec2.SecurityGroup;
import com.pulumi.aws.ec2.SecurityGroupArgs;
import com.pulumi.aws.ec2.inputs.SecurityGroupIngressArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var europeanEc2 = AwsFunctions.getIpRanges(GetIpRangesArgs.builder()
            .regions(            
                "eu-west-1",
                "eu-central-1")
            .services("ec2")
            .build());

        var fromEurope = new SecurityGroup("fromEurope", SecurityGroupArgs.builder()
            .name("from_europe")
            .ingress(SecurityGroupIngressArgs.builder()
                .fromPort("443")
                .toPort("443")
                .protocol("tcp")
                .cidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.cidrBlocks()))
                .ipv6CidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.ipv6CidrBlocks()))
                .build())
            .tags(Map.ofEntries(
                Map.entry("CreateDate", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.createDate())),
                Map.entry("SyncToken", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.syncToken()))
            ))
            .build());

    }
}
```
```yaml
resources:
  fromEurope:
    type: aws:ec2:SecurityGroup
    name: from_europe
    properties:
      name: from_europe
      ingress:
        - fromPort: '443'
          toPort: '443'
          protocol: tcp
          cidrBlocks: ${europeanEc2.cidrBlocks}
          ipv6CidrBlocks: ${europeanEc2.ipv6CidrBlocks}
      tags:
        CreateDate: ${europeanEc2.createDate}
        SyncToken: ${europeanEc2.syncToken}
variables:
  europeanEc2:
    fn::invoke:
      function: aws:getIpRanges
      arguments:
        regions:
          - eu-west-1
          - eu-central-1
        services:
          - ec2
```
<!--End PulumiCodeChooser -->


idB" �
regionsB*" �Filter IP ranges by regions (or include all regions, if
omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
(e.g., `eu-central-1`)
�
services*" �Filter IP ranges by services. Valid items are `amazon`
(for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
`codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
`route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
[`service` attribute][2] documentation for other possible values.

> **NOTE:** If the specified combination of regions and services does not yield any
CIDR blocks, this call will fail.
�
urlB" �Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
";

cidrBlocks*" 'Lexically ordered list of CIDR blocks.
"S

createDate" APublication time of the IP ranges (e.g., `2016-08-03-23-46-05`).
"
id" "D
ipv6CidrBlocks*" ,Lexically ordered list of IPv6 CIDR blocks.
"
regionsB*" "
services*" "d
	syncToken SPublication time of the IP ranges, in Unix epoch time format
(e.g., `1470267965`).
"
urlB" 2�"
3getPartition#aws:index/getPartition:getPartition�Use this data source to lookup information about the current AWS partition in
which the provider is working.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getPartition({});
const s3Policy = current.then(current => aws.iam.getPolicyDocument({
    statements: [{
        sid: "1",
        actions: ["s3:ListBucket"],
        resources: [`arn:${current.partition}:s3:::my-bucket`],
    }],
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_partition()
s3_policy = aws.iam.get_policy_document(statements=[{
    "sid": "1",
    "actions": ["s3:ListBucket"],
    "resources": [f"arn:{current.partition}:s3:::my-bucket"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetPartition.Invoke();

    var s3Policy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "1",
                Actions = new[]
                {
                    "s3:ListBucket",
                },
                Resources = new[]
                {
                    $"arn:{current.Apply(getPartitionResult => getPartitionResult.Partition)}:s3:::my-bucket",
                },
            },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid: pulumi.StringRef("1"),
					Actions: []string{
						"s3:ListBucket",
					},
					Resources: []string{
						fmt.Sprintf("arn:%v:s3:::my-bucket", current.Partition),
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getPartition();

        final var s3Policy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("1")
                .actions("s3:ListBucket")
                .resources(String.format("arn:%s:s3:::my-bucket", current.applyValue(getPartitionResult -> getPartitionResult.partition())))
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
  s3Policy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: '1'
            actions:
              - s3:ListBucket
            resources:
              - arn:${current.partition}:s3:::my-bucket
```
<!--End PulumiCodeChooser -->
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"�
	dnsSuffix" {Base DNS domain name for the current partition (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"m
	partition" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"|
reverseDnsPrefix" dPrefix of service names (e.g., `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
2�
*	getRegionaws:index/getRegion:getRegion�`aws.getRegion` provides details about a specific AWS region.

As well as validating a given region name this resource can be used to
discover the name of the region configured within the provider. The latter
can be useful in a child module which is inheriting an AWS provider
configuration from its parent module.

## Example Usage

The following example shows how the resource might be used to obtain
the name of the AWS region configured on the provider.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->
8
endpointB" &EC2 endpoint of the region to select.


idB" 1
nameB" #Full name of the region to select.
"R
description" ?Region's description in this format: "Location (Region name)".
"6
endpoint" &EC2 endpoint for the selected region.
"
id" ")
name" Name of the selected region.
2�4
-
getRegionsaws:index/getRegions:getRegions�/Provides information about AWS Regions. Can be used to filter regions i.e., by Opt-In status or only regions enabled for current account. To get details like endpoint and description of each region the data source can be combined with the `aws.getRegion` data source.

## Example Usage

Enabled AWS Regions:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments: {}
```
<!--End PulumiCodeChooser -->

All the regions regardless of the availability

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
```
<!--End PulumiCodeChooser -->

To see regions that are filtered by `"not-opted-in"`, the `all_regions` argument needs to be set to `true` or no results will be returned.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
    filters: [{
        name: "opt-in-status",
        values: ["not-opted-in"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True,
    filters=[{
        "name": "opt-in-status",
        "values": ["not-opted-in"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
        Filters = new[]
        {
            new Aws.Inputs.GetRegionsFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
			Filters: []aws.GetRegionsFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .filters(GetRegionsFilterArgs.builder()
                .name("opt-in-status")
                .values("not-opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
```
<!--End PulumiCodeChooser -->
Z

allRegionsB
 FIf true the source will query all regions regardless of availability.
�
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter:Configuration block(s) to use as filters. Detailed below.
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"

allRegionsB
 "R
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"9
names*" *Names of regions that meets the criteria.
2�9
-
getServiceaws:index/getService:getService�0Use this data source to compose and decompose AWS service DNS names.

## Example Usage

### Get Service DNS Name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const test = current.then(current => aws.getService({
    region: current.name,
    serviceId: "ec2",
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
test = aws.get_service(region=current.name,
    service_id="ec2")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var test = Aws.GetService.Invoke(new()
    {
        Region = current.Apply(getRegionResult => getRegionResult.Name),
        ServiceId = "ec2",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetService(ctx, &aws.GetServiceArgs{
			Region:    pulumi.StringRef(current.Name),
			ServiceId: pulumi.StringRef("ec2"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

        final var test = AwsFunctions.getService(GetServiceArgs.builder()
            .region(current.applyValue(getRegionResult -> getRegionResult.name()))
            .serviceId("ec2")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  test:
    fn::invoke:
      function: aws:getService
      arguments:
        region: ${current.name}
        serviceId: ec2
```
<!--End PulumiCodeChooser -->

### Use Service Reverse DNS Name to Get Components

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "cn.com.amazonaws.cn-north-1.s3",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="cn.com.amazonaws.cn-north-1.s3")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "cn.com.amazonaws.cn-north-1.s3",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("cn.com.amazonaws.cn-north-1.s3"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("cn.com.amazonaws.cn-north-1.s3")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: cn.com.amazonaws.cn-north-1.s3
```
<!--End PulumiCodeChooser -->

### Determine Regional Support for a Service

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "com.amazonaws.us-gov-west-1.waf",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="com.amazonaws.us-gov-west-1.waf")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "com.amazonaws.us-gov-west-1.waf",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("com.amazonaws.us-gov-west-1.waf"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("com.amazonaws.us-gov-west-1.waf")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: com.amazonaws.us-gov-west-1.waf
```
<!--End PulumiCodeChooser -->
�
dnsNameB" �DNS name of the service (_e.g.,_ `rds.us-east-1.amazonaws.com`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.


idB" O
regionB" ?Region of the service (_e.g.,_ `us-west-2`, `ap-northeast-1`).
�
reverseDnsNameB" �Reverse DNS name of the service (_e.g.,_ `com.amazonaws.us-west-2.s3`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
~
reverseDnsPrefixB" dPrefix of the service (_e.g.,_ `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
�
	serviceIdB" �Service endpoint ID (_e.g.,_ `s3`, `rds`, `ec2`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required. A service's endpoint ID can be found in the [_AWS General Reference_](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
"
dnsName" "
id" "
	partition" "
region" "
reverseDnsName" "
reverseDnsPrefix" "
	serviceId" "�
	supported
 uWhether the service is supported in the region's partition. New services may not be listed immediately as supported.
2�
HgetServicePrincipal1aws:index/getServicePrincipal:getServicePrincipal�Use this data source to create a Service Principal Name for a service in a given region. Service Principal Names should always end in the standard global format: `{servicename}.amazonaws.com`. However, in some AWS partitions, AWS may expect a different format.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getServicePrincipal({
    serviceName: "s3",
});
const test = aws.getServicePrincipal({
    serviceName: "s3",
    region: "us-iso-east-1",
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_service_principal(service_name="s3")
test = aws.get_service_principal(service_name="s3",
    region="us-iso-east-1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
    });

    var test = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
        Region = "us-iso-east-1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
		}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
			Region:      pulumi.StringRef("us-iso-east-1"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServicePrincipalArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .build());

        final var test = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .region("us-iso-east-1")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
  test:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
        region: us-iso-east-1
```
<!--End PulumiCodeChooser -->
T
regionB" DRegion you'd like the SPN for. By default, uses the current region.
Z
serviceName" GName of the service you want to generate a Service Principal Name for.
"�
id" �Identifier of the current Service Principal (compound of service, region and suffix). (e.g. `logs.us-east-1.amazonaws.com`in AWS Commercial, `logs.cn-north-1.amazonaws.com.cn` in AWS China).
"y
name" mService Principal Name (e.g., `logs.amazonaws.com` in AWS Commercial, `logs.amazonaws.com.cn` in AWS China).
"w
region" iRegion identifier of the generated SPN (e.g., `us-east-1` in AWS Commercial, `cn-north-1` in AWS China).
"
serviceName" "l
suffix" ^Suffix of the SPN (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
2�
_
codeguruprofilergetProfilingGroup8aws:codeguruprofiler/getProfilingGroup:getProfilingGroup�Data source for managing an AWS CodeGuru Profiler Profiling Group.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.codeguruprofiler.getProfilingGroup({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codeguruprofiler.get_profiling_group(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CodeGuruProfiler.GetProfilingGroup.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codeguruprofiler"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := codeguruprofiler.LookupProfilingGroup(ctx, &codeguruprofiler.LookupProfilingGroupArgs{
			Name: "example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codeguruprofiler.CodeguruprofilerFunctions;
import com.pulumi.aws.codeguruprofiler.inputs.GetProfilingGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = CodeguruprofilerFunctions.getProfilingGroup(GetProfilingGroupArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:codeguruprofiler:getProfilingGroup
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->
-
name" !The name of the profiling group.
"�
agentOrchestrationConfigs�*�:�
�
codeguruprofiler)getProfilingGroupAgentOrchestrationConfighaws:codeguruprofiler/getProfilingGroupAgentOrchestrationConfig:getProfilingGroupAgentOrchestrationConfig+Profiling Group agent orchestration config
"'
arn" ARN of the Profiling Group.
"D
computePlatform" -The compute platform of the profiling group.
"=
	createdAt" ,Timestamp when Profiling Group was created.
"
id" "

name" "�
profilingStatuses�*�:�
�
codeguruprofiler getProfilingGroupProfilingStatusVaws:codeguruprofiler/getProfilingGroupProfilingStatus:getProfilingGroupProfilingStatus#The status of the Profiling Group.
":
tags2" ,Mapping of Key-Value tags for the resource.
"=
	updatedAt" ,Timestamp when Profiling Group was updated.
2�*
Y
codestarconnectionsgetConnection3aws:codestarconnections/getConnection:getConnection�!Provides details about CodeStar Connection.

## Example Usage

### By ARN

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.codestarconnections.getConnection({
    arn: exampleAwsCodestarconnectionsConnection.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codestarconnections.get_connection(arn=example_aws_codestarconnections_connection["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CodeStarConnections.GetConnection.Invoke(new()
    {
        Arn = exampleAwsCodestarconnectionsConnection.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codestarconnections"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := codestarconnections.LookupConnection(ctx, &codestarconnections.LookupConnectionArgs{
			Arn: pulumi.StringRef(exampleAwsCodestarconnectionsConnection.Arn),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codestarconnections.CodestarconnectionsFunctions;
import com.pulumi.aws.codestarconnections.inputs.GetConnectionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = CodestarconnectionsFunctions.getConnection(GetConnectionArgs.builder()
            .arn(exampleAwsCodestarconnectionsConnection.arn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:codestarconnections:getConnection
      arguments:
        arn: ${exampleAwsCodestarconnectionsConnection.arn}
```
<!--End PulumiCodeChooser -->

### By Name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.codestarconnections.getConnection({
    name: exampleAwsCodestarconnectionsConnection.name,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.codestarconnections.get_connection(name=example_aws_codestarconnections_connection["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CodeStarConnections.GetConnection.Invoke(new()
    {
        Name = exampleAwsCodestarconnectionsConnection.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/codestarconnections"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := codestarconnections.LookupConnection(ctx, &codestarconnections.LookupConnectionArgs{
			Name: pulumi.StringRef(exampleAwsCodestarconnectionsConnection.Name),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.codestarconnections.CodestarconnectionsFunctions;
import com.pulumi.aws.codestarconnections.inputs.GetConnectionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = CodestarconnectionsFunctions.getConnection(GetConnectionArgs.builder()
            .name(exampleAwsCodestarconnectionsConnection.name())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:codestarconnections:getConnection
      arguments:
        name: ${exampleAwsCodestarconnectionsConnection.name}
```
<!--End PulumiCodeChooser -->
&
arnB" CodeStar Connection ARN.
w
nameB" iCodeStar Connection name.

> **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence.
O
tagsB2" ?Map of key-value resource tags to associate with the resource.
"	
arn" "l
connectionStatus" TCodeStar Connection status. Possible values are `PENDING`, `AVAILABLE` and `ERROR`.
"?
hostArn" 0ARN of the host associated with the connection.
"E
id" ;The provider-assigned unique ID for this managed resource.
"\
name" PName of the CodeStar Connection. The name is unique in the calling AWS account.
"�
providerType" �Name of the external provider where your third-party code repository is configured. Possible values are `Bitbucket`, `GitHub` and `GitLab`. For connections to GitHub Enterprise Server or GitLab Self-Managed instances, you must create an aws.codestarconnections.Host resource and use `host_arn` instead.
"M
tags2" ?Map of key-value resource tags to associate with the resource.
2�
G
cognitogetIdentityPool+aws:cognito/getIdentityPool:getIdentityPool�Data source for managing an AWS Cognito Identity Pool.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cognito.getIdentityPool({
    identityPoolName: "test pool",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.get_identity_pool(identity_pool_name="test pool")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Cognito.GetIdentityPool.Invoke(new()
    {
        IdentityPoolName = "test pool",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.LookupIdentityPool(ctx, &cognito.LookupIdentityPoolArgs{
			IdentityPoolName: "test pool",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetIdentityPoolArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = CognitoFunctions.getIdentityPool(GetIdentityPoolArgs.builder()
            .identityPoolName("test pool")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cognito:getIdentityPool
      arguments:
        identityPoolName: test pool
```
<!--End PulumiCodeChooser -->
8
identityPoolName"  The Cognito Identity Pool name.
@
tagsB2" 0A map of tags to assigned to the Identity Pool.
"T
allowClassicFlow
 <Whether the classic / basic authentication flow is enabled.
"h
allowUnauthenticatedIdentities
 BWhether the identity pool supports unauthenticated logins or not.
"
arn" ARN of the Pool.
"�
cognitoIdentityProviders�*�:�
�
cognito&getIdentityPoolCognitoIdentityProviderYaws:cognito/getIdentityPoolCognitoIdentityProvider:getIdentityPoolCognitoIdentityProviderEAn array of Amazon Cognito Identity user pools and their client IDs.
"U
developerProviderName" 8The "domain" by which Cognito will refer to your users.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
identityPoolName" "I
openidConnectProviderArns*" &Set of OpendID Connect provider ARNs.
"k
samlProviderArns*" QAn array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
"]
supportedLoginProviders2" <Key-Value pairs mapping provider names to provider app IDs.
">
tags2" 0A map of tags to assigned to the Identity Pool.
2�
>
cognitogetUserGroup%aws:cognito/getUserGroup:getUserGroup�Data source for managing an AWS Cognito IDP (Identity Provider) User Group.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cognito.getUserGroup({
    userPoolId: "us-west-2_aaaaaaaaa",
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.get_user_group(user_pool_id="us-west-2_aaaaaaaaa",
    name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Cognito.GetUserGroup.Invoke(new()
    {
        UserPoolId = "us-west-2_aaaaaaaaa",
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.LookupUserGroup(ctx, &cognito.LookupUserGroupArgs{
			UserPoolId: "us-west-2_aaaaaaaaa",
			Name:       "example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetUserGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = CognitoFunctions.getUserGroup(GetUserGroupArgs.builder()
            .userPoolId("us-west-2_aaaaaaaaa")
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cognito:getUserGroup
      arguments:
        userPoolId: us-west-2_aaaaaaaaa
        name: example
```
<!--End PulumiCodeChooser -->
$
name" Name of the user group.
3

userPoolId" !User pool the client belongs to.
"2
description" Description of the user group.
"L
id" BA comma-delimited string concatenating `name` and `user_pool_id`.
"

name" "0

precedence Precedence of the user group.
"I
roleArn" :ARN of the IAM role to be associated with the user group.
"

userPoolId" 2�
A
cognitogetUserGroups'aws:cognito/getUserGroups:getUserGroups�Data source for managing AWS Cognito IDP (Identity Provider) User Groups.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cognito.getUserGroups({
    userPoolId: "us-west-2_aaaaaaaaa",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.get_user_groups(user_pool_id="us-west-2_aaaaaaaaa")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Cognito.GetUserGroups.Invoke(new()
    {
        UserPoolId = "us-west-2_aaaaaaaaa",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.GetUserGroups(ctx, &cognito.GetUserGroupsArgs{
			UserPoolId: "us-west-2_aaaaaaaaa",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetUserGroupsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = CognitoFunctions.getUserGroups(GetUserGroupsArgs.builder()
            .userPoolId("us-west-2_aaaaaaaaa")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cognito:getUserGroups
      arguments:
        userPoolId: us-west-2_aaaaaaaaa
```
<!--End PulumiCodeChooser -->
3

userPoolId" !User pool the client belongs to.
"�
groupsV*T:R
P
cognitogetUserGroupsGroup1aws:cognito/getUserGroupsGroup:getUserGroupsGroup$List of groups. See `groups` below.
" 
id" User pool identifier.
"

userPoolId" 2�+
;
cognitogetUserPool#aws:cognito/getUserPool:getUserPool�Data source for managing an AWS Cognito User Pool.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cognito.getUserPool({
    userPoolId: "us-west-2_aaaaaaaaa",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cognito.get_user_pool(user_pool_id="us-west-2_aaaaaaaaa")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Cognito.GetUserPool.Invoke(new()
    {
        UserPoolId = "us-west-2_aaaaaaaaa",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.LookupUserPool(ctx, &cognito.LookupUserPoolArgs{
			UserPoolId: "us-west-2_aaaaaaaaa",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetUserPoolArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = CognitoFunctions.getUserPool(GetUserPoolArgs.builder()
            .userPoolId("us-west-2_aaaaaaaaa")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cognito:getUserPool
      arguments:
        userPoolId: us-west-2_aaaaaaaaa
```
<!--End PulumiCodeChooser -->
&

userPoolId" The cognito pool ID
"�
accountRecoverySettings�*�:
}
cognito!getUserPoolAccountRecoverySettingOaws:cognito/getUserPoolAccountRecoverySetting:getUserPoolAccountRecoverySetting"�
adminCreateUserConfigs�*~:|
z
cognito getUserPoolAdminCreateUserConfigMaws:cognito/getUserPoolAdminCreateUserConfig:getUserPoolAdminCreateUserConfig"�
arn" �ARN of the User Pool.
* account_recovery_setting - The available verified method a user can use to recover their password when they call ForgotPassword. You can use this setting to define a preferred method when a user has more than one method available. With this setting, SMS doesn't qualify for a valid password recovery mechanism if the user also has SMS multi-factor authentication (MFA) activated. In the absence of this setting, Amazon Cognito uses the legacy behavior to determine the recovery method where SMS is preferred through email.
* admin_create_user_config - The configuration for AdminCreateUser requests.
"V
autoVerifiedAttributes*" 6The attributes that are auto-verified in a user pool.
"V
creationDate" BThe date and time, in ISO 8601 format, when the item was created.
"�
customDomain" �A custom domain name that you provide to Amazon Cognito. This parameter applies only if you use a custom domain to host the sign-up and sign-in pages for your application. An example of a custom domain name might be auth.example.com.
"�
deletionProtection" �When active, DeletionProtection prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature.
* device_configuration - The device-remembering configuration for a user pool. A null value indicates that you have deactivated device remembering in your user pool.
"�
deviceConfigurationsz*x:v
t
cognitogetUserPoolDeviceConfigurationIaws:cognito/getUserPoolDeviceConfiguration:getUserPoolDeviceConfiguration"�
domain" �The domain prefix, if the user pool has a domain associated with it.
* email_configuration - The email configuration of your user pool. The email configuration type sets your preferred sending method, AWS Region, and sender for messages from your user pool.
"�
emailConfigurationsw*u:s
q
cognitogetUserPoolEmailConfigurationGaws:cognito/getUserPoolEmailConfiguration:getUserPoolEmailConfiguration"�
estimatedNumberOfUsers xA number estimating the size of the user pool.
* lambda_config - The AWS Lambda triggers associated with the user pool.
"
id" "v
lambdaConfigse*c:a
_
cognitogetUserPoolLambdaConfig;aws:cognito/getUserPoolLambdaConfig:getUserPoolLambdaConfig"[
lastModifiedDate" CThe date and time, in ISO 8601 format, when the item was modified.
"V
mfaConfiguration" >Can be one of the following values: `OFF` | `ON` | `OPTIONAL`
"%
name" - Name of the attribute.
"�
schemaAttributesn*l:j
h
cognitogetUserPoolSchemaAttributeAaws:cognito/getUserPoolSchemaAttribute:getUserPoolSchemaAttribute"P
smsAuthenticationMessage" 0The contents of the SMS authentication message.
"k
smsConfigurationFailure" LThe reason why the SMS configuration can't send the messages to your users.
"N
smsVerificationMessage" 0The contents of the SMS authentication message.
"4
tags2" &Map of tags assigned to the resource.
"

userPoolId" "I
userPoolTags2" 3(Deprecated) Map of tags assigned to the resource.
"
usernameAttributes*" cSpecifies whether a user can use an email address or phone number as a username when they sign up.
2�.
M
cognitogetUserPoolClient/aws:cognito/getUserPoolClient:getUserPoolClient�Provides a Cognito User Pool Client resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const client = aws.cognito.getUserPoolClient({
    clientId: "38fjsnc484p94kpqsnet7mpld0",
    userPoolId: "us-west-2_aaaaaaaaa",
});
```
```python
import pulumi
import pulumi_aws as aws

client = aws.cognito.get_user_pool_client(client_id="38fjsnc484p94kpqsnet7mpld0",
    user_pool_id="us-west-2_aaaaaaaaa")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var client = Aws.Cognito.GetUserPoolClient.Invoke(new()
    {
        ClientId = "38fjsnc484p94kpqsnet7mpld0",
        UserPoolId = "us-west-2_aaaaaaaaa",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.LookupUserPoolClient(ctx, &cognito.LookupUserPoolClientArgs{
			ClientId:   "38fjsnc484p94kpqsnet7mpld0",
			UserPoolId: "us-west-2_aaaaaaaaa",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetUserPoolClientArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var client = CognitoFunctions.getUserPoolClient(GetUserPoolClientArgs.builder()
            .clientId("38fjsnc484p94kpqsnet7mpld0")
            .userPoolId("us-west-2_aaaaaaaaa")
            .build());

    }
}
```
```yaml
variables:
  client:
    fn::invoke:
      function: aws:cognito:getUserPoolClient
      arguments:
        clientId: 38fjsnc484p94kpqsnet7mpld0
        userPoolId: us-west-2_aaaaaaaaa
```
<!--End PulumiCodeChooser -->
,
clientId" Client Id of the user pool.
3

userPoolId" !User pool the client belongs to.
"�
accessTokenValidity �(Optional) Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. This value will be overridden if you have entered a value in `token_validity_units`.
"h
allowedOauthFlows*" M(Optional) List of allowed OAuth flows (code, implicit, client_credentials).
"�
allowedOauthFlowsUserPoolClient
 p(Optional) Whether the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.
"�
allowedOauthScopes*" l(Optional) List of allowed OAuth scopes (phone, email, openid, profile, and aws.cognito.signin.user.admin).
"�
analyticsConfigurations�*�:�
�
cognito'getUserPoolClientAnalyticsConfiguration[aws:cognito/getUserPoolClientAnalyticsConfiguration:getUserPoolClientAnalyticsConfigurationx(Optional) Configuration block for Amazon Pinpoint analytics for collecting metrics for this user pool. Detailed below.
"[
callbackUrls*" E(Optional) List of allowed callback URLs for the identity providers.
"
clientId" ";
clientSecret" 'Client secret of the user pool client.
"a
defaultRedirectUri" G(Optional) Default redirect URI. Must be in the list of callback URLs.
".
(enablePropagateAdditionalUserContextData
 "N
enableTokenRevocation
 1(Optional) Enables or disables token revocation.
"�
explicitAuthFlows*" �(Optional) List of authentication flows (ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, ALLOW_REFRESH_TOKEN_AUTH).
"L
generateSecret
 6(Optional) Should an application secret be generated.
"E
id" ;The provider-assigned unique ID for this managed resource.
"�
idTokenValidity �(Optional) Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. This value will be overridden if you have entered a value in `token_validity_units`.
"W

logoutUrls*" C(Optional) List of allowed logout URLs for the identity providers.
"

name" "�
preventUserExistenceErrors" �(Optional) Choose which errors and responses are returned by Cognito APIs during authentication, account confirmation, and password recovery when the user does not exist in the user pool. When set to `ENABLED` and the user does not exist, authentication returns an error indicating either the username or password was incorrect, and account confirmation and password recovery return a response indicating a code was sent to a simulated destination. When set to `LEGACY`, those APIs will return a `UserNotFoundException` exception if the user does not exist in the user pool.
"f
readAttributes*" N(Optional) List of user pool attributes the application client can read from.
"X
refreshTokenValidity <(Optional) Time limit in days refresh tokens are valid for.
"�
supportedIdentityProviders*" �(Optional) List of provider names for the identity providers that are supported on this client. Uses the `provider_name` attribute of `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
"�
tokenValidityUnits�*�:�
�
cognito"getUserPoolClientTokenValidityUnitQaws:cognito/getUserPoolClientTokenValidityUnit:getUserPoolClientTokenValidityUniti(Optional) Configuration block for units in which the validity times are represented in. Detailed below.
"

userPoolId" "f
writeAttributes*" M(Optional) List of user pool attributes the application client can write to.
2�
P
cognitogetUserPoolClients1aws:cognito/getUserPoolClients:getUserPoolClients�Use this data source to get a list of Cognito user pools clients for a Cognito IdP user pool.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.cognito.getUserPoolClients({
    userPoolId: mainAwsCognitoUserPool.id,
});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.cognito.get_user_pool_clients(user_pool_id=main_aws_cognito_user_pool["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.Cognito.GetUserPoolClients.Invoke(new()
    {
        UserPoolId = mainAwsCognitoUserPool.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.GetUserPoolClients(ctx, &cognito.GetUserPoolClientsArgs{
			UserPoolId: mainAwsCognitoUserPool.Id,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetUserPoolClientsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var main = CognitoFunctions.getUserPoolClients(GetUserPoolClientsArgs.builder()
            .userPoolId(mainAwsCognitoUserPool.id())
            .build());

    }
}
```
```yaml
variables:
  main:
    fn::invoke:
      function: aws:cognito:getUserPoolClients
      arguments:
        userPoolId: ${mainAwsCognitoUserPool.id}
```
<!--End PulumiCodeChooser -->
(

userPoolId" Cognito user pool ID.
"9
	clientIds*" &List of Cognito user pool client IDs.
"=
clientNames*" (List of Cognito user pool client names.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

userPoolId" 2�
q
cognitogetUserPoolSigningCertificateGaws:cognito/getUserPoolSigningCertificate:getUserPoolSigningCertificate�Use this data source to get the signing certificate for a Cognito IdP user pool.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const sc = aws.cognito.getUserPoolSigningCertificate({
    userPoolId: myPool.id,
});
```
```python
import pulumi
import pulumi_aws as aws

sc = aws.cognito.get_user_pool_signing_certificate(user_pool_id=my_pool["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var sc = Aws.Cognito.GetUserPoolSigningCertificate.Invoke(new()
    {
        UserPoolId = myPool.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cognito.GetUserPoolSigningCertificate(ctx, &cognito.GetUserPoolSigningCertificateArgs{
			UserPoolId: myPool.Id,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetUserPoolSigningCertificateArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var sc = CognitoFunctions.getUserPoolSigningCertificate(GetUserPoolSigningCertificateArgs.builder()
            .userPoolId(myPool.id())
            .build());

    }
}
```
```yaml
variables:
  sc:
    fn::invoke:
      function: aws:cognito:getUserPoolSigningCertificate
      arguments:
        userPoolId: ${myPool.id}
```
<!--End PulumiCodeChooser -->
(

userPoolId" Cognito user pool ID.
"&
certificate" Certificate string
"E
id" ;The provider-assigned unique ID for this managed resource.
"

userPoolId" 2�'
>
cognitogetUserPools%aws:cognito/getUserPools:getUserPools�#Use this data source to get a list of cognito user pools.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const selected = aws.apigateway.getRestApi({
    name: apiGatewayName,
});
const selectedGetUserPools = aws.cognito.getUserPools({
    name: cognitoUserPoolName,
});
const cognito = new aws.apigateway.Authorizer("cognito", {
    name: "cognito",
    type: "COGNITO_USER_POOLS",
    restApi: selected.then(selected => selected.id),
    providerArns: selectedGetUserPools.then(selectedGetUserPools => selectedGetUserPools.arns),
});
```
```python
import pulumi
import pulumi_aws as aws

selected = aws.apigateway.get_rest_api(name=api_gateway_name)
selected_get_user_pools = aws.cognito.get_user_pools(name=cognito_user_pool_name)
cognito = aws.apigateway.Authorizer("cognito",
    name="cognito",
    type="COGNITO_USER_POOLS",
    rest_api=selected.id,
    provider_arns=selected_get_user_pools.arns)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var selected = Aws.ApiGateway.GetRestApi.Invoke(new()
    {
        Name = apiGatewayName,
    });

    var selectedGetUserPools = Aws.Cognito.GetUserPools.Invoke(new()
    {
        Name = cognitoUserPoolName,
    });

    var cognito = new Aws.ApiGateway.Authorizer("cognito", new()
    {
        Name = "cognito",
        Type = "COGNITO_USER_POOLS",
        RestApi = selected.Apply(getRestApiResult => getRestApiResult.Id),
        ProviderArns = selectedGetUserPools.Apply(getUserPoolsResult => getUserPoolsResult.Arns),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apigateway"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		selected, err := apigateway.LookupRestApi(ctx, &apigateway.LookupRestApiArgs{
			Name: apiGatewayName,
		}, nil)
		if err != nil {
			return err
		}
		selectedGetUserPools, err := cognito.GetUserPools(ctx, &cognito.GetUserPoolsArgs{
			Name: cognitoUserPoolName,
		}, nil)
		if err != nil {
			return err
		}
		_, err = apigateway.NewAuthorizer(ctx, "cognito", &apigateway.AuthorizerArgs{
			Name:         pulumi.String("cognito"),
			Type:         pulumi.String("COGNITO_USER_POOLS"),
			RestApi:      pulumi.String(selected.Id),
			ProviderArns: interface{}(selectedGetUserPools.Arns),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.apigateway.ApigatewayFunctions;
import com.pulumi.aws.apigateway.inputs.GetRestApiArgs;
import com.pulumi.aws.cognito.CognitoFunctions;
import com.pulumi.aws.cognito.inputs.GetUserPoolsArgs;
import com.pulumi.aws.apigateway.Authorizer;
import com.pulumi.aws.apigateway.AuthorizerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var selected = ApigatewayFunctions.getRestApi(GetRestApiArgs.builder()
            .name(apiGatewayName)
            .build());

        final var selectedGetUserPools = CognitoFunctions.getUserPools(GetUserPoolsArgs.builder()
            .name(cognitoUserPoolName)
            .build());

        var cognito = new Authorizer("cognito", AuthorizerArgs.builder()
            .name("cognito")
            .type("COGNITO_USER_POOLS")
            .restApi(selected.applyValue(getRestApiResult -> getRestApiResult.id()))
            .providerArns(selectedGetUserPools.applyValue(getUserPoolsResult -> getUserPoolsResult.arns()))
            .build());

    }
}
```
```yaml
resources:
  cognito:
    type: aws:apigateway:Authorizer
    properties:
      name: cognito
      type: COGNITO_USER_POOLS
      restApi: ${selected.id}
      providerArns: ${selectedGetUserPools.arns}
variables:
  selected:
    fn::invoke:
      function: aws:apigateway:getRestApi
      arguments:
        name: ${apiGatewayName}
  selectedGetUserPools:
    fn::invoke:
      function: aws:cognito:getUserPools
      arguments:
        name: ${cognitoUserPoolName}
```
<!--End PulumiCodeChooser -->
�
name" �Name of the cognito user pools. Name is not a unique attribute for cognito user pool, so multiple pools might be returned with given name. If the pool name is expected to be unique, you can reference the pool id via ```tolist(data.aws_cognito_user_pools.selected.ids)[0]```
"E
arns*" 7Set of cognito user pool Amazon Resource Names (ARNs).
"E
id" ;The provider-assigned unique ID for this managed resource.
"+
ids*" Set of cognito user pool ids.
"

name" 2�
M
connectgetBotAssociation/aws:connect/getBotAssociation:getBotAssociation�Provides details about a specific Lex (V1) Bot associated with an Amazon Connect instance.

## Example Usage

### By name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getBotAssociation({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    lexBot: {
        name: "Test",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_bot_association(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    lex_bot={
        "name": "Test",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetBotAssociation.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        LexBot = new Aws.Connect.Inputs.GetBotAssociationLexBotInputArgs
        {
            Name = "Test",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupBotAssociation(ctx, &connect.LookupBotAssociationArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			LexBot: connect.GetBotAssociationLexBot{
				Name: "Test",
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetBotAssociationArgs;
import com.pulumi.aws.connect.inputs.GetBotAssociationLexBotArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getBotAssociation(GetBotAssociationArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .lexBot(GetBotAssociationLexBotArgs.builder()
                .name("Test")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getBotAssociation
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        lexBot:
          name: Test
```
<!--End PulumiCodeChooser -->
u

instanceId" cIdentifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
�
lexBotc:a
_
connectgetBotAssociationLexBot;aws:connect/getBotAssociationLexBot:getBotAssociationLexBotEConfiguration information of an Amazon Lex (V1) bot. Detailed below.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "m
lexBotc:a
_
connectgetBotAssociationLexBot;aws:connect/getBotAssociationLexBot:getBotAssociationLexBot2�)
D
connectgetContactFlow)aws:connect/getContactFlow:getContactFlow�#Provides details about a specific Amazon Connect Contact Flow.

## Example Usage

By name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.connect.getContactFlow({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Test",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.get_contact_flow(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Test")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Connect.GetContactFlow.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Test",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupContactFlow(ctx, &connect.LookupContactFlowArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Test"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetContactFlowArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = ConnectFunctions.getContactFlow(GetContactFlowArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Test")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:connect:getContactFlow
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Test
```
<!--End PulumiCodeChooser -->

By contact_flow_id

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.connect.getContactFlow({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    contactFlowId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.get_contact_flow(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    contact_flow_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Connect.GetContactFlow.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        ContactFlowId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupContactFlow(ctx, &connect.LookupContactFlowArgs{
			InstanceId:    "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			ContactFlowId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetContactFlowArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = ConnectFunctions.getContactFlow(GetContactFlowArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .contactFlowId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:connect:getContactFlow
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        contactFlowId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
Y
contactFlowIdB" BReturns information on a specific Contact Flow by contact flow id
C

instanceId" 1Reference to the hosting Amazon Connect Instance
E
nameB" 7Returns information on a specific Contact Flow by name
4
tagsB2" $Tags to assign to the Contact Flow.
$
typeB" Type of Contact Flow.
"$
arn" ARN of the Contact Flow.
"
contactFlowId" "*
content" Logic of the Contact Flow.
"4
description" !Description of the Contact Flow.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "

name" "2
tags2" $Tags to assign to the Contact Flow.
"$
typeB" Type of Contact Flow.
2�,
V
connectgetContactFlowModule5aws:connect/getContactFlowModule:getContactFlowModule�%Provides details about a specific Amazon Connect Contact Flow Module.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getContactFlowModule({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_contact_flow_module(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetContactFlowModule.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupContactFlowModule(ctx, &connect.LookupContactFlowModuleArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetContactFlowModuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getContactFlowModule(GetContactFlowModuleArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getContactFlowModule
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: example
```
<!--End PulumiCodeChooser -->

By `contact_flow_module_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getContactFlowModule({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    contactFlowModuleId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_contact_flow_module(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    contact_flow_module_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetContactFlowModule.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        ContactFlowModuleId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupContactFlowModule(ctx, &connect.LookupContactFlowModuleArgs{
			InstanceId:          "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			ContactFlowModuleId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetContactFlowModuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getContactFlowModule(GetContactFlowModuleArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .contactFlowModuleId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getContactFlowModule
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        contactFlowModuleId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
m
contactFlowModuleIdB" PReturns information on a specific Contact Flow Module by contact flow module id
C

instanceId" 1Reference to the hosting Amazon Connect Instance
L
nameB" >Returns information on a specific Contact Flow Module by name
B
tagsB2" 2Map of tags to assign to the Contact Flow Module.
"+
arn"  ARN of the Contact Flow Module.
"
contactFlowModuleId" "1
content" "Logic of the Contact Flow Module.
";
description" (Description of the Contact Flow Module.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "

name" "[
state" NType of Contact Flow Module Module. Values are either `ACTIVE` or `ARCHIVED`.
"b
status" TStatus of the Contact Flow Module Module. Values are either `PUBLISHED` or `SAVED`.
"@
tags2" 2Map of tags to assign to the Contact Flow Module.
2�-
S
connectgetHoursOfOperation3aws:connect/getHoursOfOperation:getHoursOfOperation�$Provides details about a specific Amazon Connect Hours of Operation.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.connect.getHoursOfOperation({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Test",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.get_hours_of_operation(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Test")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Connect.GetHoursOfOperation.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Test",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupHoursOfOperation(ctx, &connect.LookupHoursOfOperationArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Test"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetHoursOfOperationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = ConnectFunctions.getHoursOfOperation(GetHoursOfOperationArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Test")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:connect:getHoursOfOperation
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Test
```
<!--End PulumiCodeChooser -->

By `hours_of_operation_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.connect.getHoursOfOperation({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hoursOfOperationId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.get_hours_of_operation(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hours_of_operation_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Connect.GetHoursOfOperation.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        HoursOfOperationId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupHoursOfOperation(ctx, &connect.LookupHoursOfOperationArgs{
			InstanceId:         "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			HoursOfOperationId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetHoursOfOperationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = ConnectFunctions.getHoursOfOperation(GetHoursOfOperationArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .hoursOfOperationId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:connect:getHoursOfOperation
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        hoursOfOperationId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
j
hoursOfOperationIdB" NReturns information on a specific Hours of Operation by hours of operation id
C

instanceId" 1Reference to the hosting Amazon Connect Instance
K
nameB" =Returns information on a specific Hours of Operation by name
A
tagsB2" 1Map of tags to assign to the Hours of Operation.
"*
arn" ARN of the Hours of Operation.
"�
configsk*i:g
e
connectgetHoursOfOperationConfig?aws:connect/getHoursOfOperationConfig:getHoursOfOperationConfig�Configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below. Config blocks are documented below.
":
description" 'Description of the Hours of Operation.
"E
hoursOfOperationId" +The identifier for the hours of operation.
"E
id" ;The provider-assigned unique ID for this managed resource.
"E

instanceId" 3Identifier of the hosting Amazon Connect Instance.
",
name"  Name of the Hours of Operation.
"?
tags2" 1Map of tags to assign to the Hours of Operation.
"5
timeZone" %Time zone of the Hours of Operation.
2�'
;
connectgetInstance#aws:connect/getInstance:getInstance�Provides details about a specific Amazon Connect Instance.

## Example Usage

By instance_alias

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = aws.connect.getInstance({
    instanceAlias: "foo",
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.connect.get_instance(instance_alias="foo")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = Aws.Connect.GetInstance.Invoke(new()
    {
        InstanceAlias = "foo",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupInstance(ctx, &connect.LookupInstanceArgs{
			InstanceAlias: pulumi.StringRef("foo"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var foo = ConnectFunctions.getInstance(GetInstanceArgs.builder()
            .instanceAlias("foo")
            .build());

    }
}
```
```yaml
variables:
  foo:
    fn::invoke:
      function: aws:connect:getInstance
      arguments:
        instanceAlias: foo
```
<!--End PulumiCodeChooser -->

By instance_id

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = aws.connect.getInstance({
    instanceId: "97afc98d-101a-ba98-ab97-ae114fc115ec",
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.connect.get_instance(instance_id="97afc98d-101a-ba98-ab97-ae114fc115ec")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = Aws.Connect.GetInstance.Invoke(new()
    {
        InstanceId = "97afc98d-101a-ba98-ab97-ae114fc115ec",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupInstance(ctx, &connect.LookupInstanceArgs{
			InstanceId: pulumi.StringRef("97afc98d-101a-ba98-ab97-ae114fc115ec"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var foo = ConnectFunctions.getInstance(GetInstanceArgs.builder()
            .instanceId("97afc98d-101a-ba98-ab97-ae114fc115ec")
            .build());

    }
}
```
```yaml
variables:
  foo:
    fn::invoke:
      function: aws:connect:getInstance
      arguments:
        instanceId: 97afc98d-101a-ba98-ab97-ae114fc115ec
```
<!--End PulumiCodeChooser -->
S
instanceAliasB" <Returns information on a specific connect instance by alias
M

instanceIdB" 9Returns information on a specific connect instance by id
;
tagsB2" +A map of tags to assigned to the instance.
" 
arn" ARN of the instance.
""
autoResolveBestVoicesEnabled
 "E
contactFlowLogsEnabled
 'Whether contact flow logs are enabled.
";
contactLensEnabled
 !Whether contact lens is enabled.
"2
createdTime" When the instance was created.
"M
earlyMediaEnabled
 4Whether early media for outbound calls is enabled .
"E
id" ;The provider-assigned unique ID for this managed resource.
"_
identityManagementType" ASpecifies The identity management type attached to the instance.
">
inboundCallsEnabled
 #Whether inbound calls are enabled.
"
instanceAlias" "

instanceId" "T
multiPartyConferenceEnabled
 1Whether multi-party calls/conference is enabled.
"@
outboundCallsEnabled
 $Whether outbound calls are enabled.
"1
serviceRole" Service role of the instance.
"%
status" State of the instance.
"9
tags2" +A map of tags to assigned to the instance.
2�
b
connectgetInstanceStorageConfig=aws:connect/getInstanceStorageConfig:getInstanceStorageConfig�Provides details about a specific Amazon Connect Instance Storage Config.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getInstanceStorageConfig({
    associationId: "1234567891234567890122345678912345678901223456789123456789012234",
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    resourceType: "CONTACT_TRACE_RECORDS",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_instance_storage_config(association_id="1234567891234567890122345678912345678901223456789123456789012234",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    resource_type="CONTACT_TRACE_RECORDS")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetInstanceStorageConfig.Invoke(new()
    {
        AssociationId = "1234567891234567890122345678912345678901223456789123456789012234",
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        ResourceType = "CONTACT_TRACE_RECORDS",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupInstanceStorageConfig(ctx, &connect.LookupInstanceStorageConfigArgs{
			AssociationId: "1234567891234567890122345678912345678901223456789123456789012234",
			InstanceId:    "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			ResourceType:  "CONTACT_TRACE_RECORDS",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetInstanceStorageConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getInstanceStorageConfig(GetInstanceStorageConfigArgs.builder()
            .associationId("1234567891234567890122345678912345678901223456789123456789012234")
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .resourceType("CONTACT_TRACE_RECORDS")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getInstanceStorageConfig
      arguments:
        associationId: 1234567891234567890122345678912345678901223456789123456789012234
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        resourceType: CONTACT_TRACE_RECORDS
```
<!--End PulumiCodeChooser -->
�
associationId" }The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.
C

instanceId" 1Reference to the hosting Amazon Connect Instance
�
resourceType" �A valid resource type. Valid Values: `AGENT_EVENTS` | `ATTACHMENTS` | `CALL_RECORDINGS` | `CHAT_TRANSCRIPTS` | `CONTACT_EVALUATIONS` | `CONTACT_TRACE_RECORDS` | `MEDIA_STREAMS` | `REAL_TIME_CONTACT_ANALYSIS_SEGMENTS` | `SCHEDULED_REPORTS` |  `SCREEN_RECORDINGS`.
"
associationId" "E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "
resourceType" "�
storageConfigs�*�:�
�
connect%getInstanceStorageConfigStorageConfigWaws:connect/getInstanceStorageConfigStorageConfig:getInstanceStorageConfigStorageConfigXSpecifies the storage configuration options for the Connect Instance. Documented below.
2�
n
connectgetLambdaFunctionAssociationEaws:connect/getLambdaFunctionAssociation:getLambdaFunctionAssociation�Provides details about a specific Connect Lambda Function Association.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getLambdaFunctionAssociation({
    functionArn: "arn:aws:lambda:us-west-2:123456789123:function:abcdefg",
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_lambda_function_association(function_arn="arn:aws:lambda:us-west-2:123456789123:function:abcdefg",
    instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetLambdaFunctionAssociation.Invoke(new()
    {
        FunctionArn = "arn:aws:lambda:us-west-2:123456789123:function:abcdefg",
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupLambdaFunctionAssociation(ctx, &connect.LookupLambdaFunctionAssociationArgs{
			FunctionArn: "arn:aws:lambda:us-west-2:123456789123:function:abcdefg",
			InstanceId:  "aaaaaaaa-bbbb-cccc-dddd-111111111111",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetLambdaFunctionAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getLambdaFunctionAssociation(GetLambdaFunctionAssociationArgs.builder()
            .functionArn("arn:aws:lambda:us-west-2:123456789123:function:abcdefg")
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getLambdaFunctionAssociation
      arguments:
        functionArn: arn:aws:lambda:us-west-2:123456789123:function:abcdefg
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
X
functionArn" EARN of the Lambda Function, omitting any version or alias qualifier.
u

instanceId" cIdentifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
"
functionArn" "E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" 2�
5
connect	getPromptaws:connect/getPrompt:getPrompt�Provides details about a specific Amazon Connect Prompt.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getPrompt({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Beep.wav",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_prompt(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Beep.wav")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetPrompt.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Beep.wav",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.GetPrompt(ctx, &connect.GetPromptArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       "Beep.wav",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetPromptArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getPrompt(GetPromptArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Beep.wav")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getPrompt
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Beep.wav
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
=
name" 1Returns information on a specific Prompt by name
"
arn" ARN of the Prompt.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "

name" "+
promptId" Identifier for the prompt.
2�+
2
connectgetQueueaws:connect/getQueue:getQueue�"Provides details about a specific Amazon Connect Queue.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getQueue({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_queue(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetQueue.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupQueue(ctx, &connect.LookupQueueArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetQueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getQueue(GetQueueArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getQueue
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Example
```
<!--End PulumiCodeChooser -->

By `queue_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getQueue({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    queueId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_queue(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    queue_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetQueue.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        QueueId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupQueue(ctx, &connect.LookupQueueArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			QueueId:    pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetQueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getQueue(GetQueueArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .queueId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getQueue
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        queueId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
>
nameB" 0Returns information on a specific Queue by name
E
queueIdB" 4Returns information on a specific Queue by Queue id
3
tagsB2" #Map of tags assigned to the Queue.
"
arn" ARN of the Queue.
"-
description" Description of the Queue.
"N
hoursOfOperationId" 4Specifies the identifier of the Hours of Operation.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "y
maxContacts fMaximum number of contacts that can be in the queue before it is considered full. Minimum value of 0.
"

name" "�
outboundCallerConfigst*r:p
n
connectgetQueueOutboundCallerConfigEaws:connect/getQueueOutboundCallerConfig:getQueueOutboundCallerConfig�A block that defines the outbound caller ID name, number, and outbound whisper flow. The Outbound Caller Config block is documented below.
")
queueId" Identifier for the Queue.
"L
status" >Description of the Queue. Values are `ENABLED` or `DISABLED`.
"1
tags2" #Map of tags assigned to the Queue.
2�,
G
connectgetQuickConnect+aws:connect/getQuickConnect:getQuickConnect�$Provides details about a specific Amazon Connect Quick Connect.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getQuickConnect({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_quick_connect(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetQuickConnect.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupQuickConnect(ctx, &connect.LookupQuickConnectArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetQuickConnectArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getQuickConnect(GetQuickConnectArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getQuickConnect
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Example
```
<!--End PulumiCodeChooser -->

By `quick_connect_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getQuickConnect({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    quickConnectId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_quick_connect(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    quick_connect_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetQuickConnect.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        QuickConnectId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupQuickConnect(ctx, &connect.LookupQuickConnectArgs{
			InstanceId:     "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			QuickConnectId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetQuickConnectArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getQuickConnect(GetQuickConnectArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .quickConnectId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getQuickConnect
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        quickConnectId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
F
nameB" 8Returns information on a specific Quick Connect by name
\
quickConnectIdB" DReturns information on a specific Quick Connect by Quick Connect id
<
tagsB2" ,Map of tags to assign to the Quick Connect.
"%
arn" ARN of the Quick Connect.
"5
description" "Description of the Quick Connect.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "

name" "�
quickConnectConfigs�*�:
}
connect!getQuickConnectQuickConnectConfigOaws:connect/getQuickConnectQuickConnectConfig:getQuickConnectQuickConnectConfig�A block that defines the configuration information for the Quick Connect: `quick_connect_type` and one of `phone_config`, `queue_config`, `user_config` . The Quick Connect Config block is documented below.
"8
quickConnectId" "Identifier for the Quick Connect.
":
tags2" ,Map of tags to assign to the Quick Connect.
2�0
M
connectgetRoutingProfile/aws:connect/getRoutingProfile:getRoutingProfile�$Provides details about a specific Amazon Connect Routing Profile.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getRoutingProfile({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_routing_profile(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetRoutingProfile.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupRoutingProfile(ctx, &connect.LookupRoutingProfileArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetRoutingProfileArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getRoutingProfile(GetRoutingProfileArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getRoutingProfile
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Example
```
<!--End PulumiCodeChooser -->

By `routing_profile_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getRoutingProfile({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    routingProfileId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_routing_profile(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    routing_profile_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetRoutingProfile.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        RoutingProfileId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupRoutingProfile(ctx, &connect.LookupRoutingProfileArgs{
			InstanceId:       "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			RoutingProfileId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetRoutingProfileArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getRoutingProfile(GetRoutingProfileArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .routingProfileId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getRoutingProfile
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        routingProfileId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
H
nameB" :Returns information on a specific Routing Profile by name
b
routingProfileIdB" HReturns information on a specific Routing Profile by Routing Profile id
>
tagsB2" .Map of tags to assign to the Routing Profile.
"'
arn" ARN of the Routing Profile.
"\
defaultOutboundQueueId" >Specifies the default outbound queue for the Routing Profile.
"7
description" $Description of the Routing Profile.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "�
mediaConcurrencies�*�:
}
connect!getRoutingProfileMediaConcurrencyOaws:connect/getRoutingProfileMediaConcurrency:getRoutingProfileMediaConcurrency�One or more `media_concurrencies` blocks that specify the channels that agents can handle in the Contact Control Panel (CCP) for this Routing Profile. The `media_concurrencies` block is documented below.
"

name" "�
queueConfigst*r:p
n
connectgetRoutingProfileQueueConfigEaws:connect/getRoutingProfileQueueConfig:getRoutingProfileQueueConfig�One or more `queue_configs` blocks that specify the inbound queues associated with the routing profile. If no queue is added, the agent only can make outbound calls. The `queue_configs` block is documented below.
"
routingProfileId" "<
tags2" .Map of tags to assign to the Routing Profile.
2�+
P
connectgetSecurityProfile1aws:connect/getSecurityProfile:getSecurityProfile�$Provides details about a specific Amazon Connect Security Profile.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getSecurityProfile({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_security_profile(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetSecurityProfile.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupSecurityProfile(ctx, &connect.LookupSecurityProfileArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetSecurityProfileArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getSecurityProfile(GetSecurityProfileArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getSecurityProfile
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Example
```
<!--End PulumiCodeChooser -->

By `security_profile_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getSecurityProfile({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    securityProfileId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_security_profile(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    security_profile_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetSecurityProfile.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        SecurityProfileId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupSecurityProfile(ctx, &connect.LookupSecurityProfileArgs{
			InstanceId:        "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			SecurityProfileId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetSecurityProfileArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getSecurityProfile(GetSecurityProfileArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .securityProfileId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getSecurityProfile
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        securityProfileId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
I
nameB" ;Returns information on a specific Security Profile by name
e
securityProfileIdB" JReturns information on a specific Security Profile by Security Profile id
?
tagsB2" /Map of tags to assign to the Security Profile.
"(
arn" ARN of the Security Profile.
"8
description" %Description of the Security Profile.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "

name" "]
organizationResourceId" ?The organization resource identifier for the security profile.
"K
permissions*" 6List of permissions assigned to the security profile.
"
securityProfileId" "=
tags2" /Map of tags to assign to the Security Profile.
2�,
/
connectgetUseraws:connect/getUser:getUser�"Provides details about a specific Amazon Connect User.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getUser({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_user(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetUser.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupUser(ctx, &connect.LookupUserArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetUserArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getUser(GetUserArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getUser
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Example
```
<!--End PulumiCodeChooser -->

By `user_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getUser({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    userId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_user(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    user_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetUser.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        UserId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupUser(ctx, &connect.LookupUserArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			UserId:     pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetUserArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getUser(GetUserArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .userId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getUser
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        userId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
=
nameB" /Returns information on a specific User by name
5
tagsB2" %A map of tags to assign to the User.
B
userIdB" 2Returns information on a specific User by User id
"7
arn" ,The Amazon Resource Name (ARN) of the User.
"i
directoryUserId" RThe identifier of the user account in the directory used for identity management.
"L
hierarchyGroupId" 4The identifier of the hierarchy group for the user.
"E
id" ;The provider-assigned unique ID for this managed resource.
"�
identityInfosY*W:U
S
connectgetUserIdentityInfo3aws:connect/getUserIdentityInfo:getUserIdentityInfoTA block that contains information about the identity of the user. Documented below.
"S

instanceId" ASpecifies the identifier of the hosting Amazon Connect Instance.
"

name" "�
phoneConfigsV*T:R
P
connectgetUserPhoneConfig1aws:connect/getUserPhoneConfig:getUserPhoneConfig[A block that contains information about the phone settings for the user. Documented below.
"L
routingProfileId" 4The identifier of the routing profile for the user.
"Z
securityProfileIds*" >A list of identifiers for the security profiles for the user.
"3
tags2" %A map of tags to assign to the User.
"
userId" 2�-
Y
connectgetUserHierarchyGroup7aws:connect/getUserHierarchyGroup:getUserHierarchyGroup�%Provides details about a specific Amazon Connect User Hierarchy Group.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getUserHierarchyGroup({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_user_hierarchy_group(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetUserHierarchyGroup.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupUserHierarchyGroup(ctx, &connect.LookupUserHierarchyGroupArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetUserHierarchyGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getUserHierarchyGroup(GetUserHierarchyGroupArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getUserHierarchyGroup
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Example
```
<!--End PulumiCodeChooser -->

By `hierarchy_group_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getUserHierarchyGroup({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hierarchyGroupId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_user_hierarchy_group(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    hierarchy_group_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetUserHierarchyGroup.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        HierarchyGroupId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupUserHierarchyGroup(ctx, &connect.LookupUserHierarchyGroupArgs{
			InstanceId:       "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			HierarchyGroupId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetUserHierarchyGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getUserHierarchyGroup(GetUserHierarchyGroupArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .hierarchyGroupId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getUserHierarchyGroup
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        hierarchyGroupId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
b
hierarchyGroupIdB" HReturns information on a specific hierarchy group by hierarchy group id
C

instanceId" 1Reference to the hosting Amazon Connect Instance
H
nameB" :Returns information on a specific hierarchy group by name
>
tagsB2" .Map of tags to assign to the hierarchy group.
"'
arn" ARN of the hierarchy group.
"
hierarchyGroupId" "�
hierarchyPaths�*�:�
�
connect"getUserHierarchyGroupHierarchyPathQaws:connect/getUserHierarchyGroupHierarchyPath:getUserHierarchyGroupHierarchyPathyBlock that contains information about the levels in the hierarchy group. The `hierarchy_path` block is documented below.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "?
levelId" 0Identifier of the level in the hierarchy group.
")
name" Name of the hierarchy group.
"<
tags2" .Map of tags to assign to the hierarchy group.
2�
e
connectgetUserHierarchyStructure?aws:connect/getUserHierarchyStructure:getUserHierarchyStructure�Provides details about a specific Amazon Connect User Hierarchy Structure

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.connect.getUserHierarchyStructure({
    instanceId: testAwsConnectInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.connect.get_user_hierarchy_structure(instance_id=test_aws_connect_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Connect.GetUserHierarchyStructure.Invoke(new()
    {
        InstanceId = testAwsConnectInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupUserHierarchyStructure(ctx, &connect.LookupUserHierarchyStructureArgs{
			InstanceId: testAwsConnectInstance.Id,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetUserHierarchyStructureArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = ConnectFunctions.getUserHierarchyStructure(GetUserHierarchyStructureArgs.builder()
            .instanceId(testAwsConnectInstance.id())
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:connect:getUserHierarchyStructure
      arguments:
        instanceId: ${testAwsConnectInstance.id}
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
"�
hierarchyStructures�*�:�
�
connect+getUserHierarchyStructureHierarchyStructurecaws:connect/getUserHierarchyStructureHierarchyStructure:getUserHierarchyStructureHierarchyStructurejBlock that defines the hierarchy structure's levels. The `hierarchy_structure` block is documented below.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" 2�1
A
connectgetVocabulary'aws:connect/getVocabulary:getVocabulary�#Provides details about a specific Amazon Connect Vocabulary.

## Example Usage

By `name`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getVocabulary({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name: "Example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_vocabulary(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    name="Example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetVocabulary.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        Name = "Example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupVocabulary(ctx, &connect.LookupVocabularyArgs{
			InstanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			Name:       pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetVocabularyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getVocabulary(GetVocabularyArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .name("Example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getVocabulary
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        name: Example
```
<!--End PulumiCodeChooser -->

By `vocabulary_id`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.connect.getVocabulary({
    instanceId: "aaaaaaaa-bbbb-cccc-dddd-111111111111",
    vocabularyId: "cccccccc-bbbb-cccc-dddd-111111111111",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.connect.get_vocabulary(instance_id="aaaaaaaa-bbbb-cccc-dddd-111111111111",
    vocabulary_id="cccccccc-bbbb-cccc-dddd-111111111111")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Connect.GetVocabulary.Invoke(new()
    {
        InstanceId = "aaaaaaaa-bbbb-cccc-dddd-111111111111",
        VocabularyId = "cccccccc-bbbb-cccc-dddd-111111111111",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/connect"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := connect.LookupVocabulary(ctx, &connect.LookupVocabularyArgs{
			InstanceId:   "aaaaaaaa-bbbb-cccc-dddd-111111111111",
			VocabularyId: pulumi.StringRef("cccccccc-bbbb-cccc-dddd-111111111111"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.connect.ConnectFunctions;
import com.pulumi.aws.connect.inputs.GetVocabularyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ConnectFunctions.getVocabulary(GetVocabularyArgs.builder()
            .instanceId("aaaaaaaa-bbbb-cccc-dddd-111111111111")
            .vocabularyId("cccccccc-bbbb-cccc-dddd-111111111111")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:connect:getVocabulary
      arguments:
        instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
        vocabularyId: cccccccc-bbbb-cccc-dddd-111111111111
```
<!--End PulumiCodeChooser -->
C

instanceId" 1Reference to the hosting Amazon Connect Instance
C
nameB" 5Returns information on a specific Vocabulary by name
;
tagsB2" +A map of tags to assign to the Vocabulary.
T
vocabularyIdB" >Returns information on a specific Vocabulary by Vocabulary id
"=
arn" 2The Amazon Resource Name (ARN) of the Vocabulary.
"�
content" �The content of the custom vocabulary in plain-text format with a table of values. Each row in the table represents a word or a phrase, described with Phrase, IPA, SoundsLike, and DisplayAs fields. Separate the fields with TAB characters. For more information, see [Create a custom vocabulary using a table](https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table).
"K
failureReason" 6The reason why the custom vocabulary was not created.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

instanceId" "�
languageCode" �The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see [What is Amazon Transcribe?](https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html). Valid Values are `ar-AE`, `de-CH`, `de-DE`, `en-AB`, `en-AU`, `en-GB`, `en-IE`, `en-IN`, `en-US`, `en-WL`, `es-ES`, `es-US`, `fr-CA`, `fr-FR`, `hi-IN`, `it-IT`, `ja-JP`, `ko-KR`, `pt-BR`, `pt-PT`, `zh-CN`.
"T
lastModifiedTime" <The timestamp when the custom vocabulary was last modified.
"

name" "�
state" �The current state of the custom vocabulary. Valid values are `CREATION_IN_PROGRESS`, `ACTIVE`, `CREATION_FAILED`, `DELETE_IN_PROGRESS`.
"9
tags2" +A map of tags to assign to the Vocabulary.
"=
vocabularyId" )The identifier of the custom vocabulary.
:�
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter�
��
name" �Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter�
��
name" �Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter�
�p
name" dName of the filter field. Valid values can be found in the [describe-regions AWS CLI Reference][1].
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
�
codeguruprofiler&ProfilingGroupAgentOrchestrationConfigbaws:codeguruprofiler/ProfilingGroupAgentOrchestrationConfig:ProfilingGroupAgentOrchestrationConfigu
sq
profilingEnabled
 Y(Required) Boolean that specifies whether the profiling agent collects profiling data or
:�
�
codeguruprofiler)getProfilingGroupAgentOrchestrationConfighaws:codeguruprofiler/getProfilingGroupAgentOrchestrationConfig:getProfilingGroupAgentOrchestrationConfig

profilingEnabled
 :�
�
codeguruprofiler getProfilingGroupProfilingStatusVaws:codeguruprofiler/getProfilingGroupProfilingStatus:getProfilingGroupProfilingStatus�
�
latestAgentOrchestratedAt" "
latestAgentProfileReportedAt" �
latestAggregatedProfiles�*�:�
�
codeguruprofiler7getProfilingGroupProfilingStatusLatestAggregatedProfile�aws:codeguruprofiler/getProfilingGroupProfilingStatusLatestAggregatedProfile:getProfilingGroupProfilingStatusLatestAggregatedProfile:�
�
codeguruprofiler7getProfilingGroupProfilingStatusLatestAggregatedProfile�aws:codeguruprofiler/getProfilingGroupProfilingStatusLatestAggregatedProfile:getProfilingGroupProfilingStatusLatestAggregatedProfile

period" 
start" :�
�
codegurureviewer"RepositoryAssociationKmsKeyDetailsZaws:codegurureviewer/RepositoryAssociationKmsKeyDetails:RepositoryAssociationKmsKeyDetails�
��
encryptionOptionB" �The encryption option for a repository association. It is either owned by AWS Key Management Service (KMS) (`AWS_OWNED_CMK`) or customer managed (`CUSTOMER_MANAGED_CMK`).
^
kmsKeyIdB" LThe ID of the AWS KMS key that is associated with a repository association.
:�
�
codegurureviewerRepositoryAssociationRepositoryTaws:codegurureviewer/RepositoryAssociationRepository:RepositoryAssociationRepository�
��
	bitbucket�B�:�
�
codegurureviewer(RepositoryAssociationRepositoryBitbucketfaws:codegurureviewer/RepositoryAssociationRepositoryBitbucket:RepositoryAssociationRepositoryBitbucket�

codecommit�B�:�
�
codegurureviewer)RepositoryAssociationRepositoryCodecommithaws:codegurureviewer/RepositoryAssociationRepositoryCodecommit:RepositoryAssociationRepositoryCodecommit�
githubEnterpriseServer�B�:�
�
codegurureviewer5RepositoryAssociationRepositoryGithubEnterpriseServer�aws:codegurureviewer/RepositoryAssociationRepositoryGithubEnterpriseServer:RepositoryAssociationRepositoryGithubEnterpriseServer�
s3Bucket�B�:�
�
codegurureviewer'RepositoryAssociationRepositoryS3Bucketdaws:codegurureviewer/RepositoryAssociationRepositoryS3Bucket:RepositoryAssociationRepositoryS3Bucket:�
�
codegurureviewer(RepositoryAssociationRepositoryBitbucketfaws:codegurureviewer/RepositoryAssociationRepositoryBitbucket:RepositoryAssociationRepositoryBitbucket�
�_
connectionArn" JThe Amazon Resource Name (ARN) of an AWS CodeStar Connections connection.
;
name" /The name of the third party source repository.
D
owner" 7The username for the account that owns the repository.
:�
�
codegurureviewer)RepositoryAssociationRepositoryCodecommithaws:codegurureviewer/RepositoryAssociationRepositoryCodecommit:RepositoryAssociationRepositoryCodecommit;
97
name" +The name of the AWS CodeCommit repository.
:�
�
codegurureviewer5RepositoryAssociationRepositoryGithubEnterpriseServer�aws:codegurureviewer/RepositoryAssociationRepositoryGithubEnterpriseServer:RepositoryAssociationRepositoryGithubEnterpriseServer�
�_
connectionArn" JThe Amazon Resource Name (ARN) of an AWS CodeStar Connections connection.
;
name" /The name of the third party source repository.
D
owner" 7The username for the account that owns the repository.
:�
�
codegurureviewer'RepositoryAssociationRepositoryS3Bucketdaws:codegurureviewer/RepositoryAssociationRepositoryS3Bucket:RepositoryAssociationRepositoryS3Bucket�
��

bucketName" yThe name of the S3 bucket used for associating a new S3 repository. Note: The name must begin with `codeguru-reviewer-`.
9
name" -The name of the repository in the S3 bucket.
:�
�
codegurureviewer'RepositoryAssociationS3RepositoryDetaildaws:codegurureviewer/RepositoryAssociationS3RepositoryDetail:RepositoryAssociationS3RepositoryDetail�
��

bucketNameB" yThe name of the S3 bucket used for associating a new S3 repository. Note: The name must begin with `codeguru-reviewer-`.
�
codeArtifacts�B�*�:�
�
codegurureviewer3RepositoryAssociationS3RepositoryDetailCodeArtifact|aws:codegurureviewer/RepositoryAssociationS3RepositoryDetailCodeArtifact:RepositoryAssociationS3RepositoryDetailCodeArtifact:�
�
codegurureviewer3RepositoryAssociationS3RepositoryDetailCodeArtifact|aws:codegurureviewer/RepositoryAssociationS3RepositoryDetailCodeArtifact:RepositoryAssociationS3RepositoryDetailCodeArtifactI
G
buildArtifactsObjectKeyB" $
sourceCodeArtifactsObjectKeyB" :�
�
codepipeline%CustomActionTypeConfigurationProperty\aws:codepipeline/CustomActionTypeConfigurationProperty:CustomActionTypeConfigurationProperty�
�K
descriptionB" 6The description of the action configuration property.
8
key
 -Whether the configuration property is a key.
;
name" /The name of the action configuration property.
]
	queryableB
 JIndicates that the property will be used in conjunction with PollForJobs.
H
required
 8Whether the configuration property is a required value.
<
secret
 .Whether the configuration property is secret.
b
typeB" TThe type of the configuration property. Valid values: `String`, `Number`, `Boolean`
:�
�
codepipeline$CustomActionTypeInputArtifactDetailsZaws:codepipeline/CustomActionTypeInputArtifactDetails:CustomActionTypeInputArtifactDetails�
�`
maximumCount LThe maximum number of artifacts allowed for the action type. Min: 0, Max: 5
`
minimumCount LThe minimum number of artifacts allowed for the action type. Min: 0, Max: 5
:�
�
codepipeline%CustomActionTypeOutputArtifactDetails\aws:codepipeline/CustomActionTypeOutputArtifactDetails:CustomActionTypeOutputArtifactDetails�
�`
maximumCount LThe maximum number of artifacts allowed for the action type. Min: 0, Max: 5
`
minimumCount LThe minimum number of artifacts allowed for the action type. Min: 0, Max: 5
:�
l
codepipelineCustomActionTypeSettingsBaws:codepipeline/CustomActionTypeSettings:CustomActionTypeSettings�
��
entityUrlTemplateB" tThe URL returned to the AWS CodePipeline console that provides a deep link to the resources of the external system.
�
executionUrlTemplateB" }The URL returned to the AWS CodePipeline console that contains a link to the top-level landing page for the external system.
�
revisionUrlTemplateB" �The URL returned to the AWS CodePipeline console that contains a link to the page where customers can update or change the configuration of the external action.
�
thirdPartyConfigurationUrlB" �The URL of a sign-up page where users can sign up for an external service and perform initial configuration of the action provided by that service.
:�
c
codepipelinePipelineArtifactStore<aws:codepipeline/PipelineArtifactStore:PipelineArtifactStore�
��
encryptionKey�B�:�
�
codepipeline"PipelineArtifactStoreEncryptionKeyVaws:codepipeline/PipelineArtifactStoreEncryptionKey:PipelineArtifactStoreEncryptionKey�The encryption key block AWS CodePipeline uses to encrypt the data in the artifact store, such as an AWS Key Management Service (AWS KMS) key. If you don't specify a key, AWS CodePipeline uses the default key for Amazon Simple Storage Service (Amazon S3). An `encryption_key` block is documented below.
w
location" gThe location where AWS CodePipeline stores artifacts for a pipeline; currently only `S3` is supported.
�
regionB" �The region where the artifact store is located. Required for a cross-region CodePipeline, do not provide for a single-region CodePipeline.
>
type" 2The type of the artifact store, such as Amazon S3
:�
�
codepipeline"PipelineArtifactStoreEncryptionKeyVaws:codepipeline/PipelineArtifactStoreEncryptionKey:PipelineArtifactStoreEncryptionKeye
c 
id" The KMS key ARN or ID
?
type" 3The type of key; currently only `KMS` is supported
:�
K
codepipelinePipelineStage,aws:codepipeline/PipelineStage:PipelineStage�
��
actionsc*a:_
]
codepipelinePipelineStageAction8aws:codepipeline/PipelineStageAction:PipelineStageActionJThe action(s) to include in the stage. Defined as an `action` block below
#
name" The name of the stage.
:�
]
codepipelinePipelineStageAction8aws:codepipeline/PipelineStageAction:PipelineStageAction�
��
category" �A category defines what kind of action can be taken in the stage, and constrains the provider type for the action. Possible values are `Approval`, `Build`, `Deploy`, `Invoke`, `Source` and `Test`.
�
configurationB2" �A map of the action declaration's configuration. Configurations options for action types and providers can be found in the [Pipeline Structure Reference](http://docs.aws.amazon.com/codepipeline/latest/userguide/reference-pipeline-structure.html#action-requirements) and [Action Structure Reference](https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference.html) documentation. Note: The `DetectChanges` parameter (optional, default value is true) in the `configuration` section causes CodePipeline to automatically start your pipeline upon new commits. Please refer to AWS Documentation for more details: https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference-CodestarConnectionSource.html#action-reference-CodestarConnectionSource-config.
D
inputArtifactsB*" *A list of artifact names to be worked on.
+
name" The action declaration's name.
M
	namespaceB" :The namespace all output variables will be accessed from.
w
outputArtifactsB*" \A list of artifact names to output. Output artifact names must be unique within a pipeline.
k
owner" ^The creator of the action being called. Possible values are `AWS`, `Custom` and `ThirdParty`.
�
provider" �The provider of the service being called by the action. Valid providers are determined by the action category. Provider names are listed in the [Action Structure Reference](https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference.html) documentation.
7
regionB" 'The region in which to run the action.
�
roleArnB" }The ARN of the IAM service role that will perform the declared action. This is assumed through the roleArn for the pipeline.
6
runOrderB $The order in which actions are run.

timeoutInMinutesB 9
version" *A string that identifies the action type.
:�
Q
codepipelinePipelineTrigger0aws:codepipeline/PipelineTrigger:PipelineTrigger�
��
gitConfiguration�:�
�
codepipelinePipelineTriggerGitConfigurationPaws:codepipeline/PipelineTriggerGitConfiguration:PipelineTriggerGitConfiguration�Provides the filter criteria and the source stage for the repository event that starts the pipeline. For more information, refer to the [AWS documentation](https://docs.aws.amazon.com/codepipeline/latest/userguide/pipelines-filter.html). A `git_configuration` block is documented below.
e
providerType" QThe source provider for the event. Possible value is `CodeStarSourceConnection`.
:�
�
codepipelinePipelineTriggerGitConfigurationPaws:codepipeline/PipelineTriggerGitConfiguration:PipelineTriggerGitConfiguration�
��
pullRequests�B�*�:�
�
codepipeline*PipelineTriggerGitConfigurationPullRequestfaws:codepipeline/PipelineTriggerGitConfigurationPullRequest:PipelineTriggerGitConfigurationPullRequest�The field where the repository event that will start the pipeline is specified as pull requests. A `pull_request` block is documented below.
�
pushes�B�*�:�
�
codepipeline#PipelineTriggerGitConfigurationPushXaws:codepipeline/PipelineTriggerGitConfigurationPush:PipelineTriggerGitConfigurationPush�The field where the repository event that will start the pipeline, such as pushing Git tags, is specified with details. A `push` block is documented below.
�
sourceActionName" �The name of the pipeline source action where the trigger configuration, such as Git tags, is specified. The trigger configuration will start the pipeline upon the specified change only.
:�
�
codepipeline*PipelineTriggerGitConfigurationPullRequestfaws:codepipeline/PipelineTriggerGitConfigurationPullRequest:PipelineTriggerGitConfigurationPullRequest�
��
branches�B�:�
�
codepipeline2PipelineTriggerGitConfigurationPullRequestBranchesvaws:codepipeline/PipelineTriggerGitConfigurationPullRequestBranches:PipelineTriggerGitConfigurationPullRequestBranches�The field that specifies to filter on branches for the pull request trigger configuration. A `branches` block is documented below.
�
eventsB*" �A list that specifies which pull request events to filter on (opened, updated, closed) for the trigger configuration. Possible values are `OPEN`, `UPDATED ` and `CLOSED`.
�
	filePaths�B�:�
�
codepipeline3PipelineTriggerGitConfigurationPullRequestFilePathsxaws:codepipeline/PipelineTriggerGitConfigurationPullRequestFilePaths:PipelineTriggerGitConfigurationPullRequestFilePaths�The field that specifies to filter on file paths for the pull request trigger configuration. A `file_paths` block is documented below.
:�
�
codepipeline2PipelineTriggerGitConfigurationPullRequestBranchesvaws:codepipeline/PipelineTriggerGitConfigurationPullRequestBranches:PipelineTriggerGitConfigurationPullRequestBranches�
��
excludesB*" qA list of patterns of Git branches that, when a commit is pushed, are to be excluded from starting the pipeline.
�
includesB*" {A list of patterns of Git branches that, when a commit is pushed, are to be included as criteria that starts the pipeline.
:�
�
codepipeline3PipelineTriggerGitConfigurationPullRequestFilePathsxaws:codepipeline/PipelineTriggerGitConfigurationPullRequestFilePaths:PipelineTriggerGitConfigurationPullRequestFilePaths�
��
excludesB*" ~A list of patterns of Git repository file paths that, when a commit is pushed, are to be excluded from starting the pipeline.
�
includesB*" �A list of patterns of Git repository file paths that, when a commit is pushed, are to be included as criteria that starts the pipeline.
:�
�
codepipeline#PipelineTriggerGitConfigurationPushXaws:codepipeline/PipelineTriggerGitConfigurationPush:PipelineTriggerGitConfigurationPush�
��
branches�B�:�
�
codepipeline+PipelineTriggerGitConfigurationPushBrancheshaws:codepipeline/PipelineTriggerGitConfigurationPushBranches:PipelineTriggerGitConfigurationPushBranches{The field that specifies to filter on branches for the push trigger configuration. A `branches` block is documented below.
�
	filePaths�B�:�
�
codepipeline,PipelineTriggerGitConfigurationPushFilePathsjaws:codepipeline/PipelineTriggerGitConfigurationPushFilePaths:PipelineTriggerGitConfigurationPushFilePathsThe field that specifies to filter on file paths for the push trigger configuration. A `file_paths` block is documented below.
�
tags�B�:�
�
codepipeline'PipelineTriggerGitConfigurationPushTags`aws:codepipeline/PipelineTriggerGitConfigurationPushTags:PipelineTriggerGitConfigurationPushTagspThe field that contains the details for the Git tags trigger configuration. A `tags` block is documented below.
:�
�
codepipeline+PipelineTriggerGitConfigurationPushBrancheshaws:codepipeline/PipelineTriggerGitConfigurationPushBranches:PipelineTriggerGitConfigurationPushBranches�
��
excludesB*" qA list of patterns of Git branches that, when a commit is pushed, are to be excluded from starting the pipeline.
�
includesB*" {A list of patterns of Git branches that, when a commit is pushed, are to be included as criteria that starts the pipeline.
:�
�
codepipeline,PipelineTriggerGitConfigurationPushFilePathsjaws:codepipeline/PipelineTriggerGitConfigurationPushFilePaths:PipelineTriggerGitConfigurationPushFilePaths�
��
excludesB*" ~A list of patterns of Git repository file paths that, when a commit is pushed, are to be excluded from starting the pipeline.
�
includesB*" �A list of patterns of Git repository file paths that, when a commit is pushed, are to be included as criteria that starts the pipeline.
:�
�
codepipeline'PipelineTriggerGitConfigurationPushTags`aws:codepipeline/PipelineTriggerGitConfigurationPushTags:PipelineTriggerGitConfigurationPushTags�
�u
excludesB*" aA list of patterns of Git tags that, when pushed, are to be excluded from starting the pipeline.

includesB*" kA list of patterns of Git tags that, when pushed, are to be included as criteria that starts the pipeline.
:�
T
codepipelinePipelineVariable2aws:codepipeline/PipelineVariable:PipelineVariable�
�F
defaultValueB" 0The default value of a pipeline-level variable.
�
descriptionB" �The description of a pipeline-level variable.

> **Note:** The input artifact of an action must exactly match the output artifact declared in a preceding action, but the input artifact does not have to be the next action in strict sequence from the action that provided the output artifact. Actions in parallel can declare different output artifacts, which are in turn consumed by different following actions.
3
name" 'The name of a pipeline-level variable.
:�
�
codepipeline"WebhookAuthenticationConfigurationVaws:codepipeline/WebhookAuthenticationConfiguration:WebhookAuthenticationConfiguration�
�R
allowedIpRangeB" :A valid CIDR block for `IP` filtering. Required for `IP`.
�
secretTokenB" �The shared secret for the GitHub repository webhook. Set this as `secret` in your `github_repository_webhook`'s `configuration` block. Required for `GITHUB_HMAC`.
:�
K
codepipelineWebhookFilter,aws:codepipeline/WebhookFilter:WebhookFilter�
�U
jsonPath" EThe [JSON path](https://github.com/json-path/JsonPath) to filter on.
�
matchEquals" �The value to match on (e.g., `refs/heads/{Branch}`). See [AWS docs](https://docs.aws.amazon.com/codepipeline/latest/APIReference/API_WebhookFilterRule.html) for details.
:�
n
codestarconnectionsHostVpcConfigurationAaws:codestarconnections/HostVpcConfiguration:HostVpcConfiguration�
��
securityGroupIds*" �ID of the security group or security groups associated with the Amazon VPC connected to the infrastructure where your provider type is installed.
�
	subnetIds*" �The ID of the subnet or subnets associated with the Amazon VPC connected to the infrastructure where your provider type is installed.
�
tlsCertificateB" �The value of the Transport Layer Security (TLS) certificate associated with the infrastructure where your provider type is installed.
m
vpcId" `The ID of the Amazon VPC connected to the infrastructure where your provider type is installed.
:�
x
codestarnotificationsNotificationRuleTargetGaws:codestarnotifications/NotificationRuleTarget:NotificationRuleTarget�
�R
address" CThe ARN of notification rule target. For example, a SNS Topic ARN.
y
statusB" iThe status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
K
typeB" =The type of the notification target. Default value is `SNS`.
:�
�
cognito#IdentityPoolCognitoIdentityProviderSaws:cognito/IdentityPoolCognitoIdentityProvider:IdentityPoolCognitoIdentityProvider�
�K
clientIdB" 9The client ID for the Amazon Cognito Identity User Pool.
R
providerNameB" <The provider name for an Amazon Cognito Identity User Pool.
z
serverSideTokenCheckB
 \Whether server-side token validation is enabled for the identity provider’s token or not.
:�	
�
cognito%IdentityPoolRoleAttachmentRoleMappingWaws:cognito/IdentityPoolRoleAttachmentRoleMapping:IdentityPoolRoleAttachmentRoleMapping�
��
ambiguousRoleResolutionB" �Specifies the action to be taken if either no rules match the claim value for the Rules type, or there is no cognito:preferred_role claim and there are multiple cognito:roles matches for the Token type. `Required` if you specify Token or Rules as the Type.
�
identityProvider" �A string identifying the identity provider, for example, "graph.facebook.com" or "cognito-idp.us-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id". Depends on `cognito_identity_providers` set on `aws.cognito.IdentityPool` resource or a `aws.cognito.IdentityProvider` resource.
�
mappingRules�B�*�:�
�
cognito0IdentityPoolRoleAttachmentRoleMappingMappingRulemaws:cognito/IdentityPoolRoleAttachmentRoleMappingMappingRule:IdentityPoolRoleAttachmentRoleMappingMappingRule�The Rules Configuration to be used for mapping users to roles. You can specify up to 25 rules per identity provider. Rules are evaluated in order. The first one to match specifies the role.
#
type" The role mapping type.
:�
�
cognito0IdentityPoolRoleAttachmentRoleMappingMappingRulemaws:cognito/IdentityPoolRoleAttachmentRoleMappingMappingRule:IdentityPoolRoleAttachmentRoleMappingMappingRule�
�a
claim" TThe claim name that must be present in the token, for example, "isAdmin" or "paid".
s
	matchType" bThe match condition that specifies how closely the claim value in the IdP token must match Value.

roleArn" The role ARN.
U
value" HA brief string that the claim must match, for example, "paid" or "yes".
:�
�
cognito+ManagedUserPoolClientAnalyticsConfigurationcaws:cognito/ManagedUserPoolClientAnalyticsConfiguration:ManagedUserPoolClientAnalyticsConfiguration�
�|
applicationArnB" dApplication ARN for an Amazon Pinpoint application. It conflicts with `external_id` and `role_arn`.
M
applicationIdB" 6Unique identifier for an Amazon Pinpoint application.
]

externalIdB" IID for the Analytics Configuration and conflicts with `application_arn`.
�
roleArnB" �ARN of an IAM role that authorizes Amazon Cognito to publish events to Amazon Pinpoint analytics. It conflicts with `application_arn`.
�
userDataSharedB
 �If `user_data_shared` is set to `true`, Amazon Cognito will include user data in the events it publishes to Amazon Pinpoint analytics.
:�
�
cognito'ManagedUserPoolClientTokenValidityUnits[aws:cognito/ManagedUserPoolClientTokenValidityUnits:ManagedUserPoolClientTokenValidityUnits�
�a
accessTokenB" LTime unit for the value in `access_token_validity` and defaults to `hours`.
]
idTokenB" LTime unit for the value in `id_token_validity`, and it defaults to `hours`.
b
refreshTokenB" LTime unit for the value in `refresh_token_validity` and defaults to `days`.
:�
S
cognitoResourceServerScope3aws:cognito/ResourceServerScope:ResourceServerScopeV
T/
scopeDescription" The scope description.
!
	scopeName" The scope name.
:�
�
cognito1RiskConfigurationAccountTakeoverRiskConfigurationoaws:cognito/RiskConfigurationAccountTakeoverRiskConfiguration:RiskConfigurationAccountTakeoverRiskConfiguration�
��
actions�:�
�
cognito8RiskConfigurationAccountTakeoverRiskConfigurationActions}aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActions:RiskConfigurationAccountTakeoverRiskConfigurationActions@Account takeover risk configuration actions. See details below.
�
notifyConfiguration�:�
�
cognitoDRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationSThe notify configuration used to construct email notifications. See details below.
:�
�
cognito8RiskConfigurationAccountTakeoverRiskConfigurationActions}aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActions:RiskConfigurationAccountTakeoverRiskConfigurationActions�
��

highAction�B�:�
�
cognitoBRiskConfigurationAccountTakeoverRiskConfigurationActionsHighAction�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActionsHighAction:RiskConfigurationAccountTakeoverRiskConfigurationActionsHighAction8Action to take for a high risk. See action block below.
�
	lowAction�B�:�
�
cognitoARiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction:RiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction7Action to take for a low risk. See action block below.
�
mediumAction�B�:�
�
cognitoDRiskConfigurationAccountTakeoverRiskConfigurationActionsMediumAction�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActionsMediumAction:RiskConfigurationAccountTakeoverRiskConfigurationActionsMediumAction:Action to take for a medium risk. See action block below.
:�
�
cognitoBRiskConfigurationAccountTakeoverRiskConfigurationActionsHighAction�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActionsHighAction:RiskConfigurationAccountTakeoverRiskConfigurationActionsHighActionE
C
eventAction" .
notify
  Whether to send a notification.
:�
�
cognitoARiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction:RiskConfigurationAccountTakeoverRiskConfigurationActionsLowActionE
C
eventAction" .
notify
  Whether to send a notification.
:�
�
cognitoDRiskConfigurationAccountTakeoverRiskConfigurationActionsMediumAction�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationActionsMediumAction:RiskConfigurationAccountTakeoverRiskConfigurationActionsMediumActionE
C
eventAction" .
notify
  Whether to send a notification.
:�
�
cognitoDRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration�
��

blockEmail�B�:�
�
cognitoNRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmail�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmail:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmailXEmail template used when a detected risk event is blocked. See notify email type below.
�
fromB" �The email address that is sending the email. The address must be either individually verified with Amazon Simple Email Service, or from a domain that has been verified with Amazon SES.
�
mfaEmail�B�:�
�
cognitoLRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail�The multi-factor authentication (MFA) email template used when MFA is challenged as part of a detected risk. See notify email type below.
�
noActionEmail�B�:�
�
cognitoQRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmail�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmail:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmail\The email template used when a detected risk event is allowed. See notify email type below.
T
replyToB" CThe destination to which the receiver of an email should reply to.
�
	sourceArn" �The Amazon Resource Name (ARN) of the identity that is associated with the sending authorization policy. This identity permits Amazon Cognito to send for the email address specified in the From parameter.
:�
�
cognitoNRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmail�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmail:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmailt
r%
htmlBody" The email HTML body.
"
subject" The email subject.
%
textBody" The email text body.
:�
�
cognitoLRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmailt
r%
htmlBody" The email HTML body.
"
subject" The email subject.
%
textBody" The email text body.
:�
�
cognitoQRiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmail�aws:cognito/RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmail:RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmailt
r%
htmlBody" The email HTML body.
"
subject" The email subject.
%
textBody" The email text body.
:�
�
cognito8RiskConfigurationCompromisedCredentialsRiskConfiguration}aws:cognito/RiskConfigurationCompromisedCredentialsRiskConfiguration:RiskConfigurationCompromisedCredentialsRiskConfiguration�
��
actions�:�
�
cognito?RiskConfigurationCompromisedCredentialsRiskConfigurationActions�aws:cognito/RiskConfigurationCompromisedCredentialsRiskConfigurationActions:RiskConfigurationCompromisedCredentialsRiskConfigurationActionsKThe compromised credentials risk configuration actions. See details below.
�
eventFiltersB*" �Perform the action for these events. The default is to perform all events if no event filter is specified. Valid values are `SIGN_IN`, `PASSWORD_CHANGE`, and `SIGN_UP`.
:�
�
cognito?RiskConfigurationCompromisedCredentialsRiskConfigurationActions�aws:cognito/RiskConfigurationCompromisedCredentialsRiskConfigurationActions:RiskConfigurationCompromisedCredentialsRiskConfigurationActionsR
PN
eventAction" ;The event action. Valid values are `BLOCK` or `NO_ACTION`.
:�
�
cognito+RiskConfigurationRiskExceptionConfigurationcaws:cognito/RiskConfigurationRiskExceptionConfiguration:RiskConfigurationRiskExceptionConfiguration�
��
blockedIpRangeListsB*" �Overrides the risk decision to always block the pre-authentication requests.
The IP range is in CIDR notation, a compact representation of an IP address and its routing prefix.
Can contain a maximum of 200 items.
�
skippedIpRangeListsB*" �Risk detection isn't performed on the IP addresses in this range list.
The IP range is in CIDR notation.
Can contain a maximum of 200 items.
:�
t
cognitoUserPoolAccountRecoverySettingIaws:cognito/UserPoolAccountRecoverySetting:UserPoolAccountRecoverySetting�
��
recoveryMechanisms�B�*�:�
�
cognito/UserPoolAccountRecoverySettingRecoveryMechanismkaws:cognito/UserPoolAccountRecoverySettingRecoveryMechanism:UserPoolAccountRecoverySettingRecoveryMechanism=List of Account Recovery Options of the following structure:
:�
�
cognito/UserPoolAccountRecoverySettingRecoveryMechanismkaws:cognito/UserPoolAccountRecoverySettingRecoveryMechanism:UserPoolAccountRecoverySettingRecoveryMechanism�
�~
name" rRecovery method for a user. Can be of the following: `verified_email`, `verified_phone_number`, and `admin_only`.
d
priority TPositive integer specifying priority of a method with 1 being the highest priority.
:�
q
cognitoUserPoolAdminCreateUserConfigGaws:cognito/UserPoolAdminCreateUserConfig:UserPoolAdminCreateUserConfig�
��
allowAdminCreateUserOnlyB
 �Set to True if only the administrator is allowed to create user profiles. Set to False if users can sign themselves up via an app.
�
inviteMessageTemplate�B�:�
�
cognito2UserPoolAdminCreateUserConfigInviteMessageTemplateqaws:cognito/UserPoolAdminCreateUserConfigInviteMessageTemplate:UserPoolAdminCreateUserConfigInviteMessageTemplate3Invite message template structure. Detailed below.
:�
�
cognito2UserPoolAdminCreateUserConfigInviteMessageTemplateqaws:cognito/UserPoolAdminCreateUserConfigInviteMessageTemplate:UserPoolAdminCreateUserConfigInviteMessageTemplate�
��
emailMessageB" �Message template for email messages. Must contain `{username}` and `{####}` placeholders, for username and temporary password, respectively.
7
emailSubjectB" !Subject line for email messages.
�

smsMessageB" �Message template for SMS messages. Must contain `{username}` and `{####}` placeholders, for username and temporary password, respectively.
:�
�
cognito$UserPoolClientAnalyticsConfigurationUaws:cognito/UserPoolClientAnalyticsConfiguration:UserPoolClientAnalyticsConfiguration�
�y
applicationArnB" aApplication ARN for an Amazon Pinpoint application. Conflicts with `external_id` and `role_arn`.
J
applicationIdB" 3Application ID for an Amazon Pinpoint application.
Z

externalIdB" FID for the Analytics Configuration. Conflicts with `application_arn`.
�
roleArnB" �ARN of an IAM role that authorizes Amazon Cognito to publish events to Amazon Pinpoint analytics. Conflicts with `application_arn`.
�
userDataSharedB
 qIf set to `true`, Amazon Cognito will include user data in the events it publishes to Amazon Pinpoint analytics.
:�
z
cognito UserPoolClientTokenValidityUnitsMaws:cognito/UserPoolClientTokenValidityUnits:UserPoolClientTokenValidityUnits�
�a
accessTokenB" LTime unit in for the value in `access_token_validity`, defaults to `hours`.
Y
idTokenB" HTime unit in for the value in `id_token_validity`, defaults to `hours`.
b
refreshTokenB" LTime unit in for the value in `refresh_token_validity`, defaults to `days`.
:�
k
cognitoUserPoolDeviceConfigurationCaws:cognito/UserPoolDeviceConfiguration:UserPoolDeviceConfiguration�
�x
challengeRequiredOnNewDeviceB
 RWhether a challenge is required on a new device. Only applicable to a new device.
�
 deviceOnlyRememberedOnUserPromptB
 �Whether a device is only remembered on user prompt. `false` equates to "Always" remember, `true` is "User Opt In," and not using a `device_configuration` block is "No."
:�
h
cognitoUserPoolEmailConfigurationAaws:cognito/UserPoolEmailConfiguration:UserPoolEmailConfiguration�
�A
configurationSetB" 'Email configuration set name from SES.
�
emailSendingAccountB" �Email delivery method to use. `COGNITO_DEFAULT` for the default email functionality built into Cognito or `DEVELOPER` to use your Amazon SES configuration. Required to be `DEVELOPER` if `from_email_address` is set.
�
fromEmailAddressB" �Sender’s email address or sender’s display name with their email address (e.g., `john@example.com`, `John Smith <john@example.com>` or `\"John Smith Ph.D.\" <john@example.com>`). Escaped double quotes are required around display names that contain certain characters as specified in [RFC 5322](https://tools.ietf.org/html/rfc5322).
5
replyToEmailAddressB" REPLY-TO email address.
}
	sourceArnB" jARN of the SES verified email identity to use. Required if `email_sending_account` is set to `DEVELOPER`.
:�
V
cognitoUserPoolLambdaConfig5aws:cognito/UserPoolLambdaConfig:UserPoolLambdaConfig�
�U
createAuthChallengeB" 8ARN of the lambda creating an authentication challenge.
�
customEmailSender�B�:�
�
cognito%UserPoolLambdaConfigCustomEmailSenderWaws:cognito/UserPoolLambdaConfigCustomEmailSender:UserPoolLambdaConfigCustomEmailSenderIA custom email sender AWS Lambda trigger. See custom_email_sender Below.
:
customMessageB" #Custom Message AWS Lambda trigger.
�
customSmsSender�B�:�
�
cognito#UserPoolLambdaConfigCustomSmsSenderSaws:cognito/UserPoolLambdaConfigCustomSmsSender:UserPoolLambdaConfigCustomSmsSenderEA custom SMS sender AWS Lambda trigger. See custom_sms_sender Below.
C
defineAuthChallengeB" &Defines the authentication challenge.
�
kmsKeyIdB" �The Amazon Resource Name of Key Management Service Customer master keys. Amazon Cognito uses the key to encrypt codes and temporary passwords sent to CustomEmailSender and CustomSMSSender.
D
postAuthenticationB" (Post-authentication AWS Lambda trigger.
@
postConfirmationB" &Post-confirmation AWS Lambda trigger.
B
preAuthenticationB" 'Pre-authentication AWS Lambda trigger.
8
	preSignUpB" %Pre-registration AWS Lambda trigger.
�
preTokenGenerationB" �Allow to customize identity token claims before token generation. Set this parameter for legacy purposes; for new instances of pre token generation triggers, set the lambda_arn of `pre_token_generation_config`.
�
preTokenGenerationConfig�B�:�
�
cognito,UserPoolLambdaConfigPreTokenGenerationConfigeaws:cognito/UserPoolLambdaConfigPreTokenGenerationConfig:UserPoolLambdaConfigPreTokenGenerationConfigCAllow to customize access tokens. See pre_token_configuration_type
:
userMigrationB" #User migration Lambda config type.
U
verifyAuthChallengeResponseB" 0Verifies the authentication challenge response.
:�
�
cognito%UserPoolLambdaConfigCustomEmailSenderWaws:cognito/UserPoolLambdaConfigCustomEmailSender:UserPoolLambdaConfigCustomEmailSender�
��
	lambdaArn" zThe Lambda Amazon Resource Name of the Lambda function that Amazon Cognito triggers to send email notifications to users.
�
lambdaVersion" �The Lambda version represents the signature of the "request" attribute in the "event" information Amazon Cognito passes to your custom email Lambda function. The only supported value is `V1_0`.
:�
�
cognito#UserPoolLambdaConfigCustomSmsSenderSaws:cognito/UserPoolLambdaConfigCustomSmsSender:UserPoolLambdaConfigCustomSmsSender�
��
	lambdaArn" xThe Lambda Amazon Resource Name of the Lambda function that Amazon Cognito triggers to send SMS notifications to users.
�
lambdaVersion" �The Lambda version represents the signature of the "request" attribute in the "event" information Amazon Cognito passes to your custom SMS Lambda function. The only supported value is `V1_0`.
:�
�
cognito,UserPoolLambdaConfigPreTokenGenerationConfigeaws:cognito/UserPoolLambdaConfigPreTokenGenerationConfig:UserPoolLambdaConfigPreTokenGenerationConfig(
&
	lambdaArn" 
lambdaVersion" :�

\
cognitoUserPoolPasswordPolicy9aws:cognito/UserPoolPasswordPolicy:UserPoolPasswordPolicy�	
�	P
minimumLengthB 9Minimum length of the password policy that you have set.
�
passwordHistorySizeB �Number of previous passwords that you want Amazon Cognito to restrict each user from reusing. Users can't set a password that matches any of number of previous passwords specified by this argument. A value of 0 means that password history is not enforced. Valid values are between 0 and 24.

**Note:** This argument requires advanced security features to be active in the user pool.
r
requireLowercaseB
 XWhether you have required users to use at least one lowercase letter in their password.
f
requireNumbersB
 NWhether you have required users to use at least one number in their password.
f
requireSymbolsB
 NWhether you have required users to use at least one symbol in their password.
r
requireUppercaseB
 XWhether you have required users to use at least one uppercase letter in their password.
�
temporaryPasswordValidityDaysB �In the password policy you have set, refers to the number of days a temporary password is valid. If the user does not sign-in during this time, their password will need to be reset by an administrator.
:�
D
cognitoUserPoolSchema)aws:cognito/UserPoolSchema:UserPoolSchema�
�h
attributeDataType" OAttribute data type. Must be one of `Boolean`, `Number`, `String`, `DateTime`.
N
developerOnlyAttributeB
 .Whether the attribute type is developer only.
P
mutableB
 ?Whether the attribute can be changed once it has been created.
#
name" Name of the attribute.
�
numberAttributeConstraints�B�:�
�
cognito(UserPoolSchemaNumberAttributeConstraints]aws:cognito/UserPoolSchemaNumberAttributeConstraints:UserPoolSchemaNumberAttributeConstraints]Configuration block for the constraints for an attribute of the number type. Detailed below.
�
requiredB
 �Whether a user pool attribute is required. If the attribute is required and the user does not provide a value, registration or sign-in will fail.
�
stringAttributeConstraints�B�:�
�
cognito(UserPoolSchemaStringAttributeConstraints]aws:cognito/UserPoolSchemaStringAttributeConstraints:UserPoolSchemaStringAttributeConstraintsAConstraints for an attribute of the string type. Detailed below.
:�
�
cognito(UserPoolSchemaNumberAttributeConstraints]aws:cognito/UserPoolSchemaNumberAttributeConstraints:UserPoolSchemaNumberAttributeConstraints�
�Q
maxValueB" ?Maximum value of an attribute that is of the number data type.
Q
minValueB" ?Minimum value of an attribute that is of the number data type.
:�
�
cognito(UserPoolSchemaStringAttributeConstraints]aws:cognito/UserPoolSchemaStringAttributeConstraints:UserPoolSchemaStringAttributeConstraints�
�L
	maxLengthB" 9Maximum length of an attribute value of the string type.
L
	minLengthB" 9Minimum length of an attribute value of the string type.
:�
b
cognitoUserPoolSmsConfiguration=aws:cognito/UserPoolSmsConfiguration:UserPoolSmsConfiguration�
��

externalId" �External ID used in IAM role trust relationships. For more information about using external IDs, see [How to Use an External ID When Granting Access to Your AWS Resources to a Third Party](http://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-user_externalid.html).

snsCallerArn" kARN of the Amazon SNS caller. This is usually the IAM role that you've given Cognito permission to assume.
�
	snsRegionB" �The AWS Region to use with Amazon SNS integration. You can choose the same Region as your user pool, or a supported Legacy Amazon SNS alternate Region. Amazon Cognito resources in the Asia Pacific (Seoul) AWS Region must use your Amazon SNS configuration in the Asia Pacific (Tokyo) Region. For more information, see [SMS message settings for Amazon Cognito user pools](https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-sms-settings.html).
:�
�
cognito%UserPoolSoftwareTokenMfaConfigurationWaws:cognito/UserPoolSoftwareTokenMfaConfiguration:UserPoolSoftwareTokenMfaConfiguration�
��
enabled
 �Boolean whether to enable software token Multi-Factor (MFA) tokens, such as Time-based One-Time Password (TOTP). To disable software token MFA When `sms_configuration` is not present, the `mfa_configuration` argument must be set to `OFF` and the `software_token_mfa_configuration` configuration block must be fully removed.
:�
�
cognito#UserPoolUserAttributeUpdateSettingsSaws:cognito/UserPoolUserAttributeUpdateSettings:UserPoolUserAttributeUpdateSettings�
��
*attributesRequireVerificationBeforeUpdates*" �A list of attributes requiring verification before update. If set, the provided value(s) must also be set in `auto_verified_attributes`. Valid values: `email`, `phone_number`.
:�
\
cognitoUserPoolUserPoolAddOns9aws:cognito/UserPoolUserPoolAddOns:UserPoolUserPoolAddOnsi
ge
advancedSecurityMode" IMode for advanced security, must be one of `OFF`, `AUDIT` or `ENFORCED`.
:�
q
cognitoUserPoolUsernameConfigurationGaws:cognito/UserPoolUsernameConfiguration:UserPoolUsernameConfiguration�
~|
caseSensitive
 gWhether username case sensitivity will be applied for all users in the user pool through Cognito APIs.
:�
�
cognito#UserPoolVerificationMessageTemplateSaws:cognito/UserPoolVerificationMessageTemplate:UserPoolVerificationMessageTemplate�
��
defaultEmailOptionB" rDefault email option. Must be either `CONFIRM_WITH_CODE` or `CONFIRM_WITH_LINK`. Defaults to `CONFIRM_WITH_CODE`.
�
emailMessageB" uEmail message template. Must contain the `{####}` placeholder. Conflicts with `email_verification_message` argument.
�
emailMessageByLinkB" xEmail message template for sending a confirmation link to the user, it must contain the `{##Click Here##}` placeholder.
y
emailSubjectB" cSubject line for the email message template. Conflicts with `email_verification_subject` argument.
u
emailSubjectByLinkB" YSubject line for the email message template for sending a confirmation link to the user.
�

smsMessageB" qSMS message template. Must contain the `{####}` placeholder. Conflicts with `sms_verification_message` argument.
:�
�
cognito&getIdentityPoolCognitoIdentityProviderYaws:cognito/getIdentityPoolCognitoIdentityProvider:getIdentityPoolCognitoIdentityProviderB
@
clientId" 
providerName" 
serverSideTokenCheck
 :�
P
cognitogetUserGroupsGroup1aws:cognito/getUserGroupsGroup:getUserGroupsGroup�
�2
description" Description of the user group.
)
	groupName" Name of the user group.
0

precedence Precedence of the user group.
I
roleArn" :ARN of the IAM role to be associated with the user group.
:�
}
cognito!getUserPoolAccountRecoverySettingOaws:cognito/getUserPoolAccountRecoverySetting:getUserPoolAccountRecoverySetting�
��
recoveryMechanisms�*�:�
�
cognito2getUserPoolAccountRecoverySettingRecoveryMechanismqaws:cognito/getUserPoolAccountRecoverySettingRecoveryMechanism:getUserPoolAccountRecoverySettingRecoveryMechanism:�
�
cognito2getUserPoolAccountRecoverySettingRecoveryMechanismqaws:cognito/getUserPoolAccountRecoverySettingRecoveryMechanism:getUserPoolAccountRecoverySettingRecoveryMechanism�
�%
name" - Name of the attribute.
j
priority Z- Priority of this mechanism in the recovery process (lower numbers are higher priority).
:�
z
cognito getUserPoolAdminCreateUserConfigMaws:cognito/getUserPoolAdminCreateUserConfig:getUserPoolAdminCreateUserConfig�
�H
allowAdminCreateUserOnly
 (- Whether only admins can create users.
�
inviteMessageTemplates�*�:�
�
cognito5getUserPoolAdminCreateUserConfigInviteMessageTemplatewaws:cognito/getUserPoolAdminCreateUserConfigInviteMessageTemplate:getUserPoolAdminCreateUserConfigInviteMessageTemplate�
unusedAccountValidityDays |- Number of days an unconfirmed user account remains valid.
* invite_message_template - Templates for invitation messages.
:�
�
cognito5getUserPoolAdminCreateUserConfigInviteMessageTemplatewaws:cognito/getUserPoolAdminCreateUserConfigInviteMessageTemplate:getUserPoolAdminCreateUserConfigInviteMessageTemplate�
�-
emailMessage" - Email message content.
-
emailSubject" - Email message subject.
)

smsMessage" - SMS message content.
:�
�
cognito'getUserPoolClientAnalyticsConfiguration[aws:cognito/getUserPoolClientAnalyticsConfiguration:getUserPoolClientAnalyticsConfiguration�
��
applicationArn" l(Optional) Application ARN for an Amazon Pinpoint application. Conflicts with `external_id` and `role_arn`.
S
applicationId" >(Optional) Application ID for an Amazon Pinpoint application.
c

externalId" Q(Optional) ID for the Analytics Configuration. Conflicts with `application_arn`.
�
roleArn" �(Optional) ARN of an IAM role that authorizes Amazon Cognito to publish events to Amazon Pinpoint analytics. Conflicts with `application_arn`.
�
userDataShared
 |(Optional) If set to `true`, Amazon Cognito will include user data in the events it publishes to Amazon Pinpoint analytics.
:�
�
cognito"getUserPoolClientTokenValidityUnitQaws:cognito/getUserPoolClientTokenValidityUnit:getUserPoolClientTokenValidityUnit�
�j
accessToken" W(Optional) Time unit in for the value in `access_token_validity`, defaults to `hours`.
b
idToken" S(Optional) Time unit in for the value in `id_token_validity`, defaults to `hours`.
k
refreshToken" W(Optional) Time unit in for the value in `refresh_token_validity`, defaults to `days`.
:�
t
cognitogetUserPoolDeviceConfigurationIaws:cognito/getUserPoolDeviceConfiguration:getUserPoolDeviceConfiguration�
�V
challengeRequiredOnNewDevice
 2- Whether a challenge is required on new devices.
f
 deviceOnlyRememberedOnUserPrompt
 >- Whether devices are only remembered if the user prompts it.
:�
q
cognitogetUserPoolEmailConfigurationGaws:cognito/getUserPoolEmailConfiguration:getUserPoolEmailConfiguration�
�E
configurationSet" -- Configuration set used for sending emails.
4
emailSendingAccount" - Email sending account.
$
from" - Email sender address.
5
replyToEmailAddress" - Reply-to email address.
A
	sourceArn" 0- Source Amazon Resource Name (ARN) for emails.
:�
_
cognitogetUserPoolLambdaConfig;aws:cognito/getUserPoolLambdaConfig:getUserPoolLambdaConfig�
�
createAuthChallenge" �
customEmailSenders�*�:�
�
cognito(getUserPoolLambdaConfigCustomEmailSender]aws:cognito/getUserPoolLambdaConfigCustomEmailSender:getUserPoolLambdaConfigCustomEmailSender
customMessage" �
customSmsSenders�*�:�
�
cognito&getUserPoolLambdaConfigCustomSmsSenderYaws:cognito/getUserPoolLambdaConfigCustomSmsSender:getUserPoolLambdaConfigCustomSmsSender
defineAuthChallenge" 
kmsKeyId" 
postAuthentication" 
postConfirmation" 
preAuthentication" 
	preSignUp" 
preTokenGeneration" �
preTokenGenerationConfigs�*�:�
�
cognito/getUserPoolLambdaConfigPreTokenGenerationConfigkaws:cognito/getUserPoolLambdaConfigPreTokenGenerationConfig:getUserPoolLambdaConfigPreTokenGenerationConfig
userMigration" !
verifyAuthChallengeResponse" :�
�
cognito(getUserPoolLambdaConfigCustomEmailSender]aws:cognito/getUserPoolLambdaConfigCustomEmailSender:getUserPoolLambdaConfigCustomEmailSenderl
j/
	lambdaArn" - ARN of the Lambda function.
7
lambdaVersion" "- Version of the Lambda function.
:�
�
cognito&getUserPoolLambdaConfigCustomSmsSenderYaws:cognito/getUserPoolLambdaConfigCustomSmsSender:getUserPoolLambdaConfigCustomSmsSenderl
j/
	lambdaArn" - ARN of the Lambda function.
7
lambdaVersion" "- Version of the Lambda function.
:�
�
cognito/getUserPoolLambdaConfigPreTokenGenerationConfigkaws:cognito/getUserPoolLambdaConfigPreTokenGenerationConfig:getUserPoolLambdaConfigPreTokenGenerationConfigl
j/
	lambdaArn" - ARN of the Lambda function.
7
lambdaVersion" "- Version of the Lambda function.
:�
h
cognitogetUserPoolSchemaAttributeAaws:cognito/getUserPoolSchemaAttribute:getUserPoolSchemaAttribute�
�P
attributeDataType" 7- Data type of the attribute (e.g., string, number).
Q
developerOnlyAttribute
 3- Whether the attribute is for developer use only.
K
mutable
 <- Whether the attribute can be changed after user creation.
%
name" - Name of the attribute.
�
numberAttributeConstraints�*�:�
�
cognito3getUserPoolSchemaAttributeNumberAttributeConstraintsaws:cognito/getUserPoolSchemaAttributeNumberAttributeConstraint:getUserPoolSchemaAttributeNumberAttributeConstraint�
required
 �- Whether the attribute is required during user registration.
* number_attribute_constraints - Constraints for numeric attributes.
* string_attribute_constraints - Constraints for string attributes.
�
stringAttributeConstraints�*�:�
�
cognito3getUserPoolSchemaAttributeStringAttributeConstraintsaws:cognito/getUserPoolSchemaAttributeStringAttributeConstraint:getUserPoolSchemaAttributeStringAttributeConstraint:�
�
cognito3getUserPoolSchemaAttributeNumberAttributeConstraintsaws:cognito/getUserPoolSchemaAttributeNumberAttributeConstraint:getUserPoolSchemaAttributeNumberAttributeConstraintX
V)
maxValue" - Maximum allowed value.
)
minValue" - Minimum allowed value.
:�
�
cognito3getUserPoolSchemaAttributeStringAttributeConstraintsaws:cognito/getUserPoolSchemaAttributeStringAttributeConstraint:getUserPoolSchemaAttributeStringAttributeConstraint\
Z+
	maxLength" - Maximum allowed length.
+
	minLength" - Minimum allowed length.
:�
�

comprehend!DocumentClassifierInputDataConfigRaws:comprehend/DocumentClassifierInputDataConfig:DocumentClassifierInputDataConfig�
��
augmentedManifests�B�*�:�
�

comprehend2DocumentClassifierInputDataConfigAugmentedManifesttaws:comprehend/DocumentClassifierInputDataConfigAugmentedManifest:DocumentClassifierInputDataConfigAugmentedManifest�List of training datasets produced by Amazon SageMaker Ground Truth.
Used if `data_format` is `AUGMENTED_MANIFEST`.
See the `augmented_manifests` Configuration Block section below.
g

dataFormatB" SThe format for the training data.
One of `COMPREHEND_CSV` or `AUGMENTED_MANIFEST`.
�
labelDelimiterB" �Delimiter between labels when training a multi-label classifier.
Valid values are `|`, `~`, `!`, `@`, `#`, `$`, `%`, `^`, `*`, `-`, `_`, `+`, `=`, `\`, `:`, `;`, `>`, `?`, `/`, `<space>`, and `<tab>`.
Default is `|`.
Z
s3UriB" KLocation of training documents.
Used if `data_format` is `COMPREHEND_CSV`.

	testS3UriB" :�
�

comprehend2DocumentClassifierInputDataConfigAugmentedManifesttaws:comprehend/DocumentClassifierInputDataConfigAugmentedManifest:DocumentClassifierInputDataConfigAugmentedManifest�
�;
annotationDataS3UriB" Location of annotation files.
e
attributeNames*" MThe JSON attribute that contains the annotations for the training documents.
n
documentTypeB" XType of augmented manifest.
One of `PLAIN_TEXT_DOCUMENT` or `SEMI_STRUCTURED_DOCUMENT`.
2
s3Uri" %Location of augmented manifest file.
<
sourceDocumentsS3UriB" Location of source PDF files.
P
splitB" APurpose of data in augmented manifest.
One of `TRAIN` or `TEST`.
:�
�

comprehend"DocumentClassifierOutputDataConfigTaws:comprehend/DocumentClassifierOutputDataConfig:DocumentClassifierOutputDataConfig�
��
kmsKeyIdB" xKMS Key used to encrypt the output documents.
Can be a KMS Key ID, a KMS Key ARN, a KMS Alias name, or a KMS Alias ARN.
9
outputS3UriB" $Full path for the output documents.
~
s3Uri" qDestination path for the output documents.
The full path to the output file will be returned in `output_s3_uri`.
:�
q

comprehendDocumentClassifierVpcConfigFaws:comprehend/DocumentClassifierVpcConfig:DocumentClassifierVpcConfigb
`6
securityGroupIds*" List of security group IDs.
&
subnets*" List of VPC subnets.
:�
}

comprehendEntityRecognizerInputDataConfigNaws:comprehend/EntityRecognizerInputDataConfig:EntityRecognizerInputDataConfig�
��
annotations�B�:�
�

comprehend*EntityRecognizerInputDataConfigAnnotationsdaws:comprehend/EntityRecognizerInputDataConfigAnnotations:EntityRecognizerInputDataConfigAnnotations�Specifies location of the document annotation data.
See the `annotations` Configuration Block section below.
One of `annotations` or `entity_list` is required.
�
augmentedManifests�B�*�:�
�

comprehend0EntityRecognizerInputDataConfigAugmentedManifestpaws:comprehend/EntityRecognizerInputDataConfigAugmentedManifest:EntityRecognizerInputDataConfigAugmentedManifest�List of training datasets produced by Amazon SageMaker Ground Truth.
Used if `data_format` is `AUGMENTED_MANIFEST`.
See the `augmented_manifests` Configuration Block section below.
g

dataFormatB" SThe format for the training data.
One of `COMPREHEND_CSV` or `AUGMENTED_MANIFEST`.
�
	documents�B�:�
�

comprehend(EntityRecognizerInputDataConfigDocuments`aws:comprehend/EntityRecognizerInputDataConfigDocuments:EntityRecognizerInputDataConfigDocuments�Specifies a collection of training documents.
Used if `data_format` is `COMPREHEND_CSV`.
See the `documents` Configuration Block section below.
�

entityList�B�:�
�

comprehend)EntityRecognizerInputDataConfigEntityListbaws:comprehend/EntityRecognizerInputDataConfigEntityList:EntityRecognizerInputDataConfigEntityList�Specifies location of the entity list data.
See the `entity_list` Configuration Block section below.
One of `entity_list` or `annotations` is required.
�
entityTypes�*�:�
�

comprehend)EntityRecognizerInputDataConfigEntityTypebaws:comprehend/EntityRecognizerInputDataConfigEntityType:EntityRecognizerInputDataConfigEntityType{Set of entity types to be recognized.
Has a maximum of 25 items.
See the `entity_types` Configuration Block section below.
:�
�

comprehend*EntityRecognizerInputDataConfigAnnotationsdaws:comprehend/EntityRecognizerInputDataConfigAnnotations:EntityRecognizerInputDataConfigAnnotationsF
D/
s3Uri" "Location of training annotations.

	testS3UriB" :�
�

comprehend0EntityRecognizerInputDataConfigAugmentedManifestpaws:comprehend/EntityRecognizerInputDataConfigAugmentedManifest:EntityRecognizerInputDataConfigAugmentedManifest�
�;
annotationDataS3UriB" Location of annotation files.
e
attributeNames*" MThe JSON attribute that contains the annotations for the training documents.
n
documentTypeB" XType of augmented manifest.
One of `PLAIN_TEXT_DOCUMENT` or `SEMI_STRUCTURED_DOCUMENT`.
2
s3Uri" %Location of augmented manifest file.
<
sourceDocumentsS3UriB" Location of source PDF files.
P
splitB" APurpose of data in augmented manifest.
One of `TRAIN` or `TEST`.
:�
�

comprehend(EntityRecognizerInputDataConfigDocuments`aws:comprehend/EntityRecognizerInputDataConfigDocuments:EntityRecognizerInputDataConfigDocuments�
�y
inputFormatB" dSpecifies how the input files should be processed.
One of `ONE_DOC_PER_LINE` or `ONE_DOC_PER_FILE`.
-
s3Uri"  Location of training documents.

	testS3UriB" :�
�

comprehend)EntityRecognizerInputDataConfigEntityListbaws:comprehend/EntityRecognizerInputDataConfigEntityList:EntityRecognizerInputDataConfigEntityList*
(&
s3Uri" Location of entity list.
:�
�

comprehend)EntityRecognizerInputDataConfigEntityTypebaws:comprehend/EntityRecognizerInputDataConfigEntityType:EntityRecognizerInputDataConfigEntityType�
��
type" An entity type to be matched by the Entity Recognizer.
Cannot contain a newline (`\n`), carriage return (`\r`), or tab (`\t`).
:�
k

comprehendEntityRecognizerVpcConfigBaws:comprehend/EntityRecognizerVpcConfig:EntityRecognizerVpcConfigb
`6
securityGroupIds*" List of security group IDs.
&
subnets*" List of VPC subnets.
:�
t
computeoptimizerEnrollmentStatusTimeoutsFaws:computeoptimizer/EnrollmentStatusTimeouts:EnrollmentStatusTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
�
computeoptimizer2RecommendationPreferencesExternalMetricsPreferencezaws:computeoptimizer/RecommendationPreferencesExternalMetricsPreference:RecommendationPreferencesExternalMetricsPreference�
��
source" rThe source options for external metrics preferences. Valid values: `Datadog`, `Dynatrace`, `NewRelic`, `Instana`.
:�
�
computeoptimizer*RecommendationPreferencesPreferredResourcejaws:computeoptimizer/RecommendationPreferencesPreferredResource:RecommendationPreferencesPreferredResource�
��
excludeListsB*" �The preferred resource type values to exclude from the recommendation candidates. If this isn’t specified, all supported resources are included by default.
�
includeListsB*" �The preferred resource type values to include in the recommendation candidates. You can specify the exact resource type value, such as `"m5.large"`, or use wild card expressions, such as `"m5"`. If this isn’t specified, all supported resources are included by default.


name" :�
�
computeoptimizerRecommendationPreferencesScopeRaws:computeoptimizer/RecommendationPreferencesScope:RecommendationPreferencesScope�
�]
name" QThe name of the scope. Valid values: `Organization`, `AccountId`, `ResourceArn`.
�
value" �The value of the scope. `ALL_ACCOUNTS` for `Organization` scopes, AWS account ID for `AccountId` scopes, ARN of an EC2 instance or an Auto Scaling group for `ResourceArn` scopes.
:�
�
computeoptimizer.RecommendationPreferencesUtilizationPreferenceraws:computeoptimizer/RecommendationPreferencesUtilizationPreference:RecommendationPreferencesUtilizationPreference�
��

metricName" tThe name of the resource utilization metric name to customize. Valid values: `CpuUtilization`, `MemoryUtilization`.
�
metricParameters�B�:�
�
computeoptimizer>RecommendationPreferencesUtilizationPreferenceMetricParameters�aws:computeoptimizer/RecommendationPreferencesUtilizationPreferenceMetricParameters:RecommendationPreferencesUtilizationPreferenceMetricParametersLThe parameters to set when customizing the resource utilization thresholds.
:�
�
computeoptimizer>RecommendationPreferencesUtilizationPreferenceMetricParameters�aws:computeoptimizer/RecommendationPreferencesUtilizationPreferenceMetricParameters:RecommendationPreferencesUtilizationPreferenceMetricParameters�
��
headroom" �The headroom value in percentage used for the specified metric parameter. Valid values: `PERCENT_30`, `PERCENT_20`, `PERCENT_10`, `PERCENT_0`.
�
	thresholdB" �The threshold value used for the specified metric parameter. You can only specify the threshold value for CPU utilization. Valid values: `P90`, `P95`, `P99_5`.
:�
V
connectBotAssociationLexBot5aws:connect/BotAssociationLexBot:BotAssociationLexBot�
�g
	lexRegionB" TThe Region that the Amazon Lex (V1) bot was created in. Defaults to current region.
1
name" %The name of the Amazon Lex (V1) bot.
:�
\
connectHoursOfOperationConfig9aws:connect/HoursOfOperationConfig:HoursOfOperationConfig�
�E
day" :Specifies the day that the hours of operation applies to.
�
endTimeu:s
q
connectHoursOfOperationConfigEndTimeGaws:connect/HoursOfOperationConfigEndTime:HoursOfOperationConfigEndTimeiA end time block specifies the time that your contact center closes. The `end_time` is documented below.
�
	startTime{:y
w
connectHoursOfOperationConfigStartTimeKaws:connect/HoursOfOperationConfigStartTime:HoursOfOperationConfigStartTimelA start time block specifies the time that your contact center opens. The `start_time` is documented below.
:�
q
connectHoursOfOperationConfigEndTimeGaws:connect/HoursOfOperationConfigEndTime:HoursOfOperationConfigEndTimeb
`,
hours Specifies the hour of closing.
0
minutes !Specifies the minute of closing.
:�
w
connectHoursOfOperationConfigStartTimeKaws:connect/HoursOfOperationConfigStartTime:HoursOfOperationConfigStartTimeb
`,
hours Specifies the hour of opening.
0
minutes !Specifies the minute of opening.
:�
�
connect"InstanceStorageConfigStorageConfigQaws:connect/InstanceStorageConfigStorageConfig:InstanceStorageConfigStorageConfig�

�
�
kinesisFirehoseConfig�B�:�
�
connect7InstanceStorageConfigStorageConfigKinesisFirehoseConfig{aws:connect/InstanceStorageConfigStorageConfigKinesisFirehoseConfig:InstanceStorageConfigStorageConfigKinesisFirehoseConfigdA block that specifies the configuration of the Kinesis Firehose delivery stream. Documented below.
�
kinesisStreamConfig�B�:�
�
connect5InstanceStorageConfigStorageConfigKinesisStreamConfigwaws:connect/InstanceStorageConfigStorageConfigKinesisStreamConfig:InstanceStorageConfigStorageConfigKinesisStreamConfigWA block that specifies the configuration of the Kinesis data stream. Documented below.
�
kinesisVideoStreamConfig�B�:�
�
connect:InstanceStorageConfigStorageConfigKinesisVideoStreamConfig�aws:connect/InstanceStorageConfigStorageConfigKinesisVideoStreamConfig:InstanceStorageConfigStorageConfigKinesisVideoStreamConfigXA block that specifies the configuration of the Kinesis video stream. Documented below.
�
s3Config�B�:�
�
connect*InstanceStorageConfigStorageConfigS3Configaaws:connect/InstanceStorageConfigStorageConfigS3Config:InstanceStorageConfigStorageConfigS3ConfigIA block that specifies the configuration of S3 Bucket. Documented below.
~
storageType" kA valid storage type. Valid Values: `S3` | `KINESIS_VIDEO_STREAM` | `KINESIS_STREAM` | `KINESIS_FIREHOSE`.
:�
�
connect7InstanceStorageConfigStorageConfigKinesisFirehoseConfig{aws:connect/InstanceStorageConfigStorageConfigKinesisFirehoseConfig:InstanceStorageConfigStorageConfigKinesisFirehoseConfigN
LJ
firehoseArn" 7The Amazon Resource Name (ARN) of the delivery stream.
:�
�
connect5InstanceStorageConfigStorageConfigKinesisStreamConfigwaws:connect/InstanceStorageConfigStorageConfigKinesisStreamConfig:InstanceStorageConfigStorageConfigKinesisStreamConfigH
FD
	streamArn" 3The Amazon Resource Name (ARN) of the data stream.
:�
�
connect:InstanceStorageConfigStorageConfigKinesisVideoStreamConfig�aws:connect/InstanceStorageConfigStorageConfigKinesisVideoStreamConfig:InstanceStorageConfigStorageConfigKinesisVideoStreamConfig�
��
encryptionConfig�:�
�
connectJInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig�aws:connect/InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig:InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig0The encryption configuration. Documented below.
�
prefix" �The prefix of the video stream. Minimum length of `1`. Maximum length of `128`. When read from the state, the value returned is `<prefix>-connect-<connect_instance_alias>-contact-` since the API appends additional details to the `prefix`.
�
retentionPeriodHours �The number of hours data is retained in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream. Minimum value of `0`. Maximum value of `87600`. A value of `0`, indicates that the stream does not persist data.
:�
�
connectJInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig�aws:connect/InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig:InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig�
�C
encryptionType" -The type of encryption. Valid Values: `KMS`.
y
keyId" lThe full ARN of the encryption key. Be sure to provide the full ARN of the encryption key, not just the ID.
:�
�
connect*InstanceStorageConfigStorageConfigS3Configaaws:connect/InstanceStorageConfigStorageConfigS3Config:InstanceStorageConfigStorageConfigS3Config�
�&

bucketName" The S3 bucket name.
*
bucketPrefix" The S3 bucket prefix.
�
encryptionConfig�B�:�
�
connect:InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig�aws:connect/InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig:InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig0The encryption configuration. Documented below.
:�
�
connect:InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig�aws:connect/InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig:InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig�
�C
encryptionType" -The type of encryption. Valid Values: `KMS`.
y
keyId" lThe full ARN of the encryption key. Be sure to provide the full ARN of the encryption key, not just the ID.
:�
M
connectPhoneNumberStatus/aws:connect/PhoneNumberStatus:PhoneNumberStatus�
�%
messageB" The status message.
d
statusB" TThe status of the phone number. Valid Values: `CLAIMED` | `IN_PROGRESS` | `FAILED`.
:�
e
connectQueueOutboundCallerConfig?aws:connect/QueueOutboundCallerConfig:QueueOutboundCallerConfig�
�<
outboundCallerIdNameB" Specifies the caller ID name.
B
outboundCallerIdNumberIdB"  Specifies the caller ID number.
\
outboundFlowIdB" DSpecifies outbound whisper flow to be used during an outbound call.
:�

t
connectQuickConnectQuickConnectConfigIaws:connect/QuickConnectQuickConnectConfig:QuickConnectQuickConnectConfig�	
�	�
phoneConfigs�B�*�:�
�
connect)QuickConnectQuickConnectConfigPhoneConfig_aws:connect/QuickConnectQuickConnectConfigPhoneConfig:QuickConnectQuickConnectConfigPhoneConfig�Specifies the phone configuration of the Quick Connect. This is required only if `quick_connect_type` is `PHONE_NUMBER`. The `phone_config` block is documented below.
�
queueConfigs�B�*�:�
�
connect)QuickConnectQuickConnectConfigQueueConfig_aws:connect/QuickConnectQuickConnectConfigQueueConfig:QuickConnectQuickConnectConfigQueueConfig�Specifies the queue configuration of the Quick Connect. This is required only if `quick_connect_type` is `QUEUE`. The `queue_config` block is documented below.
�
quickConnectType" iSpecifies the configuration type of the quick connect. valid values are `PHONE_NUMBER`, `QUEUE`, `USER`.
�
userConfigs�B�*�:�
�
connect(QuickConnectQuickConnectConfigUserConfig]aws:connect/QuickConnectQuickConnectConfigUserConfig:QuickConnectQuickConnectConfigUserConfig�Specifies the user configuration of the Quick Connect. This is required only if `quick_connect_type` is `USER`. The `user_config` block is documented below.
:�
�
connect)QuickConnectQuickConnectConfigPhoneConfig_aws:connect/QuickConnectQuickConnectConfigPhoneConfig:QuickConnectQuickConnectConfigPhoneConfigF
DB
phoneNumber" /Specifies the phone number in in E.164 format.
:�
�
connect)QuickConnectQuickConnectConfigQueueConfig_aws:connect/QuickConnectQuickConnectConfigQueueConfig:QuickConnectQuickConnectConfigQueueConfig�
~C
contactFlowId" .Specifies the identifier of the contact flow.
7
queueId" (Specifies the identifier for the queue.
:�
�
connect(QuickConnectQuickConnectConfigUserConfig]aws:connect/QuickConnectQuickConnectConfigUserConfig:QuickConnectQuickConnectConfigUserConfig~
|C
contactFlowId" .Specifies the identifier of the contact flow.
5
userId" 'Specifies the identifier for the user.
:�
t
connectRoutingProfileMediaConcurrencyIaws:connect/RoutingProfileMediaConcurrency:RoutingProfileMediaConcurrency�
��
channel" |Specifies the channels that agents can handle in the Contact Control Panel (CCP). Valid values are `VOICE`, `CHAT`, `TASK`.
�
concurrency �Specifies the number of contacts an agent can have on a channel simultaneously. Valid Range for `VOICE`: Minimum value of 1. Maximum value of 1. Valid Range for `CHAT`: Minimum value of 1. Maximum value of 10. Valid Range for `TASK`: Minimum value of 1. Maximum value of 10.
:�
e
connectRoutingProfileQueueConfig?aws:connect/RoutingProfileQueueConfig:RoutingProfileQueueConfig�
��
channel" �Specifies the channels agents can handle in the Contact Control Panel (CCP) for this routing profile. Valid values are `VOICE`, `CHAT`, `TASK`.
�
delay tSpecifies the delay, in seconds, that a contact should be in the queue before they are routed to an available agent
W
priority GSpecifies the order in which contacts are to be handled for the queue.
%
queueArnB" ARN for the queue.
7
queueId" (Specifies the identifier for the queue.
'
	queueNameB" Name for the queue.
:�
w
connectUserHierarchyGroupHierarchyPathKaws:connect/UserHierarchyGroupHierarchyPath:UserHierarchyGroupHierarchyPath�

�
�

levelFives�B�*�:�
�
connect(UserHierarchyGroupHierarchyPathLevelFife]aws:connect/UserHierarchyGroupHierarchyPathLevelFife:UserHierarchyGroupHierarchyPathLevelFifeUA block that defines the details of level five. The level block is documented below.
�

levelFours�B�*�:�
�
connect(UserHierarchyGroupHierarchyPathLevelFour]aws:connect/UserHierarchyGroupHierarchyPathLevelFour:UserHierarchyGroupHierarchyPathLevelFourUA block that defines the details of level four. The level block is documented below.
�
	levelOnes�B�*�:�
�
connect'UserHierarchyGroupHierarchyPathLevelOne[aws:connect/UserHierarchyGroupHierarchyPathLevelOne:UserHierarchyGroupHierarchyPathLevelOneTA block that defines the details of level one. The level block is documented below.
�
levelThrees�B�*�:�
�
connect)UserHierarchyGroupHierarchyPathLevelThree_aws:connect/UserHierarchyGroupHierarchyPathLevelThree:UserHierarchyGroupHierarchyPathLevelThreeVA block that defines the details of level three. The level block is documented below.
�
	levelTwos�B�*�:�
�
connect'UserHierarchyGroupHierarchyPathLevelTwo[aws:connect/UserHierarchyGroupHierarchyPathLevelTwo:UserHierarchyGroupHierarchyPathLevelTwoTA block that defines the details of level two. The level block is documented below.
:�
�
connect(UserHierarchyGroupHierarchyPathLevelFife]aws:connect/UserHierarchyGroupHierarchyPathLevelFife:UserHierarchyGroupHierarchyPathLevelFife�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy group.
3
idB" 'The identifier of the hierarchy group.
Z
nameB" LThe name of the user hierarchy group. Must not be more than 100 characters.
:�
�
connect(UserHierarchyGroupHierarchyPathLevelFour]aws:connect/UserHierarchyGroupHierarchyPathLevelFour:UserHierarchyGroupHierarchyPathLevelFour�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy group.
3
idB" 'The identifier of the hierarchy group.
Z
nameB" LThe name of the user hierarchy group. Must not be more than 100 characters.
:�
�
connect'UserHierarchyGroupHierarchyPathLevelOne[aws:connect/UserHierarchyGroupHierarchyPathLevelOne:UserHierarchyGroupHierarchyPathLevelOne�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy group.
3
idB" 'The identifier of the hierarchy group.
Z
nameB" LThe name of the user hierarchy group. Must not be more than 100 characters.
:�
�
connect)UserHierarchyGroupHierarchyPathLevelThree_aws:connect/UserHierarchyGroupHierarchyPathLevelThree:UserHierarchyGroupHierarchyPathLevelThree�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy group.
3
idB" 'The identifier of the hierarchy group.
Z
nameB" LThe name of the user hierarchy group. Must not be more than 100 characters.
:�
�
connect'UserHierarchyGroupHierarchyPathLevelTwo[aws:connect/UserHierarchyGroupHierarchyPathLevelTwo:UserHierarchyGroupHierarchyPathLevelTwo�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy group.
3
idB" 'The identifier of the hierarchy group.
Z
nameB" LThe name of the user hierarchy group. Must not be more than 100 characters.
:�
�
connect(UserHierarchyStructureHierarchyStructure]aws:connect/UserHierarchyStructureHierarchyStructure:UserHierarchyStructureHierarchyStructure�
��
	levelFive�B�:�
�
connect1UserHierarchyStructureHierarchyStructureLevelFiveoaws:connect/UserHierarchyStructureHierarchyStructureLevelFive:UserHierarchyStructureHierarchyStructureLevelFive�A block that defines the details of level five. The level block is documented below.

Each level block supports the following arguments:
�
	levelFour�B�:�
�
connect1UserHierarchyStructureHierarchyStructureLevelFouroaws:connect/UserHierarchyStructureHierarchyStructureLevelFour:UserHierarchyStructureHierarchyStructureLevelFourUA block that defines the details of level four. The level block is documented below.
�
levelOne�B�:�
�
connect0UserHierarchyStructureHierarchyStructureLevelOnemaws:connect/UserHierarchyStructureHierarchyStructureLevelOne:UserHierarchyStructureHierarchyStructureLevelOneTA block that defines the details of level one. The level block is documented below.
�

levelThree�B�:�
�
connect2UserHierarchyStructureHierarchyStructureLevelThreeqaws:connect/UserHierarchyStructureHierarchyStructureLevelThree:UserHierarchyStructureHierarchyStructureLevelThreeVA block that defines the details of level three. The level block is documented below.
�
levelTwo�B�:�
�
connect0UserHierarchyStructureHierarchyStructureLevelTwomaws:connect/UserHierarchyStructureHierarchyStructureLevelTwo:UserHierarchyStructureHierarchyStructureLevelTwoTA block that defines the details of level two. The level block is documented below.
:�
�
connect1UserHierarchyStructureHierarchyStructureLevelFiveoaws:connect/UserHierarchyStructureHierarchyStructureLevelFive:UserHierarchyStructureHierarchyStructureLevelFive�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy level.
3
idB" 'The identifier of the hierarchy level.
W
name" KThe name of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect1UserHierarchyStructureHierarchyStructureLevelFouroaws:connect/UserHierarchyStructureHierarchyStructureLevelFour:UserHierarchyStructureHierarchyStructureLevelFour�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy level.
3
idB" 'The identifier of the hierarchy level.
W
name" KThe name of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect0UserHierarchyStructureHierarchyStructureLevelOnemaws:connect/UserHierarchyStructureHierarchyStructureLevelOne:UserHierarchyStructureHierarchyStructureLevelOne�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy level.
3
idB" 'The identifier of the hierarchy level.
W
name" KThe name of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect2UserHierarchyStructureHierarchyStructureLevelThreeqaws:connect/UserHierarchyStructureHierarchyStructureLevelThree:UserHierarchyStructureHierarchyStructureLevelThree�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy level.
3
idB" 'The identifier of the hierarchy level.
W
name" KThe name of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect0UserHierarchyStructureHierarchyStructureLevelTwomaws:connect/UserHierarchyStructureHierarchyStructureLevelTwo:UserHierarchyStructureHierarchyStructureLevelTwo�
�D
arnB" 7The Amazon Resource Name (ARN) of the hierarchy level.
3
idB" 'The identifier of the hierarchy level.
W
name" KThe name of the user hierarchy level. Must not be more than 50 characters.
:�

J
connectUserIdentityInfo-aws:connect/UserIdentityInfo:UserIdentityInfo�	
�	�
emailB" �The email address. If you are using SAML for identity management and include this parameter, an error is returned. Note that updates to the `email` is supported. From the [UpdateUserIdentityInfo API documentation](https://docs.aws.amazon.com/connect/latest/APIReference/API_UpdateUserIdentityInfo.html) it is strongly recommended to limit who has the ability to invoke `UpdateUserIdentityInfo`. Someone with that ability can change the login credentials of other users by changing their email address. This poses a security risk to your organization. They can change the email address of a user to the attacker's email address, and then reset the password through email. For more information, see [Best Practices for Security Profiles](https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html) in the Amazon Connect Administrator Guide.
�
	firstNameB" �The first name. This is required if you are using Amazon Connect or SAML for identity management. Minimum length of 1. Maximum length of 100.
�
lastNameB" �The last name. This is required if you are using Amazon Connect or SAML for identity management. Minimum length of 1. Maximum length of 100.
:�
G
connectUserPhoneConfig+aws:connect/UserPhoneConfig:UserPhoneConfig�
�n
afterContactWorkTimeLimitB KThe After Call Work (ACW) timeout setting, in seconds. Minimum value of 0.
{

autoAcceptB
 gWhen Auto-Accept Call is enabled for an available agent, the agent connects to contacts automatically.
v
deskPhoneNumberB" ]The phone number for the user's desk phone. Required if `phone_type` is set as `DESK_PHONE`.
Q
	phoneType" @The phone type. Valid values are `DESK_PHONE` and `SOFT_PHONE`.
:�
_
connectgetBotAssociationLexBot;aws:connect/getBotAssociationLexBot:getBotAssociationLexBotx
vE
	lexRegion" 4Region that the Amazon Lex (V1) bot was created in.
-
name" !Name of the Amazon Lex (V1) bot.
:�
e
connectgetHoursOfOperationConfig?aws:connect/getHoursOfOperationConfig:getHoursOfOperationConfig�
�7
day" ,Day that the hours of operation applies to.
�
endTimes�*~:|
z
connect getHoursOfOperationConfigEndTimeMaws:connect/getHoursOfOperationConfigEndTime:getHoursOfOperationConfigEndTimegEnd time block specifies the time that your contact center closes. The `end_time` is documented below.
�

startTimes�*�:�
�
connect"getHoursOfOperationConfigStartTimeQaws:connect/getHoursOfOperationConfigStartTime:getHoursOfOperationConfigStartTimejStart time block specifies the time that your contact center opens. The `start_time` is documented below.
:�
z
connect getHoursOfOperationConfigEndTimeMaws:connect/getHoursOfOperationConfigEndTime:getHoursOfOperationConfigEndTimeF
D
hours Hour of opening.
"
minutes Minute of opening.
:�
�
connect"getHoursOfOperationConfigStartTimeQaws:connect/getHoursOfOperationConfigStartTime:getHoursOfOperationConfigStartTimeF
D
hours Hour of opening.
"
minutes Minute of opening.
:�
�
connect%getInstanceStorageConfigStorageConfigWaws:connect/getInstanceStorageConfigStorageConfig:getInstanceStorageConfigStorageConfig�

�
�
kinesisFirehoseConfigs�*�:�
�
connect:getInstanceStorageConfigStorageConfigKinesisFirehoseConfig�aws:connect/getInstanceStorageConfigStorageConfigKinesisFirehoseConfig:getInstanceStorageConfigStorageConfigKinesisFirehoseConfigdA block that specifies the configuration of the Kinesis Firehose delivery stream. Documented below.
�
kinesisStreamConfigs�*�:�
�
connect8getInstanceStorageConfigStorageConfigKinesisStreamConfig}aws:connect/getInstanceStorageConfigStorageConfigKinesisStreamConfig:getInstanceStorageConfigStorageConfigKinesisStreamConfigWA block that specifies the configuration of the Kinesis data stream. Documented below.
�
kinesisVideoStreamConfigs�*�:�
�
connect=getInstanceStorageConfigStorageConfigKinesisVideoStreamConfig�aws:connect/getInstanceStorageConfigStorageConfigKinesisVideoStreamConfig:getInstanceStorageConfigStorageConfigKinesisVideoStreamConfigXA block that specifies the configuration of the Kinesis video stream. Documented below.
�
	s3Configs�*�:�
�
connect-getInstanceStorageConfigStorageConfigS3Configgaws:connect/getInstanceStorageConfigStorageConfigS3Config:getInstanceStorageConfigStorageConfigS3ConfigIA block that specifies the configuration of S3 Bucket. Documented below.
~
storageType" kA valid storage type. Valid Values: `S3` | `KINESIS_VIDEO_STREAM` | `KINESIS_STREAM` | `KINESIS_FIREHOSE`.
:�
�
connect:getInstanceStorageConfigStorageConfigKinesisFirehoseConfig�aws:connect/getInstanceStorageConfigStorageConfigKinesisFirehoseConfig:getInstanceStorageConfigStorageConfigKinesisFirehoseConfigN
LJ
firehoseArn" 7The Amazon Resource Name (ARN) of the delivery stream.
:�
�
connect8getInstanceStorageConfigStorageConfigKinesisStreamConfig}aws:connect/getInstanceStorageConfigStorageConfigKinesisStreamConfig:getInstanceStorageConfigStorageConfigKinesisStreamConfigH
FD
	streamArn" 3The Amazon Resource Name (ARN) of the data stream.
:�
�
connect=getInstanceStorageConfigStorageConfigKinesisVideoStreamConfig�aws:connect/getInstanceStorageConfigStorageConfigKinesisVideoStreamConfig:getInstanceStorageConfigStorageConfigKinesisVideoStreamConfig�
��
encryptionConfigs�*�:�
�
connectMgetInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig�aws:connect/getInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig:getInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig0The encryption configuration. Documented below.
�
prefix" �The prefix of the video stream. Minimum length of `1`. Maximum length of `128`. When read from the state, the value returned is `<prefix>-connect-<connect_instance_alias>-contact-` since the API appends additional details to the `prefix`.
�
retentionPeriodHours �The number of hours to retain the data in a data store associated with the stream. Minimum value of `0`. Maximum value of `87600`. A value of `0` indicates that the stream does not persist data.
:�
�
connectMgetInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig�aws:connect/getInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig:getInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig�
�C
encryptionType" -The type of encryption. Valid Values: `KMS`.
y
keyId" lThe full ARN of the encryption key. Be sure to provide the full ARN of the encryption key, not just the ID.
:�
�
connect-getInstanceStorageConfigStorageConfigS3Configgaws:connect/getInstanceStorageConfigStorageConfigS3Config:getInstanceStorageConfigStorageConfigS3Config�
�&

bucketName" The S3 bucket name.
*
bucketPrefix" The S3 bucket prefix.
�
encryptionConfigs�*�:�
�
connect=getInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig�aws:connect/getInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig:getInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig0The encryption configuration. Documented below.
:�
�
connect=getInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig�aws:connect/getInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig:getInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig�
�C
encryptionType" -The type of encryption. Valid Values: `KMS`.
y
keyId" lThe full ARN of the encryption key. Be sure to provide the full ARN of the encryption key, not just the ID.
:�
n
connectgetQueueOutboundCallerConfigEaws:connect/getQueueOutboundCallerConfig:getQueueOutboundCallerConfig�
�:
outboundCallerIdName" Specifies the caller ID name.
@
outboundCallerIdNumberId"  Specifies the caller ID number.
P
outboundFlowId" :Outbound whisper flow to be used during an outbound call.
:�	
}
connect!getQuickConnectQuickConnectConfigOaws:connect/getQuickConnectQuickConnectConfig:getQuickConnectQuickConnectConfig�
��
phoneConfigs�*�:�
�
connect,getQuickConnectQuickConnectConfigPhoneConfigeaws:connect/getQuickConnectQuickConnectConfigPhoneConfig:getQuickConnectQuickConnectConfigPhoneConfig�Phone configuration of the Quick Connect. This is returned only if `quick_connect_type` is `PHONE_NUMBER`. The `phone_config` block is documented below.
�
queueConfigs�*�:�
�
connect,getQuickConnectQuickConnectConfigQueueConfigeaws:connect/getQuickConnectQuickConnectConfigQueueConfig:getQuickConnectQuickConnectConfigQueueConfig�Queue configuration of the Quick Connect. This is returned only if `quick_connect_type` is `QUEUE`. The `queue_config` block is documented below.
s
quickConnectType" [Configuration type of the Quick Connect. Valid values are `PHONE_NUMBER`, `QUEUE`, `USER`.
�
userConfigs�*�:�
�
connect+getQuickConnectQuickConnectConfigUserConfigcaws:connect/getQuickConnectQuickConnectConfigUserConfig:getQuickConnectQuickConnectConfigUserConfig�User configuration of the Quick Connect. This is returned only if `quick_connect_type` is `USER`. The `user_config` block is documented below.
:�
�
connect,getQuickConnectQuickConnectConfigPhoneConfigeaws:connect/getQuickConnectQuickConnectConfigPhoneConfig:getQuickConnectQuickConnectConfigPhoneConfig8
64
phoneNumber" !Phone number in in E.164 format.
:�
�
connect,getQuickConnectQuickConnectConfigQueueConfigeaws:connect/getQuickConnectQuickConnectConfigQueueConfig:getQuickConnectQuickConnectConfigQueueConfigd
b5
contactFlowId"  Identifier of the contact flow.
)
queueId" Identifier for the queue.
:�
�
connect+getQuickConnectQuickConnectConfigUserConfigcaws:connect/getQuickConnectQuickConnectConfigUserConfig:getQuickConnectQuickConnectConfigUserConfigb
`5
contactFlowId"  Identifier of the contact flow.
'
userId" Identifier for the user.
:�
}
connect!getRoutingProfileMediaConcurrencyOaws:connect/getRoutingProfileMediaConcurrency:getRoutingProfileMediaConcurrency�
��
channel" �Channels agents can handle in the Contact Control Panel (CCP) for this routing profile. Valid values are `VOICE`, `CHAT`, `TASK`.
�
concurrency �Number of contacts an agent can have on a channel simultaneously. Valid Range for `VOICE`: Minimum value of 1. Maximum value of 1. Valid Range for `CHAT`: Minimum value of 1. Maximum value of 10. Valid Range for `TASK`: Minimum value of 1. Maximum value of 10.
:�
n
connectgetRoutingProfileQueueConfigEaws:connect/getRoutingProfileQueueConfig:getRoutingProfileQueueConfig�
��
channel" �Channels agents can handle in the Contact Control Panel (CCP) for this routing profile. Valid values are `VOICE`, `CHAT`, `TASK`.
s
delay fDelay, in seconds, that a contact should be in the queue before they are routed to an available agent
I
priority 9Order in which contacts are to be handled for the queue.
#
queueArn" ARN for the queue.
)
queueId" Identifier for the queue.
%
	queueName" Name for the queue.
:�	
�
connect"getUserHierarchyGroupHierarchyPathQaws:connect/getUserHierarchyGroupHierarchyPath:getUserHierarchyGroupHierarchyPath�
��

levelFives�*�:�
�
connect+getUserHierarchyGroupHierarchyPathLevelFifecaws:connect/getUserHierarchyGroupHierarchyPathLevelFife:getUserHierarchyGroupHierarchyPathLevelFife"Details of level five. See below.
�

levelFours�*�:�
�
connect+getUserHierarchyGroupHierarchyPathLevelFourcaws:connect/getUserHierarchyGroupHierarchyPathLevelFour:getUserHierarchyGroupHierarchyPathLevelFour"Details of level four. See below.
�
	levelOnes�*�:�
�
connect*getUserHierarchyGroupHierarchyPathLevelOneaaws:connect/getUserHierarchyGroupHierarchyPathLevelOne:getUserHierarchyGroupHierarchyPathLevelOne!Details of level one. See below.
�
levelThrees�*�:�
�
connect,getUserHierarchyGroupHierarchyPathLevelThreeeaws:connect/getUserHierarchyGroupHierarchyPathLevelThree:getUserHierarchyGroupHierarchyPathLevelThree#Details of level three. See below.
�
	levelTwos�*�:�
�
connect*getUserHierarchyGroupHierarchyPathLevelTwoaaws:connect/getUserHierarchyGroupHierarchyPathLevelTwo:getUserHierarchyGroupHierarchyPathLevelTwo!Details of level two. See below.
:�
�
connect+getUserHierarchyGroupHierarchyPathLevelFifecaws:connect/getUserHierarchyGroupHierarchyPathLevelFife:getUserHierarchyGroupHierarchyPathLevelFife�
�'
arn" ARN of the hierarchy group.
1
id" 'The identifier of the hierarchy group.
F
name" :Returns information on a specific hierarchy group by name
:�
�
connect+getUserHierarchyGroupHierarchyPathLevelFourcaws:connect/getUserHierarchyGroupHierarchyPathLevelFour:getUserHierarchyGroupHierarchyPathLevelFour�
�'
arn" ARN of the hierarchy group.
1
id" 'The identifier of the hierarchy group.
F
name" :Returns information on a specific hierarchy group by name
:�
�
connect*getUserHierarchyGroupHierarchyPathLevelOneaaws:connect/getUserHierarchyGroupHierarchyPathLevelOne:getUserHierarchyGroupHierarchyPathLevelOne�
�'
arn" ARN of the hierarchy group.
1
id" 'The identifier of the hierarchy group.
F
name" :Returns information on a specific hierarchy group by name
:�
�
connect,getUserHierarchyGroupHierarchyPathLevelThreeeaws:connect/getUserHierarchyGroupHierarchyPathLevelThree:getUserHierarchyGroupHierarchyPathLevelThree�
�'
arn" ARN of the hierarchy group.
1
id" 'The identifier of the hierarchy group.
F
name" :Returns information on a specific hierarchy group by name
:�
�
connect*getUserHierarchyGroupHierarchyPathLevelTwoaaws:connect/getUserHierarchyGroupHierarchyPathLevelTwo:getUserHierarchyGroupHierarchyPathLevelTwo�
�'
arn" ARN of the hierarchy group.
1
id" 'The identifier of the hierarchy group.
F
name" :Returns information on a specific hierarchy group by name
:�

�
connect+getUserHierarchyStructureHierarchyStructurecaws:connect/getUserHierarchyStructureHierarchyStructure:getUserHierarchyStructureHierarchyStructure�	
�	�

levelFives�*�:�
�
connect4getUserHierarchyStructureHierarchyStructureLevelFifeuaws:connect/getUserHierarchyStructureHierarchyStructureLevelFife:getUserHierarchyStructureHierarchyStructureLevelFife"Details of level five. See below.
�

levelFours�*�:�
�
connect4getUserHierarchyStructureHierarchyStructureLevelFouruaws:connect/getUserHierarchyStructureHierarchyStructureLevelFour:getUserHierarchyStructureHierarchyStructureLevelFour"Details of level four. See below.
�
	levelOnes�*�:�
�
connect3getUserHierarchyStructureHierarchyStructureLevelOnesaws:connect/getUserHierarchyStructureHierarchyStructureLevelOne:getUserHierarchyStructureHierarchyStructureLevelOne!Details of level one. See below.
�
levelThrees�*�:�
�
connect5getUserHierarchyStructureHierarchyStructureLevelThreewaws:connect/getUserHierarchyStructureHierarchyStructureLevelThree:getUserHierarchyStructureHierarchyStructureLevelThree#Details of level three. See below.
�
	levelTwos�*�:�
�
connect3getUserHierarchyStructureHierarchyStructureLevelTwosaws:connect/getUserHierarchyStructureHierarchyStructureLevelTwo:getUserHierarchyStructureHierarchyStructureLevelTwo!Details of level two. See below.
:�
�
connect4getUserHierarchyStructureHierarchyStructureLevelFifeuaws:connect/getUserHierarchyStructureHierarchyStructureLevelFife:getUserHierarchyStructureHierarchyStructureLevelFife�
�'
arn" ARN of the hierarchy level.
1
id" 'The identifier of the hierarchy level.
S
name" GName of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect4getUserHierarchyStructureHierarchyStructureLevelFouruaws:connect/getUserHierarchyStructureHierarchyStructureLevelFour:getUserHierarchyStructureHierarchyStructureLevelFour�
�'
arn" ARN of the hierarchy level.
1
id" 'The identifier of the hierarchy level.
S
name" GName of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect3getUserHierarchyStructureHierarchyStructureLevelOnesaws:connect/getUserHierarchyStructureHierarchyStructureLevelOne:getUserHierarchyStructureHierarchyStructureLevelOne�
�'
arn" ARN of the hierarchy level.
1
id" 'The identifier of the hierarchy level.
S
name" GName of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect5getUserHierarchyStructureHierarchyStructureLevelThreewaws:connect/getUserHierarchyStructureHierarchyStructureLevelThree:getUserHierarchyStructureHierarchyStructureLevelThree�
�'
arn" ARN of the hierarchy level.
1
id" 'The identifier of the hierarchy level.
S
name" GName of the user hierarchy level. Must not be more than 50 characters.
:�
�
connect3getUserHierarchyStructureHierarchyStructureLevelTwosaws:connect/getUserHierarchyStructureHierarchyStructureLevelTwo:getUserHierarchyStructureHierarchyStructureLevelTwo�
�'
arn" ARN of the hierarchy level.
1
id" 'The identifier of the hierarchy level.
S
name" GName of the user hierarchy level. Must not be more than 50 characters.
:�
S
connectgetUserIdentityInfo3aws:connect/getUserIdentityInfo:getUserIdentityInfoh
f 
email" The email address.
!
	firstName" The first name.

lastName" The last name.
:�
P
connectgetUserPhoneConfig1aws:connect/getUserPhoneConfig:getUserPhoneConfig�
�X
afterContactWorkTimeLimit 7The After Call Work (ACW) timeout setting, in seconds.
y

autoAccept
 gWhen Auto-Accept Call is enabled for an available agent, the agent connects to contacts automatically.
C
deskPhoneNumber" ,The phone number for the user's desk phone.
Q
	phoneType" @The phone type. Valid values are `DESK_PHONE` and `SOFT_PHONE`.
