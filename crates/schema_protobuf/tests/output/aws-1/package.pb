
awsAWS"6.66.2*�
?
	appconfigApplication%aws:appconfig/application:Application�Provides an AppConfig Application resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appconfig.Application("example", {
    name: "example-application-tf",
    description: "Example AppConfig Application",
    tags: {
        Type: "AppConfig Application",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.Application("example",
    name="example-application-tf",
    description="Example AppConfig Application",
    tags={
        "Type": "AppConfig Application",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppConfig.Application("example", new()
    {
        Name = "example-application-tf",
        Description = "Example AppConfig Application",
        Tags = 
        {
            { "Type", "AppConfig Application" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.NewApplication(ctx, "example", &appconfig.ApplicationArgs{
			Name:        pulumi.String("example-application-tf"),
			Description: pulumi.String("Example AppConfig Application"),
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Application"),
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
import com.pulumi.aws.appconfig.Application;
import com.pulumi.aws.appconfig.ApplicationArgs;
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
        var example = new Application("example", ApplicationArgs.builder()
            .name("example-application-tf")
            .description("Example AppConfig Application")
            .tags(Map.of("Type", "AppConfig Application"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:Application
    properties:
      name: example-application-tf
      description: Example AppConfig Application
      tags:
        Type: AppConfig Application
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Applications using their application ID. For example:

```sh
$ pulumi import aws:appconfig/application:Application example 71rxuzt
```
U
descriptionB" @Description of the application. Can be at most 1024 characters.
W
nameB" IName for the application. Must be between 1 and 64 characters in length.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"-
arn" "ARN of the AppConfig Application.
"U
descriptionB" @Description of the application. Can be at most 1024 characters.
"U
name" IName for the application. Must be between 1 and 64 characters in length.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�H
Z
	appconfigConfigurationProfile7aws:appconfig/configurationProfile:ConfigurationProfile�$Provides an AppConfig Configuration Profile resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appconfig.ConfigurationProfile("example", {
    applicationId: exampleAwsAppconfigApplication.id,
    description: "Example Configuration Profile",
    name: "example-configuration-profile-tf",
    locationUri: "hosted",
    validators: [{
        content: exampleAwsLambdaFunction.arn,
        type: "LAMBDA",
    }],
    tags: {
        Type: "AppConfig Configuration Profile",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.ConfigurationProfile("example",
    application_id=example_aws_appconfig_application["id"],
    description="Example Configuration Profile",
    name="example-configuration-profile-tf",
    location_uri="hosted",
    validators=[{
        "content": example_aws_lambda_function["arn"],
        "type": "LAMBDA",
    }],
    tags={
        "Type": "AppConfig Configuration Profile",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppConfig.ConfigurationProfile("example", new()
    {
        ApplicationId = exampleAwsAppconfigApplication.Id,
        Description = "Example Configuration Profile",
        Name = "example-configuration-profile-tf",
        LocationUri = "hosted",
        Validators = new[]
        {
            new Aws.AppConfig.Inputs.ConfigurationProfileValidatorArgs
            {
                Content = exampleAwsLambdaFunction.Arn,
                Type = "LAMBDA",
            },
        },
        Tags = 
        {
            { "Type", "AppConfig Configuration Profile" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.NewConfigurationProfile(ctx, "example", &appconfig.ConfigurationProfileArgs{
			ApplicationId: pulumi.Any(exampleAwsAppconfigApplication.Id),
			Description:   pulumi.String("Example Configuration Profile"),
			Name:          pulumi.String("example-configuration-profile-tf"),
			LocationUri:   pulumi.String("hosted"),
			Validators: appconfig.ConfigurationProfileValidatorArray{
				&appconfig.ConfigurationProfileValidatorArgs{
					Content: pulumi.Any(exampleAwsLambdaFunction.Arn),
					Type:    pulumi.String("LAMBDA"),
				},
			},
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Configuration Profile"),
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
import com.pulumi.aws.appconfig.ConfigurationProfile;
import com.pulumi.aws.appconfig.ConfigurationProfileArgs;
import com.pulumi.aws.appconfig.inputs.ConfigurationProfileValidatorArgs;
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
        var example = new ConfigurationProfile("example", ConfigurationProfileArgs.builder()
            .applicationId(exampleAwsAppconfigApplication.id())
            .description("Example Configuration Profile")
            .name("example-configuration-profile-tf")
            .locationUri("hosted")
            .validators(ConfigurationProfileValidatorArgs.builder()
                .content(exampleAwsLambdaFunction.arn())
                .type("LAMBDA")
                .build())
            .tags(Map.of("Type", "AppConfig Configuration Profile"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:ConfigurationProfile
    properties:
      applicationId: ${exampleAwsAppconfigApplication.id}
      description: Example Configuration Profile
      name: example-configuration-profile-tf
      locationUri: hosted
      validators:
        - content: ${exampleAwsLambdaFunction.arn}
          type: LAMBDA
      tags:
        Type: AppConfig Configuration Profile
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Configuration Profiles using the configuration profile ID and application ID separated by a colon (`:`). For example:

```sh
$ pulumi import aws:appconfig/configurationProfile:ConfigurationProfile example 71abcde:11xxxxx
```
S
applicationId" >Application ID. Must be between 4 and 7 characters in length.
_
descriptionB" JDescription of the configuration profile. Can be at most 1024 characters.
�
kmsKeyIdentifierB" �The identifier for an Key Management Service key to encrypt new configuration data versions in the AppConfig hosted configuration store. This attribute is only used for hosted configuration types. The identifier can be an KMS key ID, alias, or the Amazon Resource Name (ARN) of the key ID or alias.
�
locationUri" �URI to locate the configuration. You can specify the AWS AppConfig hosted configuration store, Systems Manager (SSM) document, an SSM Parameter Store parameter, or an Amazon S3 object. For the hosted configuration store, specify `hosted`. For an SSM document, specify either the document name in the format `ssm-document://<Document_name>` or the ARN. For a parameter, specify either the parameter name in the format `ssm-parameter://<Parameter_name>` or the ARN. For an Amazon S3 object, specify the URI in the following format: `s3://<bucket>/<objectKey>`.
b
nameB" TName for the configuration profile. Must be between 1 and 128 characters in length.
�
retrievalRoleArnB" �ARN of an IAM role with permission to access the configuration at the specified `location_uri`. A retrieval role ARN is not required for configurations stored in the AWS AppConfig `hosted` configuration store. It is required for all other sources that store your configuration.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
typeB" �Type of configurations contained in the profile. Valid values: `AWS.AppConfig.FeatureFlags` and `AWS.Freeform`.  Default: `AWS.Freeform`.
�

validators}B{*y:w
u
	appconfigConfigurationProfileValidatorIaws:appconfig/ConfigurationProfileValidator:ConfigurationProfileValidatoreSet of methods for validating the configuration. Maximum of 2. See Validator below for more details.
"S
applicationId" >Application ID. Must be between 4 and 7 characters in length.
"7
arn" ,ARN of the AppConfig Configuration Profile.
"<
configurationProfileId" The configuration profile ID.
"_
descriptionB" JDescription of the configuration profile. Can be at most 1024 characters.
"�
kmsKeyIdentifierB" �The identifier for an Key Management Service key to encrypt new configuration data versions in the AppConfig hosted configuration store. This attribute is only used for hosted configuration types. The identifier can be an KMS key ID, alias, or the Amazon Resource Name (ARN) of the key ID or alias.
"�
locationUri" �URI to locate the configuration. You can specify the AWS AppConfig hosted configuration store, Systems Manager (SSM) document, an SSM Parameter Store parameter, or an Amazon S3 object. For the hosted configuration store, specify `hosted`. For an SSM document, specify either the document name in the format `ssm-document://<Document_name>` or the ARN. For a parameter, specify either the parameter name in the format `ssm-parameter://<Parameter_name>` or the ARN. For an Amazon S3 object, specify the URI in the following format: `s3://<bucket>/<objectKey>`.
"`
name" TName for the configuration profile. Must be between 1 and 128 characters in length.
"�
retrievalRoleArnB" �ARN of an IAM role with permission to access the configuration at the specified `location_uri`. A retrieval role ARN is not required for configurations stored in the AWS AppConfig `hosted` configuration store. It is required for all other sources that store your configuration.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
typeB" �Type of configurations contained in the profile. Valid values: `AWS.AppConfig.FeatureFlags` and `AWS.Freeform`.  Default: `AWS.Freeform`.
"�

validators}B{*y:w
u
	appconfigConfigurationProfileValidatorIaws:appconfig/ConfigurationProfileValidator:ConfigurationProfileValidatoreSet of methods for validating the configuration. Maximum of 2. See Validator below for more details.
*�>
<
	appconfig
Deployment#aws:appconfig/deployment:Deployment�)Provides an AppConfig Deployment resource for an `aws.appconfig.Application` resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appconfig.Deployment("example", {
    applicationId: exampleAwsAppconfigApplication.id,
    configurationProfileId: exampleAwsAppconfigConfigurationProfile.configurationProfileId,
    configurationVersion: exampleAwsAppconfigHostedConfigurationVersion.versionNumber,
    deploymentStrategyId: exampleAwsAppconfigDeploymentStrategy.id,
    description: "My example deployment",
    environmentId: exampleAwsAppconfigEnvironment.environmentId,
    kmsKeyIdentifier: exampleAwsKmsKey.arn,
    tags: {
        Type: "AppConfig Deployment",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.Deployment("example",
    application_id=example_aws_appconfig_application["id"],
    configuration_profile_id=example_aws_appconfig_configuration_profile["configurationProfileId"],
    configuration_version=example_aws_appconfig_hosted_configuration_version["versionNumber"],
    deployment_strategy_id=example_aws_appconfig_deployment_strategy["id"],
    description="My example deployment",
    environment_id=example_aws_appconfig_environment["environmentId"],
    kms_key_identifier=example_aws_kms_key["arn"],
    tags={
        "Type": "AppConfig Deployment",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppConfig.Deployment("example", new()
    {
        ApplicationId = exampleAwsAppconfigApplication.Id,
        ConfigurationProfileId = exampleAwsAppconfigConfigurationProfile.ConfigurationProfileId,
        ConfigurationVersion = exampleAwsAppconfigHostedConfigurationVersion.VersionNumber,
        DeploymentStrategyId = exampleAwsAppconfigDeploymentStrategy.Id,
        Description = "My example deployment",
        EnvironmentId = exampleAwsAppconfigEnvironment.EnvironmentId,
        KmsKeyIdentifier = exampleAwsKmsKey.Arn,
        Tags = 
        {
            { "Type", "AppConfig Deployment" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.NewDeployment(ctx, "example", &appconfig.DeploymentArgs{
			ApplicationId:          pulumi.Any(exampleAwsAppconfigApplication.Id),
			ConfigurationProfileId: pulumi.Any(exampleAwsAppconfigConfigurationProfile.ConfigurationProfileId),
			ConfigurationVersion:   pulumi.Any(exampleAwsAppconfigHostedConfigurationVersion.VersionNumber),
			DeploymentStrategyId:   pulumi.Any(exampleAwsAppconfigDeploymentStrategy.Id),
			Description:            pulumi.String("My example deployment"),
			EnvironmentId:          pulumi.Any(exampleAwsAppconfigEnvironment.EnvironmentId),
			KmsKeyIdentifier:       pulumi.Any(exampleAwsKmsKey.Arn),
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Deployment"),
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
import com.pulumi.aws.appconfig.Deployment;
import com.pulumi.aws.appconfig.DeploymentArgs;
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
        var example = new Deployment("example", DeploymentArgs.builder()
            .applicationId(exampleAwsAppconfigApplication.id())
            .configurationProfileId(exampleAwsAppconfigConfigurationProfile.configurationProfileId())
            .configurationVersion(exampleAwsAppconfigHostedConfigurationVersion.versionNumber())
            .deploymentStrategyId(exampleAwsAppconfigDeploymentStrategy.id())
            .description("My example deployment")
            .environmentId(exampleAwsAppconfigEnvironment.environmentId())
            .kmsKeyIdentifier(exampleAwsKmsKey.arn())
            .tags(Map.of("Type", "AppConfig Deployment"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:Deployment
    properties:
      applicationId: ${exampleAwsAppconfigApplication.id}
      configurationProfileId: ${exampleAwsAppconfigConfigurationProfile.configurationProfileId}
      configurationVersion: ${exampleAwsAppconfigHostedConfigurationVersion.versionNumber}
      deploymentStrategyId: ${exampleAwsAppconfigDeploymentStrategy.id}
      description: My example deployment
      environmentId: ${exampleAwsAppconfigEnvironment.environmentId}
      kmsKeyIdentifier: ${exampleAwsKmsKey.arn}
      tags:
        Type: AppConfig Deployment
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Deployments using the application ID, environment ID, and deployment number separated by a slash (`/`). For example:

```sh
$ pulumi import aws:appconfig/deployment:Deployment example 71abcde/11xxxxx/1
```
S
applicationId" >Application ID. Must be between 4 and 7 characters in length.
f
configurationProfileId" HConfiguration profile ID. Must be between 4 and 7 characters in length.
]
configurationVersion" AConfiguration version to deploy. Can be at most 1024 characters.
�
deploymentStrategyId" �Deployment strategy ID or name of a predefined deployment strategy. See [Predefined Deployment Strategies](https://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-creating-deployment-strategy.html#appconfig-creating-deployment-strategy-predefined) for more details.
T
descriptionB" ?Description of the deployment. Can be at most 1024 characters.
S
environmentId" >Environment ID. Must be between 4 and 7 characters in length.
�
kmsKeyIdentifierB" �The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this to encrypt the configuration data using a customer managed key.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"S
applicationId" >Application ID. Must be between 4 and 7 characters in length.
",
arn" !ARN of the AppConfig Deployment.
"f
configurationProfileId" HConfiguration profile ID. Must be between 4 and 7 characters in length.
"]
configurationVersion" AConfiguration version to deploy. Can be at most 1024 characters.
"+
deploymentNumber Deployment number.
"�
deploymentStrategyId" �Deployment strategy ID or name of a predefined deployment strategy. See [Predefined Deployment Strategies](https://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-creating-deployment-strategy.html#appconfig-creating-deployment-strategy-predefined) for more details.
"T
descriptionB" ?Description of the deployment. Can be at most 1024 characters.
"S
environmentId" >Environment ID. Must be between 4 and 7 characters in length.
"H
	kmsKeyArn" 7ARN of the KMS key used to encrypt configuration data.
"�
kmsKeyIdentifierB" �The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this to encrypt the configuration data using a customer managed key.
"&
state" State of the deployment.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�3
T
	appconfigDeploymentStrategy3aws:appconfig/deploymentStrategy:DeploymentStrategy�Provides an AppConfig Deployment Strategy resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appconfig.DeploymentStrategy("example", {
    name: "example-deployment-strategy-tf",
    description: "Example Deployment Strategy",
    deploymentDurationInMinutes: 3,
    finalBakeTimeInMinutes: 4,
    growthFactor: 10,
    growthType: "LINEAR",
    replicateTo: "NONE",
    tags: {
        Type: "AppConfig Deployment Strategy",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.DeploymentStrategy("example",
    name="example-deployment-strategy-tf",
    description="Example Deployment Strategy",
    deployment_duration_in_minutes=3,
    final_bake_time_in_minutes=4,
    growth_factor=10,
    growth_type="LINEAR",
    replicate_to="NONE",
    tags={
        "Type": "AppConfig Deployment Strategy",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppConfig.DeploymentStrategy("example", new()
    {
        Name = "example-deployment-strategy-tf",
        Description = "Example Deployment Strategy",
        DeploymentDurationInMinutes = 3,
        FinalBakeTimeInMinutes = 4,
        GrowthFactor = 10,
        GrowthType = "LINEAR",
        ReplicateTo = "NONE",
        Tags = 
        {
            { "Type", "AppConfig Deployment Strategy" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.NewDeploymentStrategy(ctx, "example", &appconfig.DeploymentStrategyArgs{
			Name:                        pulumi.String("example-deployment-strategy-tf"),
			Description:                 pulumi.String("Example Deployment Strategy"),
			DeploymentDurationInMinutes: pulumi.Int(3),
			FinalBakeTimeInMinutes:      pulumi.Int(4),
			GrowthFactor:                pulumi.Float64(10),
			GrowthType:                  pulumi.String("LINEAR"),
			ReplicateTo:                 pulumi.String("NONE"),
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Deployment Strategy"),
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
import com.pulumi.aws.appconfig.DeploymentStrategy;
import com.pulumi.aws.appconfig.DeploymentStrategyArgs;
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
        var example = new DeploymentStrategy("example", DeploymentStrategyArgs.builder()
            .name("example-deployment-strategy-tf")
            .description("Example Deployment Strategy")
            .deploymentDurationInMinutes(3)
            .finalBakeTimeInMinutes(4)
            .growthFactor(10)
            .growthType("LINEAR")
            .replicateTo("NONE")
            .tags(Map.of("Type", "AppConfig Deployment Strategy"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:DeploymentStrategy
    properties:
      name: example-deployment-strategy-tf
      description: Example Deployment Strategy
      deploymentDurationInMinutes: 3
      finalBakeTimeInMinutes: 4
      growthFactor: 10
      growthType: LINEAR
      replicateTo: NONE
      tags:
        Type: AppConfig Deployment Strategy
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Deployment Strategies using their deployment strategy ID. For example:

```sh
$ pulumi import aws:appconfig/deploymentStrategy:DeploymentStrategy example 11xxxxx
```
}
deploymentDurationInMinutes ZTotal amount of time for a deployment to last. Minimum value of 0, maximum value of 1440.
]
descriptionB" HDescription of the deployment strategy. Can be at most 1024 characters.
�
finalBakeTimeInMinutesB �Amount of time AWS AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back. Minimum value of 0, maximum value of 1440.
�
growthFactor ~Percentage of targets to receive a deployed configuration during each interval. Minimum value of 1.0, maximum value of 100.0.
�

growthTypeB" xAlgorithm used to define how percentage grows over time. Valid value: `LINEAR` and `EXPONENTIAL`. Defaults to `LINEAR`.
_
nameB" QName for the deployment strategy. Must be between 1 and 64 characters in length.
c
replicateTo" PWhere to save the deployment strategy. Valid values: `NONE` and `SSM_DOCUMENT`.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"5
arn" *ARN of the AppConfig Deployment Strategy.
"}
deploymentDurationInMinutes ZTotal amount of time for a deployment to last. Minimum value of 0, maximum value of 1440.
"]
descriptionB" HDescription of the deployment strategy. Can be at most 1024 characters.
"�
finalBakeTimeInMinutesB �Amount of time AWS AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back. Minimum value of 0, maximum value of 1440.
"�
growthFactor ~Percentage of targets to receive a deployed configuration during each interval. Minimum value of 1.0, maximum value of 100.0.
"�

growthTypeB" xAlgorithm used to define how percentage grows over time. Valid value: `LINEAR` and `EXPONENTIAL`. Defaults to `LINEAR`.
"]
name" QName for the deployment strategy. Must be between 1 and 64 characters in length.
"c
replicateTo" PWhere to save the deployment strategy. Valid values: `NONE` and `SSM_DOCUMENT`.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�=
?
	appconfigEnvironment%aws:appconfig/environment:Environment�/Provides an AppConfig Environment resource for an `aws.appconfig.Application` resource. One or more environments can be defined for an application.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleApplication = new aws.appconfig.Application("example", {
    name: "example-application-tf",
    description: "Example AppConfig Application",
    tags: {
        Type: "AppConfig Application",
    },
});
const example = new aws.appconfig.Environment("example", {
    name: "example-environment-tf",
    description: "Example AppConfig Environment",
    applicationId: exampleApplication.id,
    monitors: [{
        alarmArn: exampleAwsCloudwatchMetricAlarm.arn,
        alarmRoleArn: exampleAwsIamRole.arn,
    }],
    tags: {
        Type: "AppConfig Environment",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example_application = aws.appconfig.Application("example",
    name="example-application-tf",
    description="Example AppConfig Application",
    tags={
        "Type": "AppConfig Application",
    })
example = aws.appconfig.Environment("example",
    name="example-environment-tf",
    description="Example AppConfig Environment",
    application_id=example_application.id,
    monitors=[{
        "alarm_arn": example_aws_cloudwatch_metric_alarm["arn"],
        "alarm_role_arn": example_aws_iam_role["arn"],
    }],
    tags={
        "Type": "AppConfig Environment",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleApplication = new Aws.AppConfig.Application("example", new()
    {
        Name = "example-application-tf",
        Description = "Example AppConfig Application",
        Tags = 
        {
            { "Type", "AppConfig Application" },
        },
    });

    var example = new Aws.AppConfig.Environment("example", new()
    {
        Name = "example-environment-tf",
        Description = "Example AppConfig Environment",
        ApplicationId = exampleApplication.Id,
        Monitors = new[]
        {
            new Aws.AppConfig.Inputs.EnvironmentMonitorArgs
            {
                AlarmArn = exampleAwsCloudwatchMetricAlarm.Arn,
                AlarmRoleArn = exampleAwsIamRole.Arn,
            },
        },
        Tags = 
        {
            { "Type", "AppConfig Environment" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleApplication, err := appconfig.NewApplication(ctx, "example", &appconfig.ApplicationArgs{
			Name:        pulumi.String("example-application-tf"),
			Description: pulumi.String("Example AppConfig Application"),
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Application"),
			},
		})
		if err != nil {
			return err
		}
		_, err = appconfig.NewEnvironment(ctx, "example", &appconfig.EnvironmentArgs{
			Name:          pulumi.String("example-environment-tf"),
			Description:   pulumi.String("Example AppConfig Environment"),
			ApplicationId: exampleApplication.ID(),
			Monitors: appconfig.EnvironmentMonitorArray{
				&appconfig.EnvironmentMonitorArgs{
					AlarmArn:     pulumi.Any(exampleAwsCloudwatchMetricAlarm.Arn),
					AlarmRoleArn: pulumi.Any(exampleAwsIamRole.Arn),
				},
			},
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Environment"),
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
import com.pulumi.aws.appconfig.Application;
import com.pulumi.aws.appconfig.ApplicationArgs;
import com.pulumi.aws.appconfig.Environment;
import com.pulumi.aws.appconfig.EnvironmentArgs;
import com.pulumi.aws.appconfig.inputs.EnvironmentMonitorArgs;
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
        var exampleApplication = new Application("exampleApplication", ApplicationArgs.builder()
            .name("example-application-tf")
            .description("Example AppConfig Application")
            .tags(Map.of("Type", "AppConfig Application"))
            .build());

        var example = new Environment("example", EnvironmentArgs.builder()
            .name("example-environment-tf")
            .description("Example AppConfig Environment")
            .applicationId(exampleApplication.id())
            .monitors(EnvironmentMonitorArgs.builder()
                .alarmArn(exampleAwsCloudwatchMetricAlarm.arn())
                .alarmRoleArn(exampleAwsIamRole.arn())
                .build())
            .tags(Map.of("Type", "AppConfig Environment"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:Environment
    properties:
      name: example-environment-tf
      description: Example AppConfig Environment
      applicationId: ${exampleApplication.id}
      monitors:
        - alarmArn: ${exampleAwsCloudwatchMetricAlarm.arn}
          alarmRoleArn: ${exampleAwsIamRole.arn}
      tags:
        Type: AppConfig Environment
  exampleApplication:
    type: aws:appconfig:Application
    name: example
    properties:
      name: example-application-tf
      description: Example AppConfig Application
      tags:
        Type: AppConfig Application
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Environments using the environment ID and application ID separated by a colon (`:`). For example:

```sh
$ pulumi import aws:appconfig/environment:Environment example 71abcde:11xxxxx
```
]
applicationId" HAppConfig application ID. Must be between 4 and 7 characters in length.
U
descriptionB" @Description of the environment. Can be at most 1024 characters.
�
monitors\BZ*X:V
T
	appconfigEnvironmentMonitor3aws:appconfig/EnvironmentMonitor:EnvironmentMonitor|Set of Amazon CloudWatch alarms to monitor during the deployment process. Maximum of 5. See Monitor below for more details.
W
nameB" IName for the environment. Must be between 1 and 64 characters in length.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"]
applicationId" HAppConfig application ID. Must be between 4 and 7 characters in length.
"-
arn" "ARN of the AppConfig Environment.
"S
description" @Description of the environment. Can be at most 1024 characters.
"/
environmentId" AppConfig environment ID.
"�
monitors\BZ*X:V
T
	appconfigEnvironmentMonitor3aws:appconfig/EnvironmentMonitor:EnvironmentMonitor|Set of Amazon CloudWatch alarms to monitor during the deployment process. Maximum of 5. See Monitor below for more details.
"U
name" IName for the environment. Must be between 1 and 64 characters in length.
"�
state" tState of the environment. Possible values are `READY_FOR_DEPLOYMENT`, `DEPLOYING`, `ROLLING_BACK`
or `ROLLED_BACK`.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�(
N
	appconfigEventIntegration/aws:appconfig/eventIntegration:EventIntegration�Provides an Amazon AppIntegrations Event Integration resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appconfig.EventIntegration("example", {
    name: "example-name",
    description: "Example Description",
    eventbridgeBus: "default",
    eventFilter: {
        source: "aws.partner/examplepartner.com",
    },
    tags: {
        Name: "Example Event Integration",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.EventIntegration("example",
    name="example-name",
    description="Example Description",
    eventbridge_bus="default",
    event_filter={
        "source": "aws.partner/examplepartner.com",
    },
    tags={
        "Name": "Example Event Integration",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppConfig.EventIntegration("example", new()
    {
        Name = "example-name",
        Description = "Example Description",
        EventbridgeBus = "default",
        EventFilter = new Aws.AppConfig.Inputs.EventIntegrationEventFilterArgs
        {
            Source = "aws.partner/examplepartner.com",
        },
        Tags = 
        {
            { "Name", "Example Event Integration" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.NewEventIntegration(ctx, "example", &appconfig.EventIntegrationArgs{
			Name:           pulumi.String("example-name"),
			Description:    pulumi.String("Example Description"),
			EventbridgeBus: pulumi.String("default"),
			EventFilter: &appconfig.EventIntegrationEventFilterArgs{
				Source: pulumi.String("aws.partner/examplepartner.com"),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Event Integration"),
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
import com.pulumi.aws.appconfig.EventIntegration;
import com.pulumi.aws.appconfig.EventIntegrationArgs;
import com.pulumi.aws.appconfig.inputs.EventIntegrationEventFilterArgs;
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
        var example = new EventIntegration("example", EventIntegrationArgs.builder()
            .name("example-name")
            .description("Example Description")
            .eventbridgeBus("default")
            .eventFilter(EventIntegrationEventFilterArgs.builder()
                .source("aws.partner/examplepartner.com")
                .build())
            .tags(Map.of("Name", "Example Event Integration"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:EventIntegration
    properties:
      name: example-name
      description: Example Description
      eventbridgeBus: default
      eventFilter:
        source: aws.partner/examplepartner.com
      tags:
        Name: Example Event Integration
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon AppIntegrations Event Integrations using the `name`. For example:

```sh
$ pulumi import aws:appconfig/eventIntegration:EventIntegration example example-name
```
;
descriptionB" &Description of the Event Integration.
�
eventFilters:q
o
	appconfigEventIntegrationEventFilterEaws:appconfig/EventIntegrationEventFilter:EventIntegrationEventFiltersBlock that defines the configuration information for the event filter. The Event Filter block is documented below.
'
eventbridgeBus" EventBridge bus.
-
nameB" Name of the Event Integration.
�
tagsB2" �Tags to apply to the Event Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
")
arn" ARN of the Event Integration.
";
descriptionB" &Description of the Event Integration.
"�
eventFilters:q
o
	appconfigEventIntegrationEventFilterEaws:appconfig/EventIntegrationEventFilter:EventIntegrationEventFiltersBlock that defines the configuration information for the event filter. The Event Filter block is documented below.
"'
eventbridgeBus" EventBridge bus.
"+
name" Name of the Event Integration.
"�
tagsB2" �Tags to apply to the Event Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�S
9
	appconfig	Extension!aws:appconfig/extension:Extension�CProvides an AppConfig Extension resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testTopic = new aws.sns.Topic("test", {name: "test"});
const test = aws.iam.getPolicyDocument({
    statements: [{
        actions: ["sts:AssumeRole"],
        principals: [{
            type: "Service",
            identifiers: ["appconfig.amazonaws.com"],
        }],
    }],
});
const testRole = new aws.iam.Role("test", {
    name: "test",
    assumeRolePolicy: test.then(test => test.json),
});
const testExtension = new aws.appconfig.Extension("test", {
    name: "test",
    description: "test description",
    actionPoints: [{
        point: "ON_DEPLOYMENT_COMPLETE",
        actions: [{
            name: "test",
            roleArn: testRole.arn,
            uri: testTopic.arn,
        }],
    }],
    tags: {
        Type: "AppConfig Extension",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test_topic = aws.sns.Topic("test", name="test")
test = aws.iam.get_policy_document(statements=[{
    "actions": ["sts:AssumeRole"],
    "principals": [{
        "type": "Service",
        "identifiers": ["appconfig.amazonaws.com"],
    }],
}])
test_role = aws.iam.Role("test",
    name="test",
    assume_role_policy=test.json)
test_extension = aws.appconfig.Extension("test",
    name="test",
    description="test description",
    action_points=[{
        "point": "ON_DEPLOYMENT_COMPLETE",
        "actions": [{
            "name": "test",
            "role_arn": test_role.arn,
            "uri": test_topic.arn,
        }],
    }],
    tags={
        "Type": "AppConfig Extension",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testTopic = new Aws.Sns.Topic("test", new()
    {
        Name = "test",
    });

    var test = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "appconfig.amazonaws.com",
                        },
                    },
                },
            },
        },
    });

    var testRole = new Aws.Iam.Role("test", new()
    {
        Name = "test",
        AssumeRolePolicy = test.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var testExtension = new Aws.AppConfig.Extension("test", new()
    {
        Name = "test",
        Description = "test description",
        ActionPoints = new[]
        {
            new Aws.AppConfig.Inputs.ExtensionActionPointArgs
            {
                Point = "ON_DEPLOYMENT_COMPLETE",
                Actions = new[]
                {
                    new Aws.AppConfig.Inputs.ExtensionActionPointActionArgs
                    {
                        Name = "test",
                        RoleArn = testRole.Arn,
                        Uri = testTopic.Arn,
                    },
                },
            },
        },
        Tags = 
        {
            { "Type", "AppConfig Extension" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		testTopic, err := sns.NewTopic(ctx, "test", &sns.TopicArgs{
			Name: pulumi.String("test"),
		})
		if err != nil {
			return err
		}
		test, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Actions: []string{
						"sts:AssumeRole",
					},
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"appconfig.amazonaws.com",
							},
						},
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		testRole, err := iam.NewRole(ctx, "test", &iam.RoleArgs{
			Name:             pulumi.String("test"),
			AssumeRolePolicy: pulumi.String(test.Json),
		})
		if err != nil {
			return err
		}
		_, err = appconfig.NewExtension(ctx, "test", &appconfig.ExtensionArgs{
			Name:        pulumi.String("test"),
			Description: pulumi.String("test description"),
			ActionPoints: appconfig.ExtensionActionPointArray{
				&appconfig.ExtensionActionPointArgs{
					Point: pulumi.String("ON_DEPLOYMENT_COMPLETE"),
					Actions: appconfig.ExtensionActionPointActionArray{
						&appconfig.ExtensionActionPointActionArgs{
							Name:    pulumi.String("test"),
							RoleArn: testRole.Arn,
							Uri:     testTopic.Arn,
						},
					},
				},
			},
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Extension"),
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
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.appconfig.Extension;
import com.pulumi.aws.appconfig.ExtensionArgs;
import com.pulumi.aws.appconfig.inputs.ExtensionActionPointArgs;
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
        var testTopic = new Topic("testTopic", TopicArgs.builder()
            .name("test")
            .build());

        final var test = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sts:AssumeRole")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("appconfig.amazonaws.com")
                    .build())
                .build())
            .build());

        var testRole = new Role("testRole", RoleArgs.builder()
            .name("test")
            .assumeRolePolicy(test.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var testExtension = new Extension("testExtension", ExtensionArgs.builder()
            .name("test")
            .description("test description")
            .actionPoints(ExtensionActionPointArgs.builder()
                .point("ON_DEPLOYMENT_COMPLETE")
                .actions(ExtensionActionPointActionArgs.builder()
                    .name("test")
                    .roleArn(testRole.arn())
                    .uri(testTopic.arn())
                    .build())
                .build())
            .tags(Map.of("Type", "AppConfig Extension"))
            .build());

    }
}
```
```yaml
resources:
  testTopic:
    type: aws:sns:Topic
    name: test
    properties:
      name: test
  testRole:
    type: aws:iam:Role
    name: test
    properties:
      name: test
      assumeRolePolicy: ${test.json}
  testExtension:
    type: aws:appconfig:Extension
    name: test
    properties:
      name: test
      description: test description
      actionPoints:
        - point: ON_DEPLOYMENT_COMPLETE
          actions:
            - name: test
              roleArn: ${testRole.arn}
              uri: ${testTopic.arn}
      tags:
        Type: AppConfig Extension
variables:
  test:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sts:AssumeRole
            principals:
              - type: Service
                identifiers:
                  - appconfig.amazonaws.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Extensions using their extension ID. For example:

```sh
$ pulumi import aws:appconfig/extension:Extension example 71rxuzt
```
�
actionPoints`*^:\
Z
	appconfigExtensionActionPoint7aws:appconfig/ExtensionActionPoint:ExtensionActionPoint<The action points defined in the extension. Detailed below.
6
descriptionB" !Information about the extension.
�
nameB" tA name for the extension. Each extension name in your account must be unique. Extension versions use the same name.
�

parameters\BZ*X:V
T
	appconfigExtensionParameter3aws:appconfig/ExtensionParameter:ExtensionParameter�The parameters accepted by the extension. You specify parameter values when you associate the extension to an AppConfig resource by using the CreateExtensionAssociation API action. For Lambda extension actions, these parameters are included in the Lambda request object. Detailed below.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
actionPoints`*^:\
Z
	appconfigExtensionActionPoint7aws:appconfig/ExtensionActionPoint:ExtensionActionPoint<The action points defined in the extension. Detailed below.
"+
arn"  ARN of the AppConfig Extension.
"4
description" !Information about the extension.
"�
name" tA name for the extension. Each extension name in your account must be unique. Extension versions use the same name.
"�

parametersZ*X:V
T
	appconfigExtensionParameter3aws:appconfig/ExtensionParameter:ExtensionParameter�The parameters accepted by the extension. You specify parameter values when you associate the extension to an AppConfig resource by using the CreateExtensionAssociation API action. For Lambda extension actions, these parameters are included in the Lambda request object. Detailed below.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "5
version &The version number for the extension.
*�Y
Z
	appconfigExtensionAssociation7aws:appconfig/extensionAssociation:ExtensionAssociation�SAssociates an AppConfig Extension with a Resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testTopic = new aws.sns.Topic("test", {name: "test"});
const test = aws.iam.getPolicyDocument({
    statements: [{
        actions: ["sts:AssumeRole"],
        principals: [{
            type: "Service",
            identifiers: ["appconfig.amazonaws.com"],
        }],
    }],
});
const testRole = new aws.iam.Role("test", {
    name: "test",
    assumeRolePolicy: test.then(test => test.json),
});
const testExtension = new aws.appconfig.Extension("test", {
    name: "test",
    description: "test description",
    actionPoints: [{
        point: "ON_DEPLOYMENT_COMPLETE",
        actions: [{
            name: "test",
            roleArn: testRole.arn,
            uri: testTopic.arn,
        }],
    }],
    tags: {
        Type: "AppConfig Extension",
    },
});
const testApplication = new aws.appconfig.Application("test", {name: "test"});
const testExtensionAssociation = new aws.appconfig.ExtensionAssociation("test", {
    extensionArn: testExtension.arn,
    resourceArn: testApplication.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

test_topic = aws.sns.Topic("test", name="test")
test = aws.iam.get_policy_document(statements=[{
    "actions": ["sts:AssumeRole"],
    "principals": [{
        "type": "Service",
        "identifiers": ["appconfig.amazonaws.com"],
    }],
}])
test_role = aws.iam.Role("test",
    name="test",
    assume_role_policy=test.json)
test_extension = aws.appconfig.Extension("test",
    name="test",
    description="test description",
    action_points=[{
        "point": "ON_DEPLOYMENT_COMPLETE",
        "actions": [{
            "name": "test",
            "role_arn": test_role.arn,
            "uri": test_topic.arn,
        }],
    }],
    tags={
        "Type": "AppConfig Extension",
    })
test_application = aws.appconfig.Application("test", name="test")
test_extension_association = aws.appconfig.ExtensionAssociation("test",
    extension_arn=test_extension.arn,
    resource_arn=test_application.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testTopic = new Aws.Sns.Topic("test", new()
    {
        Name = "test",
    });

    var test = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "appconfig.amazonaws.com",
                        },
                    },
                },
            },
        },
    });

    var testRole = new Aws.Iam.Role("test", new()
    {
        Name = "test",
        AssumeRolePolicy = test.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var testExtension = new Aws.AppConfig.Extension("test", new()
    {
        Name = "test",
        Description = "test description",
        ActionPoints = new[]
        {
            new Aws.AppConfig.Inputs.ExtensionActionPointArgs
            {
                Point = "ON_DEPLOYMENT_COMPLETE",
                Actions = new[]
                {
                    new Aws.AppConfig.Inputs.ExtensionActionPointActionArgs
                    {
                        Name = "test",
                        RoleArn = testRole.Arn,
                        Uri = testTopic.Arn,
                    },
                },
            },
        },
        Tags = 
        {
            { "Type", "AppConfig Extension" },
        },
    });

    var testApplication = new Aws.AppConfig.Application("test", new()
    {
        Name = "test",
    });

    var testExtensionAssociation = new Aws.AppConfig.ExtensionAssociation("test", new()
    {
        ExtensionArn = testExtension.Arn,
        ResourceArn = testApplication.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		testTopic, err := sns.NewTopic(ctx, "test", &sns.TopicArgs{
			Name: pulumi.String("test"),
		})
		if err != nil {
			return err
		}
		test, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Actions: []string{
						"sts:AssumeRole",
					},
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"appconfig.amazonaws.com",
							},
						},
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		testRole, err := iam.NewRole(ctx, "test", &iam.RoleArgs{
			Name:             pulumi.String("test"),
			AssumeRolePolicy: pulumi.String(test.Json),
		})
		if err != nil {
			return err
		}
		testExtension, err := appconfig.NewExtension(ctx, "test", &appconfig.ExtensionArgs{
			Name:        pulumi.String("test"),
			Description: pulumi.String("test description"),
			ActionPoints: appconfig.ExtensionActionPointArray{
				&appconfig.ExtensionActionPointArgs{
					Point: pulumi.String("ON_DEPLOYMENT_COMPLETE"),
					Actions: appconfig.ExtensionActionPointActionArray{
						&appconfig.ExtensionActionPointActionArgs{
							Name:    pulumi.String("test"),
							RoleArn: testRole.Arn,
							Uri:     testTopic.Arn,
						},
					},
				},
			},
			Tags: pulumi.StringMap{
				"Type": pulumi.String("AppConfig Extension"),
			},
		})
		if err != nil {
			return err
		}
		testApplication, err := appconfig.NewApplication(ctx, "test", &appconfig.ApplicationArgs{
			Name: pulumi.String("test"),
		})
		if err != nil {
			return err
		}
		_, err = appconfig.NewExtensionAssociation(ctx, "test", &appconfig.ExtensionAssociationArgs{
			ExtensionArn: testExtension.Arn,
			ResourceArn:  testApplication.Arn,
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
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.appconfig.Extension;
import com.pulumi.aws.appconfig.ExtensionArgs;
import com.pulumi.aws.appconfig.inputs.ExtensionActionPointArgs;
import com.pulumi.aws.appconfig.Application;
import com.pulumi.aws.appconfig.ApplicationArgs;
import com.pulumi.aws.appconfig.ExtensionAssociation;
import com.pulumi.aws.appconfig.ExtensionAssociationArgs;
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
        var testTopic = new Topic("testTopic", TopicArgs.builder()
            .name("test")
            .build());

        final var test = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sts:AssumeRole")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("appconfig.amazonaws.com")
                    .build())
                .build())
            .build());

        var testRole = new Role("testRole", RoleArgs.builder()
            .name("test")
            .assumeRolePolicy(test.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var testExtension = new Extension("testExtension", ExtensionArgs.builder()
            .name("test")
            .description("test description")
            .actionPoints(ExtensionActionPointArgs.builder()
                .point("ON_DEPLOYMENT_COMPLETE")
                .actions(ExtensionActionPointActionArgs.builder()
                    .name("test")
                    .roleArn(testRole.arn())
                    .uri(testTopic.arn())
                    .build())
                .build())
            .tags(Map.of("Type", "AppConfig Extension"))
            .build());

        var testApplication = new Application("testApplication", ApplicationArgs.builder()
            .name("test")
            .build());

        var testExtensionAssociation = new ExtensionAssociation("testExtensionAssociation", ExtensionAssociationArgs.builder()
            .extensionArn(testExtension.arn())
            .resourceArn(testApplication.arn())
            .build());

    }
}
```
```yaml
resources:
  testTopic:
    type: aws:sns:Topic
    name: test
    properties:
      name: test
  testRole:
    type: aws:iam:Role
    name: test
    properties:
      name: test
      assumeRolePolicy: ${test.json}
  testExtension:
    type: aws:appconfig:Extension
    name: test
    properties:
      name: test
      description: test description
      actionPoints:
        - point: ON_DEPLOYMENT_COMPLETE
          actions:
            - name: test
              roleArn: ${testRole.arn}
              uri: ${testTopic.arn}
      tags:
        Type: AppConfig Extension
  testApplication:
    type: aws:appconfig:Application
    name: test
    properties:
      name: test
  testExtensionAssociation:
    type: aws:appconfig:ExtensionAssociation
    name: test
    properties:
      extensionArn: ${testExtension.arn}
      resourceArn: ${testApplication.arn}
variables:
  test:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sts:AssumeRole
            principals:
              - type: Service
                identifiers:
                  - appconfig.amazonaws.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Extension Associations using their extension association ID. For example:

```sh
$ pulumi import aws:appconfig/extensionAssociation:ExtensionAssociation example 71rxuzt
```
I
extensionArn" 5The ARN of the extension defined in the association.
R

parametersB2" <The parameter names and values defined for the association.
v
resourceArn" cThe ARN of the application, configuration profile, or environment to associate with the extension.
"7
arn" ,ARN of the AppConfig Extension Association.
"I
extensionArn" 5The ARN of the extension defined in the association.
"Y
extensionVersion AThe version number for the extension defined in the association.
"R

parametersB2" <The parameter names and values defined for the association.
"v
resourceArn" cThe ARN of the application, configuration profile, or environment to associate with the extension.
*�
l
	appconfigHostedConfigurationVersionCaws:appconfig/hostedConfigurationVersion:HostedConfigurationVersion��Provides an AppConfig Hosted Configuration Version resource.

## Example Usage

### Freeform

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appconfig.HostedConfigurationVersion("example", {
    applicationId: exampleAwsAppconfigApplication.id,
    configurationProfileId: exampleAwsAppconfigConfigurationProfile.configurationProfileId,
    description: "Example Freeform Hosted Configuration Version",
    contentType: "application/json",
    content: JSON.stringify({
        foo: "bar",
        fruit: [
            "apple",
            "pear",
            "orange",
        ],
        isThingEnabled: true,
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.appconfig.HostedConfigurationVersion("example",
    application_id=example_aws_appconfig_application["id"],
    configuration_profile_id=example_aws_appconfig_configuration_profile["configurationProfileId"],
    description="Example Freeform Hosted Configuration Version",
    content_type="application/json",
    content=json.dumps({
        "foo": "bar",
        "fruit": [
            "apple",
            "pear",
            "orange",
        ],
        "isThingEnabled": True,
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppConfig.HostedConfigurationVersion("example", new()
    {
        ApplicationId = exampleAwsAppconfigApplication.Id,
        ConfigurationProfileId = exampleAwsAppconfigConfigurationProfile.ConfigurationProfileId,
        Description = "Example Freeform Hosted Configuration Version",
        ContentType = "application/json",
        Content = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["foo"] = "bar",
            ["fruit"] = new[]
            {
                "apple",
                "pear",
                "orange",
            },
            ["isThingEnabled"] = true,
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"foo": "bar",
			"fruit": []string{
				"apple",
				"pear",
				"orange",
			},
			"isThingEnabled": true,
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = appconfig.NewHostedConfigurationVersion(ctx, "example", &appconfig.HostedConfigurationVersionArgs{
			ApplicationId:          pulumi.Any(exampleAwsAppconfigApplication.Id),
			ConfigurationProfileId: pulumi.Any(exampleAwsAppconfigConfigurationProfile.ConfigurationProfileId),
			Description:            pulumi.String("Example Freeform Hosted Configuration Version"),
			ContentType:            pulumi.String("application/json"),
			Content:                pulumi.String(json0),
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
import com.pulumi.aws.appconfig.HostedConfigurationVersion;
import com.pulumi.aws.appconfig.HostedConfigurationVersionArgs;
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
        var example = new HostedConfigurationVersion("example", HostedConfigurationVersionArgs.builder()
            .applicationId(exampleAwsAppconfigApplication.id())
            .configurationProfileId(exampleAwsAppconfigConfigurationProfile.configurationProfileId())
            .description("Example Freeform Hosted Configuration Version")
            .contentType("application/json")
            .content(serializeJson(
                jsonObject(
                    jsonProperty("foo", "bar"),
                    jsonProperty("fruit", jsonArray(
                        "apple", 
                        "pear", 
                        "orange"
                    )),
                    jsonProperty("isThingEnabled", true)
                )))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:HostedConfigurationVersion
    properties:
      applicationId: ${exampleAwsAppconfigApplication.id}
      configurationProfileId: ${exampleAwsAppconfigConfigurationProfile.configurationProfileId}
      description: Example Freeform Hosted Configuration Version
      contentType: application/json
      content:
        fn::toJSON:
          foo: bar
          fruit:
            - apple
            - pear
            - orange
          isThingEnabled: true
```
<!--End PulumiCodeChooser -->

### Feature Flags

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appconfig.HostedConfigurationVersion("example", {
    applicationId: exampleAwsAppconfigApplication.id,
    configurationProfileId: exampleAwsAppconfigConfigurationProfile.configurationProfileId,
    description: "Example Feature Flag Configuration Version",
    contentType: "application/json",
    content: JSON.stringify({
        flags: {
            foo: {
                name: "foo",
                _deprecation: {
                    status: "planned",
                },
            },
            bar: {
                name: "bar",
                attributes: {
                    someAttribute: {
                        constraints: {
                            type: "string",
                            required: true,
                        },
                    },
                    someOtherAttribute: {
                        constraints: {
                            type: "number",
                            required: true,
                        },
                    },
                },
            },
        },
        values: {
            foo: {
                enabled: "true",
            },
            bar: {
                enabled: "true",
                someAttribute: "Hello World",
                someOtherAttribute: 123,
            },
        },
        version: "1",
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.appconfig.HostedConfigurationVersion("example",
    application_id=example_aws_appconfig_application["id"],
    configuration_profile_id=example_aws_appconfig_configuration_profile["configurationProfileId"],
    description="Example Feature Flag Configuration Version",
    content_type="application/json",
    content=json.dumps({
        "flags": {
            "foo": {
                "name": "foo",
                "_deprecation": {
                    "status": "planned",
                },
            },
            "bar": {
                "name": "bar",
                "attributes": {
                    "someAttribute": {
                        "constraints": {
                            "type": "string",
                            "required": True,
                        },
                    },
                    "someOtherAttribute": {
                        "constraints": {
                            "type": "number",
                            "required": True,
                        },
                    },
                },
            },
        },
        "values": {
            "foo": {
                "enabled": "true",
            },
            "bar": {
                "enabled": "true",
                "someAttribute": "Hello World",
                "someOtherAttribute": 123,
            },
        },
        "version": "1",
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppConfig.HostedConfigurationVersion("example", new()
    {
        ApplicationId = exampleAwsAppconfigApplication.Id,
        ConfigurationProfileId = exampleAwsAppconfigConfigurationProfile.ConfigurationProfileId,
        Description = "Example Feature Flag Configuration Version",
        ContentType = "application/json",
        Content = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["flags"] = new Dictionary<string, object?>
            {
                ["foo"] = new Dictionary<string, object?>
                {
                    ["name"] = "foo",
                    ["_deprecation"] = new Dictionary<string, object?>
                    {
                        ["status"] = "planned",
                    },
                },
                ["bar"] = new Dictionary<string, object?>
                {
                    ["name"] = "bar",
                    ["attributes"] = new Dictionary<string, object?>
                    {
                        ["someAttribute"] = new Dictionary<string, object?>
                        {
                            ["constraints"] = new Dictionary<string, object?>
                            {
                                ["type"] = "string",
                                ["required"] = true,
                            },
                        },
                        ["someOtherAttribute"] = new Dictionary<string, object?>
                        {
                            ["constraints"] = new Dictionary<string, object?>
                            {
                                ["type"] = "number",
                                ["required"] = true,
                            },
                        },
                    },
                },
            },
            ["values"] = new Dictionary<string, object?>
            {
                ["foo"] = new Dictionary<string, object?>
                {
                    ["enabled"] = "true",
                },
                ["bar"] = new Dictionary<string, object?>
                {
                    ["enabled"] = "true",
                    ["someAttribute"] = "Hello World",
                    ["someOtherAttribute"] = 123,
                },
            },
            ["version"] = "1",
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"flags": map[string]interface{}{
				"foo": map[string]interface{}{
					"name": "foo",
					"_deprecation": map[string]interface{}{
						"status": "planned",
					},
				},
				"bar": map[string]interface{}{
					"name": "bar",
					"attributes": map[string]interface{}{
						"someAttribute": map[string]interface{}{
							"constraints": map[string]interface{}{
								"type":     "string",
								"required": true,
							},
						},
						"someOtherAttribute": map[string]interface{}{
							"constraints": map[string]interface{}{
								"type":     "number",
								"required": true,
							},
						},
					},
				},
			},
			"values": map[string]interface{}{
				"foo": map[string]interface{}{
					"enabled": "true",
				},
				"bar": map[string]interface{}{
					"enabled":            "true",
					"someAttribute":      "Hello World",
					"someOtherAttribute": 123,
				},
			},
			"version": "1",
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = appconfig.NewHostedConfigurationVersion(ctx, "example", &appconfig.HostedConfigurationVersionArgs{
			ApplicationId:          pulumi.Any(exampleAwsAppconfigApplication.Id),
			ConfigurationProfileId: pulumi.Any(exampleAwsAppconfigConfigurationProfile.ConfigurationProfileId),
			Description:            pulumi.String("Example Feature Flag Configuration Version"),
			ContentType:            pulumi.String("application/json"),
			Content:                pulumi.String(json0),
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
import com.pulumi.aws.appconfig.HostedConfigurationVersion;
import com.pulumi.aws.appconfig.HostedConfigurationVersionArgs;
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
        var example = new HostedConfigurationVersion("example", HostedConfigurationVersionArgs.builder()
            .applicationId(exampleAwsAppconfigApplication.id())
            .configurationProfileId(exampleAwsAppconfigConfigurationProfile.configurationProfileId())
            .description("Example Feature Flag Configuration Version")
            .contentType("application/json")
            .content(serializeJson(
                jsonObject(
                    jsonProperty("flags", jsonObject(
                        jsonProperty("foo", jsonObject(
                            jsonProperty("name", "foo"),
                            jsonProperty("_deprecation", jsonObject(
                                jsonProperty("status", "planned")
                            ))
                        )),
                        jsonProperty("bar", jsonObject(
                            jsonProperty("name", "bar"),
                            jsonProperty("attributes", jsonObject(
                                jsonProperty("someAttribute", jsonObject(
                                    jsonProperty("constraints", jsonObject(
                                        jsonProperty("type", "string"),
                                        jsonProperty("required", true)
                                    ))
                                )),
                                jsonProperty("someOtherAttribute", jsonObject(
                                    jsonProperty("constraints", jsonObject(
                                        jsonProperty("type", "number"),
                                        jsonProperty("required", true)
                                    ))
                                ))
                            ))
                        ))
                    )),
                    jsonProperty("values", jsonObject(
                        jsonProperty("foo", jsonObject(
                            jsonProperty("enabled", "true")
                        )),
                        jsonProperty("bar", jsonObject(
                            jsonProperty("enabled", "true"),
                            jsonProperty("someAttribute", "Hello World"),
                            jsonProperty("someOtherAttribute", 123)
                        ))
                    )),
                    jsonProperty("version", "1")
                )))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appconfig:HostedConfigurationVersion
    properties:
      applicationId: ${exampleAwsAppconfigApplication.id}
      configurationProfileId: ${exampleAwsAppconfigConfigurationProfile.configurationProfileId}
      description: Example Feature Flag Configuration Version
      contentType: application/json
      content:
        fn::toJSON:
          flags:
            foo:
              name: foo
              _deprecation:
                status: planned
            bar:
              name: bar
              attributes:
                someAttribute:
                  constraints:
                    type: string
                    required: true
                someOtherAttribute:
                  constraints:
                    type: number
                    required: true
          values:
            foo:
              enabled: 'true'
            bar:
              enabled: 'true'
              someAttribute: Hello World
              someOtherAttribute: 123
          version: '1'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppConfig Hosted Configuration Versions using the application ID, configuration profile ID, and version number separated by a slash (`/`). For example:

```sh
$ pulumi import aws:appconfig/hostedConfigurationVersion:HostedConfigurationVersion example 71abcde/11xxxxx/2
```
%
applicationId" Application ID.
8
configurationProfileId" Configuration profile ID.
G
content" 8Content of the configuration or the configuration data.
�
contentType" �Standard MIME type describing the format of the configuration content. For more information, see [Content-Type](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17).
7
descriptionB" "Description of the configuration.
"%
applicationId" Application ID.
"?
arn" 4ARN of the AppConfig  hosted configuration version.
"8
configurationProfileId" Configuration profile ID.
"G
content" 8Content of the configuration or the configuration data.
"�
contentType" �Standard MIME type describing the format of the configuration content. For more information, see [Content-Type](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17).
"7
descriptionB" "Description of the configuration.
"A
versionNumber ,Version number of the hosted configuration.
*�9
N
	appfabricAppAuthorization/aws:appfabric/appAuthorization:AppAuthorization�#Resource for managing an AWS AppFabric App Authorization.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appfabric.AppAuthorization("example", {
    app: "TERRAFORMCLOUD",
    appBundleArn: arn,
    authType: "apiKey",
    credential: {
        apiKeyCredentials: [{
            apiKey: "exampleapikeytoken",
        }],
    },
    tenants: [{
        tenantDisplayName: "example",
        tenantIdentifier: "example",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appfabric.AppAuthorization("example",
    app="TERRAFORMCLOUD",
    app_bundle_arn=arn,
    auth_type="apiKey",
    credential={
        "api_key_credentials": [{
            "api_key": "exampleapikeytoken",
        }],
    },
    tenants=[{
        "tenant_display_name": "example",
        "tenant_identifier": "example",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppFabric.AppAuthorization("example", new()
    {
        App = "TERRAFORMCLOUD",
        AppBundleArn = arn,
        AuthType = "apiKey",
        Credential = new Aws.AppFabric.Inputs.AppAuthorizationCredentialArgs
        {
            ApiKeyCredentials = new[]
            {
                new Aws.AppFabric.Inputs.AppAuthorizationCredentialApiKeyCredentialArgs
                {
                    ApiKey = "exampleapikeytoken",
                },
            },
        },
        Tenants = new[]
        {
            new Aws.AppFabric.Inputs.AppAuthorizationTenantArgs
            {
                TenantDisplayName = "example",
                TenantIdentifier = "example",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appfabric"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appfabric.NewAppAuthorization(ctx, "example", &appfabric.AppAuthorizationArgs{
			App:          pulumi.String("TERRAFORMCLOUD"),
			AppBundleArn: pulumi.Any(arn),
			AuthType:     pulumi.String("apiKey"),
			Credential: &appfabric.AppAuthorizationCredentialArgs{
				ApiKeyCredentials: appfabric.AppAuthorizationCredentialApiKeyCredentialArray{
					&appfabric.AppAuthorizationCredentialApiKeyCredentialArgs{
						ApiKey: pulumi.String("exampleapikeytoken"),
					},
				},
			},
			Tenants: appfabric.AppAuthorizationTenantArray{
				&appfabric.AppAuthorizationTenantArgs{
					TenantDisplayName: pulumi.String("example"),
					TenantIdentifier:  pulumi.String("example"),
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
import com.pulumi.aws.appfabric.AppAuthorization;
import com.pulumi.aws.appfabric.AppAuthorizationArgs;
import com.pulumi.aws.appfabric.inputs.AppAuthorizationCredentialArgs;
import com.pulumi.aws.appfabric.inputs.AppAuthorizationTenantArgs;
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
        var example = new AppAuthorization("example", AppAuthorizationArgs.builder()
            .app("TERRAFORMCLOUD")
            .appBundleArn(arn)
            .authType("apiKey")
            .credential(AppAuthorizationCredentialArgs.builder()
                .apiKeyCredentials(AppAuthorizationCredentialApiKeyCredentialArgs.builder()
                    .apiKey("exampleapikeytoken")
                    .build())
                .build())
            .tenants(AppAuthorizationTenantArgs.builder()
                .tenantDisplayName("example")
                .tenantIdentifier("example")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appfabric:AppAuthorization
    properties:
      app: TERRAFORMCLOUD
      appBundleArn: ${arn}
      authType: apiKey
      credential:
        apiKeyCredentials:
          - apiKey: exampleapikeytoken
      tenants:
        - tenantDisplayName: example
          tenantIdentifier: example
```
<!--End PulumiCodeChooser -->
�
app" �The name of the application for valid values see https://docs.aws.amazon.com/appfabric/latest/api/API_CreateAppAuthorization.html.
]
appBundleArn" IThe Amazon Resource Name (ARN) of the app bundle to use for the request.
e
authType" UThe authorization type for the app authorization valid values are oauth2 and apiKey.
�

credentialrBp:n
l
	appfabricAppAuthorizationCredentialCaws:appfabric/AppAuthorizationCredential:AppAuthorizationCredential�Contains credentials for the application, such as an API key or OAuth2 client ID and secret.
Specify credentials that match the authorization type for your request. For example, if the authorization type for your request is OAuth2 (oauth2), then you should provide only the OAuth2 credentials.

tagsB2" �
tenantshBf*d:b
`
	appfabricAppAuthorizationTenant;aws:appfabric/AppAuthorizationTenant:AppAuthorizationTenantgContains information about an application tenant, such as the application display name and identifier.
x
timeoutslBj:h
f
	appfabricAppAuthorizationTimeouts?aws:appfabric/AppAuthorizationTimeouts:AppAuthorizationTimeouts"�
app" �The name of the application for valid values see https://docs.aws.amazon.com/appfabric/latest/api/API_CreateAppAuthorization.html.
"]
appBundleArn" IThe Amazon Resource Name (ARN) of the app bundle to use for the request.
"�
arn" �ARN of the App Authorization. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
"e
authType" UThe authorization type for the app authorization valid values are oauth2 and apiKey.
"7
authUrl" (The application URL for the OAuth flow.
"
	createdAt" "�

credentialrBp:n
l
	appfabricAppAuthorizationCredentialCaws:appfabric/AppAuthorizationCredential:AppAuthorizationCredential�Contains credentials for the application, such as an API key or OAuth2 client ID and secret.
Specify credentials that match the authorization type for your request. For example, if the authorization type for your request is OAuth2 (oauth2), then you should provide only the OAuth2 credentials.
":
persona" +The user persona of the app authorization.
"
tagsB2" "
tagsAll2" "�
tenantshBf*d:b
`
	appfabricAppAuthorizationTenant;aws:appfabric/AppAuthorizationTenant:AppAuthorizationTenantgContains information about an application tenant, such as the application display name and identifier.
"x
timeoutslBj:h
f
	appfabricAppAuthorizationTimeouts?aws:appfabric/AppAuthorizationTimeouts:AppAuthorizationTimeouts"
	updatedAt" *� 
l
	appfabricAppAuthorizationConnectionCaws:appfabric/appAuthorizationConnection:AppAuthorizationConnection�Resource for managing an AWS AppFabric App Authorization Connection.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appfabric.AppAuthorizationConnection("example", {
    appAuthorizationArn: test.arn,
    appBundleArn: arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appfabric.AppAuthorizationConnection("example",
    app_authorization_arn=test["arn"],
    app_bundle_arn=arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppFabric.AppAuthorizationConnection("example", new()
    {
        AppAuthorizationArn = test.Arn,
        AppBundleArn = arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appfabric"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appfabric.NewAppAuthorizationConnection(ctx, "example", &appfabric.AppAuthorizationConnectionArgs{
			AppAuthorizationArn: pulumi.Any(test.Arn),
			AppBundleArn:        pulumi.Any(arn),
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
import com.pulumi.aws.appfabric.AppAuthorizationConnection;
import com.pulumi.aws.appfabric.AppAuthorizationConnectionArgs;
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
        var example = new AppAuthorizationConnection("example", AppAuthorizationConnectionArgs.builder()
            .appAuthorizationArn(test.arn())
            .appBundleArn(arn)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appfabric:AppAuthorizationConnection
    properties:
      appAuthorizationArn: ${test.arn}
      appBundleArn: ${arn}
```
<!--End PulumiCodeChooser -->
�
appAuthorizationArn" vThe Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app authorization to use for the request.
]
appBundleArn" IThe Amazon Resource Name (ARN) of the app bundle to use for the request.
�
authRequest�B�:�
�
	appfabric%AppAuthorizationConnectionAuthRequestYaws:appfabric/AppAuthorizationConnectionAuthRequest:AppAuthorizationConnectionAuthRequest�Contains OAuth2 authorization information.This is required if the app authorization for the request is configured with an OAuth2 (oauth2) authorization type.
�
timeouts�B�:�
�
	appfabric"AppAuthorizationConnectionTimeoutsSaws:appfabric/AppAuthorizationConnectionTimeouts:AppAuthorizationConnectionTimeouts"(
app" The name of the application.
"�
appAuthorizationArn" vThe Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app authorization to use for the request.
"]
appBundleArn" IThe Amazon Resource Name (ARN) of the app bundle to use for the request.
"�
authRequest�B�:�
�
	appfabric%AppAuthorizationConnectionAuthRequestYaws:appfabric/AppAuthorizationConnectionAuthRequest:AppAuthorizationConnectionAuthRequest�Contains OAuth2 authorization information.This is required if the app authorization for the request is configured with an OAuth2 (oauth2) authorization type.
"�
tenants�*�:�
~
	appfabric AppAuthorizationConnectionTenantOaws:appfabric/AppAuthorizationConnectionTenant:AppAuthorizationConnectionTenantgContains information about an application tenant, such as the application display name and identifier.
"�
timeouts�B�:�
�
	appfabric"AppAuthorizationConnectionTimeoutsSaws:appfabric/AppAuthorizationConnectionTimeouts:AppAuthorizationConnectionTimeouts*�
9
	appfabric	AppBundle!aws:appfabric/appBundle:AppBundle�Resource for managing an AWS AppFabric AppBundle.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appfabric.AppBundle("example", {
    customerManagedKeyArn: exampleAwmsKmsKey.arn,
    tags: {
        Environment: "test",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appfabric.AppBundle("example",
    customer_managed_key_arn=example_awms_kms_key["arn"],
    tags={
        "Environment": "test",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppFabric.AppBundle("example", new()
    {
        CustomerManagedKeyArn = exampleAwmsKmsKey.Arn,
        Tags = 
        {
            { "Environment", "test" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appfabric"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appfabric.NewAppBundle(ctx, "example", &appfabric.AppBundleArgs{
			CustomerManagedKeyArn: pulumi.Any(exampleAwmsKmsKey.Arn),
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("test"),
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
import com.pulumi.aws.appfabric.AppBundle;
import com.pulumi.aws.appfabric.AppBundleArgs;
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
        var example = new AppBundle("example", AppBundleArgs.builder()
            .customerManagedKeyArn(exampleAwmsKmsKey.arn())
            .tags(Map.of("Environment", "test"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appfabric:AppBundle
    properties:
      customerManagedKeyArn: ${exampleAwmsKmsKey.arn}
      tags:
        Environment: test
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppFabric AppBundle using the `arn`. For example:

```sh
$ pulumi import aws:appfabric/appBundle:AppBundle example arn:aws:appfabric:[region]:[account]:appbundle/ee5587b4-5765-4288-a202-xxxxxxxxxx
```
�
customerManagedKeyArnB" �The Amazon Resource Name (ARN) of the AWS Key Management Service (AWS KMS) key to use to encrypt the application data. If this is not specified, an AWS owned key is used for encryption.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"!
arn" ARN of the AppBundle.
"�
customerManagedKeyArnB" �The Amazon Resource Name (ARN) of the AWS Key Management Service (AWS KMS) key to use to encrypt the application data. If this is not specified, an AWS owned key is used for encryption.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�%
9
	appfabric	Ingestion!aws:appfabric/ingestion:Ingestion�Resource for managing an AWS AppFabric Ingestion.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appfabric.Ingestion("example", {
    app: "OKTA",
    appBundleArn: exampleAwsAppfabricAppBundle.arn,
    tenantId: "example.okta.com",
    ingestionType: "auditLog",
    tags: {
        Environment: "test",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appfabric.Ingestion("example",
    app="OKTA",
    app_bundle_arn=example_aws_appfabric_app_bundle["arn"],
    tenant_id="example.okta.com",
    ingestion_type="auditLog",
    tags={
        "Environment": "test",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppFabric.Ingestion("example", new()
    {
        App = "OKTA",
        AppBundleArn = exampleAwsAppfabricAppBundle.Arn,
        TenantId = "example.okta.com",
        IngestionType = "auditLog",
        Tags = 
        {
            { "Environment", "test" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appfabric"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appfabric.NewIngestion(ctx, "example", &appfabric.IngestionArgs{
			App:           pulumi.String("OKTA"),
			AppBundleArn:  pulumi.Any(exampleAwsAppfabricAppBundle.Arn),
			TenantId:      pulumi.String("example.okta.com"),
			IngestionType: pulumi.String("auditLog"),
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("test"),
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
import com.pulumi.aws.appfabric.Ingestion;
import com.pulumi.aws.appfabric.IngestionArgs;
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
        var example = new Ingestion("example", IngestionArgs.builder()
            .app("OKTA")
            .appBundleArn(exampleAwsAppfabricAppBundle.arn())
            .tenantId("example.okta.com")
            .ingestionType("auditLog")
            .tags(Map.of("Environment", "test"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appfabric:Ingestion
    properties:
      app: OKTA
      appBundleArn: ${exampleAwsAppfabricAppBundle.arn}
      tenantId: example.okta.com
      ingestionType: auditLog
      tags:
        Environment: test
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppFabric Ingestion using the `app_bundle_identifier` and `arn` separated by `,`. For example:

```sh
$ pulumi import aws:appfabric/ingestion:Ingestion example arn:aws:appfabric:[region]:[account]:appbundle/a9b91477-8831-43c0-970c-xxxxxxxxxx,arn:aws:appfabric:[region]:[account]:appbundle/a9b91477-8831-43c0-970c-xxxxxxxxxx/ingestion/32251416-710b-4425-96ca-xxxxxxxxxx
```
�
app" �Name of the application.
Refer to the AWS Documentation for the [list of valid values](https://docs.aws.amazon.com/appfabric/latest/api/API_CreateIngestion.html#appfabric-CreateIngestion-request-app)
Y
appBundleArn" EAmazon Resource Name (ARN) of the app bundle to use for the request.
B
ingestionType" -Ingestion type. Valid values are `auditLog`.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
.
tenantId" ID of the application tenant.
"�
app" �Name of the application.
Refer to the AWS Documentation for the [list of valid values](https://docs.aws.amazon.com/appfabric/latest/api/API_CreateIngestion.html#appfabric-CreateIngestion-request-app)
"Y
appBundleArn" EAmazon Resource Name (ARN) of the app bundle to use for the request.
"!
arn" ARN of the Ingestion.
"B
ingestionType" -Ingestion type. Valid values are `auditLog`.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
".
tenantId" ID of the application tenant.
*�F
Z
	appfabricIngestionDestination7aws:appfabric/ingestionDestination:IngestionDestination�4Resource for managing an AWS AppFabric Ingestion Destination.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appfabric.IngestionDestination("example", {
    appBundleArn: exampleAwsAppfabricAppBundle.arn,
    ingestionArn: exampleAwsAppfabricIngestion.arn,
    processingConfiguration: {
        auditLog: {
            format: "json",
            schema: "raw",
        },
    },
    destinationConfiguration: {
        auditLog: {
            destination: {
                s3Bucket: {
                    bucketName: exampleAwsS3Bucket.bucket,
                },
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appfabric.IngestionDestination("example",
    app_bundle_arn=example_aws_appfabric_app_bundle["arn"],
    ingestion_arn=example_aws_appfabric_ingestion["arn"],
    processing_configuration={
        "audit_log": {
            "format": "json",
            "schema": "raw",
        },
    },
    destination_configuration={
        "audit_log": {
            "destination": {
                "s3_bucket": {
                    "bucket_name": example_aws_s3_bucket["bucket"],
                },
            },
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
    var example = new Aws.AppFabric.IngestionDestination("example", new()
    {
        AppBundleArn = exampleAwsAppfabricAppBundle.Arn,
        IngestionArn = exampleAwsAppfabricIngestion.Arn,
        ProcessingConfiguration = new Aws.AppFabric.Inputs.IngestionDestinationProcessingConfigurationArgs
        {
            AuditLog = new Aws.AppFabric.Inputs.IngestionDestinationProcessingConfigurationAuditLogArgs
            {
                Format = "json",
                Schema = "raw",
            },
        },
        DestinationConfiguration = new Aws.AppFabric.Inputs.IngestionDestinationDestinationConfigurationArgs
        {
            AuditLog = new Aws.AppFabric.Inputs.IngestionDestinationDestinationConfigurationAuditLogArgs
            {
                Destination = new Aws.AppFabric.Inputs.IngestionDestinationDestinationConfigurationAuditLogDestinationArgs
                {
                    S3Bucket = new Aws.AppFabric.Inputs.IngestionDestinationDestinationConfigurationAuditLogDestinationS3BucketArgs
                    {
                        BucketName = exampleAwsS3Bucket.Bucket,
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appfabric"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appfabric.NewIngestionDestination(ctx, "example", &appfabric.IngestionDestinationArgs{
			AppBundleArn: pulumi.Any(exampleAwsAppfabricAppBundle.Arn),
			IngestionArn: pulumi.Any(exampleAwsAppfabricIngestion.Arn),
			ProcessingConfiguration: &appfabric.IngestionDestinationProcessingConfigurationArgs{
				AuditLog: &appfabric.IngestionDestinationProcessingConfigurationAuditLogArgs{
					Format: pulumi.String("json"),
					Schema: pulumi.String("raw"),
				},
			},
			DestinationConfiguration: &appfabric.IngestionDestinationDestinationConfigurationArgs{
				AuditLog: &appfabric.IngestionDestinationDestinationConfigurationAuditLogArgs{
					Destination: &appfabric.IngestionDestinationDestinationConfigurationAuditLogDestinationArgs{
						S3Bucket: &appfabric.IngestionDestinationDestinationConfigurationAuditLogDestinationS3BucketArgs{
							BucketName: pulumi.Any(exampleAwsS3Bucket.Bucket),
						},
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
import com.pulumi.aws.appfabric.IngestionDestination;
import com.pulumi.aws.appfabric.IngestionDestinationArgs;
import com.pulumi.aws.appfabric.inputs.IngestionDestinationProcessingConfigurationArgs;
import com.pulumi.aws.appfabric.inputs.IngestionDestinationProcessingConfigurationAuditLogArgs;
import com.pulumi.aws.appfabric.inputs.IngestionDestinationDestinationConfigurationArgs;
import com.pulumi.aws.appfabric.inputs.IngestionDestinationDestinationConfigurationAuditLogArgs;
import com.pulumi.aws.appfabric.inputs.IngestionDestinationDestinationConfigurationAuditLogDestinationArgs;
import com.pulumi.aws.appfabric.inputs.IngestionDestinationDestinationConfigurationAuditLogDestinationS3BucketArgs;
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
        var example = new IngestionDestination("example", IngestionDestinationArgs.builder()
            .appBundleArn(exampleAwsAppfabricAppBundle.arn())
            .ingestionArn(exampleAwsAppfabricIngestion.arn())
            .processingConfiguration(IngestionDestinationProcessingConfigurationArgs.builder()
                .auditLog(IngestionDestinationProcessingConfigurationAuditLogArgs.builder()
                    .format("json")
                    .schema("raw")
                    .build())
                .build())
            .destinationConfiguration(IngestionDestinationDestinationConfigurationArgs.builder()
                .auditLog(IngestionDestinationDestinationConfigurationAuditLogArgs.builder()
                    .destination(IngestionDestinationDestinationConfigurationAuditLogDestinationArgs.builder()
                        .s3Bucket(IngestionDestinationDestinationConfigurationAuditLogDestinationS3BucketArgs.builder()
                            .bucketName(exampleAwsS3Bucket.bucket())
                            .build())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appfabric:IngestionDestination
    properties:
      appBundleArn: ${exampleAwsAppfabricAppBundle.arn}
      ingestionArn: ${exampleAwsAppfabricIngestion.arn}
      processingConfiguration:
        auditLog:
          format: json
          schema: raw
      destinationConfiguration:
        auditLog:
          destination:
            s3Bucket:
              bucketName: ${exampleAwsS3Bucket.bucket}
```
<!--End PulumiCodeChooser -->
]
appBundleArn" IThe Amazon Resource Name (ARN) of the app bundle to use for the request.
�
destinationConfiguration�B�:�
�
	appfabric,IngestionDestinationDestinationConfigurationgaws:appfabric/IngestionDestinationDestinationConfiguration:IngestionDestinationDestinationConfiguration=Contains information about the destination of ingested data.
\
ingestionArn" HThe Amazon Resource Name (ARN) of the ingestion to use for the request.
�
processingConfiguration�B�:�
�
	appfabric+IngestionDestinationProcessingConfigurationeaws:appfabric/IngestionDestinationProcessingConfiguration:IngestionDestinationProcessingConfiguration;Contains information about how ingested data is processed.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
timeoutsxBv:t
r
	appfabricIngestionDestinationTimeoutsGaws:appfabric/IngestionDestinationTimeouts:IngestionDestinationTimeouts"]
appBundleArn" IThe Amazon Resource Name (ARN) of the app bundle to use for the request.
"-
arn" "ARN of the Ingestion Destination.
"�
destinationConfiguration�B�:�
�
	appfabric,IngestionDestinationDestinationConfigurationgaws:appfabric/IngestionDestinationDestinationConfiguration:IngestionDestinationDestinationConfiguration=Contains information about the destination of ingested data.
"\
ingestionArn" HThe Amazon Resource Name (ARN) of the ingestion to use for the request.
"�
processingConfiguration�B�:�
�
	appfabric+IngestionDestinationProcessingConfigurationeaws:appfabric/IngestionDestinationProcessingConfiguration:IngestionDestinationProcessingConfiguration;Contains information about how ingested data is processed.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
timeoutsxBv:t
r
	appfabricIngestionDestinationTimeoutsGaws:appfabric/IngestionDestinationTimeouts:IngestionDestinationTimeouts*��
J
appflowConnectorProfile-aws:appflow/connectorProfile:ConnectorProfile��Provides an AppFlow connector profile resource.

For information about AppFlow flows, see the [Amazon AppFlow API Reference](https://docs.aws.amazon.com/appflow/1.0/APIReference/Welcome.html).
For specific information about creating an AppFlow connector profile, see the
[CreateConnectorProfile](https://docs.aws.amazon.com/appflow/1.0/APIReference/API_CreateConnectorProfile.html) page in the Amazon AppFlow API Reference.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.iam.getPolicy({
    name: "AmazonRedshiftAllCommandsFullAccess",
});
const exampleRole = new aws.iam.Role("example", {
    name: "example_role",
    managedPolicyArns: [test.arn],
    assumeRolePolicy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Action: "sts:AssumeRole",
            Effect: "Allow",
            Sid: "",
            Principal: {
                Service: "ec2.amazonaws.com",
            },
        }],
    }),
});
const exampleBucketV2 = new aws.s3.BucketV2("example", {bucket: "example-bucket"});
const exampleCluster = new aws.redshift.Cluster("example", {
    clusterIdentifier: "example_cluster",
    databaseName: "example_db",
    masterUsername: "exampleuser",
    masterPassword: "examplePassword123!",
    nodeType: "dc1.large",
    clusterType: "single-node",
});
const exampleConnectorProfile = new aws.appflow.ConnectorProfile("example", {
    name: "example_profile",
    connectorType: "Redshift",
    connectionMode: "Public",
    connectorProfileConfig: {
        connectorProfileCredentials: {
            redshift: {
                password: exampleCluster.masterPassword,
                username: exampleCluster.masterUsername,
            },
        },
        connectorProfileProperties: {
            redshift: {
                bucketName: exampleBucketV2.name,
                databaseUrl: pulumi.interpolate`jdbc:redshift://${exampleCluster.endpoint}/${exampleCluster.databaseName}`,
                roleArn: exampleRole.arn,
            },
        },
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.iam.get_policy(name="AmazonRedshiftAllCommandsFullAccess")
example_role = aws.iam.Role("example",
    name="example_role",
    managed_policy_arns=[test["arn"]],
    assume_role_policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Action": "sts:AssumeRole",
            "Effect": "Allow",
            "Sid": "",
            "Principal": {
                "Service": "ec2.amazonaws.com",
            },
        }],
    }))
example_bucket_v2 = aws.s3.BucketV2("example", bucket="example-bucket")
example_cluster = aws.redshift.Cluster("example",
    cluster_identifier="example_cluster",
    database_name="example_db",
    master_username="exampleuser",
    master_password="examplePassword123!",
    node_type="dc1.large",
    cluster_type="single-node")
example_connector_profile = aws.appflow.ConnectorProfile("example",
    name="example_profile",
    connector_type="Redshift",
    connection_mode="Public",
    connector_profile_config={
        "connector_profile_credentials": {
            "redshift": {
                "password": example_cluster.master_password,
                "username": example_cluster.master_username,
            },
        },
        "connector_profile_properties": {
            "redshift": {
                "bucket_name": example_bucket_v2.name,
                "database_url": pulumi.Output.all(
                    endpoint=example_cluster.endpoint,
                    database_name=example_cluster.database_name
).apply(lambda resolved_outputs: f"jdbc:redshift://{resolved_outputs['endpoint']}/{resolved_outputs['database_name']}")
,
                "role_arn": example_role.arn,
            },
        },
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
    var example = Aws.Iam.GetPolicy.Invoke(new()
    {
        Name = "AmazonRedshiftAllCommandsFullAccess",
    });

    var exampleRole = new Aws.Iam.Role("example", new()
    {
        Name = "example_role",
        ManagedPolicyArns = new[]
        {
            test.Arn,
        },
        AssumeRolePolicy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Action"] = "sts:AssumeRole",
                    ["Effect"] = "Allow",
                    ["Sid"] = "",
                    ["Principal"] = new Dictionary<string, object?>
                    {
                        ["Service"] = "ec2.amazonaws.com",
                    },
                },
            },
        }),
    });

    var exampleBucketV2 = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example-bucket",
    });

    var exampleCluster = new Aws.RedShift.Cluster("example", new()
    {
        ClusterIdentifier = "example_cluster",
        DatabaseName = "example_db",
        MasterUsername = "exampleuser",
        MasterPassword = "examplePassword123!",
        NodeType = "dc1.large",
        ClusterType = "single-node",
    });

    var exampleConnectorProfile = new Aws.AppFlow.ConnectorProfile("example", new()
    {
        Name = "example_profile",
        ConnectorType = "Redshift",
        ConnectionMode = "Public",
        ConnectorProfileConfig = new Aws.AppFlow.Inputs.ConnectorProfileConnectorProfileConfigArgs
        {
            ConnectorProfileCredentials = new Aws.AppFlow.Inputs.ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsArgs
            {
                Redshift = new Aws.AppFlow.Inputs.ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshiftArgs
                {
                    Password = exampleCluster.MasterPassword,
                    Username = exampleCluster.MasterUsername,
                },
            },
            ConnectorProfileProperties = new Aws.AppFlow.Inputs.ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesArgs
            {
                Redshift = new Aws.AppFlow.Inputs.ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshiftArgs
                {
                    BucketName = exampleBucketV2.Name,
                    DatabaseUrl = Output.Tuple(exampleCluster.Endpoint, exampleCluster.DatabaseName).Apply(values =>
                    {
                        var endpoint = values.Item1;
                        var databaseName = values.Item2;
                        return $"jdbc:redshift://{endpoint}/{databaseName}";
                    }),
                    RoleArn = exampleRole.Arn,
                },
            },
        },
    });

});
```
```go
package main

import (
	"encoding/json"
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appflow"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/redshift"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := iam.LookupPolicy(ctx, &iam.LookupPolicyArgs{
			Name: pulumi.StringRef("AmazonRedshiftAllCommandsFullAccess"),
		}, nil)
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Action": "sts:AssumeRole",
					"Effect": "Allow",
					"Sid":    "",
					"Principal": map[string]interface{}{
						"Service": "ec2.amazonaws.com",
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			Name: pulumi.String("example_role"),
			ManagedPolicyArns: pulumi.StringArray{
				test.Arn,
			},
			AssumeRolePolicy: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		exampleBucketV2, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket: pulumi.String("example-bucket"),
		})
		if err != nil {
			return err
		}
		exampleCluster, err := redshift.NewCluster(ctx, "example", &redshift.ClusterArgs{
			ClusterIdentifier: pulumi.String("example_cluster"),
			DatabaseName:      pulumi.String("example_db"),
			MasterUsername:    pulumi.String("exampleuser"),
			MasterPassword:    pulumi.String("examplePassword123!"),
			NodeType:          pulumi.String("dc1.large"),
			ClusterType:       pulumi.String("single-node"),
		})
		if err != nil {
			return err
		}
		_, err = appflow.NewConnectorProfile(ctx, "example", &appflow.ConnectorProfileArgs{
			Name:           pulumi.String("example_profile"),
			ConnectorType:  pulumi.String("Redshift"),
			ConnectionMode: pulumi.String("Public"),
			ConnectorProfileConfig: &appflow.ConnectorProfileConnectorProfileConfigArgs{
				ConnectorProfileCredentials: &appflow.ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsArgs{
					Redshift: &appflow.ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshiftArgs{
						Password: exampleCluster.MasterPassword,
						Username: exampleCluster.MasterUsername,
					},
				},
				ConnectorProfileProperties: &appflow.ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesArgs{
					Redshift: &appflow.ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshiftArgs{
						BucketName: exampleBucketV2.Name,
						DatabaseUrl: pulumi.All(exampleCluster.Endpoint, exampleCluster.DatabaseName).ApplyT(func(_args []interface{}) (string, error) {
							endpoint := _args[0].(string)
							databaseName := _args[1].(string)
							return fmt.Sprintf("jdbc:redshift://%v/%v", endpoint, databaseName), nil
						}).(pulumi.StringOutput),
						RoleArn: exampleRole.Arn,
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
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.redshift.Cluster;
import com.pulumi.aws.redshift.ClusterArgs;
import com.pulumi.aws.appflow.ConnectorProfile;
import com.pulumi.aws.appflow.ConnectorProfileArgs;
import com.pulumi.aws.appflow.inputs.ConnectorProfileConnectorProfileConfigArgs;
import com.pulumi.aws.appflow.inputs.ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsArgs;
import com.pulumi.aws.appflow.inputs.ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshiftArgs;
import com.pulumi.aws.appflow.inputs.ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesArgs;
import com.pulumi.aws.appflow.inputs.ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshiftArgs;
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
        final var example = IamFunctions.getPolicy(GetPolicyArgs.builder()
            .name("AmazonRedshiftAllCommandsFullAccess")
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("example_role")
            .managedPolicyArns(test.arn())
            .assumeRolePolicy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Action", "sts:AssumeRole"),
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Sid", ""),
                        jsonProperty("Principal", jsonObject(
                            jsonProperty("Service", "ec2.amazonaws.com")
                        ))
                    )))
                )))
            .build());

        var exampleBucketV2 = new BucketV2("exampleBucketV2", BucketV2Args.builder()
            .bucket("example-bucket")
            .build());

        var exampleCluster = new Cluster("exampleCluster", ClusterArgs.builder()
            .clusterIdentifier("example_cluster")
            .databaseName("example_db")
            .masterUsername("exampleuser")
            .masterPassword("examplePassword123!")
            .nodeType("dc1.large")
            .clusterType("single-node")
            .build());

        var exampleConnectorProfile = new ConnectorProfile("exampleConnectorProfile", ConnectorProfileArgs.builder()
            .name("example_profile")
            .connectorType("Redshift")
            .connectionMode("Public")
            .connectorProfileConfig(ConnectorProfileConnectorProfileConfigArgs.builder()
                .connectorProfileCredentials(ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsArgs.builder()
                    .redshift(ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshiftArgs.builder()
                        .password(exampleCluster.masterPassword())
                        .username(exampleCluster.masterUsername())
                        .build())
                    .build())
                .connectorProfileProperties(ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesArgs.builder()
                    .redshift(ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshiftArgs.builder()
                        .bucketName(exampleBucketV2.name())
                        .databaseUrl(Output.tuple(exampleCluster.endpoint(), exampleCluster.databaseName()).applyValue(values -> {
                            var endpoint = values.t1;
                            var databaseName = values.t2;
                            return String.format("jdbc:redshift://%s/%s", endpoint,databaseName);
                        }))
                        .roleArn(exampleRole.arn())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: example_role
      managedPolicyArns:
        - ${test.arn}
      assumeRolePolicy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Action: sts:AssumeRole
              Effect: Allow
              Sid: ""
              Principal:
                Service: ec2.amazonaws.com
  exampleBucketV2:
    type: aws:s3:BucketV2
    name: example
    properties:
      bucket: example-bucket
  exampleCluster:
    type: aws:redshift:Cluster
    name: example
    properties:
      clusterIdentifier: example_cluster
      databaseName: example_db
      masterUsername: exampleuser
      masterPassword: examplePassword123!
      nodeType: dc1.large
      clusterType: single-node
  exampleConnectorProfile:
    type: aws:appflow:ConnectorProfile
    name: example
    properties:
      name: example_profile
      connectorType: Redshift
      connectionMode: Public
      connectorProfileConfig:
        connectorProfileCredentials:
          redshift:
            password: ${exampleCluster.masterPassword}
            username: ${exampleCluster.masterUsername}
        connectorProfileProperties:
          redshift:
            bucketName: ${exampleBucketV2.name}
            databaseUrl: jdbc:redshift://${exampleCluster.endpoint}/${exampleCluster.databaseName}
            roleArn: ${exampleRole.arn}
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicy
      arguments:
        name: AmazonRedshiftAllCommandsFullAccess
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppFlow Connector Profile using the connector profile `arn`. For example:

```sh
$ pulumi import aws:appflow/connectorProfile:ConnectorProfile profile arn:aws:appflow:us-west-2:123456789012:connectorprofile/example-profile
```
�
connectionMode" �Indicates the connection mode and specifies whether it is public or private. Private flows use AWS PrivateLink to route data over AWS infrastructure without exposing it to the public internet. One of: `Public`, `Private`.
�
connectorLabelB" �The label of the connector. The label is unique for each ConnectorRegistration in your AWS account. Only needed if calling for `CustomConnector` connector type.
�
connectorProfileConfig�:�
�
appflow&ConnectorProfileConnectorProfileConfigYaws:appflow/ConnectorProfileConnectorProfileConfig:ConnectorProfileConnectorProfileConfigmDefines the connector-specific configuration and credentials. See Connector Profile Config for more details.
�
connectorType" �The type of connector. One of: `Amplitude`, `CustomConnector`, `CustomerProfiles`, `Datadog`, `Dynatrace`, `EventBridge`, `Googleanalytics`, `Honeycode`, `Infornexus`, `LookoutMetrics`, `Marketo`, `Redshift`, `S3`, `Salesforce`, `SAPOData`, `Servicenow`, `Singular`, `Slack`, `Snowflake`, `Trendmicro`, `Upsolver`, `Veeva`, `Zendesk`.
�
kmsArnB" �ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.

nameB" ")
arn" ARN of the connector profile.
"�
connectionMode" �Indicates the connection mode and specifies whether it is public or private. Private flows use AWS PrivateLink to route data over AWS infrastructure without exposing it to the public internet. One of: `Public`, `Private`.
"�
connectorLabelB" �The label of the connector. The label is unique for each ConnectorRegistration in your AWS account. Only needed if calling for `CustomConnector` connector type.
"�
connectorProfileConfig�:�
�
appflow&ConnectorProfileConnectorProfileConfigYaws:appflow/ConnectorProfileConnectorProfileConfig:ConnectorProfileConnectorProfileConfigmDefines the connector-specific configuration and credentials. See Connector Profile Config for more details.
"�
connectorType" �The type of connector. One of: `Amplitude`, `CustomConnector`, `CustomerProfiles`, `Datadog`, `Dynatrace`, `EventBridge`, `Googleanalytics`, `Honeycode`, `Infornexus`, `LookoutMetrics`, `Marketo`, `Redshift`, `S3`, `Salesforce`, `SAPOData`, `Servicenow`, `Singular`, `Slack`, `Snowflake`, `Trendmicro`, `Upsolver`, `Veeva`, `Zendesk`.
"@
credentialsArn" *ARN of the connector profile credentials.
"�
kmsArn" �ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
"

name" *��
&
appflowFlowaws:appflow/flow:Flow��Provides an AppFlow flow resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleSourceBucketV2 = new aws.s3.BucketV2("example_source", {bucket: "example-source"});
const exampleSource = aws.iam.getPolicyDocument({
    statements: [{
        sid: "AllowAppFlowSourceActions",
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["appflow.amazonaws.com"],
        }],
        actions: [
            "s3:ListBucket",
            "s3:GetObject",
        ],
        resources: [
            "arn:aws:s3:::example-source",
            "arn:aws:s3:::example-source/*",
        ],
    }],
});
const exampleSourceBucketPolicy = new aws.s3.BucketPolicy("example_source", {
    bucket: exampleSourceBucketV2.id,
    policy: exampleSource.then(exampleSource => exampleSource.json),
});
const example = new aws.s3.BucketObjectv2("example", {
    bucket: exampleSourceBucketV2.id,
    key: "example_source.csv",
    source: new pulumi.asset.FileAsset("example_source.csv"),
});
const exampleDestinationBucketV2 = new aws.s3.BucketV2("example_destination", {bucket: "example-destination"});
const exampleDestination = aws.iam.getPolicyDocument({
    statements: [{
        sid: "AllowAppFlowDestinationActions",
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["appflow.amazonaws.com"],
        }],
        actions: [
            "s3:PutObject",
            "s3:AbortMultipartUpload",
            "s3:ListMultipartUploadParts",
            "s3:ListBucketMultipartUploads",
            "s3:GetBucketAcl",
            "s3:PutObjectAcl",
        ],
        resources: [
            "arn:aws:s3:::example-destination",
            "arn:aws:s3:::example-destination/*",
        ],
    }],
});
const exampleDestinationBucketPolicy = new aws.s3.BucketPolicy("example_destination", {
    bucket: exampleDestinationBucketV2.id,
    policy: exampleDestination.then(exampleDestination => exampleDestination.json),
});
const exampleFlow = new aws.appflow.Flow("example", {
    name: "example",
    sourceFlowConfig: {
        connectorType: "S3",
        sourceConnectorProperties: {
            s3: {
                bucketName: exampleSourceBucketPolicy.bucket,
                bucketPrefix: "example",
            },
        },
    },
    destinationFlowConfigs: [{
        connectorType: "S3",
        destinationConnectorProperties: {
            s3: {
                bucketName: exampleDestinationBucketPolicy.bucket,
                s3OutputFormatConfig: {
                    prefixConfig: {
                        prefixType: "PATH",
                    },
                },
            },
        },
    }],
    tasks: [{
        sourceFields: ["exampleField"],
        destinationField: "exampleField",
        taskType: "Map",
        connectorOperators: [{
            s3: "NO_OP",
        }],
    }],
    triggerConfig: {
        triggerType: "OnDemand",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example_source_bucket_v2 = aws.s3.BucketV2("example_source", bucket="example-source")
example_source = aws.iam.get_policy_document(statements=[{
    "sid": "AllowAppFlowSourceActions",
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["appflow.amazonaws.com"],
    }],
    "actions": [
        "s3:ListBucket",
        "s3:GetObject",
    ],
    "resources": [
        "arn:aws:s3:::example-source",
        "arn:aws:s3:::example-source/*",
    ],
}])
example_source_bucket_policy = aws.s3.BucketPolicy("example_source",
    bucket=example_source_bucket_v2.id,
    policy=example_source.json)
example = aws.s3.BucketObjectv2("example",
    bucket=example_source_bucket_v2.id,
    key="example_source.csv",
    source=pulumi.FileAsset("example_source.csv"))
example_destination_bucket_v2 = aws.s3.BucketV2("example_destination", bucket="example-destination")
example_destination = aws.iam.get_policy_document(statements=[{
    "sid": "AllowAppFlowDestinationActions",
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["appflow.amazonaws.com"],
    }],
    "actions": [
        "s3:PutObject",
        "s3:AbortMultipartUpload",
        "s3:ListMultipartUploadParts",
        "s3:ListBucketMultipartUploads",
        "s3:GetBucketAcl",
        "s3:PutObjectAcl",
    ],
    "resources": [
        "arn:aws:s3:::example-destination",
        "arn:aws:s3:::example-destination/*",
    ],
}])
example_destination_bucket_policy = aws.s3.BucketPolicy("example_destination",
    bucket=example_destination_bucket_v2.id,
    policy=example_destination.json)
example_flow = aws.appflow.Flow("example",
    name="example",
    source_flow_config={
        "connector_type": "S3",
        "source_connector_properties": {
            "s3": {
                "bucket_name": example_source_bucket_policy.bucket,
                "bucket_prefix": "example",
            },
        },
    },
    destination_flow_configs=[{
        "connector_type": "S3",
        "destination_connector_properties": {
            "s3": {
                "bucket_name": example_destination_bucket_policy.bucket,
                "s3_output_format_config": {
                    "prefix_config": {
                        "prefix_type": "PATH",
                    },
                },
            },
        },
    }],
    tasks=[{
        "source_fields": ["exampleField"],
        "destination_field": "exampleField",
        "task_type": "Map",
        "connector_operators": [{
            "s3": "NO_OP",
        }],
    }],
    trigger_config={
        "trigger_type": "OnDemand",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleSourceBucketV2 = new Aws.S3.BucketV2("example_source", new()
    {
        Bucket = "example-source",
    });

    var exampleSource = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "AllowAppFlowSourceActions",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "appflow.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:ListBucket",
                    "s3:GetObject",
                },
                Resources = new[]
                {
                    "arn:aws:s3:::example-source",
                    "arn:aws:s3:::example-source/*",
                },
            },
        },
    });

    var exampleSourceBucketPolicy = new Aws.S3.BucketPolicy("example_source", new()
    {
        Bucket = exampleSourceBucketV2.Id,
        Policy = exampleSource.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var example = new Aws.S3.BucketObjectv2("example", new()
    {
        Bucket = exampleSourceBucketV2.Id,
        Key = "example_source.csv",
        Source = new FileAsset("example_source.csv"),
    });

    var exampleDestinationBucketV2 = new Aws.S3.BucketV2("example_destination", new()
    {
        Bucket = "example-destination",
    });

    var exampleDestination = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "AllowAppFlowDestinationActions",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "appflow.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:PutObject",
                    "s3:AbortMultipartUpload",
                    "s3:ListMultipartUploadParts",
                    "s3:ListBucketMultipartUploads",
                    "s3:GetBucketAcl",
                    "s3:PutObjectAcl",
                },
                Resources = new[]
                {
                    "arn:aws:s3:::example-destination",
                    "arn:aws:s3:::example-destination/*",
                },
            },
        },
    });

    var exampleDestinationBucketPolicy = new Aws.S3.BucketPolicy("example_destination", new()
    {
        Bucket = exampleDestinationBucketV2.Id,
        Policy = exampleDestination.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleFlow = new Aws.AppFlow.Flow("example", new()
    {
        Name = "example",
        SourceFlowConfig = new Aws.AppFlow.Inputs.FlowSourceFlowConfigArgs
        {
            ConnectorType = "S3",
            SourceConnectorProperties = new Aws.AppFlow.Inputs.FlowSourceFlowConfigSourceConnectorPropertiesArgs
            {
                S3 = new Aws.AppFlow.Inputs.FlowSourceFlowConfigSourceConnectorPropertiesS3Args
                {
                    BucketName = exampleSourceBucketPolicy.Bucket,
                    BucketPrefix = "example",
                },
            },
        },
        DestinationFlowConfigs = new[]
        {
            new Aws.AppFlow.Inputs.FlowDestinationFlowConfigArgs
            {
                ConnectorType = "S3",
                DestinationConnectorProperties = new Aws.AppFlow.Inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesArgs
                {
                    S3 = new Aws.AppFlow.Inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesS3Args
                    {
                        BucketName = exampleDestinationBucketPolicy.Bucket,
                        S3OutputFormatConfig = new Aws.AppFlow.Inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigArgs
                        {
                            PrefixConfig = new Aws.AppFlow.Inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfigArgs
                            {
                                PrefixType = "PATH",
                            },
                        },
                    },
                },
            },
        },
        Tasks = new[]
        {
            new Aws.AppFlow.Inputs.FlowTaskArgs
            {
                SourceFields = new[]
                {
                    "exampleField",
                },
                DestinationField = "exampleField",
                TaskType = "Map",
                ConnectorOperators = new[]
                {
                    new Aws.AppFlow.Inputs.FlowTaskConnectorOperatorArgs
                    {
                        S3 = "NO_OP",
                    },
                },
            },
        },
        TriggerConfig = new Aws.AppFlow.Inputs.FlowTriggerConfigArgs
        {
            TriggerType = "OnDemand",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appflow"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleSourceBucketV2, err := s3.NewBucketV2(ctx, "example_source", &s3.BucketV2Args{
			Bucket: pulumi.String("example-source"),
		})
		if err != nil {
			return err
		}
		exampleSource, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid:    pulumi.StringRef("AllowAppFlowSourceActions"),
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"appflow.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"s3:ListBucket",
						"s3:GetObject",
					},
					Resources: []string{
						"arn:aws:s3:::example-source",
						"arn:aws:s3:::example-source/*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		exampleSourceBucketPolicy, err := s3.NewBucketPolicy(ctx, "example_source", &s3.BucketPolicyArgs{
			Bucket: exampleSourceBucketV2.ID(),
			Policy: pulumi.String(exampleSource.Json),
		})
		if err != nil {
			return err
		}
		_, err = s3.NewBucketObjectv2(ctx, "example", &s3.BucketObjectv2Args{
			Bucket: exampleSourceBucketV2.ID(),
			Key:    pulumi.String("example_source.csv"),
			Source: pulumi.NewFileAsset("example_source.csv"),
		})
		if err != nil {
			return err
		}
		exampleDestinationBucketV2, err := s3.NewBucketV2(ctx, "example_destination", &s3.BucketV2Args{
			Bucket: pulumi.String("example-destination"),
		})
		if err != nil {
			return err
		}
		exampleDestination, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid:    pulumi.StringRef("AllowAppFlowDestinationActions"),
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"appflow.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"s3:PutObject",
						"s3:AbortMultipartUpload",
						"s3:ListMultipartUploadParts",
						"s3:ListBucketMultipartUploads",
						"s3:GetBucketAcl",
						"s3:PutObjectAcl",
					},
					Resources: []string{
						"arn:aws:s3:::example-destination",
						"arn:aws:s3:::example-destination/*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		exampleDestinationBucketPolicy, err := s3.NewBucketPolicy(ctx, "example_destination", &s3.BucketPolicyArgs{
			Bucket: exampleDestinationBucketV2.ID(),
			Policy: pulumi.String(exampleDestination.Json),
		})
		if err != nil {
			return err
		}
		_, err = appflow.NewFlow(ctx, "example", &appflow.FlowArgs{
			Name: pulumi.String("example"),
			SourceFlowConfig: &appflow.FlowSourceFlowConfigArgs{
				ConnectorType: pulumi.String("S3"),
				SourceConnectorProperties: &appflow.FlowSourceFlowConfigSourceConnectorPropertiesArgs{
					S3: &appflow.FlowSourceFlowConfigSourceConnectorPropertiesS3Args{
						BucketName:   exampleSourceBucketPolicy.Bucket,
						BucketPrefix: pulumi.String("example"),
					},
				},
			},
			DestinationFlowConfigs: appflow.FlowDestinationFlowConfigArray{
				&appflow.FlowDestinationFlowConfigArgs{
					ConnectorType: pulumi.String("S3"),
					DestinationConnectorProperties: &appflow.FlowDestinationFlowConfigDestinationConnectorPropertiesArgs{
						S3: &appflow.FlowDestinationFlowConfigDestinationConnectorPropertiesS3Args{
							BucketName: exampleDestinationBucketPolicy.Bucket,
							S3OutputFormatConfig: &appflow.FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigArgs{
								PrefixConfig: &appflow.FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfigArgs{
									PrefixType: pulumi.String("PATH"),
								},
							},
						},
					},
				},
			},
			Tasks: appflow.FlowTaskArray{
				&appflow.FlowTaskArgs{
					SourceFields: pulumi.StringArray{
						pulumi.String("exampleField"),
					},
					DestinationField: pulumi.String("exampleField"),
					TaskType:         pulumi.String("Map"),
					ConnectorOperators: appflow.FlowTaskConnectorOperatorArray{
						&appflow.FlowTaskConnectorOperatorArgs{
							S3: pulumi.String("NO_OP"),
						},
					},
				},
			},
			TriggerConfig: &appflow.FlowTriggerConfigArgs{
				TriggerType: pulumi.String("OnDemand"),
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
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.s3.BucketPolicy;
import com.pulumi.aws.s3.BucketPolicyArgs;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.s3.BucketObjectv2Args;
import com.pulumi.aws.appflow.Flow;
import com.pulumi.aws.appflow.FlowArgs;
import com.pulumi.aws.appflow.inputs.FlowSourceFlowConfigArgs;
import com.pulumi.aws.appflow.inputs.FlowSourceFlowConfigSourceConnectorPropertiesArgs;
import com.pulumi.aws.appflow.inputs.FlowSourceFlowConfigSourceConnectorPropertiesS3Args;
import com.pulumi.aws.appflow.inputs.FlowDestinationFlowConfigArgs;
import com.pulumi.aws.appflow.inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesArgs;
import com.pulumi.aws.appflow.inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesS3Args;
import com.pulumi.aws.appflow.inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigArgs;
import com.pulumi.aws.appflow.inputs.FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfigArgs;
import com.pulumi.aws.appflow.inputs.FlowTaskArgs;
import com.pulumi.aws.appflow.inputs.FlowTriggerConfigArgs;
import com.pulumi.asset.FileAsset;
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
        var exampleSourceBucketV2 = new BucketV2("exampleSourceBucketV2", BucketV2Args.builder()
            .bucket("example-source")
            .build());

        final var exampleSource = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("AllowAppFlowSourceActions")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("appflow.amazonaws.com")
                    .build())
                .actions(                
                    "s3:ListBucket",
                    "s3:GetObject")
                .resources(                
                    "arn:aws:s3:::example-source",
                    "arn:aws:s3:::example-source/*")
                .build())
            .build());

        var exampleSourceBucketPolicy = new BucketPolicy("exampleSourceBucketPolicy", BucketPolicyArgs.builder()
            .bucket(exampleSourceBucketV2.id())
            .policy(exampleSource.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var example = new BucketObjectv2("example", BucketObjectv2Args.builder()
            .bucket(exampleSourceBucketV2.id())
            .key("example_source.csv")
            .source(new FileAsset("example_source.csv"))
            .build());

        var exampleDestinationBucketV2 = new BucketV2("exampleDestinationBucketV2", BucketV2Args.builder()
            .bucket("example-destination")
            .build());

        final var exampleDestination = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("AllowAppFlowDestinationActions")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("appflow.amazonaws.com")
                    .build())
                .actions(                
                    "s3:PutObject",
                    "s3:AbortMultipartUpload",
                    "s3:ListMultipartUploadParts",
                    "s3:ListBucketMultipartUploads",
                    "s3:GetBucketAcl",
                    "s3:PutObjectAcl")
                .resources(                
                    "arn:aws:s3:::example-destination",
                    "arn:aws:s3:::example-destination/*")
                .build())
            .build());

        var exampleDestinationBucketPolicy = new BucketPolicy("exampleDestinationBucketPolicy", BucketPolicyArgs.builder()
            .bucket(exampleDestinationBucketV2.id())
            .policy(exampleDestination.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var exampleFlow = new Flow("exampleFlow", FlowArgs.builder()
            .name("example")
            .sourceFlowConfig(FlowSourceFlowConfigArgs.builder()
                .connectorType("S3")
                .sourceConnectorProperties(FlowSourceFlowConfigSourceConnectorPropertiesArgs.builder()
                    .s3(FlowSourceFlowConfigSourceConnectorPropertiesS3Args.builder()
                        .bucketName(exampleSourceBucketPolicy.bucket())
                        .bucketPrefix("example")
                        .build())
                    .build())
                .build())
            .destinationFlowConfigs(FlowDestinationFlowConfigArgs.builder()
                .connectorType("S3")
                .destinationConnectorProperties(FlowDestinationFlowConfigDestinationConnectorPropertiesArgs.builder()
                    .s3(FlowDestinationFlowConfigDestinationConnectorPropertiesS3Args.builder()
                        .bucketName(exampleDestinationBucketPolicy.bucket())
                        .s3OutputFormatConfig(FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigArgs.builder()
                            .prefixConfig(FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfigArgs.builder()
                                .prefixType("PATH")
                                .build())
                            .build())
                        .build())
                    .build())
                .build())
            .tasks(FlowTaskArgs.builder()
                .sourceFields("exampleField")
                .destinationField("exampleField")
                .taskType("Map")
                .connectorOperators(FlowTaskConnectorOperatorArgs.builder()
                    .s3("NO_OP")
                    .build())
                .build())
            .triggerConfig(FlowTriggerConfigArgs.builder()
                .triggerType("OnDemand")
                .build())
            .build());

    }
}
```
```yaml
resources:
  exampleSourceBucketV2:
    type: aws:s3:BucketV2
    name: example_source
    properties:
      bucket: example-source
  exampleSourceBucketPolicy:
    type: aws:s3:BucketPolicy
    name: example_source
    properties:
      bucket: ${exampleSourceBucketV2.id}
      policy: ${exampleSource.json}
  example:
    type: aws:s3:BucketObjectv2
    properties:
      bucket: ${exampleSourceBucketV2.id}
      key: example_source.csv
      source:
        fn::FileAsset: example_source.csv
  exampleDestinationBucketV2:
    type: aws:s3:BucketV2
    name: example_destination
    properties:
      bucket: example-destination
  exampleDestinationBucketPolicy:
    type: aws:s3:BucketPolicy
    name: example_destination
    properties:
      bucket: ${exampleDestinationBucketV2.id}
      policy: ${exampleDestination.json}
  exampleFlow:
    type: aws:appflow:Flow
    name: example
    properties:
      name: example
      sourceFlowConfig:
        connectorType: S3
        sourceConnectorProperties:
          s3:
            bucketName: ${exampleSourceBucketPolicy.bucket}
            bucketPrefix: example
      destinationFlowConfigs:
        - connectorType: S3
          destinationConnectorProperties:
            s3:
              bucketName: ${exampleDestinationBucketPolicy.bucket}
              s3OutputFormatConfig:
                prefixConfig:
                  prefixType: PATH
      tasks:
        - sourceFields:
            - exampleField
          destinationField: exampleField
          taskType: Map
          connectorOperators:
            - s3: NO_OP
      triggerConfig:
        triggerType: OnDemand
variables:
  exampleSource:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: AllowAppFlowSourceActions
            effect: Allow
            principals:
              - type: Service
                identifiers:
                  - appflow.amazonaws.com
            actions:
              - s3:ListBucket
              - s3:GetObject
            resources:
              - arn:aws:s3:::example-source
              - arn:aws:s3:::example-source/*
  exampleDestination:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: AllowAppFlowDestinationActions
            effect: Allow
            principals:
              - type: Service
                identifiers:
                  - appflow.amazonaws.com
            actions:
              - s3:PutObject
              - s3:AbortMultipartUpload
              - s3:ListMultipartUploadParts
              - s3:ListBucketMultipartUploads
              - s3:GetBucketAcl
              - s3:PutObjectAcl
            resources:
              - arn:aws:s3:::example-destination
              - arn:aws:s3:::example-destination/*
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppFlow flows using the `arn`. For example:

```sh
$ pulumi import aws:appflow/flow:Flow example arn:aws:appflow:us-west-2:123456789012:flow/example-flow
```
A
descriptionB" ,Description of the flow you want to create.
�
destinationFlowConfigsk*i:g
e
appflowFlowDestinationFlowConfig?aws:appflow/FlowDestinationFlowConfig:FlowDestinationFlowConfigeA Destination Flow Config that controls how Amazon AppFlow places data in the destination connector.
�
kmsArnB" �ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
�
metadataCatalogConfigkBi:g
e
appflowFlowMetadataCatalogConfig?aws:appflow/FlowMetadataCatalogConfig:FlowMetadataCatalogConfig�A Catalog that determines the configuration that Amazon AppFlow uses when it catalogs the data that’s transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it stores metadata in a data catalog.
 
nameB" Name of the flow.
�
sourceFlowConfigZ:X
V
appflowFlowSourceFlowConfig5aws:appflow/FlowSourceFlowConfig:FlowSourceFlowConfigbThe Source Flow Config that controls how Amazon AppFlow retrieves data from the source connector.
�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
tasks8*6:4
2
appflowFlowTaskaws:appflow/FlowTask:FlowTaskQA Task that Amazon AppFlow performs while transferring the data in the flow run.
�
triggerConfigQ:O
M
appflowFlowTriggerConfig/aws:appflow/FlowTriggerConfig:FlowTriggerConfig5A Trigger that determine how and when the flow runs.
"
arn" Flow's ARN.
"A
descriptionB" ,Description of the flow you want to create.
"�
destinationFlowConfigsk*i:g
e
appflowFlowDestinationFlowConfig?aws:appflow/FlowDestinationFlowConfig:FlowDestinationFlowConfigeA Destination Flow Config that controls how Amazon AppFlow places data in the destination connector.
"2

flowStatus"  The current status of the flow.
"�
kmsArn" �ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
"�
metadataCatalogConfigi:g
e
appflowFlowMetadataCatalogConfig?aws:appflow/FlowMetadataCatalogConfig:FlowMetadataCatalogConfig�A Catalog that determines the configuration that Amazon AppFlow uses when it catalogs the data that’s transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it stores metadata in a data catalog.
"
name" Name of the flow.
"�
sourceFlowConfigZ:X
V
appflowFlowSourceFlowConfig5aws:appflow/FlowSourceFlowConfig:FlowSourceFlowConfigbThe Source Flow Config that controls how Amazon AppFlow retrieves data from the source connector.
"�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
tasks8*6:4
2
appflowFlowTaskaws:appflow/FlowTask:FlowTaskQA Task that Amazon AppFlow performs while transferring the data in the flow run.
"�
triggerConfigQ:O
M
appflowFlowTriggerConfig/aws:appflow/FlowTriggerConfig:FlowTriggerConfig5A Trigger that determine how and when the flow runs.
*�4
W
appintegrationsDataIntegration3aws:appintegrations/dataIntegration:DataIntegration�"Provides an Amazon AppIntegrations Data Integration resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appintegrations.DataIntegration("example", {
    name: "example",
    description: "example",
    kmsKey: test.arn,
    sourceUri: "Salesforce://AppFlow/example",
    scheduleConfig: {
        firstExecutionFrom: "1439788442681",
        object: "Account",
        scheduleExpression: "rate(1 hour)",
    },
    tags: {
        Key1: "Value1",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appintegrations.DataIntegration("example",
    name="example",
    description="example",
    kms_key=test["arn"],
    source_uri="Salesforce://AppFlow/example",
    schedule_config={
        "first_execution_from": "1439788442681",
        "object": "Account",
        "schedule_expression": "rate(1 hour)",
    },
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
    var example = new Aws.AppIntegrations.DataIntegration("example", new()
    {
        Name = "example",
        Description = "example",
        KmsKey = test.Arn,
        SourceUri = "Salesforce://AppFlow/example",
        ScheduleConfig = new Aws.AppIntegrations.Inputs.DataIntegrationScheduleConfigArgs
        {
            FirstExecutionFrom = "1439788442681",
            Object = "Account",
            ScheduleExpression = "rate(1 hour)",
        },
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
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appintegrations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appintegrations.NewDataIntegration(ctx, "example", &appintegrations.DataIntegrationArgs{
			Name:        pulumi.String("example"),
			Description: pulumi.String("example"),
			KmsKey:      pulumi.Any(test.Arn),
			SourceUri:   pulumi.String("Salesforce://AppFlow/example"),
			ScheduleConfig: &appintegrations.DataIntegrationScheduleConfigArgs{
				FirstExecutionFrom: pulumi.String("1439788442681"),
				Object:             pulumi.String("Account"),
				ScheduleExpression: pulumi.String("rate(1 hour)"),
			},
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
import com.pulumi.aws.appintegrations.DataIntegration;
import com.pulumi.aws.appintegrations.DataIntegrationArgs;
import com.pulumi.aws.appintegrations.inputs.DataIntegrationScheduleConfigArgs;
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
        var example = new DataIntegration("example", DataIntegrationArgs.builder()
            .name("example")
            .description("example")
            .kmsKey(test.arn())
            .sourceUri("Salesforce://AppFlow/example")
            .scheduleConfig(DataIntegrationScheduleConfigArgs.builder()
                .firstExecutionFrom("1439788442681")
                .object("Account")
                .scheduleExpression("rate(1 hour)")
                .build())
            .tags(Map.of("Key1", "Value1"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appintegrations:DataIntegration
    properties:
      name: example
      description: example
      kmsKey: ${test.arn}
      sourceUri: Salesforce://AppFlow/example
      scheduleConfig:
        firstExecutionFrom: '1439788442681'
        object: Account
        scheduleExpression: rate(1 hour)
      tags:
        Key1: Value1
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Amazon AppIntegrations Data Integrations using the `id`. For example:

```sh
$ pulumi import aws:appintegrations/dataIntegration:DataIntegration example 12345678-1234-1234-1234-123456789123
```
H
descriptionB" 3Specifies the description of the Data Integration.
Y
kmsKey" KSpecifies the KMS key Amazon Resource Name (ARN) for the Data Integration.
:
nameB" ,Specifies the name of the Data Integration.
�
scheduleConfig�:�
�
appintegrationsDataIntegrationScheduleConfigOaws:appintegrations/DataIntegrationScheduleConfig:DataIntegrationScheduleConfig�A block that defines the name of the data and how often it should be pulled from the source. The Schedule Config block is documented below.
�
	sourceUri" �Specifies the URI of the data source. Create an AppFlow Connector Profile and reference the name of the profile in the URL. An example of this value for Salesforce is `Salesforce://AppFlow/example` where `example` is the name of the AppFlow Connector Profile.
�
tagsB2" �Tags to apply to the Data Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"C
arn" 8The Amazon Resource Name (ARN) of the Data Integration.
"H
descriptionB" 3Specifies the description of the Data Integration.
"Y
kmsKey" KSpecifies the KMS key Amazon Resource Name (ARN) for the Data Integration.
"8
name" ,Specifies the name of the Data Integration.
"�
scheduleConfig�:�
�
appintegrationsDataIntegrationScheduleConfigOaws:appintegrations/DataIntegrationScheduleConfig:DataIntegrationScheduleConfig�A block that defines the name of the data and how often it should be pulled from the source. The Schedule Config block is documented below.
"�
	sourceUri" �Specifies the URI of the data source. Create an AppFlow Connector Profile and reference the name of the profile in the URL. An example of this value for Salesforce is `Salesforce://AppFlow/example` where `example` is the name of the AppFlow Connector Profile.
"�
tagsB2" �Tags to apply to the Data Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�A
S
applicationinsightsApplication/aws:applicationinsights/application:Application�+Provides a ApplicationInsights Application resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleGroup = new aws.resourcegroups.Group("example", {
    name: "example",
    resourceQuery: {
        query: JSON.stringify({
            ResourceTypeFilters: ["AWS::EC2::Instance"],
            TagFilters: [{
                Key: "Stage",
                Values: ["Test"],
            }],
        }),
    },
});
const example = new aws.applicationinsights.Application("example", {resourceGroupName: exampleGroup.name});
```
```python
import pulumi
import json
import pulumi_aws as aws

example_group = aws.resourcegroups.Group("example",
    name="example",
    resource_query={
        "query": json.dumps({
            "ResourceTypeFilters": ["AWS::EC2::Instance"],
            "TagFilters": [{
                "Key": "Stage",
                "Values": ["Test"],
            }],
        }),
    })
example = aws.applicationinsights.Application("example", resource_group_name=example_group.name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleGroup = new Aws.ResourceGroups.Group("example", new()
    {
        Name = "example",
        ResourceQuery = new Aws.ResourceGroups.Inputs.GroupResourceQueryArgs
        {
            Query = JsonSerializer.Serialize(new Dictionary<string, object?>
            {
                ["ResourceTypeFilters"] = new[]
                {
                    "AWS::EC2::Instance",
                },
                ["TagFilters"] = new[]
                {
                    new Dictionary<string, object?>
                    {
                        ["Key"] = "Stage",
                        ["Values"] = new[]
                        {
                            "Test",
                        },
                    },
                },
            }),
        },
    });

    var example = new Aws.ApplicationInsights.Application("example", new()
    {
        ResourceGroupName = exampleGroup.Name,
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/applicationinsights"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/resourcegroups"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"ResourceTypeFilters": []string{
				"AWS::EC2::Instance",
			},
			"TagFilters": []map[string]interface{}{
				map[string]interface{}{
					"Key": "Stage",
					"Values": []string{
						"Test",
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		exampleGroup, err := resourcegroups.NewGroup(ctx, "example", &resourcegroups.GroupArgs{
			Name: pulumi.String("example"),
			ResourceQuery: &resourcegroups.GroupResourceQueryArgs{
				Query: pulumi.String(json0),
			},
		})
		if err != nil {
			return err
		}
		_, err = applicationinsights.NewApplication(ctx, "example", &applicationinsights.ApplicationArgs{
			ResourceGroupName: exampleGroup.Name,
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
import com.pulumi.aws.resourcegroups.Group;
import com.pulumi.aws.resourcegroups.GroupArgs;
import com.pulumi.aws.resourcegroups.inputs.GroupResourceQueryArgs;
import com.pulumi.aws.applicationinsights.Application;
import com.pulumi.aws.applicationinsights.ApplicationArgs;
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
        var exampleGroup = new Group("exampleGroup", GroupArgs.builder()
            .name("example")
            .resourceQuery(GroupResourceQueryArgs.builder()
                .query(serializeJson(
                    jsonObject(
                        jsonProperty("ResourceTypeFilters", jsonArray("AWS::EC2::Instance")),
                        jsonProperty("TagFilters", jsonArray(jsonObject(
                            jsonProperty("Key", "Stage"),
                            jsonProperty("Values", jsonArray("Test"))
                        )))
                    )))
                .build())
            .build());

        var example = new Application("example", ApplicationArgs.builder()
            .resourceGroupName(exampleGroup.name())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:applicationinsights:Application
    properties:
      resourceGroupName: ${exampleGroup.name}
  exampleGroup:
    type: aws:resourcegroups:Group
    name: example
    properties:
      name: example
      resourceQuery:
        query:
          fn::toJSON:
            ResourceTypeFilters:
              - AWS::EC2::Instance
            TagFilters:
              - Key: Stage
                Values:
                  - Test
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import ApplicationInsights Applications using the `resource_group_name`. For example:

```sh
$ pulumi import aws:applicationinsights/application:Application some some-application
```
�
autoConfigEnabledB
 mIndicates whether Application Insights automatically configures unmonitored resources in the resource group.
v

autoCreateB
 bConfigures all of the resources in the resource group by applying the recommended configurations.
�
cweMonitorEnabledB
 �Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as instance terminated, failed deployment, and others.
�
groupingTypeB" �Application Insights can create applications based on a resource group or on an account. To create an account-based application using all of the resources in the account, set this parameter to `ACCOUNT_BASED`.
l
opsCenterEnabledB
 RWhen set to `true`, creates opsItems for any problems detected on an application.
�
opsItemSnsTopicArnB" �SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.
\
resourceGroupName" CName of the resource group.

The following arguments are optional:
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"#
arn" ARN of the Application.
"�
autoConfigEnabledB
 mIndicates whether Application Insights automatically configures unmonitored resources in the resource group.
"v

autoCreateB
 bConfigures all of the resources in the resource group by applying the recommended configurations.
"�
cweMonitorEnabledB
 �Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as instance terminated, failed deployment, and others.
"�
groupingTypeB" �Application Insights can create applications based on a resource group or on an account. To create an account-based application using all of the resources in the account, set this parameter to `ACCOUNT_BASED`.
"l
opsCenterEnabledB
 RWhen set to `true`, creates opsItems for any problems detected on an application.
"�
opsItemSnsTopicArnB" �SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.
"\
resourceGroupName" CName of the resource group.

The following arguments are optional:
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�E
>
appmeshGatewayRoute%aws:appmesh/gatewayRoute:GatewayRoute�4Provides an AWS App Mesh gateway route resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appmesh.GatewayRoute("example", {
    name: "example-gateway-route",
    meshName: "example-service-mesh",
    virtualGatewayName: exampleAwsAppmeshVirtualGateway.name,
    spec: {
        httpRoute: {
            action: {
                target: {
                    virtualService: {
                        virtualServiceName: exampleAwsAppmeshVirtualService.name,
                    },
                },
            },
            match: {
                prefix: "/",
            },
        },
    },
    tags: {
        Environment: "test",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appmesh.GatewayRoute("example",
    name="example-gateway-route",
    mesh_name="example-service-mesh",
    virtual_gateway_name=example_aws_appmesh_virtual_gateway["name"],
    spec={
        "http_route": {
            "action": {
                "target": {
                    "virtual_service": {
                        "virtual_service_name": example_aws_appmesh_virtual_service["name"],
                    },
                },
            },
            "match": {
                "prefix": "/",
            },
        },
    },
    tags={
        "Environment": "test",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppMesh.GatewayRoute("example", new()
    {
        Name = "example-gateway-route",
        MeshName = "example-service-mesh",
        VirtualGatewayName = exampleAwsAppmeshVirtualGateway.Name,
        Spec = new Aws.AppMesh.Inputs.GatewayRouteSpecArgs
        {
            HttpRoute = new Aws.AppMesh.Inputs.GatewayRouteSpecHttpRouteArgs
            {
                Action = new Aws.AppMesh.Inputs.GatewayRouteSpecHttpRouteActionArgs
                {
                    Target = new Aws.AppMesh.Inputs.GatewayRouteSpecHttpRouteActionTargetArgs
                    {
                        VirtualService = new Aws.AppMesh.Inputs.GatewayRouteSpecHttpRouteActionTargetVirtualServiceArgs
                        {
                            VirtualServiceName = exampleAwsAppmeshVirtualService.Name,
                        },
                    },
                },
                Match = new Aws.AppMesh.Inputs.GatewayRouteSpecHttpRouteMatchArgs
                {
                    Prefix = "/",
                },
            },
        },
        Tags = 
        {
            { "Environment", "test" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewGatewayRoute(ctx, "example", &appmesh.GatewayRouteArgs{
			Name:               pulumi.String("example-gateway-route"),
			MeshName:           pulumi.String("example-service-mesh"),
			VirtualGatewayName: pulumi.Any(exampleAwsAppmeshVirtualGateway.Name),
			Spec: &appmesh.GatewayRouteSpecArgs{
				HttpRoute: &appmesh.GatewayRouteSpecHttpRouteArgs{
					Action: &appmesh.GatewayRouteSpecHttpRouteActionArgs{
						Target: &appmesh.GatewayRouteSpecHttpRouteActionTargetArgs{
							VirtualService: &appmesh.GatewayRouteSpecHttpRouteActionTargetVirtualServiceArgs{
								VirtualServiceName: pulumi.Any(exampleAwsAppmeshVirtualService.Name),
							},
						},
					},
					Match: &appmesh.GatewayRouteSpecHttpRouteMatchArgs{
						Prefix: pulumi.String("/"),
					},
				},
			},
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("test"),
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
import com.pulumi.aws.appmesh.GatewayRoute;
import com.pulumi.aws.appmesh.GatewayRouteArgs;
import com.pulumi.aws.appmesh.inputs.GatewayRouteSpecArgs;
import com.pulumi.aws.appmesh.inputs.GatewayRouteSpecHttpRouteArgs;
import com.pulumi.aws.appmesh.inputs.GatewayRouteSpecHttpRouteActionArgs;
import com.pulumi.aws.appmesh.inputs.GatewayRouteSpecHttpRouteActionTargetArgs;
import com.pulumi.aws.appmesh.inputs.GatewayRouteSpecHttpRouteActionTargetVirtualServiceArgs;
import com.pulumi.aws.appmesh.inputs.GatewayRouteSpecHttpRouteMatchArgs;
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
        var example = new GatewayRoute("example", GatewayRouteArgs.builder()
            .name("example-gateway-route")
            .meshName("example-service-mesh")
            .virtualGatewayName(exampleAwsAppmeshVirtualGateway.name())
            .spec(GatewayRouteSpecArgs.builder()
                .httpRoute(GatewayRouteSpecHttpRouteArgs.builder()
                    .action(GatewayRouteSpecHttpRouteActionArgs.builder()
                        .target(GatewayRouteSpecHttpRouteActionTargetArgs.builder()
                            .virtualService(GatewayRouteSpecHttpRouteActionTargetVirtualServiceArgs.builder()
                                .virtualServiceName(exampleAwsAppmeshVirtualService.name())
                                .build())
                            .build())
                        .build())
                    .match(GatewayRouteSpecHttpRouteMatchArgs.builder()
                        .prefix("/")
                        .build())
                    .build())
                .build())
            .tags(Map.of("Environment", "test"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appmesh:GatewayRoute
    properties:
      name: example-gateway-route
      meshName: example-service-mesh
      virtualGatewayName: ${exampleAwsAppmeshVirtualGateway.name}
      spec:
        httpRoute:
          action:
            target:
              virtualService:
                virtualServiceName: ${exampleAwsAppmeshVirtualService.name}
          match:
            prefix: /
      tags:
        Environment: test
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Mesh gateway routes using `mesh_name` and `virtual_gateway_name` together with the gateway route's `name`. For example:

```sh
$ pulumi import aws:appmesh/gatewayRoute:GatewayRoute example mesh/gw1/example-gateway-route
```

meshName" oName of the service mesh in which to create the gateway route. Must be between 1 and 255 characters in length.
�
	meshOwnerB" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
a
nameB" SName to use for the gateway route. Must be between 1 and 255 characters in length.
~
specN:L
J
appmeshGatewayRouteSpec-aws:appmesh/GatewayRouteSpec:GatewayRouteSpec&Gateway route specification to apply.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
virtualGatewayName" qName of the virtual gateway to associate the gateway route with. Must be between 1 and 255 characters in length.
"%
arn" ARN of the gateway route.
"7
createdDate" $Creation date of the gateway route.
">
lastUpdatedDate" 'Last update date of the gateway route.
"
meshName" oName of the service mesh in which to create the gateway route. Must be between 1 and 255 characters in length.
"�
	meshOwner" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
"_
name" SName to use for the gateway route. Must be between 1 and 255 characters in length.
"6
resourceOwner" !Resource owner's AWS account ID.
"~
specN:L
J
appmeshGatewayRouteSpec-aws:appmesh/GatewayRouteSpec:GatewayRouteSpec&Gateway route specification to apply.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
virtualGatewayName" qName of the virtual gateway to associate the gateway route with. Must be between 1 and 255 characters in length.
*�-
&
appmeshMeshaws:appmesh/mesh:Mesh�#Provides an AWS App Mesh service mesh resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const simple = new aws.appmesh.Mesh("simple", {name: "simpleapp"});
```
```python
import pulumi
import pulumi_aws as aws

simple = aws.appmesh.Mesh("simple", name="simpleapp")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var simple = new Aws.AppMesh.Mesh("simple", new()
    {
        Name = "simpleapp",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewMesh(ctx, "simple", &appmesh.MeshArgs{
			Name: pulumi.String("simpleapp"),
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
import com.pulumi.aws.appmesh.Mesh;
import com.pulumi.aws.appmesh.MeshArgs;
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
        var simple = new Mesh("simple", MeshArgs.builder()
            .name("simpleapp")
            .build());

    }
}
```
```yaml
resources:
  simple:
    type: aws:appmesh:Mesh
    properties:
      name: simpleapp
```
<!--End PulumiCodeChooser -->

### Egress Filter

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const simple = new aws.appmesh.Mesh("simple", {
    name: "simpleapp",
    spec: {
        egressFilter: {
            type: "ALLOW_ALL",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

simple = aws.appmesh.Mesh("simple",
    name="simpleapp",
    spec={
        "egress_filter": {
            "type": "ALLOW_ALL",
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
    var simple = new Aws.AppMesh.Mesh("simple", new()
    {
        Name = "simpleapp",
        Spec = new Aws.AppMesh.Inputs.MeshSpecArgs
        {
            EgressFilter = new Aws.AppMesh.Inputs.MeshSpecEgressFilterArgs
            {
                Type = "ALLOW_ALL",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewMesh(ctx, "simple", &appmesh.MeshArgs{
			Name: pulumi.String("simpleapp"),
			Spec: &appmesh.MeshSpecArgs{
				EgressFilter: &appmesh.MeshSpecEgressFilterArgs{
					Type: pulumi.String("ALLOW_ALL"),
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
import com.pulumi.aws.appmesh.Mesh;
import com.pulumi.aws.appmesh.MeshArgs;
import com.pulumi.aws.appmesh.inputs.MeshSpecArgs;
import com.pulumi.aws.appmesh.inputs.MeshSpecEgressFilterArgs;
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
        var simple = new Mesh("simple", MeshArgs.builder()
            .name("simpleapp")
            .spec(MeshSpecArgs.builder()
                .egressFilter(MeshSpecEgressFilterArgs.builder()
                    .type("ALLOW_ALL")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  simple:
    type: aws:appmesh:Mesh
    properties:
      name: simpleapp
      spec:
        egressFilter:
          type: ALLOW_ALL
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Mesh service meshes using the `name`. For example:

```sh
$ pulumi import aws:appmesh/mesh:Mesh simple simpleapp
```
`
nameB" RName to use for the service mesh. Must be between 1 and 255 characters in length.
g
spec8B6:4
2
appmeshMeshSpecaws:appmesh/MeshSpec:MeshSpec%Service mesh specification to apply.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"$
arn" ARN of the service mesh.
"6
createdDate" #Creation date of the service mesh.
"=
lastUpdatedDate" &Last update date of the service mesh.
"=
	meshOwner" ,AWS account ID of the service mesh's owner.
"^
name" RName to use for the service mesh. Must be between 1 and 255 characters in length.
"6
resourceOwner" !Resource owner's AWS account ID.
"g
spec8B6:4
2
appmeshMeshSpecaws:appmesh/MeshSpec:MeshSpec%Service mesh specification to apply.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
)
appmeshRouteaws:appmesh/route:Route��Provides an AWS App Mesh route resource.

## Example Usage

### HTTP Routing

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb = new aws.appmesh.Route("serviceb", {
    name: "serviceB-route",
    meshName: simple.id,
    virtualRouterName: servicebAwsAppmeshVirtualRouter.name,
    spec: {
        httpRoute: {
            match: {
                prefix: "/",
            },
            action: {
                weightedTargets: [
                    {
                        virtualNode: serviceb1.name,
                        weight: 90,
                    },
                    {
                        virtualNode: serviceb2.name,
                        weight: 10,
                    },
                ],
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb = aws.appmesh.Route("serviceb",
    name="serviceB-route",
    mesh_name=simple["id"],
    virtual_router_name=serviceb_aws_appmesh_virtual_router["name"],
    spec={
        "http_route": {
            "match": {
                "prefix": "/",
            },
            "action": {
                "weighted_targets": [
                    {
                        "virtual_node": serviceb1["name"],
                        "weight": 90,
                    },
                    {
                        "virtual_node": serviceb2["name"],
                        "weight": 10,
                    },
                ],
            },
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
    var serviceb = new Aws.AppMesh.Route("serviceb", new()
    {
        Name = "serviceB-route",
        MeshName = simple.Id,
        VirtualRouterName = servicebAwsAppmeshVirtualRouter.Name,
        Spec = new Aws.AppMesh.Inputs.RouteSpecArgs
        {
            HttpRoute = new Aws.AppMesh.Inputs.RouteSpecHttpRouteArgs
            {
                Match = new Aws.AppMesh.Inputs.RouteSpecHttpRouteMatchArgs
                {
                    Prefix = "/",
                },
                Action = new Aws.AppMesh.Inputs.RouteSpecHttpRouteActionArgs
                {
                    WeightedTargets = new[]
                    {
                        new Aws.AppMesh.Inputs.RouteSpecHttpRouteActionWeightedTargetArgs
                        {
                            VirtualNode = serviceb1.Name,
                            Weight = 90,
                        },
                        new Aws.AppMesh.Inputs.RouteSpecHttpRouteActionWeightedTargetArgs
                        {
                            VirtualNode = serviceb2.Name,
                            Weight = 10,
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewRoute(ctx, "serviceb", &appmesh.RouteArgs{
			Name:              pulumi.String("serviceB-route"),
			MeshName:          pulumi.Any(simple.Id),
			VirtualRouterName: pulumi.Any(servicebAwsAppmeshVirtualRouter.Name),
			Spec: &appmesh.RouteSpecArgs{
				HttpRoute: &appmesh.RouteSpecHttpRouteArgs{
					Match: &appmesh.RouteSpecHttpRouteMatchArgs{
						Prefix: pulumi.String("/"),
					},
					Action: &appmesh.RouteSpecHttpRouteActionArgs{
						WeightedTargets: appmesh.RouteSpecHttpRouteActionWeightedTargetArray{
							&appmesh.RouteSpecHttpRouteActionWeightedTargetArgs{
								VirtualNode: pulumi.Any(serviceb1.Name),
								Weight:      pulumi.Int(90),
							},
							&appmesh.RouteSpecHttpRouteActionWeightedTargetArgs{
								VirtualNode: pulumi.Any(serviceb2.Name),
								Weight:      pulumi.Int(10),
							},
						},
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
import com.pulumi.aws.appmesh.Route;
import com.pulumi.aws.appmesh.RouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteMatchArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteActionArgs;
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
        var serviceb = new Route("serviceb", RouteArgs.builder()
            .name("serviceB-route")
            .meshName(simple.id())
            .virtualRouterName(servicebAwsAppmeshVirtualRouter.name())
            .spec(RouteSpecArgs.builder()
                .httpRoute(RouteSpecHttpRouteArgs.builder()
                    .match(RouteSpecHttpRouteMatchArgs.builder()
                        .prefix("/")
                        .build())
                    .action(RouteSpecHttpRouteActionArgs.builder()
                        .weightedTargets(                        
                            RouteSpecHttpRouteActionWeightedTargetArgs.builder()
                                .virtualNode(serviceb1.name())
                                .weight(90)
                                .build(),
                            RouteSpecHttpRouteActionWeightedTargetArgs.builder()
                                .virtualNode(serviceb2.name())
                                .weight(10)
                                .build())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb:
    type: aws:appmesh:Route
    properties:
      name: serviceB-route
      meshName: ${simple.id}
      virtualRouterName: ${servicebAwsAppmeshVirtualRouter.name}
      spec:
        httpRoute:
          match:
            prefix: /
          action:
            weightedTargets:
              - virtualNode: ${serviceb1.name}
                weight: 90
              - virtualNode: ${serviceb2.name}
                weight: 10
```
<!--End PulumiCodeChooser -->

### HTTP Header Routing

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb = new aws.appmesh.Route("serviceb", {
    name: "serviceB-route",
    meshName: simple.id,
    virtualRouterName: servicebAwsAppmeshVirtualRouter.name,
    spec: {
        httpRoute: {
            match: {
                method: "POST",
                prefix: "/",
                scheme: "https",
                headers: [{
                    name: "clientRequestId",
                    match: {
                        prefix: "123",
                    },
                }],
            },
            action: {
                weightedTargets: [{
                    virtualNode: servicebAwsAppmeshVirtualNode.name,
                    weight: 100,
                }],
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb = aws.appmesh.Route("serviceb",
    name="serviceB-route",
    mesh_name=simple["id"],
    virtual_router_name=serviceb_aws_appmesh_virtual_router["name"],
    spec={
        "http_route": {
            "match": {
                "method": "POST",
                "prefix": "/",
                "scheme": "https",
                "headers": [{
                    "name": "clientRequestId",
                    "match": {
                        "prefix": "123",
                    },
                }],
            },
            "action": {
                "weighted_targets": [{
                    "virtual_node": serviceb_aws_appmesh_virtual_node["name"],
                    "weight": 100,
                }],
            },
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
    var serviceb = new Aws.AppMesh.Route("serviceb", new()
    {
        Name = "serviceB-route",
        MeshName = simple.Id,
        VirtualRouterName = servicebAwsAppmeshVirtualRouter.Name,
        Spec = new Aws.AppMesh.Inputs.RouteSpecArgs
        {
            HttpRoute = new Aws.AppMesh.Inputs.RouteSpecHttpRouteArgs
            {
                Match = new Aws.AppMesh.Inputs.RouteSpecHttpRouteMatchArgs
                {
                    Method = "POST",
                    Prefix = "/",
                    Scheme = "https",
                    Headers = new[]
                    {
                        new Aws.AppMesh.Inputs.RouteSpecHttpRouteMatchHeaderArgs
                        {
                            Name = "clientRequestId",
                            Match = new Aws.AppMesh.Inputs.RouteSpecHttpRouteMatchHeaderMatchArgs
                            {
                                Prefix = "123",
                            },
                        },
                    },
                },
                Action = new Aws.AppMesh.Inputs.RouteSpecHttpRouteActionArgs
                {
                    WeightedTargets = new[]
                    {
                        new Aws.AppMesh.Inputs.RouteSpecHttpRouteActionWeightedTargetArgs
                        {
                            VirtualNode = servicebAwsAppmeshVirtualNode.Name,
                            Weight = 100,
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewRoute(ctx, "serviceb", &appmesh.RouteArgs{
			Name:              pulumi.String("serviceB-route"),
			MeshName:          pulumi.Any(simple.Id),
			VirtualRouterName: pulumi.Any(servicebAwsAppmeshVirtualRouter.Name),
			Spec: &appmesh.RouteSpecArgs{
				HttpRoute: &appmesh.RouteSpecHttpRouteArgs{
					Match: &appmesh.RouteSpecHttpRouteMatchArgs{
						Method: pulumi.String("POST"),
						Prefix: pulumi.String("/"),
						Scheme: pulumi.String("https"),
						Headers: appmesh.RouteSpecHttpRouteMatchHeaderArray{
							&appmesh.RouteSpecHttpRouteMatchHeaderArgs{
								Name: pulumi.String("clientRequestId"),
								Match: &appmesh.RouteSpecHttpRouteMatchHeaderMatchArgs{
									Prefix: pulumi.String("123"),
								},
							},
						},
					},
					Action: &appmesh.RouteSpecHttpRouteActionArgs{
						WeightedTargets: appmesh.RouteSpecHttpRouteActionWeightedTargetArray{
							&appmesh.RouteSpecHttpRouteActionWeightedTargetArgs{
								VirtualNode: pulumi.Any(servicebAwsAppmeshVirtualNode.Name),
								Weight:      pulumi.Int(100),
							},
						},
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
import com.pulumi.aws.appmesh.Route;
import com.pulumi.aws.appmesh.RouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteMatchArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteActionArgs;
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
        var serviceb = new Route("serviceb", RouteArgs.builder()
            .name("serviceB-route")
            .meshName(simple.id())
            .virtualRouterName(servicebAwsAppmeshVirtualRouter.name())
            .spec(RouteSpecArgs.builder()
                .httpRoute(RouteSpecHttpRouteArgs.builder()
                    .match(RouteSpecHttpRouteMatchArgs.builder()
                        .method("POST")
                        .prefix("/")
                        .scheme("https")
                        .headers(RouteSpecHttpRouteMatchHeaderArgs.builder()
                            .name("clientRequestId")
                            .match(RouteSpecHttpRouteMatchHeaderMatchArgs.builder()
                                .prefix("123")
                                .build())
                            .build())
                        .build())
                    .action(RouteSpecHttpRouteActionArgs.builder()
                        .weightedTargets(RouteSpecHttpRouteActionWeightedTargetArgs.builder()
                            .virtualNode(servicebAwsAppmeshVirtualNode.name())
                            .weight(100)
                            .build())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb:
    type: aws:appmesh:Route
    properties:
      name: serviceB-route
      meshName: ${simple.id}
      virtualRouterName: ${servicebAwsAppmeshVirtualRouter.name}
      spec:
        httpRoute:
          match:
            method: POST
            prefix: /
            scheme: https
            headers:
              - name: clientRequestId
                match:
                  prefix: '123'
          action:
            weightedTargets:
              - virtualNode: ${servicebAwsAppmeshVirtualNode.name}
                weight: 100
```
<!--End PulumiCodeChooser -->

### Retry Policy

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb = new aws.appmesh.Route("serviceb", {
    name: "serviceB-route",
    meshName: simple.id,
    virtualRouterName: servicebAwsAppmeshVirtualRouter.name,
    spec: {
        httpRoute: {
            match: {
                prefix: "/",
            },
            retryPolicy: {
                httpRetryEvents: ["server-error"],
                maxRetries: 1,
                perRetryTimeout: {
                    unit: "s",
                    value: 15,
                },
            },
            action: {
                weightedTargets: [{
                    virtualNode: servicebAwsAppmeshVirtualNode.name,
                    weight: 100,
                }],
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb = aws.appmesh.Route("serviceb",
    name="serviceB-route",
    mesh_name=simple["id"],
    virtual_router_name=serviceb_aws_appmesh_virtual_router["name"],
    spec={
        "http_route": {
            "match": {
                "prefix": "/",
            },
            "retry_policy": {
                "http_retry_events": ["server-error"],
                "max_retries": 1,
                "per_retry_timeout": {
                    "unit": "s",
                    "value": 15,
                },
            },
            "action": {
                "weighted_targets": [{
                    "virtual_node": serviceb_aws_appmesh_virtual_node["name"],
                    "weight": 100,
                }],
            },
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
    var serviceb = new Aws.AppMesh.Route("serviceb", new()
    {
        Name = "serviceB-route",
        MeshName = simple.Id,
        VirtualRouterName = servicebAwsAppmeshVirtualRouter.Name,
        Spec = new Aws.AppMesh.Inputs.RouteSpecArgs
        {
            HttpRoute = new Aws.AppMesh.Inputs.RouteSpecHttpRouteArgs
            {
                Match = new Aws.AppMesh.Inputs.RouteSpecHttpRouteMatchArgs
                {
                    Prefix = "/",
                },
                RetryPolicy = new Aws.AppMesh.Inputs.RouteSpecHttpRouteRetryPolicyArgs
                {
                    HttpRetryEvents = new[]
                    {
                        "server-error",
                    },
                    MaxRetries = 1,
                    PerRetryTimeout = new Aws.AppMesh.Inputs.RouteSpecHttpRouteRetryPolicyPerRetryTimeoutArgs
                    {
                        Unit = "s",
                        Value = 15,
                    },
                },
                Action = new Aws.AppMesh.Inputs.RouteSpecHttpRouteActionArgs
                {
                    WeightedTargets = new[]
                    {
                        new Aws.AppMesh.Inputs.RouteSpecHttpRouteActionWeightedTargetArgs
                        {
                            VirtualNode = servicebAwsAppmeshVirtualNode.Name,
                            Weight = 100,
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewRoute(ctx, "serviceb", &appmesh.RouteArgs{
			Name:              pulumi.String("serviceB-route"),
			MeshName:          pulumi.Any(simple.Id),
			VirtualRouterName: pulumi.Any(servicebAwsAppmeshVirtualRouter.Name),
			Spec: &appmesh.RouteSpecArgs{
				HttpRoute: &appmesh.RouteSpecHttpRouteArgs{
					Match: &appmesh.RouteSpecHttpRouteMatchArgs{
						Prefix: pulumi.String("/"),
					},
					RetryPolicy: &appmesh.RouteSpecHttpRouteRetryPolicyArgs{
						HttpRetryEvents: pulumi.StringArray{
							pulumi.String("server-error"),
						},
						MaxRetries: pulumi.Int(1),
						PerRetryTimeout: &appmesh.RouteSpecHttpRouteRetryPolicyPerRetryTimeoutArgs{
							Unit:  pulumi.String("s"),
							Value: pulumi.Int(15),
						},
					},
					Action: &appmesh.RouteSpecHttpRouteActionArgs{
						WeightedTargets: appmesh.RouteSpecHttpRouteActionWeightedTargetArray{
							&appmesh.RouteSpecHttpRouteActionWeightedTargetArgs{
								VirtualNode: pulumi.Any(servicebAwsAppmeshVirtualNode.Name),
								Weight:      pulumi.Int(100),
							},
						},
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
import com.pulumi.aws.appmesh.Route;
import com.pulumi.aws.appmesh.RouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteMatchArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteRetryPolicyArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteRetryPolicyPerRetryTimeoutArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecHttpRouteActionArgs;
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
        var serviceb = new Route("serviceb", RouteArgs.builder()
            .name("serviceB-route")
            .meshName(simple.id())
            .virtualRouterName(servicebAwsAppmeshVirtualRouter.name())
            .spec(RouteSpecArgs.builder()
                .httpRoute(RouteSpecHttpRouteArgs.builder()
                    .match(RouteSpecHttpRouteMatchArgs.builder()
                        .prefix("/")
                        .build())
                    .retryPolicy(RouteSpecHttpRouteRetryPolicyArgs.builder()
                        .httpRetryEvents("server-error")
                        .maxRetries(1)
                        .perRetryTimeout(RouteSpecHttpRouteRetryPolicyPerRetryTimeoutArgs.builder()
                            .unit("s")
                            .value(15)
                            .build())
                        .build())
                    .action(RouteSpecHttpRouteActionArgs.builder()
                        .weightedTargets(RouteSpecHttpRouteActionWeightedTargetArgs.builder()
                            .virtualNode(servicebAwsAppmeshVirtualNode.name())
                            .weight(100)
                            .build())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb:
    type: aws:appmesh:Route
    properties:
      name: serviceB-route
      meshName: ${simple.id}
      virtualRouterName: ${servicebAwsAppmeshVirtualRouter.name}
      spec:
        httpRoute:
          match:
            prefix: /
          retryPolicy:
            httpRetryEvents:
              - server-error
            maxRetries: 1
            perRetryTimeout:
              unit: s
              value: 15
          action:
            weightedTargets:
              - virtualNode: ${servicebAwsAppmeshVirtualNode.name}
                weight: 100
```
<!--End PulumiCodeChooser -->

### TCP Routing

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb = new aws.appmesh.Route("serviceb", {
    name: "serviceB-route",
    meshName: simple.id,
    virtualRouterName: servicebAwsAppmeshVirtualRouter.name,
    spec: {
        tcpRoute: {
            action: {
                weightedTargets: [{
                    virtualNode: serviceb1.name,
                    weight: 100,
                }],
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb = aws.appmesh.Route("serviceb",
    name="serviceB-route",
    mesh_name=simple["id"],
    virtual_router_name=serviceb_aws_appmesh_virtual_router["name"],
    spec={
        "tcp_route": {
            "action": {
                "weighted_targets": [{
                    "virtual_node": serviceb1["name"],
                    "weight": 100,
                }],
            },
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
    var serviceb = new Aws.AppMesh.Route("serviceb", new()
    {
        Name = "serviceB-route",
        MeshName = simple.Id,
        VirtualRouterName = servicebAwsAppmeshVirtualRouter.Name,
        Spec = new Aws.AppMesh.Inputs.RouteSpecArgs
        {
            TcpRoute = new Aws.AppMesh.Inputs.RouteSpecTcpRouteArgs
            {
                Action = new Aws.AppMesh.Inputs.RouteSpecTcpRouteActionArgs
                {
                    WeightedTargets = new[]
                    {
                        new Aws.AppMesh.Inputs.RouteSpecTcpRouteActionWeightedTargetArgs
                        {
                            VirtualNode = serviceb1.Name,
                            Weight = 100,
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewRoute(ctx, "serviceb", &appmesh.RouteArgs{
			Name:              pulumi.String("serviceB-route"),
			MeshName:          pulumi.Any(simple.Id),
			VirtualRouterName: pulumi.Any(servicebAwsAppmeshVirtualRouter.Name),
			Spec: &appmesh.RouteSpecArgs{
				TcpRoute: &appmesh.RouteSpecTcpRouteArgs{
					Action: &appmesh.RouteSpecTcpRouteActionArgs{
						WeightedTargets: appmesh.RouteSpecTcpRouteActionWeightedTargetArray{
							&appmesh.RouteSpecTcpRouteActionWeightedTargetArgs{
								VirtualNode: pulumi.Any(serviceb1.Name),
								Weight:      pulumi.Int(100),
							},
						},
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
import com.pulumi.aws.appmesh.Route;
import com.pulumi.aws.appmesh.RouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecTcpRouteArgs;
import com.pulumi.aws.appmesh.inputs.RouteSpecTcpRouteActionArgs;
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
        var serviceb = new Route("serviceb", RouteArgs.builder()
            .name("serviceB-route")
            .meshName(simple.id())
            .virtualRouterName(servicebAwsAppmeshVirtualRouter.name())
            .spec(RouteSpecArgs.builder()
                .tcpRoute(RouteSpecTcpRouteArgs.builder()
                    .action(RouteSpecTcpRouteActionArgs.builder()
                        .weightedTargets(RouteSpecTcpRouteActionWeightedTargetArgs.builder()
                            .virtualNode(serviceb1.name())
                            .weight(100)
                            .build())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb:
    type: aws:appmesh:Route
    properties:
      name: serviceB-route
      meshName: ${simple.id}
      virtualRouterName: ${servicebAwsAppmeshVirtualRouter.name}
      spec:
        tcpRoute:
          action:
            weightedTargets:
              - virtualNode: ${serviceb1.name}
                weight: 100
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Mesh virtual routes using `mesh_name` and `virtual_router_name` together with the route's `name`. For example:

```sh
$ pulumi import aws:appmesh/route:Route serviceb simpleapp/serviceB/serviceB-route
```
w
meshName" gName of the service mesh in which to create the route. Must be between 1 and 255 characters in length.
�
	meshOwnerB" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
Y
nameB" KName to use for the route. Must be between 1 and 255 characters in length.
a
spec9:7
5
appmesh	RouteSpecaws:appmesh/RouteSpec:RouteSpecRoute specification to apply.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
virtualRouterName" iName of the virtual router in which to create the route. Must be between 1 and 255 characters in length.
"
arn" ARN of the route.
"/
createdDate" Creation date of the route.
"6
lastUpdatedDate" Last update date of the route.
"w
meshName" gName of the service mesh in which to create the route. Must be between 1 and 255 characters in length.
"�
	meshOwner" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
"W
name" KName to use for the route. Must be between 1 and 255 characters in length.
"6
resourceOwner" !Resource owner's AWS account ID.
"a
spec9:7
5
appmesh	RouteSpecaws:appmesh/RouteSpec:RouteSpecRoute specification to apply.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
virtualRouterName" iName of the virtual router in which to create the route. Must be between 1 and 255 characters in length.
*�l
D
appmeshVirtualGateway)aws:appmesh/virtualGateway:VirtualGateway�]Provides an AWS App Mesh virtual gateway resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appmesh.VirtualGateway("example", {
    name: "example-virtual-gateway",
    meshName: "example-service-mesh",
    spec: {
        listeners: [{
            portMapping: {
                port: 8080,
                protocol: "http",
            },
        }],
    },
    tags: {
        Environment: "test",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appmesh.VirtualGateway("example",
    name="example-virtual-gateway",
    mesh_name="example-service-mesh",
    spec={
        "listeners": [{
            "port_mapping": {
                "port": 8080,
                "protocol": "http",
            },
        }],
    },
    tags={
        "Environment": "test",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppMesh.VirtualGateway("example", new()
    {
        Name = "example-virtual-gateway",
        MeshName = "example-service-mesh",
        Spec = new Aws.AppMesh.Inputs.VirtualGatewaySpecArgs
        {
            Listeners = new[]
            {
                new Aws.AppMesh.Inputs.VirtualGatewaySpecListenerArgs
                {
                    PortMapping = new Aws.AppMesh.Inputs.VirtualGatewaySpecListenerPortMappingArgs
                    {
                        Port = 8080,
                        Protocol = "http",
                    },
                },
            },
        },
        Tags = 
        {
            { "Environment", "test" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualGateway(ctx, "example", &appmesh.VirtualGatewayArgs{
			Name:     pulumi.String("example-virtual-gateway"),
			MeshName: pulumi.String("example-service-mesh"),
			Spec: &appmesh.VirtualGatewaySpecArgs{
				Listeners: appmesh.VirtualGatewaySpecListenerArray{
					&appmesh.VirtualGatewaySpecListenerArgs{
						PortMapping: &appmesh.VirtualGatewaySpecListenerPortMappingArgs{
							Port:     pulumi.Int(8080),
							Protocol: pulumi.String("http"),
						},
					},
				},
			},
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("test"),
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
import com.pulumi.aws.appmesh.VirtualGateway;
import com.pulumi.aws.appmesh.VirtualGatewayArgs;
import com.pulumi.aws.appmesh.inputs.VirtualGatewaySpecArgs;
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
        var example = new VirtualGateway("example", VirtualGatewayArgs.builder()
            .name("example-virtual-gateway")
            .meshName("example-service-mesh")
            .spec(VirtualGatewaySpecArgs.builder()
                .listeners(VirtualGatewaySpecListenerArgs.builder()
                    .portMapping(VirtualGatewaySpecListenerPortMappingArgs.builder()
                        .port(8080)
                        .protocol("http")
                        .build())
                    .build())
                .build())
            .tags(Map.of("Environment", "test"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appmesh:VirtualGateway
    properties:
      name: example-virtual-gateway
      meshName: example-service-mesh
      spec:
        listeners:
          - portMapping:
              port: 8080
              protocol: http
      tags:
        Environment: test
```
<!--End PulumiCodeChooser -->

### Access Logs and TLS

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appmesh.VirtualGateway("example", {
    name: "example-virtual-gateway",
    meshName: "example-service-mesh",
    spec: {
        listeners: [{
            portMapping: {
                port: 8080,
                protocol: "http",
            },
            tls: {
                certificate: {
                    acm: {
                        certificateArn: exampleAwsAcmCertificate.arn,
                    },
                },
                mode: "STRICT",
            },
        }],
        logging: {
            accessLog: {
                file: {
                    path: "/var/log/access.log",
                },
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appmesh.VirtualGateway("example",
    name="example-virtual-gateway",
    mesh_name="example-service-mesh",
    spec={
        "listeners": [{
            "port_mapping": {
                "port": 8080,
                "protocol": "http",
            },
            "tls": {
                "certificate": {
                    "acm": {
                        "certificate_arn": example_aws_acm_certificate["arn"],
                    },
                },
                "mode": "STRICT",
            },
        }],
        "logging": {
            "access_log": {
                "file": {
                    "path": "/var/log/access.log",
                },
            },
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
    var example = new Aws.AppMesh.VirtualGateway("example", new()
    {
        Name = "example-virtual-gateway",
        MeshName = "example-service-mesh",
        Spec = new Aws.AppMesh.Inputs.VirtualGatewaySpecArgs
        {
            Listeners = new[]
            {
                new Aws.AppMesh.Inputs.VirtualGatewaySpecListenerArgs
                {
                    PortMapping = new Aws.AppMesh.Inputs.VirtualGatewaySpecListenerPortMappingArgs
                    {
                        Port = 8080,
                        Protocol = "http",
                    },
                    Tls = new Aws.AppMesh.Inputs.VirtualGatewaySpecListenerTlsArgs
                    {
                        Certificate = new Aws.AppMesh.Inputs.VirtualGatewaySpecListenerTlsCertificateArgs
                        {
                            Acm = new Aws.AppMesh.Inputs.VirtualGatewaySpecListenerTlsCertificateAcmArgs
                            {
                                CertificateArn = exampleAwsAcmCertificate.Arn,
                            },
                        },
                        Mode = "STRICT",
                    },
                },
            },
            Logging = new Aws.AppMesh.Inputs.VirtualGatewaySpecLoggingArgs
            {
                AccessLog = new Aws.AppMesh.Inputs.VirtualGatewaySpecLoggingAccessLogArgs
                {
                    File = new Aws.AppMesh.Inputs.VirtualGatewaySpecLoggingAccessLogFileArgs
                    {
                        Path = "/var/log/access.log",
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualGateway(ctx, "example", &appmesh.VirtualGatewayArgs{
			Name:     pulumi.String("example-virtual-gateway"),
			MeshName: pulumi.String("example-service-mesh"),
			Spec: &appmesh.VirtualGatewaySpecArgs{
				Listeners: appmesh.VirtualGatewaySpecListenerArray{
					&appmesh.VirtualGatewaySpecListenerArgs{
						PortMapping: &appmesh.VirtualGatewaySpecListenerPortMappingArgs{
							Port:     pulumi.Int(8080),
							Protocol: pulumi.String("http"),
						},
						Tls: &appmesh.VirtualGatewaySpecListenerTlsArgs{
							Certificate: &appmesh.VirtualGatewaySpecListenerTlsCertificateArgs{
								Acm: &appmesh.VirtualGatewaySpecListenerTlsCertificateAcmArgs{
									CertificateArn: pulumi.Any(exampleAwsAcmCertificate.Arn),
								},
							},
							Mode: pulumi.String("STRICT"),
						},
					},
				},
				Logging: &appmesh.VirtualGatewaySpecLoggingArgs{
					AccessLog: &appmesh.VirtualGatewaySpecLoggingAccessLogArgs{
						File: &appmesh.VirtualGatewaySpecLoggingAccessLogFileArgs{
							Path: pulumi.String("/var/log/access.log"),
						},
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
import com.pulumi.aws.appmesh.VirtualGateway;
import com.pulumi.aws.appmesh.VirtualGatewayArgs;
import com.pulumi.aws.appmesh.inputs.VirtualGatewaySpecArgs;
import com.pulumi.aws.appmesh.inputs.VirtualGatewaySpecLoggingArgs;
import com.pulumi.aws.appmesh.inputs.VirtualGatewaySpecLoggingAccessLogArgs;
import com.pulumi.aws.appmesh.inputs.VirtualGatewaySpecLoggingAccessLogFileArgs;
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
        var example = new VirtualGateway("example", VirtualGatewayArgs.builder()
            .name("example-virtual-gateway")
            .meshName("example-service-mesh")
            .spec(VirtualGatewaySpecArgs.builder()
                .listeners(VirtualGatewaySpecListenerArgs.builder()
                    .portMapping(VirtualGatewaySpecListenerPortMappingArgs.builder()
                        .port(8080)
                        .protocol("http")
                        .build())
                    .tls(VirtualGatewaySpecListenerTlsArgs.builder()
                        .certificate(VirtualGatewaySpecListenerTlsCertificateArgs.builder()
                            .acm(VirtualGatewaySpecListenerTlsCertificateAcmArgs.builder()
                                .certificateArn(exampleAwsAcmCertificate.arn())
                                .build())
                            .build())
                        .mode("STRICT")
                        .build())
                    .build())
                .logging(VirtualGatewaySpecLoggingArgs.builder()
                    .accessLog(VirtualGatewaySpecLoggingAccessLogArgs.builder()
                        .file(VirtualGatewaySpecLoggingAccessLogFileArgs.builder()
                            .path("/var/log/access.log")
                            .build())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appmesh:VirtualGateway
    properties:
      name: example-virtual-gateway
      meshName: example-service-mesh
      spec:
        listeners:
          - portMapping:
              port: 8080
              protocol: http
            tls:
              certificate:
                acm:
                  certificateArn: ${exampleAwsAcmCertificate.arn}
              mode: STRICT
        logging:
          accessLog:
            file:
              path: /var/log/access.log
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Mesh virtual gateway using `mesh_name` together with the virtual gateway's `name`. For example:

```sh
$ pulumi import aws:appmesh/virtualGateway:VirtualGateway example mesh/gw1
```
�
meshName" qName of the service mesh in which to create the virtual gateway. Must be between 1 and 255 characters in length.
�
	meshOwnerB" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
c
nameB" UName to use for the virtual gateway. Must be between 1 and 255 characters in length.
�
specT:R
P
appmeshVirtualGatewaySpec1aws:appmesh/VirtualGatewaySpec:VirtualGatewaySpec(Virtual gateway specification to apply.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"'
arn" ARN of the virtual gateway.
"9
createdDate" &Creation date of the virtual gateway.
"@
lastUpdatedDate" )Last update date of the virtual gateway.
"�
meshName" qName of the service mesh in which to create the virtual gateway. Must be between 1 and 255 characters in length.
"�
	meshOwner" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
"a
name" UName to use for the virtual gateway. Must be between 1 and 255 characters in length.
"6
resourceOwner" !Resource owner's AWS account ID.
"�
specT:R
P
appmeshVirtualGatewaySpec1aws:appmesh/VirtualGatewaySpec:VirtualGatewaySpec(Virtual gateway specification to apply.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
;
appmeshVirtualNode#aws:appmesh/virtualNode:VirtualNode��Provides an AWS App Mesh virtual node resource.

## Breaking Changes

Because of backward incompatible API changes (read [here](https://github.com/awslabs/aws-app-mesh-examples/issues/92)), `aws.appmesh.VirtualNode` resource definitions created with provider versions earlier than v2.3.0 will need to be modified:

* Rename the `service_name` attribute of the `dns` object to `hostname`.

* Replace the `backends` attribute of the `spec` object with one or more `backend` configuration blocks,
setting `virtual_service_name` to the name of the service.

The state associated with existing resources will automatically be migrated.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb1 = new aws.appmesh.VirtualNode("serviceb1", {
    name: "serviceBv1",
    meshName: simple.id,
    spec: {
        backends: [{
            virtualService: {
                virtualServiceName: "servicea.simpleapp.local",
            },
        }],
        listeners: [{
            portMapping: {
                port: 8080,
                protocol: "http",
            },
        }],
        serviceDiscovery: {
            dns: {
                hostname: "serviceb.simpleapp.local",
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb1 = aws.appmesh.VirtualNode("serviceb1",
    name="serviceBv1",
    mesh_name=simple["id"],
    spec={
        "backends": [{
            "virtual_service": {
                "virtual_service_name": "servicea.simpleapp.local",
            },
        }],
        "listeners": [{
            "port_mapping": {
                "port": 8080,
                "protocol": "http",
            },
        }],
        "service_discovery": {
            "dns": {
                "hostname": "serviceb.simpleapp.local",
            },
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
    var serviceb1 = new Aws.AppMesh.VirtualNode("serviceb1", new()
    {
        Name = "serviceBv1",
        MeshName = simple.Id,
        Spec = new Aws.AppMesh.Inputs.VirtualNodeSpecArgs
        {
            Backends = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecBackendArgs
                {
                    VirtualService = new Aws.AppMesh.Inputs.VirtualNodeSpecBackendVirtualServiceArgs
                    {
                        VirtualServiceName = "servicea.simpleapp.local",
                    },
                },
            },
            Listeners = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecListenerArgs
                {
                    PortMapping = new Aws.AppMesh.Inputs.VirtualNodeSpecListenerPortMappingArgs
                    {
                        Port = 8080,
                        Protocol = "http",
                    },
                },
            },
            ServiceDiscovery = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryArgs
            {
                Dns = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryDnsArgs
                {
                    Hostname = "serviceb.simpleapp.local",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualNode(ctx, "serviceb1", &appmesh.VirtualNodeArgs{
			Name:     pulumi.String("serviceBv1"),
			MeshName: pulumi.Any(simple.Id),
			Spec: &appmesh.VirtualNodeSpecArgs{
				Backends: appmesh.VirtualNodeSpecBackendArray{
					&appmesh.VirtualNodeSpecBackendArgs{
						VirtualService: &appmesh.VirtualNodeSpecBackendVirtualServiceArgs{
							VirtualServiceName: pulumi.String("servicea.simpleapp.local"),
						},
					},
				},
				Listeners: appmesh.VirtualNodeSpecListenerArray{
					&appmesh.VirtualNodeSpecListenerArgs{
						PortMapping: &appmesh.VirtualNodeSpecListenerPortMappingArgs{
							Port:     pulumi.Int(8080),
							Protocol: pulumi.String("http"),
						},
					},
				},
				ServiceDiscovery: &appmesh.VirtualNodeSpecServiceDiscoveryArgs{
					Dns: &appmesh.VirtualNodeSpecServiceDiscoveryDnsArgs{
						Hostname: pulumi.String("serviceb.simpleapp.local"),
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
import com.pulumi.aws.appmesh.VirtualNode;
import com.pulumi.aws.appmesh.VirtualNodeArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryDnsArgs;
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
        var serviceb1 = new VirtualNode("serviceb1", VirtualNodeArgs.builder()
            .name("serviceBv1")
            .meshName(simple.id())
            .spec(VirtualNodeSpecArgs.builder()
                .backends(VirtualNodeSpecBackendArgs.builder()
                    .virtualService(VirtualNodeSpecBackendVirtualServiceArgs.builder()
                        .virtualServiceName("servicea.simpleapp.local")
                        .build())
                    .build())
                .listeners(VirtualNodeSpecListenerArgs.builder()
                    .portMapping(VirtualNodeSpecListenerPortMappingArgs.builder()
                        .port(8080)
                        .protocol("http")
                        .build())
                    .build())
                .serviceDiscovery(VirtualNodeSpecServiceDiscoveryArgs.builder()
                    .dns(VirtualNodeSpecServiceDiscoveryDnsArgs.builder()
                        .hostname("serviceb.simpleapp.local")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb1:
    type: aws:appmesh:VirtualNode
    properties:
      name: serviceBv1
      meshName: ${simple.id}
      spec:
        backends:
          - virtualService:
              virtualServiceName: servicea.simpleapp.local
        listeners:
          - portMapping:
              port: 8080
              protocol: http
        serviceDiscovery:
          dns:
            hostname: serviceb.simpleapp.local
```
<!--End PulumiCodeChooser -->

### AWS Cloud Map Service Discovery

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicediscovery.HttpNamespace("example", {name: "example-ns"});
const serviceb1 = new aws.appmesh.VirtualNode("serviceb1", {
    name: "serviceBv1",
    meshName: simple.id,
    spec: {
        backends: [{
            virtualService: {
                virtualServiceName: "servicea.simpleapp.local",
            },
        }],
        listeners: [{
            portMapping: {
                port: 8080,
                protocol: "http",
            },
        }],
        serviceDiscovery: {
            awsCloudMap: {
                attributes: {
                    stack: "blue",
                },
                serviceName: "serviceb1",
                namespaceName: example.name,
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicediscovery.HttpNamespace("example", name="example-ns")
serviceb1 = aws.appmesh.VirtualNode("serviceb1",
    name="serviceBv1",
    mesh_name=simple["id"],
    spec={
        "backends": [{
            "virtual_service": {
                "virtual_service_name": "servicea.simpleapp.local",
            },
        }],
        "listeners": [{
            "port_mapping": {
                "port": 8080,
                "protocol": "http",
            },
        }],
        "service_discovery": {
            "aws_cloud_map": {
                "attributes": {
                    "stack": "blue",
                },
                "service_name": "serviceb1",
                "namespace_name": example.name,
            },
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
    var example = new Aws.ServiceDiscovery.HttpNamespace("example", new()
    {
        Name = "example-ns",
    });

    var serviceb1 = new Aws.AppMesh.VirtualNode("serviceb1", new()
    {
        Name = "serviceBv1",
        MeshName = simple.Id,
        Spec = new Aws.AppMesh.Inputs.VirtualNodeSpecArgs
        {
            Backends = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecBackendArgs
                {
                    VirtualService = new Aws.AppMesh.Inputs.VirtualNodeSpecBackendVirtualServiceArgs
                    {
                        VirtualServiceName = "servicea.simpleapp.local",
                    },
                },
            },
            Listeners = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecListenerArgs
                {
                    PortMapping = new Aws.AppMesh.Inputs.VirtualNodeSpecListenerPortMappingArgs
                    {
                        Port = 8080,
                        Protocol = "http",
                    },
                },
            },
            ServiceDiscovery = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryArgs
            {
                AwsCloudMap = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryAwsCloudMapArgs
                {
                    Attributes = 
                    {
                        { "stack", "blue" },
                    },
                    ServiceName = "serviceb1",
                    NamespaceName = example.Name,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := servicediscovery.NewHttpNamespace(ctx, "example", &servicediscovery.HttpNamespaceArgs{
			Name: pulumi.String("example-ns"),
		})
		if err != nil {
			return err
		}
		_, err = appmesh.NewVirtualNode(ctx, "serviceb1", &appmesh.VirtualNodeArgs{
			Name:     pulumi.String("serviceBv1"),
			MeshName: pulumi.Any(simple.Id),
			Spec: &appmesh.VirtualNodeSpecArgs{
				Backends: appmesh.VirtualNodeSpecBackendArray{
					&appmesh.VirtualNodeSpecBackendArgs{
						VirtualService: &appmesh.VirtualNodeSpecBackendVirtualServiceArgs{
							VirtualServiceName: pulumi.String("servicea.simpleapp.local"),
						},
					},
				},
				Listeners: appmesh.VirtualNodeSpecListenerArray{
					&appmesh.VirtualNodeSpecListenerArgs{
						PortMapping: &appmesh.VirtualNodeSpecListenerPortMappingArgs{
							Port:     pulumi.Int(8080),
							Protocol: pulumi.String("http"),
						},
					},
				},
				ServiceDiscovery: &appmesh.VirtualNodeSpecServiceDiscoveryArgs{
					AwsCloudMap: &appmesh.VirtualNodeSpecServiceDiscoveryAwsCloudMapArgs{
						Attributes: pulumi.StringMap{
							"stack": pulumi.String("blue"),
						},
						ServiceName:   pulumi.String("serviceb1"),
						NamespaceName: example.Name,
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
import com.pulumi.aws.servicediscovery.HttpNamespace;
import com.pulumi.aws.servicediscovery.HttpNamespaceArgs;
import com.pulumi.aws.appmesh.VirtualNode;
import com.pulumi.aws.appmesh.VirtualNodeArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryAwsCloudMapArgs;
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
        var example = new HttpNamespace("example", HttpNamespaceArgs.builder()
            .name("example-ns")
            .build());

        var serviceb1 = new VirtualNode("serviceb1", VirtualNodeArgs.builder()
            .name("serviceBv1")
            .meshName(simple.id())
            .spec(VirtualNodeSpecArgs.builder()
                .backends(VirtualNodeSpecBackendArgs.builder()
                    .virtualService(VirtualNodeSpecBackendVirtualServiceArgs.builder()
                        .virtualServiceName("servicea.simpleapp.local")
                        .build())
                    .build())
                .listeners(VirtualNodeSpecListenerArgs.builder()
                    .portMapping(VirtualNodeSpecListenerPortMappingArgs.builder()
                        .port(8080)
                        .protocol("http")
                        .build())
                    .build())
                .serviceDiscovery(VirtualNodeSpecServiceDiscoveryArgs.builder()
                    .awsCloudMap(VirtualNodeSpecServiceDiscoveryAwsCloudMapArgs.builder()
                        .attributes(Map.of("stack", "blue"))
                        .serviceName("serviceb1")
                        .namespaceName(example.name())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicediscovery:HttpNamespace
    properties:
      name: example-ns
  serviceb1:
    type: aws:appmesh:VirtualNode
    properties:
      name: serviceBv1
      meshName: ${simple.id}
      spec:
        backends:
          - virtualService:
              virtualServiceName: servicea.simpleapp.local
        listeners:
          - portMapping:
              port: 8080
              protocol: http
        serviceDiscovery:
          awsCloudMap:
            attributes:
              stack: blue
            serviceName: serviceb1
            namespaceName: ${example.name}
```
<!--End PulumiCodeChooser -->

### Listener Health Check

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb1 = new aws.appmesh.VirtualNode("serviceb1", {
    name: "serviceBv1",
    meshName: simple.id,
    spec: {
        backends: [{
            virtualService: {
                virtualServiceName: "servicea.simpleapp.local",
            },
        }],
        listeners: [{
            portMapping: {
                port: 8080,
                protocol: "http",
            },
            healthCheck: {
                protocol: "http",
                path: "/ping",
                healthyThreshold: 2,
                unhealthyThreshold: 2,
                timeoutMillis: 2000,
                intervalMillis: 5000,
            },
        }],
        serviceDiscovery: {
            dns: {
                hostname: "serviceb.simpleapp.local",
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb1 = aws.appmesh.VirtualNode("serviceb1",
    name="serviceBv1",
    mesh_name=simple["id"],
    spec={
        "backends": [{
            "virtual_service": {
                "virtual_service_name": "servicea.simpleapp.local",
            },
        }],
        "listeners": [{
            "port_mapping": {
                "port": 8080,
                "protocol": "http",
            },
            "health_check": {
                "protocol": "http",
                "path": "/ping",
                "healthy_threshold": 2,
                "unhealthy_threshold": 2,
                "timeout_millis": 2000,
                "interval_millis": 5000,
            },
        }],
        "service_discovery": {
            "dns": {
                "hostname": "serviceb.simpleapp.local",
            },
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
    var serviceb1 = new Aws.AppMesh.VirtualNode("serviceb1", new()
    {
        Name = "serviceBv1",
        MeshName = simple.Id,
        Spec = new Aws.AppMesh.Inputs.VirtualNodeSpecArgs
        {
            Backends = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecBackendArgs
                {
                    VirtualService = new Aws.AppMesh.Inputs.VirtualNodeSpecBackendVirtualServiceArgs
                    {
                        VirtualServiceName = "servicea.simpleapp.local",
                    },
                },
            },
            Listeners = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecListenerArgs
                {
                    PortMapping = new Aws.AppMesh.Inputs.VirtualNodeSpecListenerPortMappingArgs
                    {
                        Port = 8080,
                        Protocol = "http",
                    },
                    HealthCheck = new Aws.AppMesh.Inputs.VirtualNodeSpecListenerHealthCheckArgs
                    {
                        Protocol = "http",
                        Path = "/ping",
                        HealthyThreshold = 2,
                        UnhealthyThreshold = 2,
                        TimeoutMillis = 2000,
                        IntervalMillis = 5000,
                    },
                },
            },
            ServiceDiscovery = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryArgs
            {
                Dns = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryDnsArgs
                {
                    Hostname = "serviceb.simpleapp.local",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualNode(ctx, "serviceb1", &appmesh.VirtualNodeArgs{
			Name:     pulumi.String("serviceBv1"),
			MeshName: pulumi.Any(simple.Id),
			Spec: &appmesh.VirtualNodeSpecArgs{
				Backends: appmesh.VirtualNodeSpecBackendArray{
					&appmesh.VirtualNodeSpecBackendArgs{
						VirtualService: &appmesh.VirtualNodeSpecBackendVirtualServiceArgs{
							VirtualServiceName: pulumi.String("servicea.simpleapp.local"),
						},
					},
				},
				Listeners: appmesh.VirtualNodeSpecListenerArray{
					&appmesh.VirtualNodeSpecListenerArgs{
						PortMapping: &appmesh.VirtualNodeSpecListenerPortMappingArgs{
							Port:     pulumi.Int(8080),
							Protocol: pulumi.String("http"),
						},
						HealthCheck: &appmesh.VirtualNodeSpecListenerHealthCheckArgs{
							Protocol:           pulumi.String("http"),
							Path:               pulumi.String("/ping"),
							HealthyThreshold:   pulumi.Int(2),
							UnhealthyThreshold: pulumi.Int(2),
							TimeoutMillis:      pulumi.Int(2000),
							IntervalMillis:     pulumi.Int(5000),
						},
					},
				},
				ServiceDiscovery: &appmesh.VirtualNodeSpecServiceDiscoveryArgs{
					Dns: &appmesh.VirtualNodeSpecServiceDiscoveryDnsArgs{
						Hostname: pulumi.String("serviceb.simpleapp.local"),
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
import com.pulumi.aws.appmesh.VirtualNode;
import com.pulumi.aws.appmesh.VirtualNodeArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryDnsArgs;
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
        var serviceb1 = new VirtualNode("serviceb1", VirtualNodeArgs.builder()
            .name("serviceBv1")
            .meshName(simple.id())
            .spec(VirtualNodeSpecArgs.builder()
                .backends(VirtualNodeSpecBackendArgs.builder()
                    .virtualService(VirtualNodeSpecBackendVirtualServiceArgs.builder()
                        .virtualServiceName("servicea.simpleapp.local")
                        .build())
                    .build())
                .listeners(VirtualNodeSpecListenerArgs.builder()
                    .portMapping(VirtualNodeSpecListenerPortMappingArgs.builder()
                        .port(8080)
                        .protocol("http")
                        .build())
                    .healthCheck(VirtualNodeSpecListenerHealthCheckArgs.builder()
                        .protocol("http")
                        .path("/ping")
                        .healthyThreshold(2)
                        .unhealthyThreshold(2)
                        .timeoutMillis(2000)
                        .intervalMillis(5000)
                        .build())
                    .build())
                .serviceDiscovery(VirtualNodeSpecServiceDiscoveryArgs.builder()
                    .dns(VirtualNodeSpecServiceDiscoveryDnsArgs.builder()
                        .hostname("serviceb.simpleapp.local")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb1:
    type: aws:appmesh:VirtualNode
    properties:
      name: serviceBv1
      meshName: ${simple.id}
      spec:
        backends:
          - virtualService:
              virtualServiceName: servicea.simpleapp.local
        listeners:
          - portMapping:
              port: 8080
              protocol: http
            healthCheck:
              protocol: http
              path: /ping
              healthyThreshold: 2
              unhealthyThreshold: 2
              timeoutMillis: 2000
              intervalMillis: 5000
        serviceDiscovery:
          dns:
            hostname: serviceb.simpleapp.local
```
<!--End PulumiCodeChooser -->

### Logging

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb1 = new aws.appmesh.VirtualNode("serviceb1", {
    name: "serviceBv1",
    meshName: simple.id,
    spec: {
        backends: [{
            virtualService: {
                virtualServiceName: "servicea.simpleapp.local",
            },
        }],
        listeners: [{
            portMapping: {
                port: 8080,
                protocol: "http",
            },
        }],
        serviceDiscovery: {
            dns: {
                hostname: "serviceb.simpleapp.local",
            },
        },
        logging: {
            accessLog: {
                file: {
                    path: "/dev/stdout",
                },
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb1 = aws.appmesh.VirtualNode("serviceb1",
    name="serviceBv1",
    mesh_name=simple["id"],
    spec={
        "backends": [{
            "virtual_service": {
                "virtual_service_name": "servicea.simpleapp.local",
            },
        }],
        "listeners": [{
            "port_mapping": {
                "port": 8080,
                "protocol": "http",
            },
        }],
        "service_discovery": {
            "dns": {
                "hostname": "serviceb.simpleapp.local",
            },
        },
        "logging": {
            "access_log": {
                "file": {
                    "path": "/dev/stdout",
                },
            },
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
    var serviceb1 = new Aws.AppMesh.VirtualNode("serviceb1", new()
    {
        Name = "serviceBv1",
        MeshName = simple.Id,
        Spec = new Aws.AppMesh.Inputs.VirtualNodeSpecArgs
        {
            Backends = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecBackendArgs
                {
                    VirtualService = new Aws.AppMesh.Inputs.VirtualNodeSpecBackendVirtualServiceArgs
                    {
                        VirtualServiceName = "servicea.simpleapp.local",
                    },
                },
            },
            Listeners = new[]
            {
                new Aws.AppMesh.Inputs.VirtualNodeSpecListenerArgs
                {
                    PortMapping = new Aws.AppMesh.Inputs.VirtualNodeSpecListenerPortMappingArgs
                    {
                        Port = 8080,
                        Protocol = "http",
                    },
                },
            },
            ServiceDiscovery = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryArgs
            {
                Dns = new Aws.AppMesh.Inputs.VirtualNodeSpecServiceDiscoveryDnsArgs
                {
                    Hostname = "serviceb.simpleapp.local",
                },
            },
            Logging = new Aws.AppMesh.Inputs.VirtualNodeSpecLoggingArgs
            {
                AccessLog = new Aws.AppMesh.Inputs.VirtualNodeSpecLoggingAccessLogArgs
                {
                    File = new Aws.AppMesh.Inputs.VirtualNodeSpecLoggingAccessLogFileArgs
                    {
                        Path = "/dev/stdout",
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualNode(ctx, "serviceb1", &appmesh.VirtualNodeArgs{
			Name:     pulumi.String("serviceBv1"),
			MeshName: pulumi.Any(simple.Id),
			Spec: &appmesh.VirtualNodeSpecArgs{
				Backends: appmesh.VirtualNodeSpecBackendArray{
					&appmesh.VirtualNodeSpecBackendArgs{
						VirtualService: &appmesh.VirtualNodeSpecBackendVirtualServiceArgs{
							VirtualServiceName: pulumi.String("servicea.simpleapp.local"),
						},
					},
				},
				Listeners: appmesh.VirtualNodeSpecListenerArray{
					&appmesh.VirtualNodeSpecListenerArgs{
						PortMapping: &appmesh.VirtualNodeSpecListenerPortMappingArgs{
							Port:     pulumi.Int(8080),
							Protocol: pulumi.String("http"),
						},
					},
				},
				ServiceDiscovery: &appmesh.VirtualNodeSpecServiceDiscoveryArgs{
					Dns: &appmesh.VirtualNodeSpecServiceDiscoveryDnsArgs{
						Hostname: pulumi.String("serviceb.simpleapp.local"),
					},
				},
				Logging: &appmesh.VirtualNodeSpecLoggingArgs{
					AccessLog: &appmesh.VirtualNodeSpecLoggingAccessLogArgs{
						File: &appmesh.VirtualNodeSpecLoggingAccessLogFileArgs{
							Path: pulumi.String("/dev/stdout"),
						},
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
import com.pulumi.aws.appmesh.VirtualNode;
import com.pulumi.aws.appmesh.VirtualNodeArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecServiceDiscoveryDnsArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecLoggingArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecLoggingAccessLogArgs;
import com.pulumi.aws.appmesh.inputs.VirtualNodeSpecLoggingAccessLogFileArgs;
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
        var serviceb1 = new VirtualNode("serviceb1", VirtualNodeArgs.builder()
            .name("serviceBv1")
            .meshName(simple.id())
            .spec(VirtualNodeSpecArgs.builder()
                .backends(VirtualNodeSpecBackendArgs.builder()
                    .virtualService(VirtualNodeSpecBackendVirtualServiceArgs.builder()
                        .virtualServiceName("servicea.simpleapp.local")
                        .build())
                    .build())
                .listeners(VirtualNodeSpecListenerArgs.builder()
                    .portMapping(VirtualNodeSpecListenerPortMappingArgs.builder()
                        .port(8080)
                        .protocol("http")
                        .build())
                    .build())
                .serviceDiscovery(VirtualNodeSpecServiceDiscoveryArgs.builder()
                    .dns(VirtualNodeSpecServiceDiscoveryDnsArgs.builder()
                        .hostname("serviceb.simpleapp.local")
                        .build())
                    .build())
                .logging(VirtualNodeSpecLoggingArgs.builder()
                    .accessLog(VirtualNodeSpecLoggingAccessLogArgs.builder()
                        .file(VirtualNodeSpecLoggingAccessLogFileArgs.builder()
                            .path("/dev/stdout")
                            .build())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb1:
    type: aws:appmesh:VirtualNode
    properties:
      name: serviceBv1
      meshName: ${simple.id}
      spec:
        backends:
          - virtualService:
              virtualServiceName: servicea.simpleapp.local
        listeners:
          - portMapping:
              port: 8080
              protocol: http
        serviceDiscovery:
          dns:
            hostname: serviceb.simpleapp.local
        logging:
          accessLog:
            file:
              path: /dev/stdout
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Mesh virtual nodes using `mesh_name` together with the virtual node's `name`. For example:

```sh
$ pulumi import aws:appmesh/virtualNode:VirtualNode serviceb1 simpleapp/serviceBv1
```
~
meshName" nName of the service mesh in which to create the virtual node. Must be between 1 and 255 characters in length.
�
	meshOwnerB" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
`
nameB" RName to use for the virtual node. Must be between 1 and 255 characters in length.
z
specK:I
G
appmeshVirtualNodeSpec+aws:appmesh/VirtualNodeSpec:VirtualNodeSpec%Virtual node specification to apply.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"$
arn" ARN of the virtual node.
"6
createdDate" #Creation date of the virtual node.
"=
lastUpdatedDate" &Last update date of the virtual node.
"~
meshName" nName of the service mesh in which to create the virtual node. Must be between 1 and 255 characters in length.
"�
	meshOwner" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
"^
name" RName to use for the virtual node. Must be between 1 and 255 characters in length.
"6
resourceOwner" !Resource owner's AWS account ID.
"z
specK:I
G
appmeshVirtualNodeSpec+aws:appmesh/VirtualNodeSpec:VirtualNodeSpec%Virtual node specification to apply.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�2
A
appmeshVirtualRouter'aws:appmesh/virtualRouter:VirtualRouter�#Provides an AWS App Mesh virtual router resource.

## Breaking Changes

Because of backward incompatible API changes (read [here](https://github.com/awslabs/aws-app-mesh-examples/issues/92) and [here](https://github.com/awslabs/aws-app-mesh-examples/issues/94)), `aws.appmesh.VirtualRouter` resource definitions created with provider versions earlier than v2.3.0 will need to be modified:

* Remove service `service_names` from the `spec` argument. AWS has created a `aws.appmesh.VirtualService` resource for each service name. Import these resource using `pulumi import`.

* Add a `listener` configuration block to the `spec` argument.

The state associated with existing resources will automatically be migrated.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const serviceb = new aws.appmesh.VirtualRouter("serviceb", {
    name: "serviceB",
    meshName: simple.id,
    spec: {
        listeners: [{
            portMapping: {
                port: 8080,
                protocol: "http",
            },
        }],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

serviceb = aws.appmesh.VirtualRouter("serviceb",
    name="serviceB",
    mesh_name=simple["id"],
    spec={
        "listeners": [{
            "port_mapping": {
                "port": 8080,
                "protocol": "http",
            },
        }],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var serviceb = new Aws.AppMesh.VirtualRouter("serviceb", new()
    {
        Name = "serviceB",
        MeshName = simple.Id,
        Spec = new Aws.AppMesh.Inputs.VirtualRouterSpecArgs
        {
            Listeners = new[]
            {
                new Aws.AppMesh.Inputs.VirtualRouterSpecListenerArgs
                {
                    PortMapping = new Aws.AppMesh.Inputs.VirtualRouterSpecListenerPortMappingArgs
                    {
                        Port = 8080,
                        Protocol = "http",
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualRouter(ctx, "serviceb", &appmesh.VirtualRouterArgs{
			Name:     pulumi.String("serviceB"),
			MeshName: pulumi.Any(simple.Id),
			Spec: &appmesh.VirtualRouterSpecArgs{
				Listeners: appmesh.VirtualRouterSpecListenerArray{
					&appmesh.VirtualRouterSpecListenerArgs{
						PortMapping: &appmesh.VirtualRouterSpecListenerPortMappingArgs{
							Port:     pulumi.Int(8080),
							Protocol: pulumi.String("http"),
						},
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
import com.pulumi.aws.appmesh.VirtualRouter;
import com.pulumi.aws.appmesh.VirtualRouterArgs;
import com.pulumi.aws.appmesh.inputs.VirtualRouterSpecArgs;
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
        var serviceb = new VirtualRouter("serviceb", VirtualRouterArgs.builder()
            .name("serviceB")
            .meshName(simple.id())
            .spec(VirtualRouterSpecArgs.builder()
                .listeners(VirtualRouterSpecListenerArgs.builder()
                    .portMapping(VirtualRouterSpecListenerPortMappingArgs.builder()
                        .port(8080)
                        .protocol("http")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  serviceb:
    type: aws:appmesh:VirtualRouter
    properties:
      name: serviceB
      meshName: ${simple.id}
      spec:
        listeners:
          - portMapping:
              port: 8080
              protocol: http
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Mesh virtual routers using `mesh_name` together with the virtual router's `name`. For example:

```sh
$ pulumi import aws:appmesh/virtualRouter:VirtualRouter serviceb simpleapp/serviceB
```
�
meshName" pName of the service mesh in which to create the virtual router. Must be between 1 and 255 characters in length.
�
	meshOwnerB" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
b
nameB" TName to use for the virtual router. Must be between 1 and 255 characters in length.
�
specQ:O
M
appmeshVirtualRouterSpec/aws:appmesh/VirtualRouterSpec:VirtualRouterSpec'Virtual router specification to apply.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"&
arn" ARN of the virtual router.
"8
createdDate" %Creation date of the virtual router.
"?
lastUpdatedDate" (Last update date of the virtual router.
"�
meshName" pName of the service mesh in which to create the virtual router. Must be between 1 and 255 characters in length.
"�
	meshOwner" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
"`
name" TName to use for the virtual router. Must be between 1 and 255 characters in length.
"6
resourceOwner" !Resource owner's AWS account ID.
"�
specQ:O
M
appmeshVirtualRouterSpec/aws:appmesh/VirtualRouterSpec:VirtualRouterSpec'Virtual router specification to apply.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�J
D
appmeshVirtualService)aws:appmesh/virtualService:VirtualService�;Provides an AWS App Mesh virtual service resource.

## Example Usage

### Virtual Node Provider

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const servicea = new aws.appmesh.VirtualService("servicea", {
    name: "servicea.simpleapp.local",
    meshName: simple.id,
    spec: {
        provider: {
            virtualNode: {
                virtualNodeName: serviceb1.name,
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

servicea = aws.appmesh.VirtualService("servicea",
    name="servicea.simpleapp.local",
    mesh_name=simple["id"],
    spec={
        "provider": {
            "virtual_node": {
                "virtual_node_name": serviceb1["name"],
            },
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
    var servicea = new Aws.AppMesh.VirtualService("servicea", new()
    {
        Name = "servicea.simpleapp.local",
        MeshName = simple.Id,
        Spec = new Aws.AppMesh.Inputs.VirtualServiceSpecArgs
        {
            Provider = new Aws.AppMesh.Inputs.VirtualServiceSpecProviderArgs
            {
                VirtualNode = new Aws.AppMesh.Inputs.VirtualServiceSpecProviderVirtualNodeArgs
                {
                    VirtualNodeName = serviceb1.Name,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualService(ctx, "servicea", &appmesh.VirtualServiceArgs{
			Name:     pulumi.String("servicea.simpleapp.local"),
			MeshName: pulumi.Any(simple.Id),
			Spec: &appmesh.VirtualServiceSpecArgs{
				Provider: &appmesh.VirtualServiceSpecProviderArgs{
					VirtualNode: &appmesh.VirtualServiceSpecProviderVirtualNodeArgs{
						VirtualNodeName: pulumi.Any(serviceb1.Name),
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
import com.pulumi.aws.appmesh.VirtualService;
import com.pulumi.aws.appmesh.VirtualServiceArgs;
import com.pulumi.aws.appmesh.inputs.VirtualServiceSpecArgs;
import com.pulumi.aws.appmesh.inputs.VirtualServiceSpecProviderArgs;
import com.pulumi.aws.appmesh.inputs.VirtualServiceSpecProviderVirtualNodeArgs;
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
        var servicea = new VirtualService("servicea", VirtualServiceArgs.builder()
            .name("servicea.simpleapp.local")
            .meshName(simple.id())
            .spec(VirtualServiceSpecArgs.builder()
                .provider(VirtualServiceSpecProviderArgs.builder()
                    .virtualNode(VirtualServiceSpecProviderVirtualNodeArgs.builder()
                        .virtualNodeName(serviceb1.name())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  servicea:
    type: aws:appmesh:VirtualService
    properties:
      name: servicea.simpleapp.local
      meshName: ${simple.id}
      spec:
        provider:
          virtualNode:
            virtualNodeName: ${serviceb1.name}
```
<!--End PulumiCodeChooser -->

### Virtual Router Provider

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const servicea = new aws.appmesh.VirtualService("servicea", {
    name: "servicea.simpleapp.local",
    meshName: simple.id,
    spec: {
        provider: {
            virtualRouter: {
                virtualRouterName: serviceb.name,
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

servicea = aws.appmesh.VirtualService("servicea",
    name="servicea.simpleapp.local",
    mesh_name=simple["id"],
    spec={
        "provider": {
            "virtual_router": {
                "virtual_router_name": serviceb["name"],
            },
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
    var servicea = new Aws.AppMesh.VirtualService("servicea", new()
    {
        Name = "servicea.simpleapp.local",
        MeshName = simple.Id,
        Spec = new Aws.AppMesh.Inputs.VirtualServiceSpecArgs
        {
            Provider = new Aws.AppMesh.Inputs.VirtualServiceSpecProviderArgs
            {
                VirtualRouter = new Aws.AppMesh.Inputs.VirtualServiceSpecProviderVirtualRouterArgs
                {
                    VirtualRouterName = serviceb.Name,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.NewVirtualService(ctx, "servicea", &appmesh.VirtualServiceArgs{
			Name:     pulumi.String("servicea.simpleapp.local"),
			MeshName: pulumi.Any(simple.Id),
			Spec: &appmesh.VirtualServiceSpecArgs{
				Provider: &appmesh.VirtualServiceSpecProviderArgs{
					VirtualRouter: &appmesh.VirtualServiceSpecProviderVirtualRouterArgs{
						VirtualRouterName: pulumi.Any(serviceb.Name),
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
import com.pulumi.aws.appmesh.VirtualService;
import com.pulumi.aws.appmesh.VirtualServiceArgs;
import com.pulumi.aws.appmesh.inputs.VirtualServiceSpecArgs;
import com.pulumi.aws.appmesh.inputs.VirtualServiceSpecProviderArgs;
import com.pulumi.aws.appmesh.inputs.VirtualServiceSpecProviderVirtualRouterArgs;
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
        var servicea = new VirtualService("servicea", VirtualServiceArgs.builder()
            .name("servicea.simpleapp.local")
            .meshName(simple.id())
            .spec(VirtualServiceSpecArgs.builder()
                .provider(VirtualServiceSpecProviderArgs.builder()
                    .virtualRouter(VirtualServiceSpecProviderVirtualRouterArgs.builder()
                        .virtualRouterName(serviceb.name())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  servicea:
    type: aws:appmesh:VirtualService
    properties:
      name: servicea.simpleapp.local
      meshName: ${simple.id}
      spec:
        provider:
          virtualRouter:
            virtualRouterName: ${serviceb.name}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Mesh virtual services using `mesh_name` together with the virtual service's `name`. For example:

```sh
$ pulumi import aws:appmesh/virtualService:VirtualService servicea simpleapp/servicea.simpleapp.local
```
�
meshName" qName of the service mesh in which to create the virtual service. Must be between 1 and 255 characters in length.
�
	meshOwnerB" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
c
nameB" UName to use for the virtual service. Must be between 1 and 255 characters in length.
�
specT:R
P
appmeshVirtualServiceSpec1aws:appmesh/VirtualServiceSpec:VirtualServiceSpec(Virtual service specification to apply.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"'
arn" ARN of the virtual service.
"9
createdDate" &Creation date of the virtual service.
"@
lastUpdatedDate" )Last update date of the virtual service.
"�
meshName" qName of the service mesh in which to create the virtual service. Must be between 1 and 255 characters in length.
"�
	meshOwner" sAWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
"a
name" UName to use for the virtual service. Must be between 1 and 255 characters in length.
"6
resourceOwner" !Resource owner's AWS account ID.
"�
specT:R
P
appmeshVirtualServiceSpec1aws:appmesh/VirtualServiceSpec:VirtualServiceSpec(Virtual service specification to apply.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�+
{
	apprunnerAutoScalingConfigurationVersionMaws:apprunner/autoScalingConfigurationVersion:AutoScalingConfigurationVersion�Manages an App Runner AutoScaling Configuration Version.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.AutoScalingConfigurationVersion("example", {
    autoScalingConfigurationName: "example",
    maxConcurrency: 50,
    maxSize: 10,
    minSize: 2,
    tags: {
        Name: "example-apprunner-autoscaling",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.AutoScalingConfigurationVersion("example",
    auto_scaling_configuration_name="example",
    max_concurrency=50,
    max_size=10,
    min_size=2,
    tags={
        "Name": "example-apprunner-autoscaling",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.AutoScalingConfigurationVersion("example", new()
    {
        AutoScalingConfigurationName = "example",
        MaxConcurrency = 50,
        MaxSize = 10,
        MinSize = 2,
        Tags = 
        {
            { "Name", "example-apprunner-autoscaling" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewAutoScalingConfigurationVersion(ctx, "example", &apprunner.AutoScalingConfigurationVersionArgs{
			AutoScalingConfigurationName: pulumi.String("example"),
			MaxConcurrency:               pulumi.Int(50),
			MaxSize:                      pulumi.Int(10),
			MinSize:                      pulumi.Int(2),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example-apprunner-autoscaling"),
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
import com.pulumi.aws.apprunner.AutoScalingConfigurationVersion;
import com.pulumi.aws.apprunner.AutoScalingConfigurationVersionArgs;
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
        var example = new AutoScalingConfigurationVersion("example", AutoScalingConfigurationVersionArgs.builder()
            .autoScalingConfigurationName("example")
            .maxConcurrency(50)
            .maxSize(10)
            .minSize(2)
            .tags(Map.of("Name", "example-apprunner-autoscaling"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:AutoScalingConfigurationVersion
    properties:
      autoScalingConfigurationName: example
      maxConcurrency: 50
      maxSize: 10
      minSize: 2
      tags:
        Name: example-apprunner-autoscaling
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner AutoScaling Configuration Versions using the `arn`. For example:

```sh
$ pulumi import aws:apprunner/autoScalingConfigurationVersion:AutoScalingConfigurationVersion example "arn:aws:apprunner:us-east-1:1234567890:autoscalingconfiguration/example/1/69bdfe0115224b0db49398b7beb68e0f
```
L
autoScalingConfigurationName" (Name of the auto scaling configuration.
�
maxConcurrencyB �Maximal number of concurrent requests that you want an instance to process. When the number of concurrent requests goes over this limit, App Runner scales up your service.
Z
maxSizeB IMaximal number of instances that App Runner provisions for your service.
Z
minSizeB IMinimal number of instances that App Runner provisions for your service.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
";
arn" 0ARN of this auto scaling configuration version.
"L
autoScalingConfigurationName" (Name of the auto scaling configuration.
"Y
 autoScalingConfigurationRevision 1The revision of this auto scaling configuration.
"
hasAssociatedService
 "
	isDefault
 "�
latest
 �Whether the auto scaling configuration has the highest `auto_scaling_configuration_revision` among all configurations that share the same `auto_scaling_configuration_name`.
"�
maxConcurrencyB �Maximal number of concurrent requests that you want an instance to process. When the number of concurrent requests goes over this limit, App Runner scales up your service.
"Z
maxSizeB IMaximal number of instances that App Runner provisions for your service.
"Z
minSizeB IMinimal number of instances that App Runner provisions for your service.
"�
status" �Current state of the auto scaling configuration. An INACTIVE configuration revision has been deleted and can't be used. It is permanently removed some time after deletion.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�
<
	apprunner
Connection#aws:apprunner/connection:Connection�Manages an App Runner Connection.

> **NOTE:** After creation, you must complete the authentication handshake using the App Runner console.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.Connection("example", {
    connectionName: "example",
    providerType: "GITHUB",
    tags: {
        Name: "example-apprunner-connection",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.Connection("example",
    connection_name="example",
    provider_type="GITHUB",
    tags={
        "Name": "example-apprunner-connection",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.Connection("example", new()
    {
        ConnectionName = "example",
        ProviderType = "GITHUB",
        Tags = 
        {
            { "Name", "example-apprunner-connection" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewConnection(ctx, "example", &apprunner.ConnectionArgs{
			ConnectionName: pulumi.String("example"),
			ProviderType:   pulumi.String("GITHUB"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example-apprunner-connection"),
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
import com.pulumi.aws.apprunner.Connection;
import com.pulumi.aws.apprunner.ConnectionArgs;
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
            .connectionName("example")
            .providerType("GITHUB")
            .tags(Map.of("Name", "example-apprunner-connection"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:Connection
    properties:
      connectionName: example
      providerType: GITHUB
      tags:
        Name: example-apprunner-connection
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner Connections using the `connection_name`. For example:

```sh
$ pulumi import aws:apprunner/connection:Connection example example
```
.
connectionName" Name of the connection.
H
providerType" 4Source repository provider. Valid values: `GITHUB`.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
""
arn" ARN of the connection.
".
connectionName" Name of the connection.
"H
providerType" 4Source repository provider. Valid values: `GITHUB`.
"�
status" �Current state of the App Runner connection. When the state is `AVAILABLE`, you can use the connection to create an `aws.apprunner.Service` resource.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�$
c
	apprunnerCustomDomainAssociation=aws:apprunner/customDomainAssociation:CustomDomainAssociation�Manages an App Runner Custom Domain association.

> **NOTE:** After creation, you must use the information in the `certification_validation_records` attribute to add CNAME records to your Domain Name System (DNS). For each mapped domain name, add a mapping to the target App Runner subdomain (found in the `dns_target` attribute) and one or more certificate validation records. App Runner then performs DNS validation to verify that you own or control the domain name you associated. App Runner tracks domain validity in a certificate stored in AWS Certificate Manager (ACM).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.CustomDomainAssociation("example", {
    domainName: "example.com",
    serviceArn: exampleAwsApprunnerService.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.CustomDomainAssociation("example",
    domain_name="example.com",
    service_arn=example_aws_apprunner_service["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.CustomDomainAssociation("example", new()
    {
        DomainName = "example.com",
        ServiceArn = exampleAwsApprunnerService.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewCustomDomainAssociation(ctx, "example", &apprunner.CustomDomainAssociationArgs{
			DomainName: pulumi.String("example.com"),
			ServiceArn: pulumi.Any(exampleAwsApprunnerService.Arn),
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
import com.pulumi.aws.apprunner.CustomDomainAssociation;
import com.pulumi.aws.apprunner.CustomDomainAssociationArgs;
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
        var example = new CustomDomainAssociation("example", CustomDomainAssociationArgs.builder()
            .domainName("example.com")
            .serviceArn(exampleAwsApprunnerService.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:CustomDomainAssociation
    properties:
      domainName: example.com
      serviceArn: ${exampleAwsApprunnerService.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner Custom Domain Associations using the `domain_name` and `service_arn` separated by a comma (`,`). For example:

```sh
$ pulumi import aws:apprunner/customDomainAssociation:CustomDomainAssociation example example.com,arn:aws:apprunner:us-east-1:123456789012:service/example-app/8fe1e10304f84fd2b0df550fe98a71fa
```
�

domainName" Custom domain endpoint to association. Specify a base domain e.g., `example.com` or a subdomain e.g., `subdomain.example.com`.
�
enableWwwSubdomainB
 sWhether to associate the subdomain with the App Runner service in addition to the base domain. Defaults to `true`.
1

serviceArn" ARN of the App Runner service.
"�
certificateValidationRecords�*�:�
�
	apprunner2CustomDomainAssociationCertificateValidationRecordsaws:apprunner/CustomDomainAssociationCertificateValidationRecord:CustomDomainAssociationCertificateValidationRecordyA set of certificate CNAME records used for this domain name. See Certificate Validation Records below for more details.
"�
	dnsTarget" �App Runner subdomain of the App Runner service. The custom domain name is mapped to this target name. Attribute only available if resource created (not imported) with this provider.
"�

domainName" Custom domain endpoint to association. Specify a base domain e.g., `example.com` or a subdomain e.g., `subdomain.example.com`.
"�
enableWwwSubdomainB
 sWhether to associate the subdomain with the App Runner service in addition to the base domain. Defaults to `true`.
"1

serviceArn" ARN of the App Runner service.
"�
status" �Current state of the certificate CNAME record validation. It should change to `SUCCESS` after App Runner completes validation with your DNS.
*�(
�
	apprunner&DefaultAutoScalingConfigurationVersion[aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion�%Manages the default App Runner auto scaling configuration.
When creating or updating this resource the existing default auto scaling configuration will be set to non-default automatically.
When creating or updating this resource the configuration is automatically assigned as the default to the new services you create in the future. The new default designation doesn't affect the associations that were previously set for existing services.
Each account can have only one default auto scaling configuration per Region.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.AutoScalingConfigurationVersion("example", {
    autoScalingConfigurationName: "example",
    maxConcurrency: 50,
    maxSize: 10,
    minSize: 2,
});
const exampleDefaultAutoScalingConfigurationVersion = new aws.apprunner.DefaultAutoScalingConfigurationVersion("example", {autoScalingConfigurationArn: example.arn});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.AutoScalingConfigurationVersion("example",
    auto_scaling_configuration_name="example",
    max_concurrency=50,
    max_size=10,
    min_size=2)
example_default_auto_scaling_configuration_version = aws.apprunner.DefaultAutoScalingConfigurationVersion("example", auto_scaling_configuration_arn=example.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.AutoScalingConfigurationVersion("example", new()
    {
        AutoScalingConfigurationName = "example",
        MaxConcurrency = 50,
        MaxSize = 10,
        MinSize = 2,
    });

    var exampleDefaultAutoScalingConfigurationVersion = new Aws.AppRunner.DefaultAutoScalingConfigurationVersion("example", new()
    {
        AutoScalingConfigurationArn = example.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := apprunner.NewAutoScalingConfigurationVersion(ctx, "example", &apprunner.AutoScalingConfigurationVersionArgs{
			AutoScalingConfigurationName: pulumi.String("example"),
			MaxConcurrency:               pulumi.Int(50),
			MaxSize:                      pulumi.Int(10),
			MinSize:                      pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		_, err = apprunner.NewDefaultAutoScalingConfigurationVersion(ctx, "example", &apprunner.DefaultAutoScalingConfigurationVersionArgs{
			AutoScalingConfigurationArn: example.Arn,
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
import com.pulumi.aws.apprunner.AutoScalingConfigurationVersion;
import com.pulumi.aws.apprunner.AutoScalingConfigurationVersionArgs;
import com.pulumi.aws.apprunner.DefaultAutoScalingConfigurationVersion;
import com.pulumi.aws.apprunner.DefaultAutoScalingConfigurationVersionArgs;
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
        var example = new AutoScalingConfigurationVersion("example", AutoScalingConfigurationVersionArgs.builder()
            .autoScalingConfigurationName("example")
            .maxConcurrency(50)
            .maxSize(10)
            .minSize(2)
            .build());

        var exampleDefaultAutoScalingConfigurationVersion = new DefaultAutoScalingConfigurationVersion("exampleDefaultAutoScalingConfigurationVersion", DefaultAutoScalingConfigurationVersionArgs.builder()
            .autoScalingConfigurationArn(example.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:AutoScalingConfigurationVersion
    properties:
      autoScalingConfigurationName: example
      maxConcurrency: 50
      maxSize: 10
      minSize: 2
  exampleDefaultAutoScalingConfigurationVersion:
    type: aws:apprunner:DefaultAutoScalingConfigurationVersion
    name: example
    properties:
      autoScalingConfigurationArn: ${example.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner default auto scaling configurations using the current Region. For example:

```sh
$ pulumi import aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion example us-west-2
```
}
autoScalingConfigurationArn" ZThe ARN of the App Runner auto scaling configuration that you want to set as the default.
"}
autoScalingConfigurationArn" ZThe ARN of the App Runner auto scaling configuration that you want to set as the default.
*�
<
	apprunner
Deployment#aws:apprunner/deployment:Deployment�Manages an App Runner Deployment Operation.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.Deployment("example", {serviceArn: exampleAwsApprunnerService.arn});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.Deployment("example", service_arn=example_aws_apprunner_service["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.Deployment("example", new()
    {
        ServiceArn = exampleAwsApprunnerService.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewDeployment(ctx, "example", &apprunner.DeploymentArgs{
			ServiceArn: pulumi.Any(exampleAwsApprunnerService.Arn),
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
import com.pulumi.aws.apprunner.Deployment;
import com.pulumi.aws.apprunner.DeploymentArgs;
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
        var example = new Deployment("example", DeploymentArgs.builder()
            .serviceArn(exampleAwsApprunnerService.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:Deployment
    properties:
      serviceArn: ${exampleAwsApprunnerService.arn}
```
<!--End PulumiCodeChooser -->
h

serviceArn" VThe Amazon Resource Name (ARN) of the App Runner service to start the deployment for.
f
timeoutsZBX:V
T
	apprunnerDeploymentTimeouts3aws:apprunner/DeploymentTimeouts:DeploymentTimeouts"N
operationId" ;The unique ID of the operation associated with deployment.
"h

serviceArn" VThe Amazon Resource Name (ARN) of the App Runner service to start the deployment for.
"G
status" 9The current status of the App Runner service deployment.
"f
timeoutsZBX:V
T
	apprunnerDeploymentTimeouts3aws:apprunner/DeploymentTimeouts:DeploymentTimeouts*�-
l
	apprunnerObservabilityConfigurationCaws:apprunner/observabilityConfiguration:ObservabilityConfiguration�Manages an App Runner Observability Configuration.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.ObservabilityConfiguration("example", {
    observabilityConfigurationName: "example",
    traceConfiguration: {
        vendor: "AWSXRAY",
    },
    tags: {
        Name: "example-apprunner-observability-configuration",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.ObservabilityConfiguration("example",
    observability_configuration_name="example",
    trace_configuration={
        "vendor": "AWSXRAY",
    },
    tags={
        "Name": "example-apprunner-observability-configuration",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.ObservabilityConfiguration("example", new()
    {
        ObservabilityConfigurationName = "example",
        TraceConfiguration = new Aws.AppRunner.Inputs.ObservabilityConfigurationTraceConfigurationArgs
        {
            Vendor = "AWSXRAY",
        },
        Tags = 
        {
            { "Name", "example-apprunner-observability-configuration" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewObservabilityConfiguration(ctx, "example", &apprunner.ObservabilityConfigurationArgs{
			ObservabilityConfigurationName: pulumi.String("example"),
			TraceConfiguration: &apprunner.ObservabilityConfigurationTraceConfigurationArgs{
				Vendor: pulumi.String("AWSXRAY"),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example-apprunner-observability-configuration"),
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
import com.pulumi.aws.apprunner.ObservabilityConfiguration;
import com.pulumi.aws.apprunner.ObservabilityConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ObservabilityConfigurationTraceConfigurationArgs;
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
        var example = new ObservabilityConfiguration("example", ObservabilityConfigurationArgs.builder()
            .observabilityConfigurationName("example")
            .traceConfiguration(ObservabilityConfigurationTraceConfigurationArgs.builder()
                .vendor("AWSXRAY")
                .build())
            .tags(Map.of("Name", "example-apprunner-observability-configuration"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:ObservabilityConfiguration
    properties:
      observabilityConfigurationName: example
      traceConfiguration:
        vendor: AWSXRAY
      tags:
        Name: example-apprunner-observability-configuration
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner Observability Configuration using the `arn`. For example:

```sh
$ pulumi import aws:apprunner/observabilityConfiguration:ObservabilityConfiguration example arn:aws:apprunner:us-east-1:1234567890:observabilityconfiguration/example/1/d75bc7ea55b71e724fe5c23452fe22a1
```
O
observabilityConfigurationName" )Name of the observability configuration.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
traceConfiguration�B�:�
�
	apprunner,ObservabilityConfigurationTraceConfigurationgaws:apprunner/ObservabilityConfigurationTraceConfiguration:ObservabilityConfigurationTraceConfiguration�Configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing. See Trace Configuration below for more details.
"4
arn" )ARN of this observability configuration.
"�
latest
 �Whether the observability configuration has the highest `observability_configuration_revision` among all configurations that share the same `observability_configuration_name`.
"O
observabilityConfigurationName" )Name of the observability configuration.
"\
"observabilityConfigurationRevision 2The revision of this observability configuration.
"�
status" �Current state of the observability configuration. An INACTIVE configuration revision has been deleted and can't be used. It is permanently removed some time after deletion.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
traceConfiguration�B�:�
�
	apprunner,ObservabilityConfigurationTraceConfigurationgaws:apprunner/ObservabilityConfigurationTraceConfiguration:ObservabilityConfigurationTraceConfiguration�Configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing. See Trace Configuration below for more details.
*��
3
	apprunnerServiceaws:apprunner/service:Service��Manages an App Runner Service.

## Example Usage

### Service with a Code Repository Source

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.Service("example", {
    serviceName: "example",
    sourceConfiguration: {
        authenticationConfiguration: {
            connectionArn: exampleAwsApprunnerConnection.arn,
        },
        codeRepository: {
            codeConfiguration: {
                codeConfigurationValues: {
                    buildCommand: "python setup.py develop",
                    port: "8000",
                    runtime: "PYTHON_3",
                    startCommand: "python runapp.py",
                },
                configurationSource: "API",
            },
            repositoryUrl: "https://github.com/example/my-example-python-app",
            sourceCodeVersion: {
                type: "BRANCH",
                value: "main",
            },
        },
    },
    networkConfiguration: {
        egressConfiguration: {
            egressType: "VPC",
            vpcConnectorArn: connector.arn,
        },
    },
    tags: {
        Name: "example-apprunner-service",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.Service("example",
    service_name="example",
    source_configuration={
        "authentication_configuration": {
            "connection_arn": example_aws_apprunner_connection["arn"],
        },
        "code_repository": {
            "code_configuration": {
                "code_configuration_values": {
                    "build_command": "python setup.py develop",
                    "port": "8000",
                    "runtime": "PYTHON_3",
                    "start_command": "python runapp.py",
                },
                "configuration_source": "API",
            },
            "repository_url": "https://github.com/example/my-example-python-app",
            "source_code_version": {
                "type": "BRANCH",
                "value": "main",
            },
        },
    },
    network_configuration={
        "egress_configuration": {
            "egress_type": "VPC",
            "vpc_connector_arn": connector["arn"],
        },
    },
    tags={
        "Name": "example-apprunner-service",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.Service("example", new()
    {
        ServiceName = "example",
        SourceConfiguration = new Aws.AppRunner.Inputs.ServiceSourceConfigurationArgs
        {
            AuthenticationConfiguration = new Aws.AppRunner.Inputs.ServiceSourceConfigurationAuthenticationConfigurationArgs
            {
                ConnectionArn = exampleAwsApprunnerConnection.Arn,
            },
            CodeRepository = new Aws.AppRunner.Inputs.ServiceSourceConfigurationCodeRepositoryArgs
            {
                CodeConfiguration = new Aws.AppRunner.Inputs.ServiceSourceConfigurationCodeRepositoryCodeConfigurationArgs
                {
                    CodeConfigurationValues = new Aws.AppRunner.Inputs.ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValuesArgs
                    {
                        BuildCommand = "python setup.py develop",
                        Port = "8000",
                        Runtime = "PYTHON_3",
                        StartCommand = "python runapp.py",
                    },
                    ConfigurationSource = "API",
                },
                RepositoryUrl = "https://github.com/example/my-example-python-app",
                SourceCodeVersion = new Aws.AppRunner.Inputs.ServiceSourceConfigurationCodeRepositorySourceCodeVersionArgs
                {
                    Type = "BRANCH",
                    Value = "main",
                },
            },
        },
        NetworkConfiguration = new Aws.AppRunner.Inputs.ServiceNetworkConfigurationArgs
        {
            EgressConfiguration = new Aws.AppRunner.Inputs.ServiceNetworkConfigurationEgressConfigurationArgs
            {
                EgressType = "VPC",
                VpcConnectorArn = connector.Arn,
            },
        },
        Tags = 
        {
            { "Name", "example-apprunner-service" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewService(ctx, "example", &apprunner.ServiceArgs{
			ServiceName: pulumi.String("example"),
			SourceConfiguration: &apprunner.ServiceSourceConfigurationArgs{
				AuthenticationConfiguration: &apprunner.ServiceSourceConfigurationAuthenticationConfigurationArgs{
					ConnectionArn: pulumi.Any(exampleAwsApprunnerConnection.Arn),
				},
				CodeRepository: &apprunner.ServiceSourceConfigurationCodeRepositoryArgs{
					CodeConfiguration: &apprunner.ServiceSourceConfigurationCodeRepositoryCodeConfigurationArgs{
						CodeConfigurationValues: &apprunner.ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValuesArgs{
							BuildCommand: pulumi.String("python setup.py develop"),
							Port:         pulumi.String("8000"),
							Runtime:      pulumi.String("PYTHON_3"),
							StartCommand: pulumi.String("python runapp.py"),
						},
						ConfigurationSource: pulumi.String("API"),
					},
					RepositoryUrl: pulumi.String("https://github.com/example/my-example-python-app"),
					SourceCodeVersion: &apprunner.ServiceSourceConfigurationCodeRepositorySourceCodeVersionArgs{
						Type:  pulumi.String("BRANCH"),
						Value: pulumi.String("main"),
					},
				},
			},
			NetworkConfiguration: &apprunner.ServiceNetworkConfigurationArgs{
				EgressConfiguration: &apprunner.ServiceNetworkConfigurationEgressConfigurationArgs{
					EgressType:      pulumi.String("VPC"),
					VpcConnectorArn: pulumi.Any(connector.Arn),
				},
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example-apprunner-service"),
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
import com.pulumi.aws.apprunner.Service;
import com.pulumi.aws.apprunner.ServiceArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationAuthenticationConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationCodeRepositoryArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationCodeRepositoryCodeConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValuesArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationCodeRepositorySourceCodeVersionArgs;
import com.pulumi.aws.apprunner.inputs.ServiceNetworkConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ServiceNetworkConfigurationEgressConfigurationArgs;
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
        var example = new Service("example", ServiceArgs.builder()
            .serviceName("example")
            .sourceConfiguration(ServiceSourceConfigurationArgs.builder()
                .authenticationConfiguration(ServiceSourceConfigurationAuthenticationConfigurationArgs.builder()
                    .connectionArn(exampleAwsApprunnerConnection.arn())
                    .build())
                .codeRepository(ServiceSourceConfigurationCodeRepositoryArgs.builder()
                    .codeConfiguration(ServiceSourceConfigurationCodeRepositoryCodeConfigurationArgs.builder()
                        .codeConfigurationValues(ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValuesArgs.builder()
                            .buildCommand("python setup.py develop")
                            .port("8000")
                            .runtime("PYTHON_3")
                            .startCommand("python runapp.py")
                            .build())
                        .configurationSource("API")
                        .build())
                    .repositoryUrl("https://github.com/example/my-example-python-app")
                    .sourceCodeVersion(ServiceSourceConfigurationCodeRepositorySourceCodeVersionArgs.builder()
                        .type("BRANCH")
                        .value("main")
                        .build())
                    .build())
                .build())
            .networkConfiguration(ServiceNetworkConfigurationArgs.builder()
                .egressConfiguration(ServiceNetworkConfigurationEgressConfigurationArgs.builder()
                    .egressType("VPC")
                    .vpcConnectorArn(connector.arn())
                    .build())
                .build())
            .tags(Map.of("Name", "example-apprunner-service"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:Service
    properties:
      serviceName: example
      sourceConfiguration:
        authenticationConfiguration:
          connectionArn: ${exampleAwsApprunnerConnection.arn}
        codeRepository:
          codeConfiguration:
            codeConfigurationValues:
              buildCommand: python setup.py develop
              port: '8000'
              runtime: PYTHON_3
              startCommand: python runapp.py
            configurationSource: API
          repositoryUrl: https://github.com/example/my-example-python-app
          sourceCodeVersion:
            type: BRANCH
            value: main
      networkConfiguration:
        egressConfiguration:
          egressType: VPC
          vpcConnectorArn: ${connector.arn}
      tags:
        Name: example-apprunner-service
```
<!--End PulumiCodeChooser -->

### Service with an Image Repository Source

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.Service("example", {
    serviceName: "example",
    sourceConfiguration: {
        imageRepository: {
            imageConfiguration: {
                port: "8000",
            },
            imageIdentifier: "public.ecr.aws/aws-containers/hello-app-runner:latest",
            imageRepositoryType: "ECR_PUBLIC",
        },
        autoDeploymentsEnabled: false,
    },
    tags: {
        Name: "example-apprunner-service",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.Service("example",
    service_name="example",
    source_configuration={
        "image_repository": {
            "image_configuration": {
                "port": "8000",
            },
            "image_identifier": "public.ecr.aws/aws-containers/hello-app-runner:latest",
            "image_repository_type": "ECR_PUBLIC",
        },
        "auto_deployments_enabled": False,
    },
    tags={
        "Name": "example-apprunner-service",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.Service("example", new()
    {
        ServiceName = "example",
        SourceConfiguration = new Aws.AppRunner.Inputs.ServiceSourceConfigurationArgs
        {
            ImageRepository = new Aws.AppRunner.Inputs.ServiceSourceConfigurationImageRepositoryArgs
            {
                ImageConfiguration = new Aws.AppRunner.Inputs.ServiceSourceConfigurationImageRepositoryImageConfigurationArgs
                {
                    Port = "8000",
                },
                ImageIdentifier = "public.ecr.aws/aws-containers/hello-app-runner:latest",
                ImageRepositoryType = "ECR_PUBLIC",
            },
            AutoDeploymentsEnabled = false,
        },
        Tags = 
        {
            { "Name", "example-apprunner-service" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewService(ctx, "example", &apprunner.ServiceArgs{
			ServiceName: pulumi.String("example"),
			SourceConfiguration: &apprunner.ServiceSourceConfigurationArgs{
				ImageRepository: &apprunner.ServiceSourceConfigurationImageRepositoryArgs{
					ImageConfiguration: &apprunner.ServiceSourceConfigurationImageRepositoryImageConfigurationArgs{
						Port: pulumi.String("8000"),
					},
					ImageIdentifier:     pulumi.String("public.ecr.aws/aws-containers/hello-app-runner:latest"),
					ImageRepositoryType: pulumi.String("ECR_PUBLIC"),
				},
				AutoDeploymentsEnabled: pulumi.Bool(false),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example-apprunner-service"),
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
import com.pulumi.aws.apprunner.Service;
import com.pulumi.aws.apprunner.ServiceArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationImageRepositoryArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationImageRepositoryImageConfigurationArgs;
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
        var example = new Service("example", ServiceArgs.builder()
            .serviceName("example")
            .sourceConfiguration(ServiceSourceConfigurationArgs.builder()
                .imageRepository(ServiceSourceConfigurationImageRepositoryArgs.builder()
                    .imageConfiguration(ServiceSourceConfigurationImageRepositoryImageConfigurationArgs.builder()
                        .port("8000")
                        .build())
                    .imageIdentifier("public.ecr.aws/aws-containers/hello-app-runner:latest")
                    .imageRepositoryType("ECR_PUBLIC")
                    .build())
                .autoDeploymentsEnabled(false)
                .build())
            .tags(Map.of("Name", "example-apprunner-service"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:Service
    properties:
      serviceName: example
      sourceConfiguration:
        imageRepository:
          imageConfiguration:
            port: '8000'
          imageIdentifier: public.ecr.aws/aws-containers/hello-app-runner:latest
          imageRepositoryType: ECR_PUBLIC
        autoDeploymentsEnabled: false
      tags:
        Name: example-apprunner-service
```
<!--End PulumiCodeChooser -->

### Service with Observability Configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleObservabilityConfiguration = new aws.apprunner.ObservabilityConfiguration("example", {
    observabilityConfigurationName: "example",
    traceConfiguration: {
        vendor: "AWSXRAY",
    },
});
const example = new aws.apprunner.Service("example", {
    serviceName: "example",
    observabilityConfiguration: {
        observabilityConfigurationArn: exampleObservabilityConfiguration.arn,
        observabilityEnabled: true,
    },
    sourceConfiguration: {
        imageRepository: {
            imageConfiguration: {
                port: "8000",
            },
            imageIdentifier: "public.ecr.aws/aws-containers/hello-app-runner:latest",
            imageRepositoryType: "ECR_PUBLIC",
        },
        autoDeploymentsEnabled: false,
    },
    tags: {
        Name: "example-apprunner-service",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example_observability_configuration = aws.apprunner.ObservabilityConfiguration("example",
    observability_configuration_name="example",
    trace_configuration={
        "vendor": "AWSXRAY",
    })
example = aws.apprunner.Service("example",
    service_name="example",
    observability_configuration={
        "observability_configuration_arn": example_observability_configuration.arn,
        "observability_enabled": True,
    },
    source_configuration={
        "image_repository": {
            "image_configuration": {
                "port": "8000",
            },
            "image_identifier": "public.ecr.aws/aws-containers/hello-app-runner:latest",
            "image_repository_type": "ECR_PUBLIC",
        },
        "auto_deployments_enabled": False,
    },
    tags={
        "Name": "example-apprunner-service",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleObservabilityConfiguration = new Aws.AppRunner.ObservabilityConfiguration("example", new()
    {
        ObservabilityConfigurationName = "example",
        TraceConfiguration = new Aws.AppRunner.Inputs.ObservabilityConfigurationTraceConfigurationArgs
        {
            Vendor = "AWSXRAY",
        },
    });

    var example = new Aws.AppRunner.Service("example", new()
    {
        ServiceName = "example",
        ObservabilityConfiguration = new Aws.AppRunner.Inputs.ServiceObservabilityConfigurationArgs
        {
            ObservabilityConfigurationArn = exampleObservabilityConfiguration.Arn,
            ObservabilityEnabled = true,
        },
        SourceConfiguration = new Aws.AppRunner.Inputs.ServiceSourceConfigurationArgs
        {
            ImageRepository = new Aws.AppRunner.Inputs.ServiceSourceConfigurationImageRepositoryArgs
            {
                ImageConfiguration = new Aws.AppRunner.Inputs.ServiceSourceConfigurationImageRepositoryImageConfigurationArgs
                {
                    Port = "8000",
                },
                ImageIdentifier = "public.ecr.aws/aws-containers/hello-app-runner:latest",
                ImageRepositoryType = "ECR_PUBLIC",
            },
            AutoDeploymentsEnabled = false,
        },
        Tags = 
        {
            { "Name", "example-apprunner-service" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleObservabilityConfiguration, err := apprunner.NewObservabilityConfiguration(ctx, "example", &apprunner.ObservabilityConfigurationArgs{
			ObservabilityConfigurationName: pulumi.String("example"),
			TraceConfiguration: &apprunner.ObservabilityConfigurationTraceConfigurationArgs{
				Vendor: pulumi.String("AWSXRAY"),
			},
		})
		if err != nil {
			return err
		}
		_, err = apprunner.NewService(ctx, "example", &apprunner.ServiceArgs{
			ServiceName: pulumi.String("example"),
			ObservabilityConfiguration: &apprunner.ServiceObservabilityConfigurationArgs{
				ObservabilityConfigurationArn: exampleObservabilityConfiguration.Arn,
				ObservabilityEnabled:          pulumi.Bool(true),
			},
			SourceConfiguration: &apprunner.ServiceSourceConfigurationArgs{
				ImageRepository: &apprunner.ServiceSourceConfigurationImageRepositoryArgs{
					ImageConfiguration: &apprunner.ServiceSourceConfigurationImageRepositoryImageConfigurationArgs{
						Port: pulumi.String("8000"),
					},
					ImageIdentifier:     pulumi.String("public.ecr.aws/aws-containers/hello-app-runner:latest"),
					ImageRepositoryType: pulumi.String("ECR_PUBLIC"),
				},
				AutoDeploymentsEnabled: pulumi.Bool(false),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example-apprunner-service"),
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
import com.pulumi.aws.apprunner.ObservabilityConfiguration;
import com.pulumi.aws.apprunner.ObservabilityConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ObservabilityConfigurationTraceConfigurationArgs;
import com.pulumi.aws.apprunner.Service;
import com.pulumi.aws.apprunner.ServiceArgs;
import com.pulumi.aws.apprunner.inputs.ServiceObservabilityConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationImageRepositoryArgs;
import com.pulumi.aws.apprunner.inputs.ServiceSourceConfigurationImageRepositoryImageConfigurationArgs;
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
        var exampleObservabilityConfiguration = new ObservabilityConfiguration("exampleObservabilityConfiguration", ObservabilityConfigurationArgs.builder()
            .observabilityConfigurationName("example")
            .traceConfiguration(ObservabilityConfigurationTraceConfigurationArgs.builder()
                .vendor("AWSXRAY")
                .build())
            .build());

        var example = new Service("example", ServiceArgs.builder()
            .serviceName("example")
            .observabilityConfiguration(ServiceObservabilityConfigurationArgs.builder()
                .observabilityConfigurationArn(exampleObservabilityConfiguration.arn())
                .observabilityEnabled(true)
                .build())
            .sourceConfiguration(ServiceSourceConfigurationArgs.builder()
                .imageRepository(ServiceSourceConfigurationImageRepositoryArgs.builder()
                    .imageConfiguration(ServiceSourceConfigurationImageRepositoryImageConfigurationArgs.builder()
                        .port("8000")
                        .build())
                    .imageIdentifier("public.ecr.aws/aws-containers/hello-app-runner:latest")
                    .imageRepositoryType("ECR_PUBLIC")
                    .build())
                .autoDeploymentsEnabled(false)
                .build())
            .tags(Map.of("Name", "example-apprunner-service"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:Service
    properties:
      serviceName: example
      observabilityConfiguration:
        observabilityConfigurationArn: ${exampleObservabilityConfiguration.arn}
        observabilityEnabled: true
      sourceConfiguration:
        imageRepository:
          imageConfiguration:
            port: '8000'
          imageIdentifier: public.ecr.aws/aws-containers/hello-app-runner:latest
          imageRepositoryType: ECR_PUBLIC
        autoDeploymentsEnabled: false
      tags:
        Name: example-apprunner-service
  exampleObservabilityConfiguration:
    type: aws:apprunner:ObservabilityConfiguration
    name: example
    properties:
      observabilityConfigurationName: example
      traceConfiguration:
        vendor: AWSXRAY
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner Services using the `arn`. For example:

```sh
$ pulumi import aws:apprunner/service:Service example arn:aws:apprunner:us-east-1:1234567890:service/example/0a03292a89764e5882c41d8f991c82fe
```
�
autoScalingConfigurationArnB" �ARN of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner associates the latest revision of a default auto scaling configuration.
�
encryptionConfiguration~B|:z
x
	apprunnerServiceEncryptionConfigurationKaws:apprunner/ServiceEncryptionConfiguration:ServiceEncryptionConfiguration�An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default, App Runner uses an AWS managed CMK. See Encryption Configuration below for more details.
�
healthCheckConfiguration�B:}
{
	apprunnerServiceHealthCheckConfigurationMaws:apprunner/ServiceHealthCheckConfiguration:ServiceHealthCheckConfiguration�Settings of the health check that AWS App Runner performs to monitor the health of your service. See Health Check Configuration below for more details.
�
instanceConfigurationxBv:t
r
	apprunnerServiceInstanceConfigurationGaws:apprunner/ServiceInstanceConfiguration:ServiceInstanceConfiguration�The runtime configuration of instances (scaling units) of the App Runner service. See Instance Configuration below for more details.
�
networkConfigurationuBs:q
o
	apprunnerServiceNetworkConfigurationEaws:apprunner/ServiceNetworkConfiguration:ServiceNetworkConfiguration�Configuration settings related to network traffic of the web application that the App Runner service runs. See Network Configuration below for more details.
�
observabilityConfiguration�B�:�
�
	apprunner!ServiceObservabilityConfigurationQaws:apprunner/ServiceObservabilityConfiguration:ServiceObservabilityConfigurationiThe observability configuration of your service. See Observability Configuration below for more details.
(
serviceName" Name of the service.
�
sourceConfigurationp:n
l
	apprunnerServiceSourceConfigurationCaws:apprunner/ServiceSourceConfiguration:ServiceSourceConfiguration�The source to deploy to the App Runner service. Can be a code or an image repository. See Source Configuration below for more details.

The following arguments are optional:
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"*
arn" ARN of the App Runner service.
"�
autoScalingConfigurationArn" �ARN of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner associates the latest revision of a default auto scaling configuration.
"�
encryptionConfiguration~B|:z
x
	apprunnerServiceEncryptionConfigurationKaws:apprunner/ServiceEncryptionConfiguration:ServiceEncryptionConfiguration�An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default, App Runner uses an AWS managed CMK. See Encryption Configuration below for more details.
"�
healthCheckConfiguration:}
{
	apprunnerServiceHealthCheckConfigurationMaws:apprunner/ServiceHealthCheckConfiguration:ServiceHealthCheckConfiguration�Settings of the health check that AWS App Runner performs to monitor the health of your service. See Health Check Configuration below for more details.
"�
instanceConfigurationv:t
r
	apprunnerServiceInstanceConfigurationGaws:apprunner/ServiceInstanceConfiguration:ServiceInstanceConfiguration�The runtime configuration of instances (scaling units) of the App Runner service. See Instance Configuration below for more details.
"�
networkConfigurations:q
o
	apprunnerServiceNetworkConfigurationEaws:apprunner/ServiceNetworkConfiguration:ServiceNetworkConfiguration�Configuration settings related to network traffic of the web application that the App Runner service runs. See Network Configuration below for more details.
"�
observabilityConfiguration�B�:�
�
	apprunner!ServiceObservabilityConfigurationQaws:apprunner/ServiceObservabilityConfiguration:ServiceObservabilityConfigurationiThe observability configuration of your service. See Observability Configuration below for more details.
"n
	serviceId" ]An alphanumeric ID that App Runner generated for this service. Unique within the AWS Region.
"(
serviceName" Name of the service.
"�

serviceUrl" wSubdomain URL that App Runner generated for this service. You can use this URL to access your service web application.
"�
sourceConfigurationp:n
l
	apprunnerServiceSourceConfigurationCaws:apprunner/ServiceSourceConfiguration:ServiceSourceConfiguration�The source to deploy to the App Runner service. Can be a code or an image repository. See Source Configuration below for more details.

The following arguments are optional:
"7
status" )Current state of the App Runner service.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�(
B
	apprunnerVpcConnector'aws:apprunner/vpcConnector:VpcConnector�Manages an App Runner VPC Connector.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const connector = new aws.apprunner.VpcConnector("connector", {
    vpcConnectorName: "name",
    subnets: [
        "subnet1",
        "subnet2",
    ],
    securityGroups: [
        "sg1",
        "sg2",
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

connector = aws.apprunner.VpcConnector("connector",
    vpc_connector_name="name",
    subnets=[
        "subnet1",
        "subnet2",
    ],
    security_groups=[
        "sg1",
        "sg2",
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var connector = new Aws.AppRunner.VpcConnector("connector", new()
    {
        VpcConnectorName = "name",
        Subnets = new[]
        {
            "subnet1",
            "subnet2",
        },
        SecurityGroups = new[]
        {
            "sg1",
            "sg2",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewVpcConnector(ctx, "connector", &apprunner.VpcConnectorArgs{
			VpcConnectorName: pulumi.String("name"),
			Subnets: pulumi.StringArray{
				pulumi.String("subnet1"),
				pulumi.String("subnet2"),
			},
			SecurityGroups: pulumi.StringArray{
				pulumi.String("sg1"),
				pulumi.String("sg2"),
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
import com.pulumi.aws.apprunner.VpcConnector;
import com.pulumi.aws.apprunner.VpcConnectorArgs;
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
        var connector = new VpcConnector("connector", VpcConnectorArgs.builder()
            .vpcConnectorName("name")
            .subnets(            
                "subnet1",
                "subnet2")
            .securityGroups(            
                "sg1",
                "sg2")
            .build());

    }
}
```
```yaml
resources:
  connector:
    type: aws:apprunner:VpcConnector
    properties:
      vpcConnectorName: name
      subnets:
        - subnet1
        - subnet2
      securityGroups:
        - sg1
        - sg2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner vpc connector using the `arn`. For example:

```sh
$ pulumi import aws:apprunner/vpcConnector:VpcConnector example arn:aws:apprunner:us-east-1:1234567890:vpcconnector/example/1/0a03292a89764e5882c41d8f991c82fe
```
�
securityGroups*" �List of IDs of security groups that App Runner should use for access to AWS resources under the specified subnets. If not specified, App Runner uses the default security group of the Amazon VPC. The default security group allows all outbound traffic.
�
subnets*" �List of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
4
vpcConnectorName" Name for the VPC connector.
"!
arn" ARN of VPC connector.
"�
securityGroups*" �List of IDs of security groups that App Runner should use for access to AWS resources under the specified subnets. If not specified, App Runner uses the default security group of the Amazon VPC. The default security group allows all outbound traffic.
"�
status" �Current state of the VPC connector. If the status of a connector revision is INACTIVE, it was deleted and can't be used. Inactive connector revisions are permanently removed some time after they are deleted.
"�
subnets*" �List of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"4
vpcConnectorName" Name for the VPC connector.
"�
vpcConnectorRevision zThe revision of VPC connector. It's unique among all the active connectors ("Status": "ACTIVE") that share the same Name.
*�/
Z
	apprunnerVpcIngressConnection7aws:apprunner/vpcIngressConnection:VpcIngressConnection�Manages an App Runner VPC Ingress Connection.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.apprunner.VpcIngressConnection("example", {
    name: "example",
    serviceArn: exampleAwsApprunnerService.arn,
    ingressVpcConfiguration: {
        vpcId: _default.id,
        vpcEndpointId: apprunner.id,
    },
    tags: {
        foo: "bar",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.apprunner.VpcIngressConnection("example",
    name="example",
    service_arn=example_aws_apprunner_service["arn"],
    ingress_vpc_configuration={
        "vpc_id": default["id"],
        "vpc_endpoint_id": apprunner["id"],
    },
    tags={
        "foo": "bar",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppRunner.VpcIngressConnection("example", new()
    {
        Name = "example",
        ServiceArn = exampleAwsApprunnerService.Arn,
        IngressVpcConfiguration = new Aws.AppRunner.Inputs.VpcIngressConnectionIngressVpcConfigurationArgs
        {
            VpcId = @default.Id,
            VpcEndpointId = apprunner.Id,
        },
        Tags = 
        {
            { "foo", "bar" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := apprunner.NewVpcIngressConnection(ctx, "example", &apprunner.VpcIngressConnectionArgs{
			Name:       pulumi.String("example"),
			ServiceArn: pulumi.Any(exampleAwsApprunnerService.Arn),
			IngressVpcConfiguration: &apprunner.VpcIngressConnectionIngressVpcConfigurationArgs{
				VpcId:         pulumi.Any(_default.Id),
				VpcEndpointId: pulumi.Any(apprunner.Id),
			},
			Tags: pulumi.StringMap{
				"foo": pulumi.String("bar"),
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
import com.pulumi.aws.apprunner.VpcIngressConnection;
import com.pulumi.aws.apprunner.VpcIngressConnectionArgs;
import com.pulumi.aws.apprunner.inputs.VpcIngressConnectionIngressVpcConfigurationArgs;
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
        var example = new VpcIngressConnection("example", VpcIngressConnectionArgs.builder()
            .name("example")
            .serviceArn(exampleAwsApprunnerService.arn())
            .ingressVpcConfiguration(VpcIngressConnectionIngressVpcConfigurationArgs.builder()
                .vpcId(default_.id())
                .vpcEndpointId(apprunner.id())
                .build())
            .tags(Map.of("foo", "bar"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:apprunner:VpcIngressConnection
    properties:
      name: example
      serviceArn: ${exampleAwsApprunnerService.arn}
      ingressVpcConfiguration:
        vpcId: ${default.id}
        vpcEndpointId: ${apprunner.id}
      tags:
        foo: bar
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import App Runner VPC Ingress Connection using the `arn`. For example:

```sh
$ pulumi import aws:apprunner/vpcIngressConnection:VpcIngressConnection example "arn:aws:apprunner:us-west-2:837424938642:vpcingressconnection/example/b379f86381d74825832c2e82080342fa"
```
�
ingressVpcConfiguration�:�
�
	apprunner+VpcIngressConnectionIngressVpcConfigurationeaws:apprunner/VpcIngressConnectionIngressVpcConfiguration:VpcIngressConnectionIngressVpcConfiguration�Specifications for the customer’s Amazon VPC and the related AWS PrivateLink VPC endpoint that are used to create the VPC Ingress Connection resource. See Ingress VPC Configuration below for more details.
�
nameB" �A name for the VPC Ingress Connection resource. It must be unique across all the active VPC Ingress Connections in your AWS account in the AWS Region.
�

serviceArn" wThe Amazon Resource Name (ARN) for this App Runner service that is used to create the VPC Ingress Connection resource.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"I
arn" >The Amazon Resource Name (ARN) of the VPC Ingress Connection.
"W

domainName" EThe domain name associated with the VPC Ingress Connection resource.
"�
ingressVpcConfiguration�:�
�
	apprunner+VpcIngressConnectionIngressVpcConfigurationeaws:apprunner/VpcIngressConnectionIngressVpcConfiguration:VpcIngressConnectionIngressVpcConfiguration�Specifications for the customer’s Amazon VPC and the related AWS PrivateLink VPC endpoint that are used to create the VPC Ingress Connection resource. See Ingress VPC Configuration below for more details.
"�
name" �A name for the VPC Ingress Connection resource. It must be unique across all the active VPC Ingress Connections in your AWS account in the AWS Region.
"�

serviceArn" wThe Amazon Resource Name (ARN) for this App Runner service that is used to create the VPC Ingress Connection resource.
"@
status" 2The current status of the VPC Ingress Connection.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�'
K
	appstreamDirectoryConfig-aws:appstream/directoryConfig:DirectoryConfig�Provides an AppStream Directory Config.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appstream.DirectoryConfig("example", {
    directoryName: "NAME OF DIRECTORY",
    organizationalUnitDistinguishedNames: ["DISTINGUISHED NAME"],
    serviceAccountCredentials: {
        accountName: "NAME OF ACCOUNT",
        accountPassword: "PASSWORD OF ACCOUNT",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appstream.DirectoryConfig("example",
    directory_name="NAME OF DIRECTORY",
    organizational_unit_distinguished_names=["DISTINGUISHED NAME"],
    service_account_credentials={
        "account_name": "NAME OF ACCOUNT",
        "account_password": "PASSWORD OF ACCOUNT",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppStream.DirectoryConfig("example", new()
    {
        DirectoryName = "NAME OF DIRECTORY",
        OrganizationalUnitDistinguishedNames = new[]
        {
            "DISTINGUISHED NAME",
        },
        ServiceAccountCredentials = new Aws.AppStream.Inputs.DirectoryConfigServiceAccountCredentialsArgs
        {
            AccountName = "NAME OF ACCOUNT",
            AccountPassword = "PASSWORD OF ACCOUNT",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appstream"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appstream.NewDirectoryConfig(ctx, "example", &appstream.DirectoryConfigArgs{
			DirectoryName: pulumi.String("NAME OF DIRECTORY"),
			OrganizationalUnitDistinguishedNames: pulumi.StringArray{
				pulumi.String("DISTINGUISHED NAME"),
			},
			ServiceAccountCredentials: &appstream.DirectoryConfigServiceAccountCredentialsArgs{
				AccountName:     pulumi.String("NAME OF ACCOUNT"),
				AccountPassword: pulumi.String("PASSWORD OF ACCOUNT"),
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
import com.pulumi.aws.appstream.DirectoryConfig;
import com.pulumi.aws.appstream.DirectoryConfigArgs;
import com.pulumi.aws.appstream.inputs.DirectoryConfigServiceAccountCredentialsArgs;
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
        var example = new DirectoryConfig("example", DirectoryConfigArgs.builder()
            .directoryName("NAME OF DIRECTORY")
            .organizationalUnitDistinguishedNames("DISTINGUISHED NAME")
            .serviceAccountCredentials(DirectoryConfigServiceAccountCredentialsArgs.builder()
                .accountName("NAME OF ACCOUNT")
                .accountPassword("PASSWORD OF ACCOUNT")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appstream:DirectoryConfig
    properties:
      directoryName: NAME OF DIRECTORY
      organizationalUnitDistinguishedNames:
        - DISTINGUISHED NAME
      serviceAccountCredentials:
        accountName: NAME OF ACCOUNT
        accountPassword: PASSWORD OF ACCOUNT
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appstream_directory_config` using the id. For example:

```sh
$ pulumi import aws:appstream/directoryConfig:DirectoryConfig example directoryNameExample
```
<
directoryName" 'Fully qualified name of the directory.
u
$organizationalUnitDistinguishedNames*" GDistinguished names of the organizational units for computer accounts.
�
serviceAccountCredentials�:�
�
	appstream(DirectoryConfigServiceAccountCredentials_aws:appstream/DirectoryConfigServiceAccountCredentials:DirectoryConfigServiceAccountCredentials�Configuration block for the name of the directory and organizational unit (OU) to use to join the directory config to a Microsoft Active Directory domain. See `service_account_credentials` below.
"n
createdTime" [Date and time, in UTC and extended RFC 3339 format, when the directory config was created.
"<
directoryName" 'Fully qualified name of the directory.
"u
$organizationalUnitDistinguishedNames*" GDistinguished names of the organizational units for computer accounts.
"�
serviceAccountCredentials�:�
�
	appstream(DirectoryConfigServiceAccountCredentials_aws:appstream/DirectoryConfigServiceAccountCredentials:DirectoryConfigServiceAccountCredentials�Configuration block for the name of the directory and organizational unit (OU) to use to join the directory config to a Microsoft Active Directory domain. See `service_account_credentials` below.
*�S
-
	appstreamFleetaws:appstream/fleet:Fleet�-Provides an AppStream fleet.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testFleet = new aws.appstream.Fleet("test_fleet", {
    name: "test-fleet",
    computeCapacity: {
        desiredInstances: 1,
    },
    description: "test fleet",
    idleDisconnectTimeoutInSeconds: 60,
    displayName: "test-fleet",
    enableDefaultInternetAccess: false,
    fleetType: "ON_DEMAND",
    imageName: "Amazon-AppStream2-Sample-Image-03-11-2023",
    instanceType: "stream.standard.large",
    maxUserDurationInSeconds: 600,
    vpcConfig: {
        subnetIds: ["subnet-06e9b13400c225127"],
    },
    tags: {
        TagName: "tag-value",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test_fleet = aws.appstream.Fleet("test_fleet",
    name="test-fleet",
    compute_capacity={
        "desired_instances": 1,
    },
    description="test fleet",
    idle_disconnect_timeout_in_seconds=60,
    display_name="test-fleet",
    enable_default_internet_access=False,
    fleet_type="ON_DEMAND",
    image_name="Amazon-AppStream2-Sample-Image-03-11-2023",
    instance_type="stream.standard.large",
    max_user_duration_in_seconds=600,
    vpc_config={
        "subnet_ids": ["subnet-06e9b13400c225127"],
    },
    tags={
        "TagName": "tag-value",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testFleet = new Aws.AppStream.Fleet("test_fleet", new()
    {
        Name = "test-fleet",
        ComputeCapacity = new Aws.AppStream.Inputs.FleetComputeCapacityArgs
        {
            DesiredInstances = 1,
        },
        Description = "test fleet",
        IdleDisconnectTimeoutInSeconds = 60,
        DisplayName = "test-fleet",
        EnableDefaultInternetAccess = false,
        FleetType = "ON_DEMAND",
        ImageName = "Amazon-AppStream2-Sample-Image-03-11-2023",
        InstanceType = "stream.standard.large",
        MaxUserDurationInSeconds = 600,
        VpcConfig = new Aws.AppStream.Inputs.FleetVpcConfigArgs
        {
            SubnetIds = new[]
            {
                "subnet-06e9b13400c225127",
            },
        },
        Tags = 
        {
            { "TagName", "tag-value" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appstream"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appstream.NewFleet(ctx, "test_fleet", &appstream.FleetArgs{
			Name: pulumi.String("test-fleet"),
			ComputeCapacity: &appstream.FleetComputeCapacityArgs{
				DesiredInstances: pulumi.Int(1),
			},
			Description:                    pulumi.String("test fleet"),
			IdleDisconnectTimeoutInSeconds: pulumi.Int(60),
			DisplayName:                    pulumi.String("test-fleet"),
			EnableDefaultInternetAccess:    pulumi.Bool(false),
			FleetType:                      pulumi.String("ON_DEMAND"),
			ImageName:                      pulumi.String("Amazon-AppStream2-Sample-Image-03-11-2023"),
			InstanceType:                   pulumi.String("stream.standard.large"),
			MaxUserDurationInSeconds:       pulumi.Int(600),
			VpcConfig: &appstream.FleetVpcConfigArgs{
				SubnetIds: pulumi.StringArray{
					pulumi.String("subnet-06e9b13400c225127"),
				},
			},
			Tags: pulumi.StringMap{
				"TagName": pulumi.String("tag-value"),
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
import com.pulumi.aws.appstream.Fleet;
import com.pulumi.aws.appstream.FleetArgs;
import com.pulumi.aws.appstream.inputs.FleetComputeCapacityArgs;
import com.pulumi.aws.appstream.inputs.FleetVpcConfigArgs;
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
        var testFleet = new Fleet("testFleet", FleetArgs.builder()
            .name("test-fleet")
            .computeCapacity(FleetComputeCapacityArgs.builder()
                .desiredInstances(1)
                .build())
            .description("test fleet")
            .idleDisconnectTimeoutInSeconds(60)
            .displayName("test-fleet")
            .enableDefaultInternetAccess(false)
            .fleetType("ON_DEMAND")
            .imageName("Amazon-AppStream2-Sample-Image-03-11-2023")
            .instanceType("stream.standard.large")
            .maxUserDurationInSeconds(600)
            .vpcConfig(FleetVpcConfigArgs.builder()
                .subnetIds("subnet-06e9b13400c225127")
                .build())
            .tags(Map.of("TagName", "tag-value"))
            .build());

    }
}
```
```yaml
resources:
  testFleet:
    type: aws:appstream:Fleet
    name: test_fleet
    properties:
      name: test-fleet
      computeCapacity:
        desiredInstances: 1
      description: test fleet
      idleDisconnectTimeoutInSeconds: 60
      displayName: test-fleet
      enableDefaultInternetAccess: false
      fleetType: ON_DEMAND
      imageName: Amazon-AppStream2-Sample-Image-03-11-2023
      instanceType: stream.standard.large
      maxUserDurationInSeconds: 600
      vpcConfig:
        subnetIds:
          - subnet-06e9b13400c225127
      tags:
        TagName: tag-value
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appstream_fleet` using the id. For example:

```sh
$ pulumi import aws:appstream/fleet:Fleet example fleetNameExample
```
�
computeCapacity^:\
Z
	appstreamFleetComputeCapacity7aws:appstream/FleetComputeCapacity:FleetComputeCapacityFConfiguration block for the desired capacity of the fleet. See below.
-
descriptionB" Description to display.
s
disconnectTimeoutInSecondsB OAmount of time that a streaming session remains active after users disconnect.
K
displayNameB" 6Human-readable friendly name for the AppStream fleet.
�
domainJoinInfo]B[:Y
W
	appstreamFleetDomainJoinInfo5aws:appstream/FleetDomainJoinInfo:FleetDomainJoinInfo�Configuration block for the name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. See below.
`
enableDefaultInternetAccessB
 ;Enables or disables default internet access for the fleet.
J
	fleetTypeB" 7Fleet type. Valid values are: `ON_DEMAND`, `ALWAYS_ON`
?

iamRoleArnB" +ARN of the IAM role to apply to the fleet.
�
idleDisconnectTimeoutInSecondsB �Amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the `disconnect_timeout_in_seconds` time interval begins. Defaults to `0`. Valid value is between `60` and `3600 `seconds.
F
imageArnB" 4ARN of the public, private, or shared image to use.
?
	imageNameB" ,Name of the image used to create the fleet.
I
instanceType" 5Instance type to use when launching fleet instances.

maxSessionsPerInstanceB _The maximum number of user sessions on an instance. This only applies to multi-session fleets.
q
maxUserDurationInSecondsB OMaximum amount of time that a streaming session can remain active, in seconds.
P
nameB" BUnique name for the fleet.

The following arguments are optional:
�

streamViewB" �AppStream 2.0 view that is displayed to your users when they stream from the fleet. When `APP` is specified, only the windows of applications opened by users display. When `DESKTOP` is specified, the standard desktop that is provided by the operating system displays. If not specified, defaults to `APP`.
>
tagsB2" .Map of tags to attach to AppStream instances.
�
	vpcConfigNBL:J
H
	appstreamFleetVpcConfig+aws:appstream/FleetVpcConfig:FleetVpcConfigPConfiguration block for the VPC configuration for the image builder. See below.
"'
arn" ARN of the appstream fleet.
"�
computeCapacity^:\
Z
	appstreamFleetComputeCapacity7aws:appstream/FleetComputeCapacity:FleetComputeCapacityFConfiguration block for the desired capacity of the fleet. See below.
"c
createdTime" PDate and time, in UTC and extended RFC 3339 format, when the fleet was created.
"+
description" Description to display.
"q
disconnectTimeoutInSeconds OAmount of time that a streaming session remains active after users disconnect.
"I
displayName" 6Human-readable friendly name for the AppStream fleet.
"�
domainJoinInfo[:Y
W
	appstreamFleetDomainJoinInfo5aws:appstream/FleetDomainJoinInfo:FleetDomainJoinInfo�Configuration block for the name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. See below.
"^
enableDefaultInternetAccess
 ;Enables or disables default internet access for the fleet.
"H
	fleetType" 7Fleet type. Valid values are: `ON_DEMAND`, `ALWAYS_ON`
"=

iamRoleArn" +ARN of the IAM role to apply to the fleet.
"�
idleDisconnectTimeoutInSecondsB �Amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the `disconnect_timeout_in_seconds` time interval begins. Defaults to `0`. Valid value is between `60` and `3600 `seconds.
"D
imageArn" 4ARN of the public, private, or shared image to use.
"=
	imageName" ,Name of the image used to create the fleet.
"I
instanceType" 5Instance type to use when launching fleet instances.
"
maxSessionsPerInstanceB _The maximum number of user sessions on an instance. This only applies to multi-session fleets.
"o
maxUserDurationInSeconds OMaximum amount of time that a streaming session can remain active, in seconds.
"N
name" BUnique name for the fleet.

The following arguments are optional:
"W
state" JState of the fleet. Can be `STARTING`, `RUNNING`, `STOPPING` or `STOPPED`
"�

streamView" �AppStream 2.0 view that is displayed to your users when they stream from the fleet. When `APP` is specified, only the windows of applications opened by users display. When `DESKTOP` is specified, the standard desktop that is provided by the operating system displays. If not specified, defaults to `APP`.
">
tagsB2" .Map of tags to attach to AppStream instances.
"
tagsAll2" "�
	vpcConfigL:J
H
	appstreamFleetVpcConfig+aws:appstream/FleetVpcConfig:FleetVpcConfigPConfiguration block for the VPC configuration for the image builder. See below.
*�*
]
	appstreamFleetStackAssociation9aws:appstream/fleetStackAssociation:FleetStackAssociation�(Manages an AppStream Fleet Stack association.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appstream.Fleet("example", {
    name: "NAME",
    imageName: "Amazon-AppStream2-Sample-Image-03-11-2023",
    instanceType: "stream.standard.small",
    computeCapacity: {
        desiredInstances: 1,
    },
});
const exampleStack = new aws.appstream.Stack("example", {name: "STACK NAME"});
const exampleFleetStackAssociation = new aws.appstream.FleetStackAssociation("example", {
    fleetName: example.name,
    stackName: exampleStack.name,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appstream.Fleet("example",
    name="NAME",
    image_name="Amazon-AppStream2-Sample-Image-03-11-2023",
    instance_type="stream.standard.small",
    compute_capacity={
        "desired_instances": 1,
    })
example_stack = aws.appstream.Stack("example", name="STACK NAME")
example_fleet_stack_association = aws.appstream.FleetStackAssociation("example",
    fleet_name=example.name,
    stack_name=example_stack.name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppStream.Fleet("example", new()
    {
        Name = "NAME",
        ImageName = "Amazon-AppStream2-Sample-Image-03-11-2023",
        InstanceType = "stream.standard.small",
        ComputeCapacity = new Aws.AppStream.Inputs.FleetComputeCapacityArgs
        {
            DesiredInstances = 1,
        },
    });

    var exampleStack = new Aws.AppStream.Stack("example", new()
    {
        Name = "STACK NAME",
    });

    var exampleFleetStackAssociation = new Aws.AppStream.FleetStackAssociation("example", new()
    {
        FleetName = example.Name,
        StackName = exampleStack.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appstream"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := appstream.NewFleet(ctx, "example", &appstream.FleetArgs{
			Name:         pulumi.String("NAME"),
			ImageName:    pulumi.String("Amazon-AppStream2-Sample-Image-03-11-2023"),
			InstanceType: pulumi.String("stream.standard.small"),
			ComputeCapacity: &appstream.FleetComputeCapacityArgs{
				DesiredInstances: pulumi.Int(1),
			},
		})
		if err != nil {
			return err
		}
		exampleStack, err := appstream.NewStack(ctx, "example", &appstream.StackArgs{
			Name: pulumi.String("STACK NAME"),
		})
		if err != nil {
			return err
		}
		_, err = appstream.NewFleetStackAssociation(ctx, "example", &appstream.FleetStackAssociationArgs{
			FleetName: example.Name,
			StackName: exampleStack.Name,
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
import com.pulumi.aws.appstream.Fleet;
import com.pulumi.aws.appstream.FleetArgs;
import com.pulumi.aws.appstream.inputs.FleetComputeCapacityArgs;
import com.pulumi.aws.appstream.Stack;
import com.pulumi.aws.appstream.StackArgs;
import com.pulumi.aws.appstream.FleetStackAssociation;
import com.pulumi.aws.appstream.FleetStackAssociationArgs;
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
        var example = new Fleet("example", FleetArgs.builder()
            .name("NAME")
            .imageName("Amazon-AppStream2-Sample-Image-03-11-2023")
            .instanceType("stream.standard.small")
            .computeCapacity(FleetComputeCapacityArgs.builder()
                .desiredInstances(1)
                .build())
            .build());

        var exampleStack = new Stack("exampleStack", StackArgs.builder()
            .name("STACK NAME")
            .build());

        var exampleFleetStackAssociation = new FleetStackAssociation("exampleFleetStackAssociation", FleetStackAssociationArgs.builder()
            .fleetName(example.name())
            .stackName(exampleStack.name())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appstream:Fleet
    properties:
      name: NAME
      imageName: Amazon-AppStream2-Sample-Image-03-11-2023
      instanceType: stream.standard.small
      computeCapacity:
        desiredInstances: 1
  exampleStack:
    type: aws:appstream:Stack
    name: example
    properties:
      name: STACK NAME
  exampleFleetStackAssociation:
    type: aws:appstream:FleetStackAssociation
    name: example
    properties:
      fleetName: ${example.name}
      stackName: ${exampleStack.name}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppStream Stack Fleet Association using the `fleet_name` and `stack_name` separated by a slash (`/`). For example:

```sh
$ pulumi import aws:appstream/fleetStackAssociation:FleetStackAssociation example fleetName/stackName
```
$
	fleetName" Name of the fleet.
$
	stackName" Name of the stack.
"$
	fleetName" Name of the fleet.
"$
	stackName" Name of the stack.
*�C
B
	appstreamImageBuilder'aws:appstream/imageBuilder:ImageBuilder�%Provides an AppStream image builder.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testFleet = new aws.appstream.ImageBuilder("test_fleet", {
    name: "Name",
    description: "Description of a ImageBuilder",
    displayName: "Display name of a ImageBuilder",
    enableDefaultInternetAccess: false,
    imageName: "AppStream-WinServer2019-10-05-2022",
    instanceType: "stream.standard.large",
    vpcConfig: {
        subnetIds: [example.id],
    },
    tags: {
        Name: "Example Image Builder",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test_fleet = aws.appstream.ImageBuilder("test_fleet",
    name="Name",
    description="Description of a ImageBuilder",
    display_name="Display name of a ImageBuilder",
    enable_default_internet_access=False,
    image_name="AppStream-WinServer2019-10-05-2022",
    instance_type="stream.standard.large",
    vpc_config={
        "subnet_ids": [example["id"]],
    },
    tags={
        "Name": "Example Image Builder",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testFleet = new Aws.AppStream.ImageBuilder("test_fleet", new()
    {
        Name = "Name",
        Description = "Description of a ImageBuilder",
        DisplayName = "Display name of a ImageBuilder",
        EnableDefaultInternetAccess = false,
        ImageName = "AppStream-WinServer2019-10-05-2022",
        InstanceType = "stream.standard.large",
        VpcConfig = new Aws.AppStream.Inputs.ImageBuilderVpcConfigArgs
        {
            SubnetIds = new[]
            {
                example.Id,
            },
        },
        Tags = 
        {
            { "Name", "Example Image Builder" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appstream"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appstream.NewImageBuilder(ctx, "test_fleet", &appstream.ImageBuilderArgs{
			Name:                        pulumi.String("Name"),
			Description:                 pulumi.String("Description of a ImageBuilder"),
			DisplayName:                 pulumi.String("Display name of a ImageBuilder"),
			EnableDefaultInternetAccess: pulumi.Bool(false),
			ImageName:                   pulumi.String("AppStream-WinServer2019-10-05-2022"),
			InstanceType:                pulumi.String("stream.standard.large"),
			VpcConfig: &appstream.ImageBuilderVpcConfigArgs{
				SubnetIds: pulumi.StringArray{
					example.Id,
				},
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example Image Builder"),
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
import com.pulumi.aws.appstream.ImageBuilder;
import com.pulumi.aws.appstream.ImageBuilderArgs;
import com.pulumi.aws.appstream.inputs.ImageBuilderVpcConfigArgs;
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
        var testFleet = new ImageBuilder("testFleet", ImageBuilderArgs.builder()
            .name("Name")
            .description("Description of a ImageBuilder")
            .displayName("Display name of a ImageBuilder")
            .enableDefaultInternetAccess(false)
            .imageName("AppStream-WinServer2019-10-05-2022")
            .instanceType("stream.standard.large")
            .vpcConfig(ImageBuilderVpcConfigArgs.builder()
                .subnetIds(example.id())
                .build())
            .tags(Map.of("Name", "Example Image Builder"))
            .build());

    }
}
```
```yaml
resources:
  testFleet:
    type: aws:appstream:ImageBuilder
    name: test_fleet
    properties:
      name: Name
      description: Description of a ImageBuilder
      displayName: Display name of a ImageBuilder
      enableDefaultInternetAccess: false
      imageName: AppStream-WinServer2019-10-05-2022
      instanceType: stream.standard.large
      vpcConfig:
        subnetIds:
          - ${example.id}
      tags:
        Name: Example Image Builder
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appstream_image_builder` using the `name`. For example:

```sh
$ pulumi import aws:appstream/imageBuilder:ImageBuilder example imageBuilderExample
```
�
accessEndpointstBr*p:n
l
	appstreamImageBuilderAccessEndpointCaws:appstream/ImageBuilderAccessEndpoint:ImageBuilderAccessEndpointUSet of interface VPC endpoint (interface endpoint) objects. Maximum of 4. See below.
a
appstreamAgentVersionB" BVersion of the AppStream 2.0 agent to use for this image builder.
-
descriptionB" Description to display.
S
displayNameB" >Human-readable friendly name for the AppStream image builder.
�
domainJoinInforBp:n
l
	appstreamImageBuilderDomainJoinInfoCaws:appstream/ImageBuilderDomainJoinInfo:ImageBuilderDomainJoinInfo�Configuration block for the name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain. See below.
h
enableDefaultInternetAccessB
 CEnables or disables default internet access for the image builder.
G

iamRoleArnB" 3ARN of the IAM role to apply to the image builder.
F
imageArnB" 4ARN of the public, private, or shared image to use.
G
	imageNameB" 4Name of the image used to create the image builder.
K
instanceType" 7Instance type to use when launching the image builder.
X
nameB" JUnique name for the image builder.

The following arguments are optional:
�
tagsB2" �Map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
	vpcConfigcBa:_
]
	appstreamImageBuilderVpcConfig9aws:appstream/ImageBuilderVpcConfig:ImageBuilderVpcConfigPConfiguration block for the VPC configuration for the image builder. See below.
"�
accessEndpointstBr*p:n
l
	appstreamImageBuilderAccessEndpointCaws:appstream/ImageBuilderAccessEndpoint:ImageBuilderAccessEndpointUSet of interface VPC endpoint (interface endpoint) objects. Maximum of 4. See below.
"_
appstreamAgentVersion" BVersion of the AppStream 2.0 agent to use for this image builder.
"/
arn" $ARN of the appstream image builder.
"k
createdTime" XDate and time, in UTC and extended RFC 3339 format, when the image builder was created.
"+
description" Description to display.
"Q
displayName" >Human-readable friendly name for the AppStream image builder.
"�
domainJoinInfop:n
l
	appstreamImageBuilderDomainJoinInfoCaws:appstream/ImageBuilderDomainJoinInfo:ImageBuilderDomainJoinInfo�Configuration block for the name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain. See below.
"f
enableDefaultInternetAccess
 CEnables or disables default internet access for the image builder.
"E

iamRoleArn" 3ARN of the IAM role to apply to the image builder.
"D
imageArn" 4ARN of the public, private, or shared image to use.
"E
	imageName" 4Name of the image used to create the image builder.
"K
instanceType" 7Instance type to use when launching the image builder.
"V
name" JUnique name for the image builder.

The following arguments are optional:
"�
state" �State of the image builder. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/appstream2/latest/APIReference/API_ImageBuilder.html#AppStream2-Type-ImageBuilder-State).
"�
tagsB2" �Map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
	vpcConfiga:_
]
	appstreamImageBuilderVpcConfig9aws:appstream/ImageBuilderVpcConfig:ImageBuilderVpcConfigPConfiguration block for the VPC configuration for the image builder. See below.
*Ҁ
-
	appstreamStackaws:appstream/stack:Stack�ZProvides an AppStream stack.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appstream.Stack("example", {
    name: "stack name",
    description: "stack description",
    displayName: "stack display name",
    feedbackUrl: "http://your-domain/feedback",
    redirectUrl: "http://your-domain/redirect",
    storageConnectors: [{
        connectorType: "HOMEFOLDERS",
    }],
    userSettings: [
        {
            action: "AUTO_TIME_ZONE_REDIRECTION",
            permission: "DISABLED",
        },
        {
            action: "CLIPBOARD_COPY_FROM_LOCAL_DEVICE",
            permission: "ENABLED",
        },
        {
            action: "CLIPBOARD_COPY_TO_LOCAL_DEVICE",
            permission: "ENABLED",
        },
        {
            action: "DOMAIN_PASSWORD_SIGNIN",
            permission: "ENABLED",
        },
        {
            action: "DOMAIN_SMART_CARD_SIGNIN",
            permission: "DISABLED",
        },
        {
            action: "FILE_DOWNLOAD",
            permission: "ENABLED",
        },
        {
            action: "FILE_UPLOAD",
            permission: "ENABLED",
        },
        {
            action: "PRINTING_TO_LOCAL_DEVICE",
            permission: "ENABLED",
        },
    ],
    applicationSettings: {
        enabled: true,
        settingsGroup: "SettingsGroup",
    },
    tags: {
        TagName: "TagValue",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appstream.Stack("example",
    name="stack name",
    description="stack description",
    display_name="stack display name",
    feedback_url="http://your-domain/feedback",
    redirect_url="http://your-domain/redirect",
    storage_connectors=[{
        "connector_type": "HOMEFOLDERS",
    }],
    user_settings=[
        {
            "action": "AUTO_TIME_ZONE_REDIRECTION",
            "permission": "DISABLED",
        },
        {
            "action": "CLIPBOARD_COPY_FROM_LOCAL_DEVICE",
            "permission": "ENABLED",
        },
        {
            "action": "CLIPBOARD_COPY_TO_LOCAL_DEVICE",
            "permission": "ENABLED",
        },
        {
            "action": "DOMAIN_PASSWORD_SIGNIN",
            "permission": "ENABLED",
        },
        {
            "action": "DOMAIN_SMART_CARD_SIGNIN",
            "permission": "DISABLED",
        },
        {
            "action": "FILE_DOWNLOAD",
            "permission": "ENABLED",
        },
        {
            "action": "FILE_UPLOAD",
            "permission": "ENABLED",
        },
        {
            "action": "PRINTING_TO_LOCAL_DEVICE",
            "permission": "ENABLED",
        },
    ],
    application_settings={
        "enabled": True,
        "settings_group": "SettingsGroup",
    },
    tags={
        "TagName": "TagValue",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppStream.Stack("example", new()
    {
        Name = "stack name",
        Description = "stack description",
        DisplayName = "stack display name",
        FeedbackUrl = "http://your-domain/feedback",
        RedirectUrl = "http://your-domain/redirect",
        StorageConnectors = new[]
        {
            new Aws.AppStream.Inputs.StackStorageConnectorArgs
            {
                ConnectorType = "HOMEFOLDERS",
            },
        },
        UserSettings = new[]
        {
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "AUTO_TIME_ZONE_REDIRECTION",
                Permission = "DISABLED",
            },
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "CLIPBOARD_COPY_FROM_LOCAL_DEVICE",
                Permission = "ENABLED",
            },
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "CLIPBOARD_COPY_TO_LOCAL_DEVICE",
                Permission = "ENABLED",
            },
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "DOMAIN_PASSWORD_SIGNIN",
                Permission = "ENABLED",
            },
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "DOMAIN_SMART_CARD_SIGNIN",
                Permission = "DISABLED",
            },
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "FILE_DOWNLOAD",
                Permission = "ENABLED",
            },
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "FILE_UPLOAD",
                Permission = "ENABLED",
            },
            new Aws.AppStream.Inputs.StackUserSettingArgs
            {
                Action = "PRINTING_TO_LOCAL_DEVICE",
                Permission = "ENABLED",
            },
        },
        ApplicationSettings = new Aws.AppStream.Inputs.StackApplicationSettingsArgs
        {
            Enabled = true,
            SettingsGroup = "SettingsGroup",
        },
        Tags = 
        {
            { "TagName", "TagValue" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appstream"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appstream.NewStack(ctx, "example", &appstream.StackArgs{
			Name:        pulumi.String("stack name"),
			Description: pulumi.String("stack description"),
			DisplayName: pulumi.String("stack display name"),
			FeedbackUrl: pulumi.String("http://your-domain/feedback"),
			RedirectUrl: pulumi.String("http://your-domain/redirect"),
			StorageConnectors: appstream.StackStorageConnectorArray{
				&appstream.StackStorageConnectorArgs{
					ConnectorType: pulumi.String("HOMEFOLDERS"),
				},
			},
			UserSettings: appstream.StackUserSettingArray{
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("AUTO_TIME_ZONE_REDIRECTION"),
					Permission: pulumi.String("DISABLED"),
				},
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("CLIPBOARD_COPY_FROM_LOCAL_DEVICE"),
					Permission: pulumi.String("ENABLED"),
				},
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("CLIPBOARD_COPY_TO_LOCAL_DEVICE"),
					Permission: pulumi.String("ENABLED"),
				},
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("DOMAIN_PASSWORD_SIGNIN"),
					Permission: pulumi.String("ENABLED"),
				},
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("DOMAIN_SMART_CARD_SIGNIN"),
					Permission: pulumi.String("DISABLED"),
				},
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("FILE_DOWNLOAD"),
					Permission: pulumi.String("ENABLED"),
				},
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("FILE_UPLOAD"),
					Permission: pulumi.String("ENABLED"),
				},
				&appstream.StackUserSettingArgs{
					Action:     pulumi.String("PRINTING_TO_LOCAL_DEVICE"),
					Permission: pulumi.String("ENABLED"),
				},
			},
			ApplicationSettings: &appstream.StackApplicationSettingsArgs{
				Enabled:       pulumi.Bool(true),
				SettingsGroup: pulumi.String("SettingsGroup"),
			},
			Tags: pulumi.StringMap{
				"TagName": pulumi.String("TagValue"),
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
import com.pulumi.aws.appstream.Stack;
import com.pulumi.aws.appstream.StackArgs;
import com.pulumi.aws.appstream.inputs.StackStorageConnectorArgs;
import com.pulumi.aws.appstream.inputs.StackUserSettingArgs;
import com.pulumi.aws.appstream.inputs.StackApplicationSettingsArgs;
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
        var example = new Stack("example", StackArgs.builder()
            .name("stack name")
            .description("stack description")
            .displayName("stack display name")
            .feedbackUrl("http://your-domain/feedback")
            .redirectUrl("http://your-domain/redirect")
            .storageConnectors(StackStorageConnectorArgs.builder()
                .connectorType("HOMEFOLDERS")
                .build())
            .userSettings(            
                StackUserSettingArgs.builder()
                    .action("AUTO_TIME_ZONE_REDIRECTION")
                    .permission("DISABLED")
                    .build(),
                StackUserSettingArgs.builder()
                    .action("CLIPBOARD_COPY_FROM_LOCAL_DEVICE")
                    .permission("ENABLED")
                    .build(),
                StackUserSettingArgs.builder()
                    .action("CLIPBOARD_COPY_TO_LOCAL_DEVICE")
                    .permission("ENABLED")
                    .build(),
                StackUserSettingArgs.builder()
                    .action("DOMAIN_PASSWORD_SIGNIN")
                    .permission("ENABLED")
                    .build(),
                StackUserSettingArgs.builder()
                    .action("DOMAIN_SMART_CARD_SIGNIN")
                    .permission("DISABLED")
                    .build(),
                StackUserSettingArgs.builder()
                    .action("FILE_DOWNLOAD")
                    .permission("ENABLED")
                    .build(),
                StackUserSettingArgs.builder()
                    .action("FILE_UPLOAD")
                    .permission("ENABLED")
                    .build(),
                StackUserSettingArgs.builder()
                    .action("PRINTING_TO_LOCAL_DEVICE")
                    .permission("ENABLED")
                    .build())
            .applicationSettings(StackApplicationSettingsArgs.builder()
                .enabled(true)
                .settingsGroup("SettingsGroup")
                .build())
            .tags(Map.of("TagName", "TagValue"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appstream:Stack
    properties:
      name: stack name
      description: stack description
      displayName: stack display name
      feedbackUrl: http://your-domain/feedback
      redirectUrl: http://your-domain/redirect
      storageConnectors:
        - connectorType: HOMEFOLDERS
      userSettings:
        - action: AUTO_TIME_ZONE_REDIRECTION
          permission: DISABLED
        - action: CLIPBOARD_COPY_FROM_LOCAL_DEVICE
          permission: ENABLED
        - action: CLIPBOARD_COPY_TO_LOCAL_DEVICE
          permission: ENABLED
        - action: DOMAIN_PASSWORD_SIGNIN
          permission: ENABLED
        - action: DOMAIN_SMART_CARD_SIGNIN
          permission: DISABLED
        - action: FILE_DOWNLOAD
          permission: ENABLED
        - action: FILE_UPLOAD
          permission: ENABLED
        - action: PRINTING_TO_LOCAL_DEVICE
          permission: ENABLED
      applicationSettings:
        enabled: true
        settingsGroup: SettingsGroup
      tags:
        TagName: TagValue
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appstream_stack` using the id. For example:

```sh
$ pulumi import aws:appstream/stack:Stack example stackID
```
�
accessEndpoints_B]*[:Y
W
	appstreamStackAccessEndpoint5aws:appstream/StackAccessEndpoint:StackAccessEndpoint�Set of configuration blocks defining the interface VPC endpoints. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.
See `access_endpoints` below.
�
applicationSettingslBj:h
f
	appstreamStackApplicationSettings?aws:appstream/StackApplicationSettings:StackApplicationSettingsQSettings for application settings persistence.
See `application_settings` below.
:
descriptionB" %Description for the AppStream stack.
,
displayNameB" Stack name to display.
�
embedHostDomainsB*" �Domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.
�
feedbackUrlB" �URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed. .
Z
nameB" LUnique name for the AppStream stack.

The following arguments are optional:
Z
redirectUrlB" EURL that users are redirected to after their streaming session ends.
�
storageConnectorseBc*a:_
]
	appstreamStackStorageConnector9aws:appstream/StackStorageConnector:StackStorageConnectorZConfiguration block for the storage connectors to enable.
See `storage_connectors` below.
�
streamingExperienceSettings�B�:�
~
	appstream StackStreamingExperienceSettingsOaws:appstream/StackStreamingExperienceSettings:StackStreamingExperienceSettings�The streaming protocol you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
See `streaming_experience_settings` below.
�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
userSettingsVBT*R:P
N
	appstreamStackUserSetting/aws:appstream/StackUserSetting:StackUserSetting�Configuration block for the actions that are enabled or disabled for users during their streaming sessions. If not provided, these settings are configured automatically by AWS. If provided, the configuration should include a block for each configurable action.
See `user_settings` below.
"�
accessEndpoints]*[:Y
W
	appstreamStackAccessEndpoint5aws:appstream/StackAccessEndpoint:StackAccessEndpoint�Set of configuration blocks defining the interface VPC endpoints. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.
See `access_endpoints` below.
"�
applicationSettingsj:h
f
	appstreamStackApplicationSettings?aws:appstream/StackApplicationSettings:StackApplicationSettingsQSettings for application settings persistence.
See `application_settings` below.
"'
arn" ARN of the appstream stack.
"c
createdTime" PDate and time, in UTC and extended RFC 3339 format, when the stack was created.
":
descriptionB" %Description for the AppStream stack.
",
displayNameB" Stack name to display.
"�
embedHostDomains*" �Domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.
"�
feedbackUrl" �URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed. .
"X
name" LUnique name for the AppStream stack.

The following arguments are optional:
"X
redirectUrl" EURL that users are redirected to after their streaming session ends.
"�
storageConnectorsc*a:_
]
	appstreamStackStorageConnector9aws:appstream/StackStorageConnector:StackStorageConnectorZConfiguration block for the storage connectors to enable.
See `storage_connectors` below.
"�
streamingExperienceSettings�:�
~
	appstream StackStreamingExperienceSettingsOaws:appstream/StackStreamingExperienceSettings:StackStreamingExperienceSettings�The streaming protocol you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
See `streaming_experience_settings` below.
"�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "�
userSettingsT*R:P
N
	appstreamStackUserSetting/aws:appstream/StackUserSetting:StackUserSetting�Configuration block for the actions that are enabled or disabled for users during their streaming sessions. If not provided, these settings are configured automatically by AWS. If provided, the configuration should include a block for each configurable action.
See `user_settings` below.
*�
*
	appstreamUseraws:appstream/user:User�Provides an AppStream user.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appstream.User("example", {
    authenticationType: "USERPOOL",
    userName: "EMAIL",
    firstName: "FIRST NAME",
    lastName: "LAST NAME",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appstream.User("example",
    authentication_type="USERPOOL",
    user_name="EMAIL",
    first_name="FIRST NAME",
    last_name="LAST NAME")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppStream.User("example", new()
    {
        AuthenticationType = "USERPOOL",
        UserName = "EMAIL",
        FirstName = "FIRST NAME",
        LastName = "LAST NAME",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appstream"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appstream.NewUser(ctx, "example", &appstream.UserArgs{
			AuthenticationType: pulumi.String("USERPOOL"),
			UserName:           pulumi.String("EMAIL"),
			FirstName:          pulumi.String("FIRST NAME"),
			LastName:           pulumi.String("LAST NAME"),
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
import com.pulumi.aws.appstream.User;
import com.pulumi.aws.appstream.UserArgs;
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
            .authenticationType("USERPOOL")
            .userName("EMAIL")
            .firstName("FIRST NAME")
            .lastName("LAST NAME")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appstream:User
    properties:
      authenticationType: USERPOOL
      userName: EMAIL
      firstName: FIRST NAME
      lastName: LAST NAME
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appstream_user` using the `user_name` and `authentication_type` separated by a slash (`/`). For example:

```sh
$ pulumi import aws:appstream/user:User example UserName/AuthenticationType
```

authenticationType" eAuthentication type for the user. You must specify USERPOOL. Valid values: `API`, `SAML`, `USERPOOL`
?
enabledB
 .Whether the user in the user pool is enabled.
;
	firstNameB" (First name, or given name, of the user.
6
lastNameB" $Last name, or surname, of the user.
;
sendEmailNotificationB
 Send an email notification.
R
userName" BEmail address of the user.

The following arguments are optional:
"&
arn" ARN of the appstream user.
"
authenticationType" eAuthentication type for the user. You must specify USERPOOL. Valid values: `API`, `SAML`, `USERPOOL`
"b
createdTime" ODate and time, in UTC and extended RFC 3339 format, when the user was created.
"?
enabledB
 .Whether the user in the user pool is enabled.
";
	firstNameB" (First name, or given name, of the user.
"6
lastNameB" $Last name, or surname, of the user.
";
sendEmailNotificationB
 Send an email notification.
"R
userName" BEmail address of the user.

The following arguments are optional:
*�)
Z
	appstreamUserStackAssociation7aws:appstream/userStackAssociation:UserStackAssociation�#Manages an AppStream User Stack association.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.appstream.Stack("test", {name: "STACK NAME"});
const testUser = new aws.appstream.User("test", {
    authenticationType: "USERPOOL",
    userName: "EMAIL",
});
const testUserStackAssociation = new aws.appstream.UserStackAssociation("test", {
    authenticationType: testUser.authenticationType,
    stackName: test.name,
    userName: testUser.userName,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.appstream.Stack("test", name="STACK NAME")
test_user = aws.appstream.User("test",
    authentication_type="USERPOOL",
    user_name="EMAIL")
test_user_stack_association = aws.appstream.UserStackAssociation("test",
    authentication_type=test_user.authentication_type,
    stack_name=test.name,
    user_name=test_user.user_name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.AppStream.Stack("test", new()
    {
        Name = "STACK NAME",
    });

    var testUser = new Aws.AppStream.User("test", new()
    {
        AuthenticationType = "USERPOOL",
        UserName = "EMAIL",
    });

    var testUserStackAssociation = new Aws.AppStream.UserStackAssociation("test", new()
    {
        AuthenticationType = testUser.AuthenticationType,
        StackName = test.Name,
        UserName = testUser.UserName,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appstream"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := appstream.NewStack(ctx, "test", &appstream.StackArgs{
			Name: pulumi.String("STACK NAME"),
		})
		if err != nil {
			return err
		}
		testUser, err := appstream.NewUser(ctx, "test", &appstream.UserArgs{
			AuthenticationType: pulumi.String("USERPOOL"),
			UserName:           pulumi.String("EMAIL"),
		})
		if err != nil {
			return err
		}
		_, err = appstream.NewUserStackAssociation(ctx, "test", &appstream.UserStackAssociationArgs{
			AuthenticationType: testUser.AuthenticationType,
			StackName:          test.Name,
			UserName:           testUser.UserName,
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
import com.pulumi.aws.appstream.Stack;
import com.pulumi.aws.appstream.StackArgs;
import com.pulumi.aws.appstream.User;
import com.pulumi.aws.appstream.UserArgs;
import com.pulumi.aws.appstream.UserStackAssociation;
import com.pulumi.aws.appstream.UserStackAssociationArgs;
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
        var test = new Stack("test", StackArgs.builder()
            .name("STACK NAME")
            .build());

        var testUser = new User("testUser", UserArgs.builder()
            .authenticationType("USERPOOL")
            .userName("EMAIL")
            .build());

        var testUserStackAssociation = new UserStackAssociation("testUserStackAssociation", UserStackAssociationArgs.builder()
            .authenticationType(testUser.authenticationType())
            .stackName(test.name())
            .userName(testUser.userName())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:appstream:Stack
    properties:
      name: STACK NAME
  testUser:
    type: aws:appstream:User
    name: test
    properties:
      authenticationType: USERPOOL
      userName: EMAIL
  testUserStackAssociation:
    type: aws:appstream:UserStackAssociation
    name: test
    properties:
      authenticationType: ${testUser.authenticationType}
      stackName: ${test.name}
      userName: ${testUser.userName}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppStream User Stack Association using the `user_name`, `authentication_type`, and `stack_name`, separated by a slash (`/`). For example:

```sh
$ pulumi import aws:appstream/userStackAssociation:UserStackAssociation example userName/auhtenticationType/stackName
```
<
authenticationType" "Authentication type for the user.
u
sendEmailNotificationB
 VWhether a welcome email is sent to a user after the user is created in the user pool.
E
	stackName" 4Name of the stack that is associated with the user.
s
userName" cEmail address of the user who is associated with the stack.

The following arguments are optional:
"<
authenticationType" "Authentication type for the user.
"u
sendEmailNotificationB
 VWhether a welcome email is sent to a user after the user is created in the user pool.
"E
	stackName" 4Name of the stack that is associated with the user.
"s
userName" cEmail address of the user who is associated with the stack.

The following arguments are optional:
*�&
2
appsyncApiCacheaws:appsync/apiCache:ApiCache�Provides an AppSync API Cache.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appsync.GraphQLApi("example", {
    authenticationType: "API_KEY",
    name: "example",
});
const exampleApiCache = new aws.appsync.ApiCache("example", {
    apiId: example.id,
    apiCachingBehavior: "FULL_REQUEST_CACHING",
    type: "LARGE",
    ttl: 900,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appsync.GraphQLApi("example",
    authentication_type="API_KEY",
    name="example")
example_api_cache = aws.appsync.ApiCache("example",
    api_id=example.id,
    api_caching_behavior="FULL_REQUEST_CACHING",
    type="LARGE",
    ttl=900)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppSync.GraphQLApi("example", new()
    {
        AuthenticationType = "API_KEY",
        Name = "example",
    });

    var exampleApiCache = new Aws.AppSync.ApiCache("example", new()
    {
        ApiId = example.Id,
        ApiCachingBehavior = "FULL_REQUEST_CACHING",
        Type = "LARGE",
        Ttl = 900,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := appsync.NewGraphQLApi(ctx, "example", &appsync.GraphQLApiArgs{
			AuthenticationType: pulumi.String("API_KEY"),
			Name:               pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = appsync.NewApiCache(ctx, "example", &appsync.ApiCacheArgs{
			ApiId:              example.ID(),
			ApiCachingBehavior: pulumi.String("FULL_REQUEST_CACHING"),
			Type:               pulumi.String("LARGE"),
			Ttl:                pulumi.Int(900),
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
import com.pulumi.aws.appsync.GraphQLApi;
import com.pulumi.aws.appsync.GraphQLApiArgs;
import com.pulumi.aws.appsync.ApiCache;
import com.pulumi.aws.appsync.ApiCacheArgs;
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
        var example = new GraphQLApi("example", GraphQLApiArgs.builder()
            .authenticationType("API_KEY")
            .name("example")
            .build());

        var exampleApiCache = new ApiCache("exampleApiCache", ApiCacheArgs.builder()
            .apiId(example.id())
            .apiCachingBehavior("FULL_REQUEST_CACHING")
            .type("LARGE")
            .ttl(900)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:GraphQLApi
    properties:
      authenticationType: API_KEY
      name: example
  exampleApiCache:
    type: aws:appsync:ApiCache
    name: example
    properties:
      apiId: ${example.id}
      apiCachingBehavior: FULL_REQUEST_CACHING
      type: LARGE
      ttl: 900
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appsync_api_cache` using the AppSync API ID. For example:

```sh
$ pulumi import aws:appsync/apiCache:ApiCache example xxxxx
```
p
apiCachingBehavior" VCaching behavior. Valid values are `FULL_REQUEST_CACHING` and `PER_RESOLVER_CACHING`.

apiId" GraphQL API ID.
s
atRestEncryptionEnabledB
 RAt-rest encryption flag for cache. You cannot update this setting after creation.
�
transitEncryptionEnabledB
 aTransit encryption flag when connecting to cache. You cannot update this setting after creation.
-
ttl "TTL in seconds for cache entries.
�
type" �Cache instance type. Valid values are `SMALL`, `MEDIUM`, `LARGE`, `XLARGE`, `LARGE_2X`, `LARGE_4X`, `LARGE_8X`, `LARGE_12X`, `T2_SMALL`, `T2_MEDIUM`, `R4_LARGE`, `R4_XLARGE`, `R4_2XLARGE`, `R4_4XLARGE`, `R4_8XLARGE`.
"p
apiCachingBehavior" VCaching behavior. Valid values are `FULL_REQUEST_CACHING` and `PER_RESOLVER_CACHING`.
"
apiId" GraphQL API ID.
"s
atRestEncryptionEnabledB
 RAt-rest encryption flag for cache. You cannot update this setting after creation.
"�
transitEncryptionEnabledB
 aTransit encryption flag when connecting to cache. You cannot update this setting after creation.
"-
ttl "TTL in seconds for cache entries.
"�
type" �Cache instance type. Valid values are `SMALL`, `MEDIUM`, `LARGE`, `XLARGE`, `LARGE_2X`, `LARGE_4X`, `LARGE_8X`, `LARGE_12X`, `T2_SMALL`, `T2_MEDIUM`, `R4_LARGE`, `R4_XLARGE`, `R4_2XLARGE`, `R4_4XLARGE`, `R4_8XLARGE`.
*�
,
appsyncApiKeyaws:appsync/apiKey:ApiKey�Provides an AppSync API Key.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appsync.GraphQLApi("example", {
    authenticationType: "API_KEY",
    name: "example",
});
const exampleApiKey = new aws.appsync.ApiKey("example", {
    apiId: example.id,
    expires: "2018-05-03T04:00:00Z",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appsync.GraphQLApi("example",
    authentication_type="API_KEY",
    name="example")
example_api_key = aws.appsync.ApiKey("example",
    api_id=example.id,
    expires="2018-05-03T04:00:00Z")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppSync.GraphQLApi("example", new()
    {
        AuthenticationType = "API_KEY",
        Name = "example",
    });

    var exampleApiKey = new Aws.AppSync.ApiKey("example", new()
    {
        ApiId = example.Id,
        Expires = "2018-05-03T04:00:00Z",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := appsync.NewGraphQLApi(ctx, "example", &appsync.GraphQLApiArgs{
			AuthenticationType: pulumi.String("API_KEY"),
			Name:               pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = appsync.NewApiKey(ctx, "example", &appsync.ApiKeyArgs{
			ApiId:   example.ID(),
			Expires: pulumi.String("2018-05-03T04:00:00Z"),
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
import com.pulumi.aws.appsync.GraphQLApi;
import com.pulumi.aws.appsync.GraphQLApiArgs;
import com.pulumi.aws.appsync.ApiKey;
import com.pulumi.aws.appsync.ApiKeyArgs;
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
        var example = new GraphQLApi("example", GraphQLApiArgs.builder()
            .authenticationType("API_KEY")
            .name("example")
            .build());

        var exampleApiKey = new ApiKey("exampleApiKey", ApiKeyArgs.builder()
            .apiId(example.id())
            .expires("2018-05-03T04:00:00Z")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:GraphQLApi
    properties:
      authenticationType: API_KEY
      name: example
  exampleApiKey:
    type: aws:appsync:ApiKey
    name: example
    properties:
      apiId: ${example.id}
      expires: 2018-05-03T04:00:00Z
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appsync_api_key` using the AppSync API ID and key separated by `:`. For example:

```sh
$ pulumi import aws:appsync/apiKey:ApiKey example xxxxx:yyyyy
```
.
apiId" !ID of the associated AppSync API
K
descriptionB" 6API key description. Defaults to "Managed by Pulumi".
�
expiresB" �RFC3339 string representation of the expiry date. Rounded down to nearest hour. By default, it is 7 days from the date of creation.
".
apiId" !ID of the associated AppSync API
"
apiKeyId" "I
description" 6API key description. Defaults to "Managed by Pulumi".
"�
expiresB" �RFC3339 string representation of the expiry date. Rounded down to nearest hour. By default, it is 7 days from the date of creation.
"
key" API key
*��
8
appsync
DataSource!aws:appsync/dataSource:DataSource�iProvides an AppSync Data Source.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleTable = new aws.dynamodb.Table("example", {
    name: "example",
    readCapacity: 1,
    writeCapacity: 1,
    hashKey: "UserId",
    attributes: [{
        name: "UserId",
        type: "S",
    }],
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["appsync.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const exampleRole = new aws.iam.Role("example", {
    name: "example",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const example = aws.iam.getPolicyDocumentOutput({
    statements: [{
        effect: "Allow",
        actions: ["dynamodb:*"],
        resources: [exampleTable.arn],
    }],
});
const exampleRolePolicy = new aws.iam.RolePolicy("example", {
    name: "example",
    role: exampleRole.id,
    policy: example.apply(example => example.json),
});
const exampleGraphQLApi = new aws.appsync.GraphQLApi("example", {
    authenticationType: "API_KEY",
    name: "my_appsync_example",
});
const exampleDataSource = new aws.appsync.DataSource("example", {
    apiId: exampleGraphQLApi.id,
    name: "my_appsync_example",
    serviceRoleArn: exampleRole.arn,
    type: "AMAZON_DYNAMODB",
    dynamodbConfig: {
        tableName: exampleTable.name,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example_table = aws.dynamodb.Table("example",
    name="example",
    read_capacity=1,
    write_capacity=1,
    hash_key="UserId",
    attributes=[{
        "name": "UserId",
        "type": "S",
    }])
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["appsync.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
example_role = aws.iam.Role("example",
    name="example",
    assume_role_policy=assume_role.json)
example = aws.iam.get_policy_document_output(statements=[{
    "effect": "Allow",
    "actions": ["dynamodb:*"],
    "resources": [example_table.arn],
}])
example_role_policy = aws.iam.RolePolicy("example",
    name="example",
    role=example_role.id,
    policy=example.json)
example_graph_ql_api = aws.appsync.GraphQLApi("example",
    authentication_type="API_KEY",
    name="my_appsync_example")
example_data_source = aws.appsync.DataSource("example",
    api_id=example_graph_ql_api.id,
    name="my_appsync_example",
    service_role_arn=example_role.arn,
    type="AMAZON_DYNAMODB",
    dynamodb_config={
        "table_name": example_table.name,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleTable = new Aws.DynamoDB.Table("example", new()
    {
        Name = "example",
        ReadCapacity = 1,
        WriteCapacity = 1,
        HashKey = "UserId",
        Attributes = new[]
        {
            new Aws.DynamoDB.Inputs.TableAttributeArgs
            {
                Name = "UserId",
                Type = "S",
            },
        },
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
                            "appsync.amazonaws.com",
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

    var exampleRole = new Aws.Iam.Role("example", new()
    {
        Name = "example",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "dynamodb:*",
                },
                Resources = new[]
                {
                    exampleTable.Arn,
                },
            },
        },
    });

    var exampleRolePolicy = new Aws.Iam.RolePolicy("example", new()
    {
        Name = "example",
        Role = exampleRole.Id,
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleGraphQLApi = new Aws.AppSync.GraphQLApi("example", new()
    {
        AuthenticationType = "API_KEY",
        Name = "my_appsync_example",
    });

    var exampleDataSource = new Aws.AppSync.DataSource("example", new()
    {
        ApiId = exampleGraphQLApi.Id,
        Name = "my_appsync_example",
        ServiceRoleArn = exampleRole.Arn,
        Type = "AMAZON_DYNAMODB",
        DynamodbConfig = new Aws.AppSync.Inputs.DataSourceDynamodbConfigArgs
        {
            TableName = exampleTable.Name,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/dynamodb"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleTable, err := dynamodb.NewTable(ctx, "example", &dynamodb.TableArgs{
			Name:          pulumi.String("example"),
			ReadCapacity:  pulumi.Int(1),
			WriteCapacity: pulumi.Int(1),
			HashKey:       pulumi.String("UserId"),
			Attributes: dynamodb.TableAttributeArray{
				&dynamodb.TableAttributeArgs{
					Name: pulumi.String("UserId"),
					Type: pulumi.String("S"),
				},
			},
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
								"appsync.amazonaws.com",
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
		exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			Name:             pulumi.String("example"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		example := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Effect: pulumi.String("Allow"),
					Actions: pulumi.StringArray{
						pulumi.String("dynamodb:*"),
					},
					Resources: pulumi.StringArray{
						exampleTable.Arn,
					},
				},
			},
		}, nil)
		_, err = iam.NewRolePolicy(ctx, "example", &iam.RolePolicyArgs{
			Name: pulumi.String("example"),
			Role: exampleRole.ID(),
			Policy: pulumi.String(example.ApplyT(func(example iam.GetPolicyDocumentResult) (*string, error) {
				return &example.Json, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		exampleGraphQLApi, err := appsync.NewGraphQLApi(ctx, "example", &appsync.GraphQLApiArgs{
			AuthenticationType: pulumi.String("API_KEY"),
			Name:               pulumi.String("my_appsync_example"),
		})
		if err != nil {
			return err
		}
		_, err = appsync.NewDataSource(ctx, "example", &appsync.DataSourceArgs{
			ApiId:          exampleGraphQLApi.ID(),
			Name:           pulumi.String("my_appsync_example"),
			ServiceRoleArn: exampleRole.Arn,
			Type:           pulumi.String("AMAZON_DYNAMODB"),
			DynamodbConfig: &appsync.DataSourceDynamodbConfigArgs{
				TableName: exampleTable.Name,
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
import com.pulumi.aws.dynamodb.Table;
import com.pulumi.aws.dynamodb.TableArgs;
import com.pulumi.aws.dynamodb.inputs.TableAttributeArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import com.pulumi.aws.appsync.GraphQLApi;
import com.pulumi.aws.appsync.GraphQLApiArgs;
import com.pulumi.aws.appsync.DataSource;
import com.pulumi.aws.appsync.DataSourceArgs;
import com.pulumi.aws.appsync.inputs.DataSourceDynamodbConfigArgs;
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
        var exampleTable = new Table("exampleTable", TableArgs.builder()
            .name("example")
            .readCapacity(1)
            .writeCapacity(1)
            .hashKey("UserId")
            .attributes(TableAttributeArgs.builder()
                .name("UserId")
                .type("S")
                .build())
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("appsync.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("example")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions("dynamodb:*")
                .resources(exampleTable.arn())
                .build())
            .build());

        var exampleRolePolicy = new RolePolicy("exampleRolePolicy", RolePolicyArgs.builder()
            .name("example")
            .role(exampleRole.id())
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(example -> example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

        var exampleGraphQLApi = new GraphQLApi("exampleGraphQLApi", GraphQLApiArgs.builder()
            .authenticationType("API_KEY")
            .name("my_appsync_example")
            .build());

        var exampleDataSource = new DataSource("exampleDataSource", DataSourceArgs.builder()
            .apiId(exampleGraphQLApi.id())
            .name("my_appsync_example")
            .serviceRoleArn(exampleRole.arn())
            .type("AMAZON_DYNAMODB")
            .dynamodbConfig(DataSourceDynamodbConfigArgs.builder()
                .tableName(exampleTable.name())
                .build())
            .build());

    }
}
```
```yaml
resources:
  exampleTable:
    type: aws:dynamodb:Table
    name: example
    properties:
      name: example
      readCapacity: 1
      writeCapacity: 1
      hashKey: UserId
      attributes:
        - name: UserId
          type: S
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: example
      assumeRolePolicy: ${assumeRole.json}
  exampleRolePolicy:
    type: aws:iam:RolePolicy
    name: example
    properties:
      name: example
      role: ${exampleRole.id}
      policy: ${example.json}
  exampleGraphQLApi:
    type: aws:appsync:GraphQLApi
    name: example
    properties:
      authenticationType: API_KEY
      name: my_appsync_example
  exampleDataSource:
    type: aws:appsync:DataSource
    name: example
    properties:
      apiId: ${exampleGraphQLApi.id}
      name: my_appsync_example
      serviceRoleArn: ${exampleRole.arn}
      type: AMAZON_DYNAMODB
      dynamodbConfig:
        tableName: ${exampleTable.name}
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
                  - appsync.amazonaws.com
            actions:
              - sts:AssumeRole
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - dynamodb:*
            resources:
              - ${exampleTable.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appsync_datasource` using the `api_id`, a hyphen, and `name`. For example:

```sh
$ pulumi import aws:appsync/dataSource:DataSource example abcdef123456-example
```
=
apiId" 0API ID for the GraphQL API for the data source.
5
descriptionB"  Description of the data source.
�
dynamodbConfighBf:d
b
appsyncDataSourceDynamodbConfig=aws:appsync/DataSourceDynamodbConfig:DataSourceDynamodbConfig<DynamoDB settings. See `dynamodb_config` Block for details.
�
elasticsearchConfigwBu:s
q
appsyncDataSourceElasticsearchConfigGaws:appsync/DataSourceElasticsearchConfig:DataSourceElasticsearchConfigMAmazon Elasticsearch settings. See `elasticsearch_config` Block for details.
�
eventBridgeConfigqBo:m
k
appsyncDataSourceEventBridgeConfigCaws:appsync/DataSourceEventBridgeConfig:DataSourceEventBridgeConfigGAWS EventBridge settings. See `event_bridge_config` Block for details.
�

httpConfig\BZ:X
V
appsyncDataSourceHttpConfig5aws:appsync/DataSourceHttpConfig:DataSourceHttpConfig4HTTP settings. See `http_config` Block for details.
�
lambdaConfigbB`:^
\
appsyncDataSourceLambdaConfig9aws:appsync/DataSourceLambdaConfig:DataSourceLambdaConfig<AWS Lambda settings. See `lambda_config` Block for details.
6
nameB" (User-supplied name for the data source.
�
opensearchserviceConfig�B�:
}
appsync!DataSourceOpensearchserviceConfigOaws:appsync/DataSourceOpensearchserviceConfig:DataSourceOpensearchserviceConfigVAmazon OpenSearch Service settings. See `opensearchservice_config` Block for details.
�
relationalDatabaseConfig�B�:�
�
appsync"DataSourceRelationalDatabaseConfigQaws:appsync/DataSourceRelationalDatabaseConfig:DataSourceRelationalDatabaseConfigFAWS RDS settings. See `relational_database_config` Block for details.
�
serviceRoleArnB" �IAM service role ARN for the data source. Required if `type` is specified as `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `AMAZON_EVENTBRIDGE`, or `AMAZON_OPENSEARCH_SERVICE`.
�
type" �Type of the Data Source. Valid values: `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `HTTP`, `NONE`, `RELATIONAL_DATABASE`, `AMAZON_EVENTBRIDGE`, `AMAZON_OPENSEARCH_SERVICE`.
"=
apiId" 0API ID for the GraphQL API for the data source.
"
arn" ARN
"5
descriptionB"  Description of the data source.
"�
dynamodbConfighBf:d
b
appsyncDataSourceDynamodbConfig=aws:appsync/DataSourceDynamodbConfig:DataSourceDynamodbConfig<DynamoDB settings. See `dynamodb_config` Block for details.
"�
elasticsearchConfigwBu:s
q
appsyncDataSourceElasticsearchConfigGaws:appsync/DataSourceElasticsearchConfig:DataSourceElasticsearchConfigMAmazon Elasticsearch settings. See `elasticsearch_config` Block for details.
"�
eventBridgeConfigqBo:m
k
appsyncDataSourceEventBridgeConfigCaws:appsync/DataSourceEventBridgeConfig:DataSourceEventBridgeConfigGAWS EventBridge settings. See `event_bridge_config` Block for details.
"�

httpConfig\BZ:X
V
appsyncDataSourceHttpConfig5aws:appsync/DataSourceHttpConfig:DataSourceHttpConfig4HTTP settings. See `http_config` Block for details.
"�
lambdaConfigbB`:^
\
appsyncDataSourceLambdaConfig9aws:appsync/DataSourceLambdaConfig:DataSourceLambdaConfig<AWS Lambda settings. See `lambda_config` Block for details.
"4
name" (User-supplied name for the data source.
"�
opensearchserviceConfig�B�:
}
appsync!DataSourceOpensearchserviceConfigOaws:appsync/DataSourceOpensearchserviceConfig:DataSourceOpensearchserviceConfigVAmazon OpenSearch Service settings. See `opensearchservice_config` Block for details.
"�
relationalDatabaseConfig�B�:�
�
appsync"DataSourceRelationalDatabaseConfigQaws:appsync/DataSourceRelationalDatabaseConfig:DataSourceRelationalDatabaseConfigFAWS RDS settings. See `relational_database_config` Block for details.
"�
serviceRoleArnB" �IAM service role ARN for the data source. Required if `type` is specified as `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `AMAZON_EVENTBRIDGE`, or `AMAZON_OPENSEARCH_SERVICE`.
"�
type" �Type of the Data Source. Valid values: `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `HTTP`, `NONE`, `RELATIONAL_DATABASE`, `AMAZON_EVENTBRIDGE`, `AMAZON_OPENSEARCH_SERVICE`.
*�
8
appsync
DomainName!aws:appsync/domainName:DomainName�Provides an AppSync Domain Name.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appsync.DomainName("example", {
    domainName: "api.example.com",
    certificateArn: exampleAwsAcmCertificate.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appsync.DomainName("example",
    domain_name="api.example.com",
    certificate_arn=example_aws_acm_certificate["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppSync.DomainName("example", new()
    {
        Name = "api.example.com",
        CertificateArn = exampleAwsAcmCertificate.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appsync.NewDomainName(ctx, "example", &appsync.DomainNameArgs{
			DomainName:     pulumi.String("api.example.com"),
			CertificateArn: pulumi.Any(exampleAwsAcmCertificate.Arn),
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
import com.pulumi.aws.appsync.DomainName;
import com.pulumi.aws.appsync.DomainNameArgs;
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
        var example = new DomainName("example", DomainNameArgs.builder()
            .domainName("api.example.com")
            .certificateArn(exampleAwsAcmCertificate.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:DomainName
    properties:
      domainName: api.example.com
      certificateArn: ${exampleAwsAcmCertificate.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appsync_domain_name` using the AppSync domain name. For example:

```sh
$ pulumi import aws:appsync/domainName:DomainName example example.com
```
�
certificateArn" �ARN of the certificate. This can be an Certificate Manager (ACM) certificate or an Identity and Access Management (IAM) server certificate. The certifiacte must reside in us-east-1.
7
descriptionB" "A description of the Domain Name.


domainName" Domain name.
"<
appsyncDomainName" #Domain name that AppSync provides.
"�
certificateArn" �ARN of the certificate. This can be an Certificate Manager (ACM) certificate or an Identity and Access Management (IAM) server certificate. The certifiacte must reside in us-east-1.
"7
descriptionB" "A description of the Domain Name.
"

domainName" Domain name.
"<
hostedZoneId" (ID of your Amazon Route 53 hosted zone.
*�
b
appsyncDomainNameApiAssociation=aws:appsync/domainNameApiAssociation:DomainNameApiAssociation�Provides an AppSync API Association.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appsync.DomainNameApiAssociation("example", {
    apiId: exampleAwsAppsyncGraphqlApi.id,
    domainName: exampleAwsAppsyncDomainName.domainName,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appsync.DomainNameApiAssociation("example",
    api_id=example_aws_appsync_graphql_api["id"],
    domain_name=example_aws_appsync_domain_name["domainName"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppSync.DomainNameApiAssociation("example", new()
    {
        ApiId = exampleAwsAppsyncGraphqlApi.Id,
        DomainName = exampleAwsAppsyncDomainName.DomainName,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appsync.NewDomainNameApiAssociation(ctx, "example", &appsync.DomainNameApiAssociationArgs{
			ApiId:      pulumi.Any(exampleAwsAppsyncGraphqlApi.Id),
			DomainName: pulumi.Any(exampleAwsAppsyncDomainName.DomainName),
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
import com.pulumi.aws.appsync.DomainNameApiAssociation;
import com.pulumi.aws.appsync.DomainNameApiAssociationArgs;
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
        var example = new DomainNameApiAssociation("example", DomainNameApiAssociationArgs.builder()
            .apiId(exampleAwsAppsyncGraphqlApi.id())
            .domainName(exampleAwsAppsyncDomainName.domainName())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:DomainNameApiAssociation
    properties:
      apiId: ${exampleAwsAppsyncGraphqlApi.id}
      domainName: ${exampleAwsAppsyncDomainName.domainName}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appsync_domain_name_api_association` using the AppSync domain name. For example:

```sh
$ pulumi import aws:appsync/domainNameApiAssociation:DomainNameApiAssociation example example.com
```

apiId" API ID.
'

domainName" Appsync domain name.
"
apiId" API ID.
"'

domainName" Appsync domain name.
*�{
2
appsyncFunctionaws:appsync/function:Function�dProvides an AppSync Function.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appsync.GraphQLApi("example", {
    authenticationType: "API_KEY",
    name: "example",
    schema: `type Mutation {
  putPost(id: ID!, title: String!): Post
}

type Post {
  id: ID!
  title: String!
}

type Query {
  singlePost(id: ID!): Post
}

schema {
  query: Query
  mutation: Mutation
}
`,
});
const exampleDataSource = new aws.appsync.DataSource("example", {
    apiId: example.id,
    name: "example",
    type: "HTTP",
    httpConfig: {
        endpoint: "http://example.com",
    },
});
const exampleFunction = new aws.appsync.Function("example", {
    apiId: example.id,
    dataSource: exampleDataSource.name,
    name: "example",
    requestMappingTemplate: `{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": utils.http.copyheaders(ctx.request.headers)
    }
}
`,
    responseMappingTemplate: `#if(ctx.result.statusCode == 200)
    ctx.result.body
#else
    utils.appendError(ctx.result.body, ctx.result.statusCode)
#end
`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appsync.GraphQLApi("example",
    authentication_type="API_KEY",
    name="example",
    schema="""type Mutation {
  putPost(id: ID!, title: String!): Post
}

type Post {
  id: ID!
  title: String!
}

type Query {
  singlePost(id: ID!): Post
}

schema {
  query: Query
  mutation: Mutation
}
""")
example_data_source = aws.appsync.DataSource("example",
    api_id=example.id,
    name="example",
    type="HTTP",
    http_config={
        "endpoint": "http://example.com",
    })
example_function = aws.appsync.Function("example",
    api_id=example.id,
    data_source=example_data_source.name,
    name="example",
    request_mapping_template="""{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": $utils.http.copyheaders($ctx.request.headers)
    }
}
""",
    response_mapping_template="""#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppSync.GraphQLApi("example", new()
    {
        AuthenticationType = "API_KEY",
        Name = "example",
        Schema = @"type Mutation {
  putPost(id: ID!, title: String!): Post
}

type Post {
  id: ID!
  title: String!
}

type Query {
  singlePost(id: ID!): Post
}

schema {
  query: Query
  mutation: Mutation
}
",
    });

    var exampleDataSource = new Aws.AppSync.DataSource("example", new()
    {
        ApiId = example.Id,
        Name = "example",
        Type = "HTTP",
        HttpConfig = new Aws.AppSync.Inputs.DataSourceHttpConfigArgs
        {
            Endpoint = "http://example.com",
        },
    });

    var exampleFunction = new Aws.AppSync.Function("example", new()
    {
        ApiId = example.Id,
        DataSource = exampleDataSource.Name,
        Name = "example",
        RequestMappingTemplate = @"{
    ""version"": ""2018-05-29"",
    ""method"": ""GET"",
    ""resourcePath"": ""/"",
    ""params"":{
        ""headers"": $utils.http.copyheaders($ctx.request.headers)
    }
}
",
        ResponseMappingTemplate = @"#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := appsync.NewGraphQLApi(ctx, "example", &appsync.GraphQLApiArgs{
			AuthenticationType: pulumi.String("API_KEY"),
			Name:               pulumi.String("example"),
			Schema: pulumi.String(`type Mutation {
  putPost(id: ID!, title: String!): Post
}

type Post {
  id: ID!
  title: String!
}

type Query {
  singlePost(id: ID!): Post
}

schema {
  query: Query
  mutation: Mutation
}
`),
		})
		if err != nil {
			return err
		}
		exampleDataSource, err := appsync.NewDataSource(ctx, "example", &appsync.DataSourceArgs{
			ApiId: example.ID(),
			Name:  pulumi.String("example"),
			Type:  pulumi.String("HTTP"),
			HttpConfig: &appsync.DataSourceHttpConfigArgs{
				Endpoint: pulumi.String("http://example.com"),
			},
		})
		if err != nil {
			return err
		}
		_, err = appsync.NewFunction(ctx, "example", &appsync.FunctionArgs{
			ApiId:      example.ID(),
			DataSource: exampleDataSource.Name,
			Name:       pulumi.String("example"),
			RequestMappingTemplate: pulumi.String(`{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": $utils.http.copyheaders($ctx.request.headers)
    }
}
`),
			ResponseMappingTemplate: pulumi.String(`#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
`),
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
import com.pulumi.aws.appsync.GraphQLApi;
import com.pulumi.aws.appsync.GraphQLApiArgs;
import com.pulumi.aws.appsync.DataSource;
import com.pulumi.aws.appsync.DataSourceArgs;
import com.pulumi.aws.appsync.inputs.DataSourceHttpConfigArgs;
import com.pulumi.aws.appsync.Function;
import com.pulumi.aws.appsync.FunctionArgs;
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
        var example = new GraphQLApi("example", GraphQLApiArgs.builder()
            .authenticationType("API_KEY")
            .name("example")
            .schema("""
type Mutation {
  putPost(id: ID!, title: String!): Post
}

type Post {
  id: ID!
  title: String!
}

type Query {
  singlePost(id: ID!): Post
}

schema {
  query: Query
  mutation: Mutation
}
            """)
            .build());

        var exampleDataSource = new DataSource("exampleDataSource", DataSourceArgs.builder()
            .apiId(example.id())
            .name("example")
            .type("HTTP")
            .httpConfig(DataSourceHttpConfigArgs.builder()
                .endpoint("http://example.com")
                .build())
            .build());

        var exampleFunction = new Function("exampleFunction", FunctionArgs.builder()
            .apiId(example.id())
            .dataSource(exampleDataSource.name())
            .name("example")
            .requestMappingTemplate("""
{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": $utils.http.copyheaders($ctx.request.headers)
    }
}
            """)
            .responseMappingTemplate("""
#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
            """)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:GraphQLApi
    properties:
      authenticationType: API_KEY
      name: example
      schema: |
        type Mutation {
          putPost(id: ID!, title: String!): Post
        }

        type Post {
          id: ID!
          title: String!
        }

        type Query {
          singlePost(id: ID!): Post
        }

        schema {
          query: Query
          mutation: Mutation
        }
  exampleDataSource:
    type: aws:appsync:DataSource
    name: example
    properties:
      apiId: ${example.id}
      name: example
      type: HTTP
      httpConfig:
        endpoint: http://example.com
  exampleFunction:
    type: aws:appsync:Function
    name: example
    properties:
      apiId: ${example.id}
      dataSource: ${exampleDataSource.name}
      name: example
      requestMappingTemplate: |
        {
            "version": "2018-05-29",
            "method": "GET",
            "resourcePath": "/",
            "params":{
                "headers": $utils.http.copyheaders($ctx.request.headers)
            }
        }
      responseMappingTemplate: |
        #if($ctx.result.statusCode == 200)
            $ctx.result.body
        #else
            $utils.appendError($ctx.result.body, $ctx.result.statusCode)
        #end
```
<!--End PulumiCodeChooser -->


### With Code

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.appsync.Function("example", {
    apiId: exampleAwsAppsyncGraphqlApi.id,
    dataSource: exampleAwsAppsyncDatasource.name,
    name: "example",
    code: std.file({
        input: "some-code-dir",
    }).then(invoke => invoke.result),
    runtime: {
        name: "APPSYNC_JS",
        runtimeVersion: "1.0.0",
    },
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.appsync.Function("example",
    api_id=example_aws_appsync_graphql_api["id"],
    data_source=example_aws_appsync_datasource["name"],
    name="example",
    code=std.file(input="some-code-dir").result,
    runtime={
        "name": "APPSYNC_JS",
        "runtime_version": "1.0.0",
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
    var example = new Aws.AppSync.Function("example", new()
    {
        ApiId = exampleAwsAppsyncGraphqlApi.Id,
        DataSource = exampleAwsAppsyncDatasource.Name,
        Name = "example",
        Code = Std.File.Invoke(new()
        {
            Input = "some-code-dir",
        }).Apply(invoke => invoke.Result),
        Runtime = new Aws.AppSync.Inputs.FunctionRuntimeArgs
        {
            Name = "APPSYNC_JS",
            RuntimeVersion = "1.0.0",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "some-code-dir",
		}, nil)
		if err != nil {
			return err
		}
		_, err = appsync.NewFunction(ctx, "example", &appsync.FunctionArgs{
			ApiId:      pulumi.Any(exampleAwsAppsyncGraphqlApi.Id),
			DataSource: pulumi.Any(exampleAwsAppsyncDatasource.Name),
			Name:       pulumi.String("example"),
			Code:       pulumi.String(invokeFile.Result),
			Runtime: &appsync.FunctionRuntimeArgs{
				Name:           pulumi.String("APPSYNC_JS"),
				RuntimeVersion: pulumi.String("1.0.0"),
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
import com.pulumi.aws.appsync.Function;
import com.pulumi.aws.appsync.FunctionArgs;
import com.pulumi.aws.appsync.inputs.FunctionRuntimeArgs;
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
        var example = new Function("example", FunctionArgs.builder()
            .apiId(exampleAwsAppsyncGraphqlApi.id())
            .dataSource(exampleAwsAppsyncDatasource.name())
            .name("example")
            .code(StdFunctions.file(FileArgs.builder()
                .input("some-code-dir")
                .build()).result())
            .runtime(FunctionRuntimeArgs.builder()
                .name("APPSYNC_JS")
                .runtimeVersion("1.0.0")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:Function
    properties:
      apiId: ${exampleAwsAppsyncGraphqlApi.id}
      dataSource: ${exampleAwsAppsyncDatasource.name}
      name: example
      code:
        fn::invoke:
          function: std:file
          arguments:
            input: some-code-dir
          return: result
      runtime:
        name: APPSYNC_JS
        runtimeVersion: 1.0.0
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appsync_function` using the AppSync API ID and Function ID separated by `-`. For example:

```sh
$ pulumi import aws:appsync/function:Function example xxxxx-yyyyy
```
/
apiId" "ID of the associated AppSync API.
�
codeB" �The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
-

dataSource" Function data source name.
+
descriptionB" Function description.
�
functionVersionB" Version of the request mapping template. Currently the supported value is `2018-05-29`. Does not apply when specifying `code`.
e
maxBatchSizeB OMaximum batching size for a resolver. Valid values are between `0` and `2000`.
K
nameB" =Function name. The function name does not have to be unique.
�
requestMappingTemplateB" rFunction request mapping template. Functions support only the 2018-05-29 version of the request mapping template.
E
responseMappingTemplateB" $Function response mapping template.
�
runtimeMBK:I
G
appsyncFunctionRuntime+aws:appsync/FunctionRuntime:FunctionRuntime�Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See `runtime` Block for details.
�

syncConfigVBT:R
P
appsyncFunctionSyncConfig1aws:appsync/FunctionSyncConfig:FunctionSyncConfigTDescribes a Sync configuration for a resolver. See `sync_config` Block for details.
"/
apiId" "ID of the associated AppSync API.
"'
arn" ARN of the Function object.
"�
codeB" �The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
"-

dataSource" Function data source name.
"+
descriptionB" Function description.
">

functionId" ,Unique ID representing the Function object.
"�
functionVersion" Version of the request mapping template. Currently the supported value is `2018-05-29`. Does not apply when specifying `code`.
"e
maxBatchSizeB OMaximum batching size for a resolver. Valid values are between `0` and `2000`.
"I
name" =Function name. The function name does not have to be unique.
"�
requestMappingTemplateB" rFunction request mapping template. Functions support only the 2018-05-29 version of the request mapping template.
"E
responseMappingTemplateB" $Function response mapping template.
"�
runtimeMBK:I
G
appsyncFunctionRuntime+aws:appsync/FunctionRuntime:FunctionRuntime�Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See `runtime` Block for details.
"�

syncConfigVBT:R
P
appsyncFunctionSyncConfig1aws:appsync/FunctionSyncConfig:FunctionSyncConfigTDescribes a Sync configuration for a resolver. See `sync_config` Block for details.
*�A
8
appsync
GraphQLApi!aws:appsync/graphQLApi:GraphQLApi�
!additionalAuthenticationProviders�B�*�:�
�
appsync*GraphQLApiAdditionalAuthenticationProvideraaws:appsync/GraphQLApiAdditionalAuthenticationProvider:GraphQLApiAdditionalAuthenticationProvider�One or more additional authentication providers for the GraphQL API. See `additional_authentication_provider` Block for details.
�
apiTypeB" vAPI type. Valid values are `GRAPHQL` or `MERGED`. A `MERGED` type requires `merged_api_execution_role_arn` to be set.
�
authenticationType" uAuthentication type. Valid values: `API_KEY`, `AWS_IAM`, `AMAZON_COGNITO_USER_POOLS`, `OPENID_CONNECT`, `AWS_LAMBDA`
�
enhancedMetricsConfig}B{:y
w
appsyncGraphQLApiEnhancedMetricsConfigKaws:appsync/GraphQLApiEnhancedMetricsConfig:GraphQLApiEnhancedMetricsConfigdEnables and controls the enhanced metrics feature. See `enhanced_metrics_config` Block for details.
�
introspectionConfigB" �Sets the value of the GraphQL API to enable (`ENABLED`) or disable (`DISABLED`) introspection. If no value is provided, the introspection configuration will be set to ENABLED by default. This field will produce an error if the operation attempts to use the introspection feature while this field is disabled. For more information about introspection, see [GraphQL introspection](https://graphql.org/learn/introspection/).
�
lambdaAuthorizerConfig�B~:|
z
appsync GraphQLApiLambdaAuthorizerConfigMaws:appsync/GraphQLApiLambdaAuthorizerConfig:GraphQLApiLambdaAuthorizerConfignNested argument containing Lambda authorizer configuration. See `lambda_authorizer_config` Block for details.
�
	logConfigYBW:U
S
appsyncGraphQLApiLogConfig3aws:appsync/GraphQLApiLogConfig:GraphQLApiLogConfigVNested argument containing logging configuration. See `log_config` Block for details.
a
mergedApiExecutionRoleArnB" >ARN of the execution role when `api_type` is set to `MERGED`.
]
nameB" OUser-supplied name for the GraphQL API.

The following arguments are optional:
�
openidConnectConfigwBu:s
q
appsyncGraphQLApiOpenidConnectConfigGaws:appsync/GraphQLApiOpenidConnectConfig:GraphQLApiOpenidConnectConfighNested argument containing OpenID Connect configuration. See `openid_connect_config` Block for details.
�
queryDepthLimitB �The maximum depth a query can have in a single request. Depth refers to the amount of nested levels allowed in the body of query. The default value is `0` (or unspecified), which indicates there's no depth limit. If you set a limit, it can be between `1` and `75` nested levels. This field will produce a limit error if the operation falls out of bounds.

Note that fields can still be set to nullable or non-nullable. If a non-nullable field produces an error, the error will be thrown upwards to the first nullable field available.
�
resolverCountLimitB �The maximum number of resolvers that can be invoked in a single request. The default value is `0` (or unspecified), which will set the limit to `10000`. When specified, the limit value can be between `1` and `10000`. This field will produce a limit error if the operation falls out of bounds.
�
schemaB" zSchema definition, in GraphQL schema language format. This provider cannot perform drift detection of this configuration.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
userPoolConfighBf:d
b
appsyncGraphQLApiUserPoolConfig=aws:appsync/GraphQLApiUserPoolConfig:GraphQLApiUserPoolConfigRAmazon Cognito User Pool configuration. See `user_pool_config` Block for details.
�

visibilityB" �Sets the value of the GraphQL API to public (`GLOBAL`) or private (`PRIVATE`). If no value is provided, the visibility will be set to `GLOBAL` by default. This value cannot be changed once the API has been created.
O
xrayEnabledB
 :Whether tracing with X-ray is enabled. Defaults to false.
"�
!additionalAuthenticationProviders�B�*�:�
�
appsync*GraphQLApiAdditionalAuthenticationProvideraaws:appsync/GraphQLApiAdditionalAuthenticationProvider:GraphQLApiAdditionalAuthenticationProvider�One or more additional authentication providers for the GraphQL API. See `additional_authentication_provider` Block for details.
"�
apiTypeB" vAPI type. Valid values are `GRAPHQL` or `MERGED`. A `MERGED` type requires `merged_api_execution_role_arn` to be set.
"
arn" ARN
"�
authenticationType" uAuthentication type. Valid values: `API_KEY`, `AWS_IAM`, `AMAZON_COGNITO_USER_POOLS`, `OPENID_CONNECT`, `AWS_LAMBDA`
"�
enhancedMetricsConfig}B{:y
w
appsyncGraphQLApiEnhancedMetricsConfigKaws:appsync/GraphQLApiEnhancedMetricsConfig:GraphQLApiEnhancedMetricsConfigdEnables and controls the enhanced metrics feature. See `enhanced_metrics_config` Block for details.
"�
introspectionConfigB" �Sets the value of the GraphQL API to enable (`ENABLED`) or disable (`DISABLED`) introspection. If no value is provided, the introspection configuration will be set to ENABLED by default. This field will produce an error if the operation attempts to use the introspection feature while this field is disabled. For more information about introspection, see [GraphQL introspection](https://graphql.org/learn/introspection/).
"�
lambdaAuthorizerConfig�B~:|
z
appsync GraphQLApiLambdaAuthorizerConfigMaws:appsync/GraphQLApiLambdaAuthorizerConfig:GraphQLApiLambdaAuthorizerConfignNested argument containing Lambda authorizer configuration. See `lambda_authorizer_config` Block for details.
"�
	logConfigYBW:U
S
appsyncGraphQLApiLogConfig3aws:appsync/GraphQLApiLogConfig:GraphQLApiLogConfigVNested argument containing logging configuration. See `log_config` Block for details.
"a
mergedApiExecutionRoleArnB" >ARN of the execution role when `api_type` is set to `MERGED`.
"[
name" OUser-supplied name for the GraphQL API.

The following arguments are optional:
"�
openidConnectConfigwBu:s
q
appsyncGraphQLApiOpenidConnectConfigGaws:appsync/GraphQLApiOpenidConnectConfig:GraphQLApiOpenidConnectConfighNested argument containing OpenID Connect configuration. See `openid_connect_config` Block for details.
"�
queryDepthLimitB �The maximum depth a query can have in a single request. Depth refers to the amount of nested levels allowed in the body of query. The default value is `0` (or unspecified), which indicates there's no depth limit. If you set a limit, it can be between `1` and `75` nested levels. This field will produce a limit error if the operation falls out of bounds.

Note that fields can still be set to nullable or non-nullable. If a non-nullable field produces an error, the error will be thrown upwards to the first nullable field available.
"�
resolverCountLimitB �The maximum number of resolvers that can be invoked in a single request. The default value is `0` (or unspecified), which will set the limit to `10000`. When specified, the limit value can be between `1` and `10000`. This field will produce a limit error if the operation falls out of bounds.
"�
schemaB" zSchema definition, in GraphQL schema language format. This provider cannot perform drift detection of this configuration.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
uris2" rMap of URIs associated with the API E.g., `uris["GRAPHQL"] = https://ID.appsync-api.REGION.amazonaws.com/graphql`
"�
userPoolConfighBf:d
b
appsyncGraphQLApiUserPoolConfig=aws:appsync/GraphQLApiUserPoolConfig:GraphQLApiUserPoolConfigRAmazon Cognito User Pool configuration. See `user_pool_config` Block for details.
"�

visibilityB" �Sets the value of the GraphQL API to public (`GLOBAL`) or private (`PRIVATE`). If no value is provided, the visibility will be set to `GLOBAL` by default. This value cannot be changed once the API has been created.
"O
xrayEnabledB
 :Whether tracing with X-ray is enabled. Defaults to false.
*��
2
appsyncResolveraws:appsync/resolver:Resolver�Provides an AppSync Resolver.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.appsync.GraphQLApi("test", {
    authenticationType: "API_KEY",
    name: "tf-example",
    schema: `type Mutation {
\x09putPost(id: ID!, title: String!): Post
}

type Post {
\x09id: ID!
\x09title: String!
}

type Query {
\x09singlePost(id: ID!): Post
}

schema {
\x09query: Query
\x09mutation: Mutation
}
`,
});
const testDataSource = new aws.appsync.DataSource("test", {
    apiId: test.id,
    name: "my_example",
    type: "HTTP",
    httpConfig: {
        endpoint: "http://example.com",
    },
});
// UNIT type resolver (default)
const testResolver = new aws.appsync.Resolver("test", {
    apiId: test.id,
    field: "singlePost",
    type: "Query",
    dataSource: testDataSource.name,
    requestTemplate: `{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": utils.http.copyheaders(ctx.request.headers)
    }
}
`,
    responseTemplate: `#if(ctx.result.statusCode == 200)
    ctx.result.body
#else
    utils.appendError(ctx.result.body, ctx.result.statusCode)
#end
`,
    cachingConfig: {
        cachingKeys: [
            "$context.identity.sub",
            "$context.arguments.id",
        ],
        ttl: 60,
    },
});
// PIPELINE type resolver
const mutationPipelineTest = new aws.appsync.Resolver("Mutation_pipelineTest", {
    type: "Mutation",
    apiId: test.id,
    field: "pipelineTest",
    requestTemplate: "{}",
    responseTemplate: "$util.toJson($ctx.result)",
    kind: "PIPELINE",
    pipelineConfig: {
        functions: [
            test1.functionId,
            test2.functionId,
            test3.functionId,
        ],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.appsync.GraphQLApi("test",
    authentication_type="API_KEY",
    name="tf-example",
    schema="""type Mutation {
\x09putPost(id: ID!, title: String!): Post
}

type Post {
\x09id: ID!
\x09title: String!
}

type Query {
\x09singlePost(id: ID!): Post
}

schema {
\x09query: Query
\x09mutation: Mutation
}
""")
test_data_source = aws.appsync.DataSource("test",
    api_id=test.id,
    name="my_example",
    type="HTTP",
    http_config={
        "endpoint": "http://example.com",
    })
# UNIT type resolver (default)
test_resolver = aws.appsync.Resolver("test",
    api_id=test.id,
    field="singlePost",
    type="Query",
    data_source=test_data_source.name,
    request_template="""{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": $utils.http.copyheaders($ctx.request.headers)
    }
}
""",
    response_template="""#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
""",
    caching_config={
        "caching_keys": [
            "$context.identity.sub",
            "$context.arguments.id",
        ],
        "ttl": 60,
    })
# PIPELINE type resolver
mutation_pipeline_test = aws.appsync.Resolver("Mutation_pipelineTest",
    type="Mutation",
    api_id=test.id,
    field="pipelineTest",
    request_template="{}",
    response_template="$util.toJson($ctx.result)",
    kind="PIPELINE",
    pipeline_config={
        "functions": [
            test1["functionId"],
            test2["functionId"],
            test3["functionId"],
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
    var test = new Aws.AppSync.GraphQLApi("test", new()
    {
        AuthenticationType = "API_KEY",
        Name = "tf-example",
        Schema = @"type Mutation {
	putPost(id: ID!, title: String!): Post
}

type Post {
	id: ID!
	title: String!
}

type Query {
	singlePost(id: ID!): Post
}

schema {
	query: Query
	mutation: Mutation
}
",
    });

    var testDataSource = new Aws.AppSync.DataSource("test", new()
    {
        ApiId = test.Id,
        Name = "my_example",
        Type = "HTTP",
        HttpConfig = new Aws.AppSync.Inputs.DataSourceHttpConfigArgs
        {
            Endpoint = "http://example.com",
        },
    });

    // UNIT type resolver (default)
    var testResolver = new Aws.AppSync.Resolver("test", new()
    {
        ApiId = test.Id,
        Field = "singlePost",
        Type = "Query",
        DataSource = testDataSource.Name,
        RequestTemplate = @"{
    ""version"": ""2018-05-29"",
    ""method"": ""GET"",
    ""resourcePath"": ""/"",
    ""params"":{
        ""headers"": $utils.http.copyheaders($ctx.request.headers)
    }
}
",
        ResponseTemplate = @"#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
",
        CachingConfig = new Aws.AppSync.Inputs.ResolverCachingConfigArgs
        {
            CachingKeys = new[]
            {
                "$context.identity.sub",
                "$context.arguments.id",
            },
            Ttl = 60,
        },
    });

    // PIPELINE type resolver
    var mutationPipelineTest = new Aws.AppSync.Resolver("Mutation_pipelineTest", new()
    {
        Type = "Mutation",
        ApiId = test.Id,
        Field = "pipelineTest",
        RequestTemplate = "{}",
        ResponseTemplate = "$util.toJson($ctx.result)",
        Kind = "PIPELINE",
        PipelineConfig = new Aws.AppSync.Inputs.ResolverPipelineConfigArgs
        {
            Functions = new[]
            {
                test1.FunctionId,
                test2.FunctionId,
                test3.FunctionId,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := appsync.NewGraphQLApi(ctx, "test", &appsync.GraphQLApiArgs{
			AuthenticationType: pulumi.String("API_KEY"),
			Name:               pulumi.String("tf-example"),
			Schema: pulumi.String(`type Mutation {
	putPost(id: ID!, title: String!): Post
}

type Post {
	id: ID!
	title: String!
}

type Query {
	singlePost(id: ID!): Post
}

schema {
	query: Query
	mutation: Mutation
}
`),
		})
		if err != nil {
			return err
		}
		testDataSource, err := appsync.NewDataSource(ctx, "test", &appsync.DataSourceArgs{
			ApiId: test.ID(),
			Name:  pulumi.String("my_example"),
			Type:  pulumi.String("HTTP"),
			HttpConfig: &appsync.DataSourceHttpConfigArgs{
				Endpoint: pulumi.String("http://example.com"),
			},
		})
		if err != nil {
			return err
		}
		// UNIT type resolver (default)
		_, err = appsync.NewResolver(ctx, "test", &appsync.ResolverArgs{
			ApiId:      test.ID(),
			Field:      pulumi.String("singlePost"),
			Type:       pulumi.String("Query"),
			DataSource: testDataSource.Name,
			RequestTemplate: pulumi.String(`{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": $utils.http.copyheaders($ctx.request.headers)
    }
}
`),
			ResponseTemplate: pulumi.String(`#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
`),
			CachingConfig: &appsync.ResolverCachingConfigArgs{
				CachingKeys: pulumi.StringArray{
					pulumi.String("$context.identity.sub"),
					pulumi.String("$context.arguments.id"),
				},
				Ttl: pulumi.Int(60),
			},
		})
		if err != nil {
			return err
		}
		// PIPELINE type resolver
		_, err = appsync.NewResolver(ctx, "Mutation_pipelineTest", &appsync.ResolverArgs{
			Type:             pulumi.String("Mutation"),
			ApiId:            test.ID(),
			Field:            pulumi.String("pipelineTest"),
			RequestTemplate:  pulumi.String("{}"),
			ResponseTemplate: pulumi.String("$util.toJson($ctx.result)"),
			Kind:             pulumi.String("PIPELINE"),
			PipelineConfig: &appsync.ResolverPipelineConfigArgs{
				Functions: pulumi.StringArray{
					test1.FunctionId,
					test2.FunctionId,
					test3.FunctionId,
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
import com.pulumi.aws.appsync.GraphQLApi;
import com.pulumi.aws.appsync.GraphQLApiArgs;
import com.pulumi.aws.appsync.DataSource;
import com.pulumi.aws.appsync.DataSourceArgs;
import com.pulumi.aws.appsync.inputs.DataSourceHttpConfigArgs;
import com.pulumi.aws.appsync.Resolver;
import com.pulumi.aws.appsync.ResolverArgs;
import com.pulumi.aws.appsync.inputs.ResolverCachingConfigArgs;
import com.pulumi.aws.appsync.inputs.ResolverPipelineConfigArgs;
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
        var test = new GraphQLApi("test", GraphQLApiArgs.builder()
            .authenticationType("API_KEY")
            .name("tf-example")
            .schema("""
type Mutation {
	putPost(id: ID!, title: String!): Post
}

type Post {
	id: ID!
	title: String!
}

type Query {
	singlePost(id: ID!): Post
}

schema {
	query: Query
	mutation: Mutation
}
            """)
            .build());

        var testDataSource = new DataSource("testDataSource", DataSourceArgs.builder()
            .apiId(test.id())
            .name("my_example")
            .type("HTTP")
            .httpConfig(DataSourceHttpConfigArgs.builder()
                .endpoint("http://example.com")
                .build())
            .build());

        // UNIT type resolver (default)
        var testResolver = new Resolver("testResolver", ResolverArgs.builder()
            .apiId(test.id())
            .field("singlePost")
            .type("Query")
            .dataSource(testDataSource.name())
            .requestTemplate("""
{
    "version": "2018-05-29",
    "method": "GET",
    "resourcePath": "/",
    "params":{
        "headers": $utils.http.copyheaders($ctx.request.headers)
    }
}
            """)
            .responseTemplate("""
#if($ctx.result.statusCode == 200)
    $ctx.result.body
#else
    $utils.appendError($ctx.result.body, $ctx.result.statusCode)
#end
            """)
            .cachingConfig(ResolverCachingConfigArgs.builder()
                .cachingKeys(                
                    "$context.identity.sub",
                    "$context.arguments.id")
                .ttl(60)
                .build())
            .build());

        // PIPELINE type resolver
        var mutationPipelineTest = new Resolver("mutationPipelineTest", ResolverArgs.builder()
            .type("Mutation")
            .apiId(test.id())
            .field("pipelineTest")
            .requestTemplate("{}")
            .responseTemplate("$util.toJson($ctx.result)")
            .kind("PIPELINE")
            .pipelineConfig(ResolverPipelineConfigArgs.builder()
                .functions(                
                    test1.functionId(),
                    test2.functionId(),
                    test3.functionId())
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:appsync:GraphQLApi
    properties:
      authenticationType: API_KEY
      name: tf-example
      schema: |
        type Mutation {
        	putPost(id: ID!, title: String!): Post
        }

        type Post {
        	id: ID!
        	title: String!
        }

        type Query {
        	singlePost(id: ID!): Post
        }

        schema {
        	query: Query
        	mutation: Mutation
        }
  testDataSource:
    type: aws:appsync:DataSource
    name: test
    properties:
      apiId: ${test.id}
      name: my_example
      type: HTTP
      httpConfig:
        endpoint: http://example.com
  # UNIT type resolver (default)
  testResolver:
    type: aws:appsync:Resolver
    name: test
    properties:
      apiId: ${test.id}
      field: singlePost
      type: Query
      dataSource: ${testDataSource.name}
      requestTemplate: |
        {
            "version": "2018-05-29",
            "method": "GET",
            "resourcePath": "/",
            "params":{
                "headers": $utils.http.copyheaders($ctx.request.headers)
            }
        }
      responseTemplate: |
        #if($ctx.result.statusCode == 200)
            $ctx.result.body
        #else
            $utils.appendError($ctx.result.body, $ctx.result.statusCode)
        #end
      cachingConfig:
        cachingKeys:
          - $context.identity.sub
          - $context.arguments.id
        ttl: 60
  # PIPELINE type resolver
  mutationPipelineTest:
    type: aws:appsync:Resolver
    name: Mutation_pipelineTest
    properties:
      type: Mutation
      apiId: ${test.id}
      field: pipelineTest
      requestTemplate: '{}'
      responseTemplate: $util.toJson($ctx.result)
      kind: PIPELINE
      pipelineConfig:
        functions:
          - ${test1.functionId}
          - ${test2.functionId}
          - ${test3.functionId}
```
<!--End PulumiCodeChooser -->


### JS

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.appsync.Resolver("example", {
    type: "Query",
    apiId: testAwsAppsyncGraphqlApi.id,
    field: "pipelineTest",
    kind: "PIPELINE",
    code: std.file({
        input: "some-code-dir",
    }).then(invoke => invoke.result),
    runtime: {
        name: "APPSYNC_JS",
        runtimeVersion: "1.0.0",
    },
    pipelineConfig: {
        functions: [test.functionId],
    },
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.appsync.Resolver("example",
    type="Query",
    api_id=test_aws_appsync_graphql_api["id"],
    field="pipelineTest",
    kind="PIPELINE",
    code=std.file(input="some-code-dir").result,
    runtime={
        "name": "APPSYNC_JS",
        "runtime_version": "1.0.0",
    },
    pipeline_config={
        "functions": [test["functionId"]],
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
    var example = new Aws.AppSync.Resolver("example", new()
    {
        Type = "Query",
        ApiId = testAwsAppsyncGraphqlApi.Id,
        Field = "pipelineTest",
        Kind = "PIPELINE",
        Code = Std.File.Invoke(new()
        {
            Input = "some-code-dir",
        }).Apply(invoke => invoke.Result),
        Runtime = new Aws.AppSync.Inputs.ResolverRuntimeArgs
        {
            Name = "APPSYNC_JS",
            RuntimeVersion = "1.0.0",
        },
        PipelineConfig = new Aws.AppSync.Inputs.ResolverPipelineConfigArgs
        {
            Functions = new[]
            {
                test.FunctionId,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "some-code-dir",
		}, nil)
		if err != nil {
			return err
		}
		_, err = appsync.NewResolver(ctx, "example", &appsync.ResolverArgs{
			Type:  pulumi.String("Query"),
			ApiId: pulumi.Any(testAwsAppsyncGraphqlApi.Id),
			Field: pulumi.String("pipelineTest"),
			Kind:  pulumi.String("PIPELINE"),
			Code:  pulumi.String(invokeFile.Result),
			Runtime: &appsync.ResolverRuntimeArgs{
				Name:           pulumi.String("APPSYNC_JS"),
				RuntimeVersion: pulumi.String("1.0.0"),
			},
			PipelineConfig: &appsync.ResolverPipelineConfigArgs{
				Functions: pulumi.StringArray{
					test.FunctionId,
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
import com.pulumi.aws.appsync.Resolver;
import com.pulumi.aws.appsync.ResolverArgs;
import com.pulumi.aws.appsync.inputs.ResolverRuntimeArgs;
import com.pulumi.aws.appsync.inputs.ResolverPipelineConfigArgs;
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
        var example = new Resolver("example", ResolverArgs.builder()
            .type("Query")
            .apiId(testAwsAppsyncGraphqlApi.id())
            .field("pipelineTest")
            .kind("PIPELINE")
            .code(StdFunctions.file(FileArgs.builder()
                .input("some-code-dir")
                .build()).result())
            .runtime(ResolverRuntimeArgs.builder()
                .name("APPSYNC_JS")
                .runtimeVersion("1.0.0")
                .build())
            .pipelineConfig(ResolverPipelineConfigArgs.builder()
                .functions(test.functionId())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:Resolver
    properties:
      type: Query
      apiId: ${testAwsAppsyncGraphqlApi.id}
      field: pipelineTest
      kind: PIPELINE
      code:
        fn::invoke:
          function: std:file
          arguments:
            input: some-code-dir
          return: result
      runtime:
        name: APPSYNC_JS
        runtimeVersion: 1.0.0
      pipelineConfig:
        functions:
          - ${test.functionId}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_appsync_resolver` using the `api_id`, a hyphen, `type`, a hypen and `field`. For example:

```sh
$ pulumi import aws:appsync/resolver:Resolver example abcdef123456-exampleType-exampleField
```
)
apiId" API ID for the GraphQL API.
�
cachingConfig_B]:[
Y
appsyncResolverCachingConfig7aws:appsync/ResolverCachingConfig:ResolverCachingConfig(The Caching Config. See Caching Config.
�
codeB" �The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
&

dataSourceB" Data source name.
D
field" 7Field name from the schema defined in the GraphQL API.
E
kindB" 7Resolver type. Valid values are `UNIT` and `PIPELINE`.
e
maxBatchSizeB OMaximum batching size for a resolver. Valid values are between `0` and `2000`.
�
pipelineConfigbB`:^
\
appsyncResolverPipelineConfig9aws:appsync/ResolverPipelineConfig:ResolverPipelineConfigAThe caching configuration for the resolver. See Pipeline Config.
�
requestTemplateB" �Request mapping template for UNIT resolver or 'before mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
�
responseTemplateB" �Response mapping template for UNIT resolver or 'after mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
�
runtimeMBK:I
G
appsyncResolverRuntime+aws:appsync/ResolverRuntime:ResolverRuntime�Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See Runtime.
�

syncConfigVBT:R
P
appsyncResolverSyncConfig1aws:appsync/ResolverSyncConfig:ResolverSyncConfig@Describes a Sync configuration for a resolver. See Sync Config.
B
type" 6Type name from the schema defined in the GraphQL API.
")
apiId" API ID for the GraphQL API.
"
arn" ARN
"�
cachingConfig_B]:[
Y
appsyncResolverCachingConfig7aws:appsync/ResolverCachingConfig:ResolverCachingConfig(The Caching Config. See Caching Config.
"�
codeB" �The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
"&

dataSourceB" Data source name.
"D
field" 7Field name from the schema defined in the GraphQL API.
"E
kindB" 7Resolver type. Valid values are `UNIT` and `PIPELINE`.
"e
maxBatchSizeB OMaximum batching size for a resolver. Valid values are between `0` and `2000`.
"�
pipelineConfigbB`:^
\
appsyncResolverPipelineConfig9aws:appsync/ResolverPipelineConfig:ResolverPipelineConfigAThe caching configuration for the resolver. See Pipeline Config.
"�
requestTemplateB" �Request mapping template for UNIT resolver or 'before mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
"�
responseTemplateB" �Response mapping template for UNIT resolver or 'after mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
"�
runtimeMBK:I
G
appsyncResolverRuntime+aws:appsync/ResolverRuntime:ResolverRuntime�Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See Runtime.
"�

syncConfigVBT:R
P
appsyncResolverSyncConfig1aws:appsync/ResolverSyncConfig:ResolverSyncConfig@Describes a Sync configuration for a resolver. See Sync Config.
"B
type" 6Type name from the schema defined in the GraphQL API.
*�%
V
appsyncSourceApiAssociation5aws:appsync/sourceApiAssociation:SourceApiAssociation�Resource for managing an AWS AppSync Source Api Association.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.appsync.SourceApiAssociation("test", {
    description: "My source API Merged",
    mergedApiId: "gzos6bteufdunffzzifiowisoe",
    sourceApiId: "fzzifiowisoegzos6bteufdunf",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.appsync.SourceApiAssociation("test",
    description="My source API Merged",
    merged_api_id="gzos6bteufdunffzzifiowisoe",
    source_api_id="fzzifiowisoegzos6bteufdunf")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.AppSync.SourceApiAssociation("test", new()
    {
        Description = "My source API Merged",
        MergedApiId = "gzos6bteufdunffzzifiowisoe",
        SourceApiId = "fzzifiowisoegzos6bteufdunf",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appsync.NewSourceApiAssociation(ctx, "test", &appsync.SourceApiAssociationArgs{
			Description: pulumi.String("My source API Merged"),
			MergedApiId: pulumi.String("gzos6bteufdunffzzifiowisoe"),
			SourceApiId: pulumi.String("fzzifiowisoegzos6bteufdunf"),
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
import com.pulumi.aws.appsync.SourceApiAssociation;
import com.pulumi.aws.appsync.SourceApiAssociationArgs;
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
        var test = new SourceApiAssociation("test", SourceApiAssociationArgs.builder()
            .description("My source API Merged")
            .mergedApiId("gzos6bteufdunffzzifiowisoe")
            .sourceApiId("fzzifiowisoegzos6bteufdunf")
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:appsync:SourceApiAssociation
    properties:
      description: My source API Merged
      mergedApiId: gzos6bteufdunffzzifiowisoe
      sourceApiId: fzzifiowisoegzos6bteufdunf
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AppSync Source Api Association using the `gzos6bteufdunffzzifiowisoe,243685a0-9347-4a1a-89c1-9b57dea01e31`. For example:

```sh
$ pulumi import aws:appsync/sourceApiAssociation:SourceApiAssociation example gzos6bteufdunffzzifiowisoe,243685a0-9347-4a1a-89c1-9b57dea01e31
```
A
descriptionB" ,Description of the source API being merged.
k
mergedApiArnB" UARN of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
i
mergedApiIdB" TID of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
k
sourceApiArnB" UARN of the source API. One of `source_api_arn` or `source_api_id` must be specified.
�
sourceApiAssociationConfigs�B�*�:�
�
appsync.SourceApiAssociationSourceApiAssociationConfigiaws:appsync/SourceApiAssociationSourceApiAssociationConfig:SourceApiAssociationSourceApiAssociationConfigi
sourceApiIdB" TID of the source API. One of `source_api_arn` or `source_api_id` must be specified.
�
timeoutstBr:p
n
appsyncSourceApiAssociationTimeoutsEaws:appsync/SourceApiAssociationTimeouts:SourceApiAssociationTimeouts".
arn" #ARN of the Source Api Association.
"7
associationId" "ID of the Source Api Association.
"A
descriptionB" ,Description of the source API being merged.
"i
mergedApiArn" UARN of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
"g
mergedApiId" TID of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
"i
sourceApiArn" UARN of the source API. One of `source_api_arn` or `source_api_id` must be specified.
"�
sourceApiAssociationConfigs�*�:�
�
appsync.SourceApiAssociationSourceApiAssociationConfigiaws:appsync/SourceApiAssociationSourceApiAssociationConfig:SourceApiAssociationSourceApiAssociationConfig"g
sourceApiId" TID of the source API. One of `source_api_arn` or `source_api_id` must be specified.
"�
timeoutstBr:p
n
appsyncSourceApiAssociationTimeoutsEaws:appsync/SourceApiAssociationTimeouts:SourceApiAssociationTimeouts*�
&
appsyncTypeaws:appsync/type:Type�Provides an AppSync Type.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.appsync.GraphQLApi("example", {
    authenticationType: "API_KEY",
    name: "example",
});
const exampleType = new aws.appsync.Type("example", {
    apiId: example.id,
    format: "SDL",
    definition: `type Mutation

{
putPost(id: ID!,title: String! ): Post

}
`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appsync.GraphQLApi("example",
    authentication_type="API_KEY",
    name="example")
example_type = aws.appsync.Type("example",
    api_id=example.id,
    format="SDL",
    definition="""type Mutation

{
putPost(id: ID!,title: String! ): Post

}
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.AppSync.GraphQLApi("example", new()
    {
        AuthenticationType = "API_KEY",
        Name = "example",
    });

    var exampleType = new Aws.AppSync.Type("example", new()
    {
        ApiId = example.Id,
        Format = "SDL",
        Definition = @"type Mutation

{
putPost(id: ID!,title: String! ): Post

}
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appsync"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := appsync.NewGraphQLApi(ctx, "example", &appsync.GraphQLApiArgs{
			AuthenticationType: pulumi.String("API_KEY"),
			Name:               pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = appsync.NewType(ctx, "example", &appsync.TypeArgs{
			ApiId:  example.ID(),
			Format: pulumi.String("SDL"),
			Definition: pulumi.String(`type Mutation

{
putPost(id: ID!,title: String! ): Post

}
`),
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
import com.pulumi.aws.appsync.GraphQLApi;
import com.pulumi.aws.appsync.GraphQLApiArgs;
import com.pulumi.aws.appsync.Type;
import com.pulumi.aws.appsync.TypeArgs;
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
        var example = new GraphQLApi("example", GraphQLApiArgs.builder()
            .authenticationType("API_KEY")
            .name("example")
            .build());

        var exampleType = new Type("exampleType", TypeArgs.builder()
            .apiId(example.id())
            .format("SDL")
            .definition("""
type Mutation

{
putPost(id: ID!,title: String! ): Post

}
            """)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appsync:GraphQLApi
    properties:
      authenticationType: API_KEY
      name: example
  exampleType:
    type: aws:appsync:Type
    name: example
    properties:
      apiId: ${example.id}
      format: SDL
      definition: |
        type Mutation

        {
        putPost(id: ID!,title: String! ): Post

        }
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Appsync Types using the `id`. For example:

```sh
$ pulumi import aws:appsync/type:Type example api-id:format:name
```

apiId" GraphQL API ID.
'

definition" The type definition.
0
format" "The type format: `SDL` or `JSON`.
"
apiId" GraphQL API ID.
" 
arn" The ARN of the type.
"'

definition" The type definition.
")
description" The type description.
"0
format" "The type format: `SDL` or `JSON`.
"
name" The type name.
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
2�
c
	appconfiggetConfigurationProfile=aws:appconfig/getConfigurationProfile:getConfigurationProfile�Provides access to an AppConfig Configuration Profile.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.appconfig.getConfigurationProfile({
    applicationId: "b5d5gpj",
    configurationProfileId: "qrbb1c1",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.get_configuration_profile(application_id="b5d5gpj",
    configuration_profile_id="qrbb1c1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.AppConfig.GetConfigurationProfile.Invoke(new()
    {
        ApplicationId = "b5d5gpj",
        ConfigurationProfileId = "qrbb1c1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.LookupConfigurationProfile(ctx, &appconfig.LookupConfigurationProfileArgs{
			ApplicationId:          "b5d5gpj",
			ConfigurationProfileId: "qrbb1c1",
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
import com.pulumi.aws.appconfig.AppconfigFunctions;
import com.pulumi.aws.appconfig.inputs.GetConfigurationProfileArgs;
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
        final var example = AppconfigFunctions.getConfigurationProfile(GetConfigurationProfileArgs.builder()
            .applicationId("b5d5gpj")
            .configurationProfileId("qrbb1c1")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:appconfig:getConfigurationProfile
      arguments:
        applicationId: b5d5gpj
        configurationProfileId: qrbb1c1
```
<!--End PulumiCodeChooser -->
b
applicationId" MID of the AppConfig application to which this configuration profile belongs.
?
configurationProfileId" !ID of the Configuration Profile.
.
tagsB2" Map of tags for the resource.
"
applicationId" "-
arn" "ARN of the Configuration Profile.
"
configurationProfileId" "=
description" *Description of the Configuration Profile.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
kmsKeyIdentifier" ">
locationUri" +Location URI of the Configuration Profile.
"/
name" #Name of the Configuration Profile.
"v
retrievalRoleArn" ^ARN of an IAM role with permission to access the configuration at the specified location_uri.
",
tags2" Map of tags for the resource.
"E
type" 9Type of validator. Valid values: JSON_SCHEMA and LAMBDA.
"�

validators�*�:�
~
	appconfig getConfigurationProfileValidatorOaws:appconfig/getConfigurationProfileValidator:getConfigurationProfileValidator9Nested list of methods for validating the configuration.
2�
f
	appconfiggetConfigurationProfiles?aws:appconfig/getConfigurationProfiles:getConfigurationProfiles�
Provides access to all Configuration Properties for an AppConfig Application. This will allow you to pass Configuration
Profile IDs to another resource.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.appconfig.getConfigurationProfiles({
    applicationId: "a1d3rpe",
});
const exampleGetConfigurationProfile = example.then(example => .reduce((__obj, [__key, __value]) => ({ ...__obj, [__key]: aws.appconfig.getConfigurationProfile({
    configurationProfileId: __value,
    applicationId: exampleAwsAppconfigApplication.id,
}) })));
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.get_configuration_profiles(application_id="a1d3rpe")
example_get_configuration_profile = {__key: aws.appconfig.get_configuration_profile(configuration_profile_id=__value,
    application_id=example_aws_appconfig_application["id"]) for __key, __value in example.configuration_profile_ids}
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.AppConfig.GetConfigurationProfiles.Invoke(new()
    {
        ApplicationId = "a1d3rpe",
    });

    var exampleGetConfigurationProfile = ;

});
```
<!--End PulumiCodeChooser -->
6
applicationId" !ID of the AppConfig Application.
"
applicationId" "m
configurationProfileIds*" LSet of Configuration Profile IDs associated with the AppConfig Application.
"E
id" ;The provider-assigned unique ID for this managed resource.
2�
H
	appconfiggetEnvironment+aws:appconfig/getEnvironment:getEnvironment�Provides access to an AppConfig Environment.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.appconfig.getEnvironment({
    applicationId: "b5d5gpj",
    environmentId: "qrbb1c1",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.get_environment(application_id="b5d5gpj",
    environment_id="qrbb1c1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.AppConfig.GetEnvironment.Invoke(new()
    {
        ApplicationId = "b5d5gpj",
        EnvironmentId = "qrbb1c1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.LookupEnvironment(ctx, &appconfig.LookupEnvironmentArgs{
			ApplicationId: "b5d5gpj",
			EnvironmentId: "qrbb1c1",
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
import com.pulumi.aws.appconfig.AppconfigFunctions;
import com.pulumi.aws.appconfig.inputs.GetEnvironmentArgs;
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
        final var example = AppconfigFunctions.getEnvironment(GetEnvironmentArgs.builder()
            .applicationId("b5d5gpj")
            .environmentId("qrbb1c1")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:appconfig:getEnvironment
      arguments:
        applicationId: b5d5gpj
        environmentId: qrbb1c1
```
<!--End PulumiCodeChooser -->
X
applicationId" CID of the AppConfig Application to which this Environment belongs.
6
environmentId" !ID of the AppConfig Environment.
.
tagsB2" Map of tags for the resource.
"
applicationId" "#
arn" ARN of the environment.
",
description" Name of the environment.
"
environmentId" "E
id" ;The provider-assigned unique ID for this managed resource.
"�
monitorsc*a:_
]
	appconfiggetEnvironmentMonitor9aws:appconfig/getEnvironmentMonitor:getEnvironmentMonitorJSet of Amazon CloudWatch alarms to monitor during the deployment process.
"%
name" Name of the environment.
"�
state" tState of the environment. Possible values are `READY_FOR_DEPLOYMENT`, `DEPLOYING`, `ROLLING_BACK`
or `ROLLED_BACK`.
",
tags2" Map of tags for the resource.
2�
K
	appconfiggetEnvironments-aws:appconfig/getEnvironments:getEnvironments�Provides access to all Environments for an AppConfig Application. This will allow you to pass Environment IDs to another
resource.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.appconfig.getEnvironments({
    applicationId: "a1d3rpe",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appconfig.get_environments(application_id="a1d3rpe")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.AppConfig.GetEnvironments.Invoke(new()
    {
        ApplicationId = "a1d3rpe",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appconfig"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appconfig.GetEnvironments(ctx, &appconfig.GetEnvironmentsArgs{
			ApplicationId: "a1d3rpe",
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
import com.pulumi.aws.appconfig.AppconfigFunctions;
import com.pulumi.aws.appconfig.inputs.GetEnvironmentsArgs;
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
        final var example = AppconfigFunctions.getEnvironments(GetEnvironmentsArgs.builder()
            .applicationId("a1d3rpe")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:appconfig:getEnvironments
      arguments:
        applicationId: a1d3rpe
```
<!--End PulumiCodeChooser -->
6
applicationId" !ID of the AppConfig Application.
"
applicationId" "[
environmentIds*" CSet of Environment IDs associated with this AppConfig Application.
"E
id" ;The provider-assigned unique ID for this managed resource.
2�
c
appintegrationsgetEventIntegration;aws:appintegrations/getEventIntegration:getEventIntegration�Use this data source to get information on an existing AppIntegrations Event Integration.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.appintegrations.getEventIntegration({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appintegrations.get_event_integration(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.AppIntegrations.GetEventIntegration.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appintegrations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appintegrations.GetEventIntegration(ctx, &appintegrations.GetEventIntegrationArgs{
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
import com.pulumi.aws.appintegrations.AppintegrationsFunctions;
import com.pulumi.aws.appintegrations.inputs.GetEventIntegrationArgs;
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
        final var example = AppintegrationsFunctions.getEventIntegration(GetEventIntegrationArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:appintegrations:getEventIntegration
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->
8
name" ,The AppIntegrations Event Integration name.
[
tagsB2" KMetadata that you can assign to help organize the report plans you create.
"=
arn" 2The ARN of the AppIntegrations Event Integration.
"=
description" *The description of the Event Integration.
"�
eventFilters�*�:�
�
appintegrationsgetEventIntegrationEventFilterQaws:appintegrations/getEventIntegrationEventFilter:getEventIntegrationEventFilteruA block that defines the configuration information for the event filter. The Event Filter block is documented below.
"+
eventbridgeBus" The EventBridge bus.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "Y
tags2" KMetadata that you can assign to help organize the report plans you create.
2�
G
appmeshgetGatewayRoute+aws:appmesh/getGatewayRoute:getGatewayRoute�The App Mesh Gateway Route data source allows details of an App Mesh Gateway Route to be retrieved by its name, mesh_name, virtual_gateway_name, and optionally the mesh_owner.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.appmesh.getGatewayRoute({
    name: "test-route",
    meshName: "test-mesh",
    virtualGatewayName: "test-gateway",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.appmesh.get_gateway_route(name="test-route",
    mesh_name="test-mesh",
    virtual_gateway_name="test-gateway")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.AppMesh.GetGatewayRoute.Invoke(new()
    {
        Name = "test-route",
        MeshName = "test-mesh",
        VirtualGatewayName = "test-gateway",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.LookupGatewayRoute(ctx, &appmesh.LookupGatewayRouteArgs{
			Name:               "test-route",
			MeshName:           "test-mesh",
			VirtualGatewayName: "test-gateway",
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
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetGatewayRouteArgs;
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
        final var test = AppmeshFunctions.getGatewayRoute(GetGatewayRouteArgs.builder()
            .name("test-route")
            .meshName("test-mesh")
            .virtualGatewayName("test-gateway")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:appmesh:getGatewayRoute
      arguments:
        name: test-route
        meshName: test-mesh
        virtualGatewayName: test-gateway
```
<!--End PulumiCodeChooser -->
N
meshName" >Name of the service mesh in which the virtual gateway exists.
?
	meshOwnerB" ,AWS account ID of the service mesh's owner.
'
name" Name of the gateway route.

tagsB2" Map of tags.
Q
virtualGatewayName" 7Name of the virtual gateway in which the route exists.
"%
arn" ARN of the gateway route.
"7
createdDate" $Creation date of the gateway route.
"E
id" ;The provider-assigned unique ID for this managed resource.
">
lastUpdatedDate" 'Last update date of the gateway route.
"
meshName" "
	meshOwner" "

name" "6
resourceOwner" !Resource owner's AWS account ID.
"�
specsY*W:U
S
appmeshgetGatewayRouteSpec3aws:appmesh/getGatewayRouteSpec:getGatewayRouteSpecVGateway route specification. See the `aws.appmesh.GatewayRoute` resource for details.
"
tags2" Map of tags.
"
virtualGatewayName" 2�(
/
appmeshgetMeshaws:appmesh/getMesh:getMesh�"The App Mesh Mesh data source allows details of an App Mesh Mesh to be retrieved by its name and optionally the mesh_owner.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const simple = aws.appmesh.getMesh({
    name: "simpleapp",
});
```
```python
import pulumi
import pulumi_aws as aws

simple = aws.appmesh.get_mesh(name="simpleapp")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var simple = Aws.AppMesh.GetMesh.Invoke(new()
    {
        Name = "simpleapp",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.LookupMesh(ctx, &appmesh.LookupMeshArgs{
			Name: "simpleapp",
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
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetMeshArgs;
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
        final var simple = AppmeshFunctions.getMesh(GetMeshArgs.builder()
            .name("simpleapp")
            .build());

    }
}
```
```yaml
variables:
  simple:
    fn::invoke:
      function: aws:appmesh:getMesh
      arguments:
        name: simpleapp
```
<!--End PulumiCodeChooser -->

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
const simple = current.then(current => aws.appmesh.getMesh({
    name: "simpleapp",
    meshOwner: current.accountId,
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_caller_identity()
simple = aws.appmesh.get_mesh(name="simpleapp",
    mesh_owner=current.account_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    var simple = Aws.AppMesh.GetMesh.Invoke(new()
    {
        Name = "simpleapp",
        MeshOwner = current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = appmesh.LookupMesh(ctx, &appmesh.LookupMeshArgs{
			Name:      "simpleapp",
			MeshOwner: pulumi.StringRef(current.AccountId),
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
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetMeshArgs;
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

        final var simple = AppmeshFunctions.getMesh(GetMeshArgs.builder()
            .name("simpleapp")
            .meshOwner(current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()))
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  simple:
    fn::invoke:
      function: aws:appmesh:getMesh
      arguments:
        name: simpleapp
        meshOwner: ${current.accountId}
```
<!--End PulumiCodeChooser -->
?
	meshOwnerB" ,AWS account ID of the service mesh's owner.
&
name" Name of the service mesh.

tagsB2" Map of tags.
"$
arn" ARN of the service mesh.
"6
createdDate" #Creation date of the service mesh.
"E
id" ;The provider-assigned unique ID for this managed resource.
"=
lastUpdatedDate" &Last update date of the service mesh.
"
	meshOwner" "

name" "6
resourceOwner" !Resource owner's AWS account ID.
"�
specsA*?:=
;
appmeshgetMeshSpec#aws:appmesh/getMeshSpec:getMeshSpecMService mesh specification. See the `aws.appmesh.Mesh` resource for details.
"
tags2" Map of tags.
2�
2
appmeshgetRouteaws:appmesh/getRoute:getRoute�	The App Mesh Route data source allows details of an App Mesh Route to be retrieved by its name, mesh_name, virtual_router_name, and optionally the mesh_owner.

## Example Usage

<!--Start PulumiCodeChooser -->
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetVirtualServiceArgs;
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
        final var test = AppmeshFunctions.getVirtualService(GetVirtualServiceArgs.builder()
            .name("test-route")
            .meshName("test-mesh")
            .virtualRouterName("test-router")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:appmesh:getVirtualService
      arguments:
        name: test-route
        meshName: test-mesh
        virtualRouterName: test-router
```
<!--End PulumiCodeChooser -->
M
meshName" =Name of the service mesh in which the virtual router exists.
?
	meshOwnerB" ,AWS account ID of the service mesh's owner.

name" Name of the route.

tagsB2" Map of tags.
O
virtualRouterName" 6Name of the virtual router in which the route exists.
"
arn" ARN of the route.
"/
createdDate" Creation date of the route.
"E
id" ;The provider-assigned unique ID for this managed resource.
"6
lastUpdatedDate" Last update date of the route.
"
meshName" "
	meshOwner" "

name" "6
resourceOwner" !Resource owner's AWS account ID.
"�
specsD*B:@
>
appmeshgetRouteSpec%aws:appmesh/getRouteSpec:getRouteSpecGRoute specification. See the `aws.appmesh.Route` resource for details.
"
tags2" Map of tags.
"
virtualRouterName" 2�!
M
appmeshgetVirtualGateway/aws:appmesh/getVirtualGateway:getVirtualGateway�Data source for managing an AWS App Mesh Virtual Gateway.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.appmesh.getVirtualGateway({
    meshName: "mesh-gateway",
    name: "example-mesh",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.appmesh.get_virtual_gateway(mesh_name="mesh-gateway",
    name="example-mesh")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.AppMesh.GetVirtualGateway.Invoke(new()
    {
        MeshName = "mesh-gateway",
        Name = "example-mesh",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.LookupVirtualGateway(ctx, &appmesh.LookupVirtualGatewayArgs{
			MeshName: "mesh-gateway",
			Name:     "example-mesh",
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
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetVirtualGatewayArgs;
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
        final var example = AppmeshFunctions.getVirtualGateway(GetVirtualGatewayArgs.builder()
            .meshName("mesh-gateway")
            .name("example-mesh")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:appmesh:getVirtualGateway
      arguments:
        meshName: mesh-gateway
        name: example-mesh
```
<!--End PulumiCodeChooser -->

<!--Start PulumiCodeChooser -->
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetVirtualGatewayArgs;
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

        final var test = AppmeshFunctions.getVirtualGateway(GetVirtualGatewayArgs.builder()
            .name("example.mesh.local")
            .meshName("example-mesh")
            .meshOwner(current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()))
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  test:
    fn::invoke:
      function: aws:appmesh:getVirtualGateway
      arguments:
        name: example.mesh.local
        meshName: example-mesh
        meshOwner: ${current.accountId}
```
<!--End PulumiCodeChooser -->
N
meshName" >Name of the service mesh in which the virtual gateway exists.
)
name" Name of the virtual gateway.

tagsB2" Map of tags.
"'
arn" ARN of the virtual gateway.
"9
createdDate" &Creation date of the virtual gateway.
"E
id" ;The provider-assigned unique ID for this managed resource.
"@
lastUpdatedDate" )Last update date of the virtual gateway.
"
meshName" "
	meshOwner" "

name" "6
resourceOwner" !Resource owner's AWS account ID.
"�
specs_*]:[
Y
appmeshgetVirtualGatewaySpec7aws:appmesh/getVirtualGatewaySpec:getVirtualGatewaySpecZVirtual gateway specification. See the `aws.appmesh.VirtualGateway` resource for details.
"
tags2" Map of tags.
2�
D
appmeshgetVirtualNode)aws:appmesh/getVirtualNode:getVirtualNode�Data source for managing an AWS App Mesh Virtual Node.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.appmesh.getVirtualNode({
    name: "serviceBv1",
    meshName: "example-mesh",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.appmesh.get_virtual_node(name="serviceBv1",
    mesh_name="example-mesh")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.AppMesh.GetVirtualNode.Invoke(new()
    {
        Name = "serviceBv1",
        MeshName = "example-mesh",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.LookupVirtualNode(ctx, &appmesh.LookupVirtualNodeArgs{
			Name:     "serviceBv1",
			MeshName: "example-mesh",
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
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetVirtualNodeArgs;
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
        final var test = AppmeshFunctions.getVirtualNode(GetVirtualNodeArgs.builder()
            .name("serviceBv1")
            .meshName("example-mesh")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:appmesh:getVirtualNode
      arguments:
        name: serviceBv1
        meshName: example-mesh
```
<!--End PulumiCodeChooser -->
K
meshName" ;Name of the service mesh in which the virtual node exists.
?
	meshOwnerB" ,AWS account ID of the service mesh's owner.
&
name" Name of the virtual node.

tagsB2" Map of tags.
"$
arn" ARN of the virtual node.
"6
createdDate" #Creation date of the virtual node.
"E
id" ;The provider-assigned unique ID for this managed resource.
"=
lastUpdatedDate" &Last update date of the virtual node.
"
meshName" "
	meshOwner" "

name" "6
resourceOwner" !Resource owner's AWS account ID.
"�
specsV*T:R
P
appmeshgetVirtualNodeSpec1aws:appmesh/getVirtualNodeSpec:getVirtualNodeSpecTVirtual node specification. See the `aws.appmesh.VirtualNode` resource for details.
"
tags2" Map of tags.
2�
J
appmeshgetVirtualRouter-aws:appmesh/getVirtualRouter:getVirtualRouter�The App Mesh Virtual Router data source allows details of an App Mesh Virtual Service to be retrieved by its name and mesh_name.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.appmesh.getVirtualRouter({
    name: "example-router-name",
    meshName: "example-mesh-name",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.appmesh.get_virtual_router(name="example-router-name",
    mesh_name="example-mesh-name")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.AppMesh.GetVirtualRouter.Invoke(new()
    {
        Name = "example-router-name",
        MeshName = "example-mesh-name",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.LookupVirtualRouter(ctx, &appmesh.LookupVirtualRouterArgs{
			Name:     "example-router-name",
			MeshName: "example-mesh-name",
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
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetVirtualRouterArgs;
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
        final var test = AppmeshFunctions.getVirtualRouter(GetVirtualRouterArgs.builder()
            .name("example-router-name")
            .meshName("example-mesh-name")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:appmesh:getVirtualRouter
      arguments:
        name: example-router-name
        meshName: example-mesh-name
```
<!--End PulumiCodeChooser -->
D
meshName" 4Name of the mesh in which the virtual router exists

	meshOwnerB" (
name" Name of the virtual router.

tagsB2" Map of tags.
"&
arn" ARN of the virtual router.
"8
createdDate" %Creation date of the virtual router.
"E
id" ;The provider-assigned unique ID for this managed resource.
"?
lastUpdatedDate" (Last update date of the virtual router.
"
meshName" "
	meshOwner" "

name" "6
resourceOwner" !Resource owner's AWS account ID.
"�
specs\*Z:X
V
appmeshgetVirtualRouterSpec5aws:appmesh/getVirtualRouterSpec:getVirtualRouterSpecYVirtual routers specification. See the `aws.appmesh.VirtualRouter` resource for details.
"
tags2" Map of tags.
2�/
M
appmeshgetVirtualService/aws:appmesh/getVirtualService:getVirtualService�(The App Mesh Virtual Service data source allows details of an App Mesh Virtual Service to be retrieved by its name, mesh_name, and optionally the mesh_owner.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.appmesh.getVirtualService({
    name: "example.mesh.local",
    meshName: "example-mesh",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.appmesh.get_virtual_service(name="example.mesh.local",
    mesh_name="example-mesh")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.AppMesh.GetVirtualService.Invoke(new()
    {
        Name = "example.mesh.local",
        MeshName = "example-mesh",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := appmesh.LookupVirtualService(ctx, &appmesh.LookupVirtualServiceArgs{
			Name:     "example.mesh.local",
			MeshName: "example-mesh",
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
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetVirtualServiceArgs;
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
        final var test = AppmeshFunctions.getVirtualService(GetVirtualServiceArgs.builder()
            .name("example.mesh.local")
            .meshName("example-mesh")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:appmesh:getVirtualService
      arguments:
        name: example.mesh.local
        meshName: example-mesh
```
<!--End PulumiCodeChooser -->

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
const test = current.then(current => aws.appmesh.getVirtualService({
    name: "example.mesh.local",
    meshName: "example-mesh",
    meshOwner: current.accountId,
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_caller_identity()
test = aws.appmesh.get_virtual_service(name="example.mesh.local",
    mesh_name="example-mesh",
    mesh_owner=current.account_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    var test = Aws.AppMesh.GetVirtualService.Invoke(new()
    {
        Name = "example.mesh.local",
        MeshName = "example-mesh",
        MeshOwner = current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/appmesh"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = appmesh.LookupVirtualService(ctx, &appmesh.LookupVirtualServiceArgs{
			Name:      "example.mesh.local",
			MeshName:  "example-mesh",
			MeshOwner: pulumi.StringRef(current.AccountId),
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
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.appmesh.AppmeshFunctions;
import com.pulumi.aws.appmesh.inputs.GetVirtualServiceArgs;
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

        final var test = AppmeshFunctions.getVirtualService(GetVirtualServiceArgs.builder()
            .name("example.mesh.local")
            .meshName("example-mesh")
            .meshOwner(current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()))
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  test:
    fn::invoke:
      function: aws:appmesh:getVirtualService
      arguments:
        name: example.mesh.local
        meshName: example-mesh
        meshOwner: ${current.accountId}
```
<!--End PulumiCodeChooser -->
N
meshName" >Name of the service mesh in which the virtual service exists.
?
	meshOwnerB" ,AWS account ID of the service mesh's owner.
)
name" Name of the virtual service.

tagsB2" Map of tags.
"'
arn" ARN of the virtual service.
"9
createdDate" &Creation date of the virtual service.
"E
id" ;The provider-assigned unique ID for this managed resource.
"@
lastUpdatedDate" )Last update date of the virtual service.
"
meshName" "
	meshOwner" "

name" "6
resourceOwner" !Resource owner's AWS account ID.
"�
specs_*]:[
Y
appmeshgetVirtualServiceSpec7aws:appmesh/getVirtualServiceSpec:getVirtualServiceSpecZVirtual service specification. See the `aws.appmesh.VirtualService` resource for details.
"
tags2" Map of tags.
2�#
K
	apprunnergetHostedZoneId-aws:apprunner/getHostedZoneId:getHostedZoneId�!Use this data source to get the HostedZoneId of an AWS App Runner service deployed
in a given region for the purpose of using it in an AWS Route53 Alias record.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.apprunner.getHostedZoneId({});
const www = new aws.route53.Record("www", {
    zoneId: primary.zoneId,
    name: "example.com",
    type: aws.route53.RecordType.A,
    aliases: [{
        name: mainAwsApprunnerCustomDomainAssociation.dnsTarget,
        zoneId: main.then(main => main.id),
        evaluateTargetHealth: true,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.apprunner.get_hosted_zone_id()
www = aws.route53.Record("www",
    zone_id=primary["zoneId"],
    name="example.com",
    type=aws.route53.RecordType.A,
    aliases=[{
        "name": main_aws_apprunner_custom_domain_association["dnsTarget"],
        "zone_id": main.id,
        "evaluate_target_health": True,
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.AppRunner.GetHostedZoneId.Invoke();

    var www = new Aws.Route53.Record("www", new()
    {
        ZoneId = primary.ZoneId,
        Name = "example.com",
        Type = Aws.Route53.RecordType.A,
        Aliases = new[]
        {
            new Aws.Route53.Inputs.RecordAliasArgs
            {
                Name = mainAwsApprunnerCustomDomainAssociation.DnsTarget,
                ZoneId = main.Apply(getHostedZoneIdResult => getHostedZoneIdResult.Id),
                EvaluateTargetHealth = true,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/apprunner"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/route53"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		main, err := apprunner.GetHostedZoneId(ctx, &apprunner.GetHostedZoneIdArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = route53.NewRecord(ctx, "www", &route53.RecordArgs{
			ZoneId: pulumi.Any(primary.ZoneId),
			Name:   pulumi.String("example.com"),
			Type:   pulumi.String(route53.RecordTypeA),
			Aliases: route53.RecordAliasArray{
				&route53.RecordAliasArgs{
					Name:                 pulumi.Any(mainAwsApprunnerCustomDomainAssociation.DnsTarget),
					ZoneId:               pulumi.String(main.Id),
					EvaluateTargetHealth: pulumi.Bool(true),
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
import com.pulumi.aws.apprunner.ApprunnerFunctions;
import com.pulumi.aws.apprunner.inputs.GetHostedZoneIdArgs;
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
        final var main = ApprunnerFunctions.getHostedZoneId();

        var www = new Record("www", RecordArgs.builder()
            .zoneId(primary.zoneId())
            .name("example.com")
            .type("A")
            .aliases(RecordAliasArgs.builder()
                .name(mainAwsApprunnerCustomDomainAssociation.dnsTarget())
                .zoneId(main.applyValue(getHostedZoneIdResult -> getHostedZoneIdResult.id()))
                .evaluateTargetHealth(true)
                .build())
            .build());

    }
}
```
```yaml
resources:
  www:
    type: aws:route53:Record
    properties:
      zoneId: ${primary.zoneId}
      name: example.com
      type: A
      aliases:
        - name: ${mainAwsApprunnerCustomDomainAssociation.dnsTarget}
          zoneId: ${main.id}
          evaluateTargetHealth: true
variables:
  main:
    fn::invoke:
      function: aws:apprunner:getHostedZoneId
      arguments: {}
```
<!--End PulumiCodeChooser -->
�
regionB" �Name of the region whose AWS App Runner service HostedZoneId is desired.
Defaults to the region from the AWS provider configuration.
"P
id" FID of the AWS App Runner service HostedZoneId in the selected region.
"
region" 2�
6
	appstreamgetImageaws:appstream/getImage:getImage5Data source for managing an AWS AppStream 2.0 Image.
Z
arnB" MArn of the image being searched for. Cannot be used with name_regex or name.
�

mostRecentB
 �Boolean that if it is set to true and there are multiple images returned the most recent will be returned. If it is set to false and there are multiple images return the datasource will error.
[
nameB" MName of the image being searched for. Cannot be used with name_regex or arn.
m
	nameRegexB" ZRegular expression name of the image being searched for. Cannot be used with arn or name.
L
typeB" >The type of image which must be (PUBLIC, PRIVATE, or SHARED).
"m
applications]*[:Y
W
	appstreamgetImageApplication5aws:appstream/getImageApplication:getImageApplication"�
appstreamAgentVersion" �Version of the AppStream 2.0 agent to use for instances that are launched from this image. Has a maximum length of 100 characters.
"
arn" ARN of the image.
"G
baseImageArn" 3ARN of the image from which the image was created.
"9
createdTime" &Time at which this image was created.
")
description" Description of image.
"*
displayName" Image name to display.
"E
id" ;The provider-assigned unique ID for this managed resource.
"�
imageBuilderName" xThe name of the image builder that was used to created the private image. If the image is sharedthen the value is null.
"�
imageBuilderSupported
 �Boolean to indicate whether an image builder can be launched from this image.
* `image error` - Resource error object that describes the error containing the following:
"�
imagePermissionsi*g:e
c
	appstreamgetImageImagePermission=aws:appstream/getImageImagePermission:getImageImagePermissionKList of strings describing the image permissions containing the following:
"

mostRecentB
 "

name" "
	nameRegexB" "�
platform" �Operating system platform of the image. Values will be from: WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019 | WINDOWS_SERVER_2022 | AMAZON_LINUX2
"!
publicBaseImageReleasedDate" "�
state" �Current state of image. Image starts in PENDING state which changes to AVAILABLE if creation passes and FAILED if it fails. Values will be from: PENDING | AVAILABLE | FAILED | COPYING | DELETING | CREATING | IMPORTING.
"�
stateChangeReasonso*m:k
i
	appstreamgetImageStateChangeReasonAaws:appstream/getImageStateChangeReason:getImageStateChangeReason"
typeB" :�
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
u
	appconfigConfigurationProfileValidatorIaws:appconfig/ConfigurationProfileValidator:ConfigurationProfileValidator�
�V
contentB" EEither the JSON Schema content or the ARN of an AWS Lambda function.
I
type" =Type of validator. Valid values: `JSON_SCHEMA` and `LAMBDA`.
:�
T
	appconfigEnvironmentMonitor3aws:appconfig/EnvironmentMonitor:EnvironmentMonitor�
�4
alarmArn" $ARN of the Amazon CloudWatch alarm.
S
alarmRoleArnB" =ARN of an IAM role for AWS AppConfig to monitor `alarm_arn`.
:�
o
	appconfigEventIntegrationEventFilterEaws:appconfig/EventIntegrationEventFilter:EventIntegrationEventFilter(
&$
source" Source of the events.
:�
Z
	appconfigExtensionActionPoint7aws:appconfig/ExtensionActionPoint:ExtensionActionPoint�
��
actionsr*p:n
l
	appconfigExtensionActionPointActionCaws:appconfig/ExtensionActionPointAction:ExtensionActionPointActionbAn action defines the tasks the extension performs during the AppConfig workflow. Detailed below.
�
point" �The point at which to perform the defined actions. Valid points are `PRE_CREATE_HOSTED_CONFIGURATION_VERSION`, `PRE_START_DEPLOYMENT`, `ON_DEPLOYMENT_START`, `ON_DEPLOYMENT_STEP`, `ON_DEPLOYMENT_BAKING`, `ON_DEPLOYMENT_COMPLETE`, `ON_DEPLOYMENT_ROLLED_BACK`.
:�
l
	appconfigExtensionActionPointActionCaws:appconfig/ExtensionActionPointAction:ExtensionActionPointAction�
�3
descriptionB" Information about the action.

name" The action name.
b
roleArnB" QAn Amazon Resource Name (ARN) for an Identity and Access Management assume role.
�
uri" �The extension URI associated to the action point in the extension definition. The URI can be an Amazon Resource Name (ARN) for one of the following: an Lambda function, an Amazon Simple Queue Service queue, an Amazon Simple Notification Service topic, or the Amazon EventBridge default event bus.
:�
T
	appconfigExtensionParameter3aws:appconfig/ExtensionParameter:ExtensionParameter�
�6
descriptionB" !Information about the parameter.
 
name" The parameter name.
b
requiredB
 PDetermines if a parameter value must be specified in the extension association.
:�
~
	appconfig getConfigurationProfileValidatorOaws:appconfig/getConfigurationProfileValidator:getConfigurationProfileValidator�
�T
content" EEither the JSON Schema content or the ARN of an AWS Lambda function.
E
type" 9Type of validator. Valid values: JSON_SCHEMA and LAMBDA.
:�
]
	appconfiggetEnvironmentMonitor9aws:appconfig/getEnvironmentMonitor:getEnvironmentMonitor
}4
alarmArn" $ARN of the Amazon CloudWatch alarm.
E
alarmRoleArn" 1ARN of an IAM role for AWS AppConfig to monitor.
:�
�
	appfabric%AppAuthorizationConnectionAuthRequestYaws:appfabric/AppAuthorizationConnectionAuthRequest:AppAuthorizationConnectionAuthRequest�
��
code" �The authorization code returned by the application after permission is granted in the application OAuth page (after clicking on the AuthURL)..
a
redirectUri" NThe redirect URL that is specified in the AuthURL and the application client.
:�
~
	appfabric AppAuthorizationConnectionTenantOaws:appfabric/AppAuthorizationConnectionTenant:AppAuthorizationConnectionTenant3
1
tenantDisplayName" 
tenantIdentifier" :�
�
	appfabric"AppAuthorizationConnectionTimeoutsSaws:appfabric/AppAuthorizationConnectionTimeouts:AppAuthorizationConnectionTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
l
	appfabricAppAuthorizationCredentialCaws:appfabric/AppAuthorizationCredential:AppAuthorizationCredential�
��
apiKeyCredentials�B�*�:�
�
	appfabric*AppAuthorizationCredentialApiKeyCredentialcaws:appfabric/AppAuthorizationCredentialApiKeyCredential:AppAuthorizationCredentialApiKeyCredential)Contains API key credential information.
�
oauth2Credential�B�:�
�
	appfabric*AppAuthorizationCredentialOauth2Credentialcaws:appfabric/AppAuthorizationCredentialOauth2Credential:AppAuthorizationCredentialOauth2Credential/Contains OAuth2 client credential information.
:�
�
	appfabric*AppAuthorizationCredentialApiKeyCredentialcaws:appfabric/AppAuthorizationCredentialApiKeyCredential:AppAuthorizationCredentialApiKeyCredential;
97
apiKey" )Contains API key credential information.
:�
�
	appfabric*AppAuthorizationCredentialOauth2Credentialcaws:appfabric/AppAuthorizationCredentialOauth2Credential:AppAuthorizationCredentialOauth2Credential�
~9
clientId" )The client ID of the client application.
A
clientSecret" -The client secret of the client application.
:�
`
	appfabricAppAuthorizationTenant;aws:appfabric/AppAuthorizationTenant:AppAuthorizationTenanty
w9
tenantDisplayName"  The display name of the tenant.
:
tenantIdentifier" "The ID of the application tenant.
:�
f
	appfabricAppAuthorizationTimeouts?aws:appfabric/AppAuthorizationTimeouts:AppAuthorizationTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
�
	appfabric,IngestionDestinationDestinationConfigurationgaws:appfabric/IngestionDestinationDestinationConfiguration:IngestionDestinationDestinationConfiguration�
��
auditLog�B�:�
�
	appfabric4IngestionDestinationDestinationConfigurationAuditLogwaws:appfabric/IngestionDestinationDestinationConfigurationAuditLog:IngestionDestinationDestinationConfigurationAuditLogBContains information about an audit log processing configuration.
:�
�
	appfabric4IngestionDestinationDestinationConfigurationAuditLogwaws:appfabric/IngestionDestinationDestinationConfigurationAuditLog:IngestionDestinationDestinationConfigurationAuditLog�
��
destination�B�:�
�
	appfabric?IngestionDestinationDestinationConfigurationAuditLogDestination�aws:appfabric/IngestionDestinationDestinationConfigurationAuditLogDestination:IngestionDestinationDestinationConfigurationAuditLogDestination}Contains information about an audit log destination. Only one destination (Firehose Stream) or (S3 Bucket) can be specified.
:�
�
	appfabric?IngestionDestinationDestinationConfigurationAuditLogDestination�aws:appfabric/IngestionDestinationDestinationConfigurationAuditLogDestination:IngestionDestinationDestinationConfigurationAuditLogDestination�
��
firehoseStream�B�:�
�
	appfabricMIngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream�aws:appfabric/IngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream:IngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStreamDContains information about an Amazon Data Firehose delivery stream.
�
s3Bucket�B�:�
�
	appfabricGIngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket�aws:appfabric/IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket:IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket0Contains information about an Amazon S3 bucket.
:�
�
	appfabricMIngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream�aws:appfabric/IngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream:IngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream


streamName" :�
�
	appfabricGIngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket�aws:appfabric/IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket:IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket=
;

bucketName" '
prefixB" The object key to use.
:�
�
	appfabric+IngestionDestinationProcessingConfigurationeaws:appfabric/IngestionDestinationProcessingConfiguration:IngestionDestinationProcessingConfiguration�
��
auditLog�B�:�
�
	appfabric3IngestionDestinationProcessingConfigurationAuditLoguaws:appfabric/IngestionDestinationProcessingConfigurationAuditLog:IngestionDestinationProcessingConfigurationAuditLogBContains information about an audit log processing configuration.
:�
�
	appfabric3IngestionDestinationProcessingConfigurationAuditLoguaws:appfabric/IngestionDestinationProcessingConfigurationAuditLog:IngestionDestinationProcessingConfigurationAuditLog�
�h
format" ZThe format in which the audit logs need to be formatted. Valid values: `json`, `parquet`.
j
schema" \The event schema in which the audit logs need to be formatted. Valid values: `ocsf`, `raw`.
:�
r
	appfabricIngestionDestinationTimeoutsGaws:appfabric/IngestionDestinationTimeouts:IngestionDestinationTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
�
appflow&ConnectorProfileConnectorProfileConfigYaws:appflow/ConnectorProfileConnectorProfileConfig:ConnectorProfileConnectorProfileConfig�
��
connectorProfileCredentials�:�
�
appflowAConnectorProfileConnectorProfileConfigConnectorProfileCredentials�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentials:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialssThe connector-specific credentials required by each connector. See Connector Profile Credentials for more details.
�
connectorProfileProperties�:�
�
appflow@ConnectorProfileConnectorProfileConfigConnectorProfileProperties�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileProperties:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiessThe connector-specific properties of the profile configuration. See Connector Profile Properties for more details.
:�:
�
appflowAConnectorProfileConnectorProfileConfigConnectorProfileCredentials�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentials:ConnectorProfileConnectorProfileConfigConnectorProfileCredentials�8
�8�
	amplitude�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude�The connector-specific credentials required when using Amplitude. See Amplitude Connector Profile Credentials for more details.
�
customConnector�B�:�
�
appflowPConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector�The connector-specific profile credentials required when using the custom connector. See Custom Connector Profile Credentials for more details.
�
datadog�B�:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadogxConnector-specific credentials required when using Datadog. See Datadog Connector Profile Credentials for more details.
�
	dynatrace�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace�The connector-specific credentials required when using Dynatrace. See Dynatrace Connector Profile Credentials for more details.
�
googleAnalytics�B�:�
�
appflowPConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics�The connector-specific credentials required when using Google Analytics. See Google Analytics Connector Profile Credentials for more details.
�
	honeycode�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode�The connector-specific credentials required when using Amazon Honeycode. See Honeycode Connector Profile Credentials for more details.
�

inforNexus�B�:�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus�The connector-specific credentials required when using Infor Nexus. See Infor Nexus Connector Profile Credentials for more details.
�
marketo�B�:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketo�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketo:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoxConnector-specific credentials required when using Marketo. See Marketo Connector Profile Credentials for more details.
�
redshift�B�:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift�Connector-specific credentials required when using Amazon Redshift. See Redshift Connector Profile Credentials for more details.
�

salesforce�B�:�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce�The connector-specific credentials required when using Salesforce. See Salesforce Connector Profile Credentials for more details.
�
sapoData�B�:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData~The connector-specific credentials required when using SAPOData. See SAPOData Connector Profile Credentials for more details.
�

serviceNow�B�:�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow�The connector-specific credentials required when using ServiceNow. See ServiceNow Connector Profile Credentials for more details.
�
singular�B�:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingular�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingular:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingularzConnector-specific credentials required when using Singular. See Singular Connector Profile Credentials for more details.
�
slack�B�:�
�
appflowFConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlack�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlack:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlacktConnector-specific credentials required when using Slack. See Slack Connector Profile Credentials for more details.
�
	snowflake�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake�The connector-specific credentials required when using Snowflake. See Snowflake Connector Profile Credentials for more details.
�

trendmicro�B�:�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro�The connector-specific credentials required when using Trend Micro. See Trend Micro Connector Profile Credentials for more details.
�
veeva�B�:�
�
appflowFConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeeva�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeeva:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeevatConnector-specific credentials required when using Veeva. See Veeva Connector Profile Credentials for more details.
�
zendesk�B�:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendesk�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendesk:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskxConnector-specific credentials required when using Zendesk. See Zendesk Connector Profile Credentials for more details.
:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitudeU
S
apiKey" C
	secretKey" 2The Secret Access Key portion of the credentials.
:�
�
appflowPConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector�
��
apiKey�B�:�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey�
authenticationType" �The authentication type that the custom connector uses for authenticating while creating a connector profile. One of: `APIKEY`, `BASIC`, `CUSTOM`, `OAUTH2`.
�
basic�B�:�
�
appflowUConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasic�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasic:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasicHBasic credentials that are required for the authentication of the user.
�
custom�B�:�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom`If the connector uses the custom authentication mechanism, this holds the required credentials.
�
oauth2�B�:�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2COAuth 2.0 credentials required for the authentication of the user.
:�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey&
$
apiKey" 
apiSecretKeyB" :�
�
appflowUConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasic�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasic:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasic"
 
password" 
username" :�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom�
�N
credentialsMapB2" 4A map that holds custom authentication credentials.
X
customAuthenticationType" 8The custom authentication type that the connector uses.
:�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2�
�
accessTokenB" 
clientIdB" 
clientSecretB" �
oauthRequest�B�:�
�
appflowbConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2OauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2OauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2OauthRequest
refreshTokenB" :�
�
appflowbConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2OauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2OauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2OauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog�
�
apiKey" �
applicationKey" �Application keys, in conjunction with your API key, give you full access to Datadog’s programmatic API. Application keys are associated with the user account that created them. The application key is used to log all requests made to the API.
:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace\
ZX
apiToken" HThe API tokens used by Dynatrace API to authenticate various API calls.
:�
�
appflowPConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics�
�
accessTokenB" 
clientId" 
clientSecret" �
oauthRequest�B�:�
�
appflow\ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalyticsOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalyticsOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalyticsOauthRequest
refreshTokenB" :�
�
appflow\ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalyticsOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalyticsOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalyticsOauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode�
�
accessTokenB" �
oauthRequest�B�:�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest
refreshTokenB" :�
�
appflowVConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus�
�>
accessKeyId" +The Access Key portion of the credentials.
5
datakey" &Encryption keys used to encrypt data.
=
secretAccessKey" &The secret key used to sign requests.
'
userId" Identifier for the user.
:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketo�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketo:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketo�
�
accessTokenB" 
clientId" 
clientSecret" �
oauthRequest�B�:�
�
appflowTConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoOauthRequest:�
�
appflowTConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketoOauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift"
 
password" 
username" :�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce�
�
accessTokenB" {
clientCredentialsArnB" ]The secret manager ARN, which contains the client ID and client secret of the connected app.
W
jwtTokenB" EA JSON web token (JWT) that authorizes access to Salesforce records.

oauth2GrantTypeB" �
oauthRequest�B�:�
�
appflowWConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest
refreshTokenB" :�
�
appflowWConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData�
��
basicAuthCredentials�B�:�
�
appflow]ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials/The SAPOData basic authentication credentials.
�
oauthCredentials�B�:�
�
appflowYConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials4The SAPOData OAuth type authentication credentials.
:�
�
appflow]ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials"
 
password" 
username" :�
�
appflowYConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials�
�
accessTokenB" 
clientId" 
clientSecret" �
oauthRequest�B�:�
�
appfloweConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentialsOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentialsOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentialsOauthRequest
refreshTokenB" :�
�
appfloweConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentialsOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentialsOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentialsOauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow"
 
password" 
username" :�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingular�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingular:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingular

apiKey" :�
�
appflowFConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlack�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlack:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlack�
�
accessTokenB" 
clientId" 
clientSecret" �
oauthRequest�B�:�
�
appflowRConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlackOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlackOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlackOauthRequest:�
�
appflowRConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlackOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlackOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlackOauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake"
 
password" 
username" :�
�
appflowKConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro

apiSecretKey" :�
�
appflowFConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeeva�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeeva:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeeva"
 
password" 
username" :�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendesk�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendesk:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendesk�
�
accessTokenB" 
clientId" 
clientSecret" �
oauthRequest�B�:�
�
appflowTConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskOauthRequest:�
�
appflowTConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskOauthRequest�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskOauthRequest:ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskOauthRequest�
�k
authCodeB" YThe code provided by the connector when it has been authenticated via the connected app.
|
redirectUriB" gThe URL to which the authentication server redirects the browser after authorization has been granted.
:�:
�
appflow@ConnectorProfileConnectorProfileConfigConnectorProfileProperties�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfileProperties:ConnectorProfileConnectorProfileConfigConnectorProfileProperties�8
�8�
	amplitude�B�:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude�The connector-specific credentials required when using Amplitude. See Amplitude Connector Profile Credentials for more details.
�
customConnector�B�:�
�
appflowOConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector�The connector-specific profile properties required when using the custom connector. See Custom Connector Profile Properties for more details.
�
datadog�B�:�
�
appflowGConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadogvConnector-specific properties required when using Datadog. See Generic Connector Profile Properties for more details.
�
	dynatrace�B�:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace|The connector-specific properties required when using Dynatrace. See Generic Connector Profile Properties for more details.
�
googleAnalytics�B�:�
�
appflowOConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics�The connector-specific credentials required when using Google Analytics. See Google Analytics Connector Profile Credentials for more details.
�
	honeycode�B�:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode�The connector-specific credentials required when using Amazon Honeycode. See Honeycode Connector Profile Credentials for more details.
�

inforNexus�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus~The connector-specific properties required when using Infor Nexus. See Generic Connector Profile Properties for more details.
�
marketo�B�:�
�
appflowGConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketovConnector-specific properties required when using Marketo. See Generic Connector Profile Properties for more details.
�
redshift�B�:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshiftConnector-specific properties required when using Amazon Redshift. See Redshift Connector Profile Properties for more details.
�

salesforce�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce�The connector-specific properties required when using Salesforce. See Salesforce Connector Profile Properties for more details.
�
sapoData�B�:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData|The connector-specific properties required when using SAPOData. See SAPOData Connector Profile Properties for more details.
�

serviceNow�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow}The connector-specific properties required when using ServiceNow. See Generic Connector Profile Properties for more details.
�
singular�B�:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingularzConnector-specific credentials required when using Singular. See Singular Connector Profile Credentials for more details.
�
slack�B�:�
�
appflowEConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlacktConnector-specific properties required when using Slack. See Generic Connector Profile Properties for more details.
�
	snowflake�B�:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake~The connector-specific properties required when using Snowflake. See Snowflake Connector Profile Properties for more details.
�

trendmicro�B�:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro�The connector-specific credentials required when using Trend Micro. See Trend Micro Connector Profile Credentials for more details.
�
veeva�B�:�
�
appflowEConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeevatConnector-specific properties required when using Veeva. See Generic Connector Profile Properties for more details.
�
zendesk�B�:�
�
appflowGConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendeskvConnector-specific properties required when using Zendesk. See Generic Connector Profile Properties for more details.
:�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude
 :�
�
appflowOConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector�
��
oauth2Properties�B�:�
�
appflow_ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties@The OAuth 2.0 properties required for OAuth 2.0 authentication.
q
profilePropertiesB2" TA map of properties that are required to create a profile for the custom connector.
:�
�
appflow_ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties�
�
oauth2GrantType" 
tokenUrl" �
tokenUrlCustomPropertiesB2" �Associates your token URL with a map of properties that you define. Use this parameter to provide any additional details that the connector requires to authenticate your request.
:�
�
appflowGConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog

instanceUrl" :�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace

instanceUrl" :�
�
appflowOConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics
 :�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode
 :�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus

instanceUrl" :�
�
appflowGConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo

instanceUrl" :�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift�
�

bucketName" 
bucketPrefixB" X
clusterIdentifierB" =The unique ID that's assigned to an Amazon Redshift cluster.
j
dataApiRoleArnB" RARN of the IAM role that permits AppFlow to access the database through Data API.
?
databaseNameB" )The name of an Amazon Redshift database.
B
databaseUrlB" -The JDBC URL of the Amazon Redshift cluster.
$
roleArn" ARN of the IAM role.
:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce�
�
instanceUrlB" v
isSandboxEnvironmentB
 XIndicates whether the connector profile applies to a sandbox or production environment.
:�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData�
�A
applicationHostUrl" 'The location of the SAPOData resource.
G
applicationServicePath" )The application path to catalog service.
N
clientNumber" :The client number for the client creating the connection.
@
logonLanguageB" )The logon language of SAPOData instance.
�
oauthProperties�B�:�
�
appflowWConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthPropertiesFThe SAPOData OAuth properties required for OAuth type authentication.
<

portNumber *The port number of the SAPOData instance.

privateLinkServiceNameB" :�
�
appflowWConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties�
��
authCodeUrl" }The authorization code url required to redirect to SAP Login Page to fetch authorization code for OAuth type authentication.
N
oauthScopes*" 9The OAuth scopes required for OAuth type authentication.

tokenUrl" :�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow

instanceUrl" :�
�
appflowHConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular
 :�
�
appflowEConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack

instanceUrl" :�
�
appflowIConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake�
�.
accountNameB" The name of the account.


bucketName" 
bucketPrefixB" 
privateLinkServiceNameB" 5
regionB" %AWS Region of the Snowflake account.
�
stage" �Name of the Amazon S3 stage that was created while setting up an Amazon S3 stage in the Snowflake account. This is written in the following format: `<Database>.<Schema>.<Stage Name>`.
6
	warehouse" %The name of the Snowflake warehouse.
:�
�
appflowJConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro
 :�
�
appflowEConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva

instanceUrl" :�
�
appflowGConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk�aws:appflow/ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk:ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk

instanceUrl" :�
e
appflowFlowDestinationFlowConfig?aws:appflow/FlowDestinationFlowConfig:FlowDestinationFlowConfig�
�E

apiVersionB" 1API version that the destination connector uses.
�
connectorProfileNameB" gName of the connector profile. This name must be unique for each connector profile in the AWS account.
�
connectorType" �Type of connector, such as Salesforce, Amplitude, and so on. Valid values are `Salesforce`, `Singular`, `Slack`, `Redshift`, `S3`, `Marketo`, `Googleanalytics`, `Zendesk`, `Servicenow`, `Datadog`, `Trendmicro`, `Snowflake`, `Dynatrace`, `Infornexus`, `Amplitude`, `Veeva`, `EventBridge`, `LookoutMetrics`, `Upsolver`, `Honeycode`, `CustomerProfiles`, `SAPOData`, and `CustomConnector`.
�
destinationConnectorProperties�:�
�
appflow7FlowDestinationFlowConfigDestinationConnectorProperties{aws:appflow/FlowDestinationFlowConfigDestinationConnectorProperties:FlowDestinationFlowConfigDestinationConnectorProperties�This stores the information that is required to query a particular connector. See Destination Connector Properties for more information.
:�$
�
appflow7FlowDestinationFlowConfigDestinationConnectorProperties{aws:appflow/FlowDestinationFlowConfigDestinationConnectorProperties:FlowDestinationFlowConfigDestinationConnectorProperties�#
�#�
customConnector�B�:�
�
appflowFFlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector:FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorzProperties that are required to query the custom Connector. See Custom Connector Destination Properties for more details.
�
customerProfiles�B�:�
�
appflowGFlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles:FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles�Properties that are required to query Amazon Connect Customer Profiles. See Customer Profiles Destination Properties for more details.
�
eventBridge�B�:�
�
appflowBFlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge:FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeoProperties that are required to query Amazon EventBridge. See Generic Destination Properties for more details.
�
	honeycode�B�:�
�
appflow@FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode:FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycodemProperties that are required to query Amazon Honeycode. See Generic Destination Properties for more details.
�
lookoutMetrics�B�:�
�
appflowEFlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics:FlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics�
marketo�B�:�
�
appflow>FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo:FlowDestinationFlowConfigDestinationConnectorPropertiesMarketodProperties that are required to query Marketo. See Generic Destination Properties for more details.
�
redshift�B�:�
�
appflow?FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift:FlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftmProperties that are required to query Amazon Redshift. See Redshift Destination Properties for more details.
�
s3�B�:�
�
appflow9FlowDestinationFlowConfigDestinationConnectorPropertiesS3aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3:FlowDestinationFlowConfigDestinationConnectorPropertiesS3aProperties that are required to query Amazon S3. See S3 Destination Properties for more details.
�

salesforce�B�:�
�
appflowAFlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce:FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforcejProperties that are required to query Salesforce. See Salesforce Destination Properties for more details.
�
sapoData�B�:�
�
appflow?FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData:FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDatafProperties that are required to query SAPOData. See SAPOData Destination Properties for more details.
�
	snowflake�B�:�
�
appflow@FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake:FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakehProperties that are required to query Snowflake. See Snowflake Destination Properties for more details.
�
upsolver�B�:�
�
appflow?FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverfProperties that are required to query Upsolver. See Upsolver Destination Properties for more details.
�
zendesk�B�:�
�
appflow>FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk:FlowDestinationFlowConfigDestinationConnectorPropertiesZendeskdProperties that are required to query Zendesk. See Zendesk Destination Properties for more details.
:�
�
appflowFFlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector:FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector�
�
customPropertiesB2" 

entityName" �
errorHandlingConfig�B�:�
�
appflowYFlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig
idFieldNamesB*" 
writeOperationTypeB" :�
�
appflowYFlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflowGFlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles:FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles�
�N

domainName" <Unique name of the Amazon Connect Customer Profiles domain.
c
objectTypeNameB" KObject specified in the Amazon Connect Customer Profiles flow destination.
:�
�
appflowBFlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge:FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge�
��
errorHandlingConfig�B�:�
�
appflowUFlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeErrorHandlingConfig
object" :�
�
appflowUFlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflow@FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode:FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode�
��
errorHandlingConfig�B�:�
�
appflowSFlowDestinationFlowConfigDestinationConnectorPropertiesHoneycodeErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycodeErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycodeErrorHandlingConfig
object" :�
�
appflowSFlowDestinationFlowConfigDestinationConnectorPropertiesHoneycodeErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycodeErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycodeErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflowEFlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics:FlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics
 :�
�
appflow>FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo:FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo�
��
errorHandlingConfig�B�:�
�
appflowQFlowDestinationFlowConfigDestinationConnectorPropertiesMarketoErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesMarketoErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesMarketoErrorHandlingConfig
object" :�
�
appflowQFlowDestinationFlowConfigDestinationConnectorPropertiesMarketoErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesMarketoErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesMarketoErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflow?FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift:FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift�
�
bucketPrefixB" �
errorHandlingConfig�B�:�
�
appflowRFlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftErrorHandlingConfig
intermediateBucketName" 
object" :�
�
appflowRFlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflow9FlowDestinationFlowConfigDestinationConnectorPropertiesS3aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3:FlowDestinationFlowConfigDestinationConnectorPropertiesS3�
�

bucketName" 
bucketPrefixB" �
s3OutputFormatConfig�B�:�
�
appflowMFlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig:�
�
appflowMFlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig�	
�	�
aggregationConfig�B�:�
�
appflow^FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig�Aggregation settings that you can use to customize the output format of your flow data. See Aggregation Config for more details.

fileTypeB" mFile type that Amazon AppFlow places in the Amazon S3 bucket. Valid values are `CSV`, `JSON`, and `PARQUET`.
�
prefixConfig�B�:�
�
appflowYFlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig�Determines the prefix that Amazon AppFlow applies to the folder name in the Amazon S3 bucket. You can name folders according to the flow frequency and date. See Prefix Config for more details.
�
preserveSourceDataTypingB
 hWhether the data types from the source system need to be preserved (Only valid for `Parquet` file type)
:�
�
appflow^FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig�
��
aggregationTypeB" �Whether Amazon AppFlow aggregates the flow records into a single file, or leave them unaggregated. Valid values are `None` and `SingleFile`.
�
targetFileSizeB vThe desired file size, in MB, for each output file that Amazon AppFlow writes to the flow destination. Integer value.
:�
�
appflowYFlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig�
��
prefixFormatB" �Determines the level of granularity that's included in the prefix. Valid values are `YEAR`, `MONTH`, `DAY`, `HOUR`, and `MINUTE`.
�
prefixHierarchiesB*" �Determines whether the destination file path includes either or both of the selected elements. Valid values are `EXECUTION_ID` and `SCHEMA_VERSION`
�

prefixTypeB" �Determines the format of the prefix, and whether it applies to the file name, file path, or both. Valid values are `FILENAME`, `PATH`, and `PATH_AND_FILENAME`.
:�
�
appflowAFlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce:FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce�
��
errorHandlingConfig�B�:�
�
appflowTFlowDestinationFlowConfigDestinationConnectorPropertiesSalesforceErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforceErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforceErrorHandlingConfig
idFieldNamesB*" 
object" 
writeOperationTypeB" :�
�
appflowTFlowDestinationFlowConfigDestinationConnectorPropertiesSalesforceErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforceErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforceErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflow?FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData:FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData�
��
errorHandlingConfig�B�:�
�
appflowRFlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataErrorHandlingConfig
idFieldNamesB*" 

objectPath" �
successResponseHandlingConfig�B�:�
�
appflow\FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfig�Determines how Amazon AppFlow handles the success response that it gets from the connector after placing data. See Success Response Handling Config for more details.

writeOperationTypeB" :�
�
appflowRFlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflow\FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfigg
e2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
:�
�
appflow@FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake:FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake�
�
bucketPrefixB" �
errorHandlingConfig�B�:�
�
appflowSFlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig
intermediateBucketName" 
object" :�
�
appflowSFlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflakeErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
�
appflow?FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver�
�

bucketName" 
bucketPrefixB" �
s3OutputFormatConfig�:�
�
appflowSFlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig:�
�
appflowSFlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig�	
�	�
aggregationConfig�B�:�
�
appflowdFlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig�Aggregation settings that you can use to customize the output format of your flow data. See Aggregation Config for more details.

fileTypeB" mFile type that Amazon AppFlow places in the Amazon S3 bucket. Valid values are `CSV`, `JSON`, and `PARQUET`.
�
prefixConfig�:�
�
appflow_FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig�Determines the prefix that Amazon AppFlow applies to the folder name in the Amazon S3 bucket. You can name folders according to the flow frequency and date. See Prefix Config for more details.
:�
�
appflowdFlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig�
��
aggregationTypeB" �Whether Amazon AppFlow aggregates the flow records into a single file, or leave them unaggregated. Valid values are `None` and `SingleFile`.
:�
�
appflow_FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig�
��
prefixFormatB" �Determines the level of granularity that's included in the prefix. Valid values are `YEAR`, `MONTH`, `DAY`, `HOUR`, and `MINUTE`.
�
prefixHierarchiesB*" �Determines whether the destination file path includes either or both of the selected elements. Valid values are `EXECUTION_ID` and `SCHEMA_VERSION`
�

prefixType" �Determines the format of the prefix, and whether it applies to the file name, file path, or both. Valid values are `FILENAME`, `PATH`, and `PATH_AND_FILENAME`.
:�
�
appflow>FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk:FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk�
��
errorHandlingConfig�B�:�
�
appflowQFlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig
idFieldNamesB*" 
object" 
writeOperationTypeB" :�
�
appflowQFlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig�aws:appflow/FlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig:FlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig�
�2

bucketNameB" Name of the Amazon S3 bucket.
/
bucketPrefixB" Amazon S3 bucket prefix.
�
failOnFirstDestinationErrorB
 pIf the flow should fail after the first instance of a failure when attempting to place data in the destination.
:�
e
appflowFlowMetadataCatalogConfig?aws:appflow/FlowMetadataCatalogConfig:FlowMetadataCatalogConfig�
��
glueDataCatalog�B�:�
�
appflow(FlowMetadataCatalogConfigGlueDataCatalog]aws:appflow/FlowMetadataCatalogConfigGlueDataCatalog:FlowMetadataCatalogConfigGlueDataCatalog:�
�
appflow(FlowMetadataCatalogConfigGlueDataCatalog]aws:appflow/FlowMetadataCatalogConfigGlueDataCatalog:FlowMetadataCatalogConfigGlueDataCatalog�
�t
databaseName" `The name of an existing Glue database to store the metadata tables that Amazon AppFlow creates.
�
roleArn" ~The ARN of an IAM role that grants AppFlow the permissions it needs to create Data Catalog tables, databases, and partitions.
[
tablePrefix" HA naming prefix for each Data Catalog table that Amazon AppFlow creates
:�
V
appflowFlowSourceFlowConfig5aws:appflow/FlowSourceFlowConfig:FlowSourceFlowConfig�

�
E

apiVersionB" 1API version that the destination connector uses.
�
connectorProfileNameB" gName of the connector profile. This name must be unique for each connector profile in the AWS account.
�
connectorType" �Type of connector, such as Salesforce, Amplitude, and so on. Valid values are `Salesforce`, `Singular`, `Slack`, `Redshift`, `S3`, `Marketo`, `Googleanalytics`, `Zendesk`, `Servicenow`, `Datadog`, `Trendmicro`, `Snowflake`, `Dynatrace`, `Infornexus`, `Amplitude`, `Veeva`, `EventBridge`, `LookoutMetrics`, `Upsolver`, `Honeycode`, `CustomerProfiles`, `SAPOData`, and `CustomConnector`.
�
incrementalPullConfig�B�:�
�
appflow)FlowSourceFlowConfigIncrementalPullConfig_aws:appflow/FlowSourceFlowConfigIncrementalPullConfig:FlowSourceFlowConfigIncrementalPullConfig�Defines the configuration for a scheduled incremental data pull. If a valid configuration is provided, the fields specified in the configuration are used when querying for the incremental data pull. See Incremental Pull Config for more details.
�
sourceConnectorProperties�:�
�
appflow-FlowSourceFlowConfigSourceConnectorPropertiesgaws:appflow/FlowSourceFlowConfigSourceConnectorProperties:FlowSourceFlowConfigSourceConnectorPropertiesrInformation that is required to query a particular source connector. See Source Connector Properties for details.
:�
�
appflow)FlowSourceFlowConfigIncrementalPullConfig_aws:appflow/FlowSourceFlowConfigIncrementalPullConfig:FlowSourceFlowConfigIncrementalPullConfig�
��
datetimeTypeFieldNameB" �Field that specifies the date time or timestamp field as the criteria to use when importing incremental records from the source.
:�,
�
appflow-FlowSourceFlowConfigSourceConnectorPropertiesgaws:appflow/FlowSourceFlowConfigSourceConnectorProperties:FlowSourceFlowConfigSourceConnectorProperties�+
�+�
	amplitude�B�:�
�
appflow6FlowSourceFlowConfigSourceConnectorPropertiesAmplitudeyaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesAmplitude:FlowSourceFlowConfigSourceConnectorPropertiesAmplitudeeInformation that is required for querying Amplitude. See Generic Source Properties for more details.
�
customConnector�B�:�
�
appflow<FlowSourceFlowConfigSourceConnectorPropertiesCustomConnector�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesCustomConnector:FlowSourceFlowConfigSourceConnectorPropertiesCustomConnectoryProperties that are applied when the custom connector is being used as a source. See Custom Connector Source Properties.
�
datadog�B�:�
�
appflow4FlowSourceFlowConfigSourceConnectorPropertiesDatadoguaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesDatadog:FlowSourceFlowConfigSourceConnectorPropertiesDatadogcInformation that is required for querying Datadog. See Generic Source Properties for more details.
�
	dynatrace�B�:�
�
appflow6FlowSourceFlowConfigSourceConnectorPropertiesDynatraceyaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesDynatrace:FlowSourceFlowConfigSourceConnectorPropertiesDynatrace�Operation to be performed on the provided Dynatrace source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
googleAnalytics�B�:�
�
appflow<FlowSourceFlowConfigSourceConnectorPropertiesGoogleAnalytics�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesGoogleAnalytics:FlowSourceFlowConfigSourceConnectorPropertiesGoogleAnalyticswOperation to be performed on the provided Google Analytics source fields. Valid values are `PROJECTION` and `BETWEEN`.
�

inforNexus�B�:�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesInforNexus{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesInforNexus:FlowSourceFlowConfigSourceConnectorPropertiesInforNexusgInformation that is required for querying Infor Nexus. See Generic Source Properties for more details.
�
marketo�B�:�
�
appflow4FlowSourceFlowConfigSourceConnectorPropertiesMarketouaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesMarketo:FlowSourceFlowConfigSourceConnectorPropertiesMarketocInformation that is required for querying Marketo. See Generic Source Properties for more details.
�
s3�B�:�
�
appflow/FlowSourceFlowConfigSourceConnectorPropertiesS3kaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesS3:FlowSourceFlowConfigSourceConnectorPropertiesS3`Information that is required for querying Amazon S3. See S3 Source Properties for more details.
�

salesforce�B�:�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesSalesforce{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSalesforce:FlowSourceFlowConfigSourceConnectorPropertiesSalesforceiInformation that is required for querying Salesforce. See Salesforce Source Properties for more details.
�
sapoData�B�:�
�
appflow5FlowSourceFlowConfigSourceConnectorPropertiesSapoDatawaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSapoData:FlowSourceFlowConfigSourceConnectorPropertiesSapoDatarInformation that is required for querying SAPOData as a flow source. See SAPO Source Properties for more details.
�

serviceNow�B�:�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesServiceNow{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesServiceNow:FlowSourceFlowConfigSourceConnectorPropertiesServiceNowfInformation that is required for querying ServiceNow. See Generic Source Properties for more details.
�
singular�B�:�
�
appflow5FlowSourceFlowConfigSourceConnectorPropertiesSingularwaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSingular:FlowSourceFlowConfigSourceConnectorPropertiesSingulardInformation that is required for querying Singular. See Generic Source Properties for more details.
�
slack�B�:�
�
appflow2FlowSourceFlowConfigSourceConnectorPropertiesSlackqaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSlack:FlowSourceFlowConfigSourceConnectorPropertiesSlackaInformation that is required for querying Slack. See Generic Source Properties for more details.
�

trendmicro�B�:�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesTrendmicro{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesTrendmicro:FlowSourceFlowConfigSourceConnectorPropertiesTrendmicro�Operation to be performed on the provided Trend Micro source fields. Valid values are `PROJECTION`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
veeva�B�:�
�
appflow2FlowSourceFlowConfigSourceConnectorPropertiesVeevaqaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesVeeva:FlowSourceFlowConfigSourceConnectorPropertiesVeeva_Information that is required for querying Veeva. See Veeva Source Properties for more details.
�
zendesk�B�:�
�
appflow4FlowSourceFlowConfigSourceConnectorPropertiesZendeskuaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesZendesk:FlowSourceFlowConfigSourceConnectorPropertiesZendeskcInformation that is required for querying Zendesk. See Generic Source Properties for more details.
:�
�
appflow6FlowSourceFlowConfigSourceConnectorPropertiesAmplitudeyaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesAmplitude:FlowSourceFlowConfigSourceConnectorPropertiesAmplitude

object" :�
�
appflow<FlowSourceFlowConfigSourceConnectorPropertiesCustomConnector�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesCustomConnector:FlowSourceFlowConfigSourceConnectorPropertiesCustomConnector0
.
customPropertiesB2" 

entityName" :�
�
appflow4FlowSourceFlowConfigSourceConnectorPropertiesDatadoguaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesDatadog:FlowSourceFlowConfigSourceConnectorPropertiesDatadog

object" :�
�
appflow6FlowSourceFlowConfigSourceConnectorPropertiesDynatraceyaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesDynatrace:FlowSourceFlowConfigSourceConnectorPropertiesDynatrace

object" :�
�
appflow<FlowSourceFlowConfigSourceConnectorPropertiesGoogleAnalytics�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesGoogleAnalytics:FlowSourceFlowConfigSourceConnectorPropertiesGoogleAnalytics

object" :�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesInforNexus{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesInforNexus:FlowSourceFlowConfigSourceConnectorPropertiesInforNexus

object" :�
�
appflow4FlowSourceFlowConfigSourceConnectorPropertiesMarketouaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesMarketo:FlowSourceFlowConfigSourceConnectorPropertiesMarketo

object" :�
�
appflow/FlowSourceFlowConfigSourceConnectorPropertiesS3kaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesS3:FlowSourceFlowConfigSourceConnectorPropertiesS3�
�

bucketName" 
bucketPrefix" �
s3InputFormatConfig�B�:�
�
appflowBFlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig:FlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig�When you use Amazon S3 as the source, the configuration format that you provide the flow input data. See S3 Input Format Config for details.
:�
�
appflowBFlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig:FlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig
}{
s3InputFileTypeB" bFile type that Amazon AppFlow gets from your Amazon S3 bucket. Valid values are `CSV` and `JSON`.
:�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesSalesforce{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSalesforce:FlowSourceFlowConfigSourceConnectorPropertiesSalesforce�
��
enableDynamicFieldUpdateB
 rFlag that enables dynamic fetching of new (recently added) fields in the Salesforce objects while running a flow.
^
includeDeletedRecordsB
 ?Whether Amazon AppFlow includes deleted files in the flow run.

object" :�
�
appflow5FlowSourceFlowConfigSourceConnectorPropertiesSapoDatawaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSapoData:FlowSourceFlowConfigSourceConnectorPropertiesSapoData�
�

objectPath" �
paginationConfig�B�:�
�
appflowEFlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig:FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfigdSets the page size for each concurrent process that transfers OData records from your SAP instance.
�
parallelismConfig�B�:�
�
appflowFFlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig:FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig]Sets the number of concurrent processes that transfers OData records from your SAP instance.
:�
�
appflowEFlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig:FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig�
��
maxPageSize rhe maximum number of records that Amazon AppFlow receives in each page of the response from your SAP application.
:�
�
appflowFFlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig�aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig:FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig�
��
maxPageSize rhe maximum number of records that Amazon AppFlow receives in each page of the response from your SAP application.
:�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesServiceNow{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesServiceNow:FlowSourceFlowConfigSourceConnectorPropertiesServiceNow

object" :�
�
appflow5FlowSourceFlowConfigSourceConnectorPropertiesSingularwaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSingular:FlowSourceFlowConfigSourceConnectorPropertiesSingular

object" :�
�
appflow2FlowSourceFlowConfigSourceConnectorPropertiesSlackqaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesSlack:FlowSourceFlowConfigSourceConnectorPropertiesSlack

object" :�
�
appflow7FlowSourceFlowConfigSourceConnectorPropertiesTrendmicro{aws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesTrendmicro:FlowSourceFlowConfigSourceConnectorPropertiesTrendmicro

object" :�
�
appflow2FlowSourceFlowConfigSourceConnectorPropertiesVeevaqaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesVeeva:FlowSourceFlowConfigSourceConnectorPropertiesVeeva�
�R
documentTypeB" <Document type specified in the Veeva document extract flow.
k
includeAllVersionsB
 OBoolean value to include All Versions of files in Veeva document extract flow.
d
includeRenditionsB
 IBoolean value to include file renditions in Veeva document extract flow.
b
includeSourceFilesB
 FBoolean value to include source files in Veeva document extract flow.

object" :�
�
appflow4FlowSourceFlowConfigSourceConnectorPropertiesZendeskuaws:appflow/FlowSourceFlowConfigSourceConnectorPropertiesZendesk:FlowSourceFlowConfigSourceConnectorPropertiesZendesk

object" :�	
2
appflowFlowTaskaws:appflow/FlowTask:FlowTask�
��
connectorOperatorsmBk*i:g
e
appflowFlowTaskConnectorOperator?aws:appflow/FlowTaskConnectorOperator:FlowTaskConnectorOperator]Operation to be performed on the provided source fields. See Connector Operator for details.
�
destinationFieldB" jField in a destination connector, or a field value against which Amazon AppFlow validates a source field.
M
sourceFieldsB*" 5Source fields to which a particular task is applied.
�
taskPropertiesB2" �Map used to store task-related information. The execution service looks for particular information based on the `TaskType`. Valid keys are `VALUE`, `VALUES`, `DATA_TYPE`, `UPPER_BOUND`, `LOWER_BOUND`, `SOURCE_DATA_TYPE`, `DESTINATION_DATA_TYPE`, `VALIDATION_ACTION`, `MASK_VALUE`, `MASK_LENGTH`, `TRUNCATE_LENGTH`, `MATH_OPERATION_FIELDS_ORDER`, `CONCAT_FORMAT`, `SUBFIELD_CATEGORY_MAP`, and `EXCLUDE_SOURCE_FIELDS_LIST`.
�
taskType" �Particular task implementation that Amazon AppFlow performs. Valid values are `Arithmetic`, `Filter`, `Map`, `Map_all`, `Mask`, `Merge`, `Passthrough`, `Truncate`, and `Validate`.
:�-
e
appflowFlowTaskConnectorOperator?aws:appflow/FlowTaskConnectorOperator:FlowTaskConnectorOperator�,
�,y
	amplitudeB" fOperation to be performed on the provided Amplitude source fields. The only valid value is `BETWEEN`.
�
customConnectorB" �Operators supported by the custom connector. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
datadogB" �Operation to be performed on the provided Datadog source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
	dynatraceB" �Operation to be performed on the provided Dynatrace source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
googleAnalyticsB" wOperation to be performed on the provided Google Analytics source fields. Valid values are `PROJECTION` and `BETWEEN`.
�

inforNexusB" �Operation to be performed on the provided Infor Nexus source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
marketoB" �Operation to be performed on the provided Marketo source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
s3B" �Operation to be performed on the provided Amazon S3 source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�

salesforceB" �Operation to be performed on the provided Salesforce source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
sapoDataB" �Operation to be performed on the provided SAPOData source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�

serviceNowB" �Operation to be performed on the provided ServiceNow source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
singularB" �Operation to be performed on the provided Singular source fields. Valid values are `PROJECTION`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
slackB" �Operation to be performed on the provided Slack source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�

trendmicroB" �Operation to be performed on the provided Trend Micro source fields. Valid values are `PROJECTION`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
veevaB" �Operation to be performed on the provided Veeva source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
�
zendeskB" �Operation to be performed on the provided Zendesk source fields. Valid values are `PROJECTION`, `GREATER_THAN`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
:�
M
appflowFlowTriggerConfig/aws:appflow/FlowTriggerConfig:FlowTriggerConfig�
��
triggerProperties�B�:�
�
appflow"FlowTriggerConfigTriggerPropertiesQaws:appflow/FlowTriggerConfigTriggerProperties:FlowTriggerConfigTriggerProperties�Configuration details of a schedule-triggered flow as defined by the user. Currently, these settings only apply to the `Scheduled` trigger type. See Scheduled Trigger Properties for details.
`
triggerType" MType of flow trigger. Valid values are `Scheduled`, `Event`, and `OnDemand`.
:�
�
appflow"FlowTriggerConfigTriggerPropertiesQaws:appflow/FlowTriggerConfigTriggerProperties:FlowTriggerConfigTriggerProperties�
��
	scheduled�B�:�
�
appflow+FlowTriggerConfigTriggerPropertiesScheduledcaws:appflow/FlowTriggerConfigTriggerPropertiesScheduled:FlowTriggerConfigTriggerPropertiesScheduled:�
�
appflow+FlowTriggerConfigTriggerPropertiesScheduledcaws:appflow/FlowTriggerConfigTriggerPropertiesScheduled:FlowTriggerConfigTriggerPropertiesScheduled�
��
dataPullModeB" �Whether a scheduled flow has an incremental data transfer or a complete data transfer for each flow run. Valid values are `Incremental` and `Complete`.
�
firstExecutionFromB" rDate range for the records to import from the connector in the first flow run. Must be a valid RFC3339 timestamp.
n
scheduleEndTimeB" UScheduled end time for a schedule-triggered flow. Must be a valid RFC3339 timestamp.
�
scheduleExpression" mScheduling expression that determines the rate at which the schedule will run, for example `rate(5minutes)`.
�
scheduleOffsetB jOptional offset that is added to the time interval for a schedule-triggered flow. Maximum value of 36000.
r
scheduleStartTimeB" WScheduled start time for a schedule-triggered flow. Must be a valid RFC3339 timestamp.
�	
timezoneB" �	Time zone used when referring to the date and time of a scheduled-triggered flow, such as `America/New_York`.

<!--Start PulumiCodeChooser -->
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.appflow.Flow;
import com.pulumi.aws.appflow.FlowArgs;
import com.pulumi.aws.appflow.inputs.FlowTriggerConfigArgs;
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
        var example = new Flow("example", FlowArgs.builder()
            .triggerConfig(FlowTriggerConfigArgs.builder()
                .scheduled(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference))
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:appflow:Flow
    properties:
      triggerConfig:
        scheduled:
          - scheduleExpression: rate(1minutes)
```
<!--End PulumiCodeChooser -->
:�
�
appintegrationsDataIntegrationScheduleConfigOaws:appintegrations/DataIntegrationScheduleConfig:DataIntegrationScheduleConfig�
��
firstExecutionFrom" �The start date for objects to import in the first flow run as an Unix/epoch timestamp in milliseconds or in ISO-8601 format. This needs to be a time in the past, meaning that the data created or updated before this given date will not be downloaded.
�
object" }The name of the object to pull from the data source. Examples of objects in Salesforce include `Case`, `Account`, or `Lead`.
�
scheduleExpression" wHow often the data should be pulled from data source. Examples include `rate(1 hour)`, `rate(3 hours)`, `rate(1 day)`.
:�
�
appintegrationsgetEventIntegrationEventFilterQaws:appintegrations/getEventIntegrationEventFilter:getEventIntegrationEventFilter,
*(
source" The source of the events.
:�
J
appmeshGatewayRouteSpec-aws:appmesh/GatewayRouteSpec:GatewayRouteSpec�
��
	grpcRoutekBi:g
e
appmeshGatewayRouteSpecGrpcRoute?aws:appmesh/GatewayRouteSpecGrpcRoute:GatewayRouteSpecGrpcRoute'Specification of a gRPC gateway route.
�

http2RoutenBl:j
h
appmeshGatewayRouteSpecHttp2RouteAaws:appmesh/GatewayRouteSpecHttp2Route:GatewayRouteSpecHttp2Route*Specification of an HTTP/2 gateway route.
�
	httpRoutekBi:g
e
appmeshGatewayRouteSpecHttpRoute?aws:appmesh/GatewayRouteSpecHttpRoute:GatewayRouteSpecHttpRoute(Specification of an HTTP gateway route.
J
priorityB 8Priority for the gateway route, between `0` and `1000`.
:�
e
appmeshGatewayRouteSpecGrpcRoute?aws:appmesh/GatewayRouteSpecGrpcRoute:GatewayRouteSpecGrpcRoute�
��
action{:y
w
appmeshGatewayRouteSpecGrpcRouteActionKaws:appmesh/GatewayRouteSpecGrpcRouteAction:GatewayRouteSpecGrpcRouteAction)Action to take if a match is determined.
�
matchx:v
t
appmeshGatewayRouteSpecGrpcRouteMatchIaws:appmesh/GatewayRouteSpecGrpcRouteMatch:GatewayRouteSpecGrpcRouteMatch*Criteria for determining a request match.
:�
w
appmeshGatewayRouteSpecGrpcRouteActionKaws:appmesh/GatewayRouteSpecGrpcRouteAction:GatewayRouteSpecGrpcRouteAction�
��
target�:�
�
appmesh%GatewayRouteSpecGrpcRouteActionTargetWaws:appmesh/GatewayRouteSpecGrpcRouteActionTarget:GatewayRouteSpecGrpcRouteActionTargetKTarget that traffic is routed to when a request matches the gateway route.
:�
�
appmesh%GatewayRouteSpecGrpcRouteActionTargetWaws:appmesh/GatewayRouteSpecGrpcRouteActionTarget:GatewayRouteSpecGrpcRouteActionTarget�
��
portB �The port number that corresponds to the target for Virtual Service provider port. This is required when the provider (router or node) of the Virtual Service has multiple listeners.
�
virtualService�:�
�
appmesh3GatewayRouteSpecGrpcRouteActionTargetVirtualServicesaws:appmesh/GatewayRouteSpecGrpcRouteActionTargetVirtualService:GatewayRouteSpecGrpcRouteActionTargetVirtualService&Virtual service gateway route target.
:�
�
appmesh3GatewayRouteSpecGrpcRouteActionTargetVirtualServicesaws:appmesh/GatewayRouteSpecGrpcRouteActionTargetVirtualService:GatewayRouteSpecGrpcRouteActionTargetVirtualService�
��
virtualServiceName" gName of the virtual service that traffic is routed to. Must be between 1 and 255 characters in length.
:�
t
appmeshGatewayRouteSpecGrpcRouteMatchIaws:appmesh/GatewayRouteSpecGrpcRouteMatch:GatewayRouteSpecGrpcRouteMatch�
�9
portB +The port number to match from the request.
Z
serviceName" GFully qualified domain name for the service to match from the request.
:�
h
appmeshGatewayRouteSpecHttp2RouteAaws:appmesh/GatewayRouteSpecHttp2Route:GatewayRouteSpecHttp2Route�
��
action~:|
z
appmesh GatewayRouteSpecHttp2RouteActionMaws:appmesh/GatewayRouteSpecHttp2RouteAction:GatewayRouteSpecHttp2RouteAction)Action to take if a match is determined.
�
match{:y
w
appmeshGatewayRouteSpecHttp2RouteMatchKaws:appmesh/GatewayRouteSpecHttp2RouteMatch:GatewayRouteSpecHttp2RouteMatch*Criteria for determining a request match.
:�
z
appmesh GatewayRouteSpecHttp2RouteActionMaws:appmesh/GatewayRouteSpecHttp2RouteAction:GatewayRouteSpecHttp2RouteAction�
��
rewrite�B�:�
�
appmesh'GatewayRouteSpecHttp2RouteActionRewrite[aws:appmesh/GatewayRouteSpecHttp2RouteActionRewrite:GatewayRouteSpecHttp2RouteActionRewrite!Gateway route action to rewrite.
�
target�:�
�
appmesh&GatewayRouteSpecHttp2RouteActionTargetYaws:appmesh/GatewayRouteSpecHttp2RouteActionTarget:GatewayRouteSpecHttp2RouteActionTargetKTarget that traffic is routed to when a request matches the gateway route.
:�
�
appmesh'GatewayRouteSpecHttp2RouteActionRewrite[aws:appmesh/GatewayRouteSpecHttp2RouteActionRewrite:GatewayRouteSpecHttp2RouteActionRewrite�
��
hostname�B�:�
�
appmesh/GatewayRouteSpecHttp2RouteActionRewriteHostnamekaws:appmesh/GatewayRouteSpecHttp2RouteActionRewriteHostname:GatewayRouteSpecHttp2RouteActionRewriteHostnameHost name to rewrite.
�
path�B�:�
�
appmesh+GatewayRouteSpecHttp2RouteActionRewritePathcaws:appmesh/GatewayRouteSpecHttp2RouteActionRewritePath:GatewayRouteSpecHttp2RouteActionRewritePathExact path to rewrite.
�
prefix�B�:�
�
appmesh-GatewayRouteSpecHttp2RouteActionRewritePrefixgaws:appmesh/GatewayRouteSpecHttp2RouteActionRewritePrefix:GatewayRouteSpecHttp2RouteActionRewritePrefix+Specified beginning characters to rewrite.
:�
�
appmesh/GatewayRouteSpecHttp2RouteActionRewriteHostnamekaws:appmesh/GatewayRouteSpecHttp2RouteActionRewriteHostname:GatewayRouteSpecHttp2RouteActionRewriteHostnamel
jh
defaultTargetHostname" KDefault target host name to write to. Valid values: `ENABLED`, `DISABLED`.
:�
�
appmesh+GatewayRouteSpecHttp2RouteActionRewritePathcaws:appmesh/GatewayRouteSpecHttp2RouteActionRewritePath:GatewayRouteSpecHttp2RouteActionRewritePath-
+)
exact" The exact path to match on.
:�
�
appmesh-GatewayRouteSpecHttp2RouteActionRewritePrefixgaws:appmesh/GatewayRouteSpecHttp2RouteActionRewritePrefix:GatewayRouteSpecHttp2RouteActionRewritePrefix�
��
defaultPrefixB" nDefault prefix used to replace the incoming route prefix when rewritten. Valid values: `ENABLED`, `DISABLED`.
O
valueB" @Value used to replace the incoming route prefix when rewritten.
:�
�
appmesh&GatewayRouteSpecHttp2RouteActionTargetYaws:appmesh/GatewayRouteSpecHttp2RouteActionTarget:GatewayRouteSpecHttp2RouteActionTarget�
��
portB �The port number that corresponds to the target for Virtual Service provider port. This is required when the provider (router or node) of the Virtual Service has multiple listeners.
�
virtualService�:�
�
appmesh4GatewayRouteSpecHttp2RouteActionTargetVirtualServiceuaws:appmesh/GatewayRouteSpecHttp2RouteActionTargetVirtualService:GatewayRouteSpecHttp2RouteActionTargetVirtualService&Virtual service gateway route target.
:�
�
appmesh4GatewayRouteSpecHttp2RouteActionTargetVirtualServiceuaws:appmesh/GatewayRouteSpecHttp2RouteActionTargetVirtualService:GatewayRouteSpecHttp2RouteActionTargetVirtualService�
��
virtualServiceName" gName of the virtual service that traffic is routed to. Must be between 1 and 255 characters in length.
:�	
w
appmeshGatewayRouteSpecHttp2RouteMatchKaws:appmesh/GatewayRouteSpecHttp2RouteMatch:GatewayRouteSpecHttp2RouteMatch�
��
headers�B�*�:�
�
appmesh%GatewayRouteSpecHttp2RouteMatchHeaderWaws:appmesh/GatewayRouteSpecHttp2RouteMatchHeader:GatewayRouteSpecHttp2RouteMatchHeader$Client request headers to match on.
�
hostname�B�:�
�
appmesh'GatewayRouteSpecHttp2RouteMatchHostname[aws:appmesh/GatewayRouteSpecHttp2RouteMatchHostname:GatewayRouteSpecHttp2RouteMatchHostnameHost name to match on.
�
path�B�:�
�
appmesh#GatewayRouteSpecHttp2RouteMatchPathSaws:appmesh/GatewayRouteSpecHttp2RouteMatchPath:GatewayRouteSpecHttp2RouteMatchPath!Client request path to match on.
9
portB +The port number to match from the request.
�
prefixB" �Path to match requests with. This parameter must always start with `/`, which by itself matches all requests to the virtual service name.
�
queryParameters�B�*�:�
�
appmesh-GatewayRouteSpecHttp2RouteMatchQueryParametergaws:appmesh/GatewayRouteSpecHttp2RouteMatchQueryParameter:GatewayRouteSpecHttp2RouteMatchQueryParameter-Client request query parameters to match on.
:�
�
appmesh%GatewayRouteSpecHttp2RouteMatchHeaderWaws:appmesh/GatewayRouteSpecHttp2RouteMatchHeader:GatewayRouteSpecHttp2RouteMatchHeader�
�m
invertB
 ]If `true`, the match is on the opposite of the `match` method and value. Default is `false`.
�
match�B�:�
�
appmesh*GatewayRouteSpecHttp2RouteMatchHeaderMatchaaws:appmesh/GatewayRouteSpecHttp2RouteMatchHeaderMatch:GatewayRouteSpecHttp2RouteMatchHeaderMatchZMethod and value to match the header value sent with a request. Specify one match method.
T
name" HName for the HTTP header in the client request that will be matched on.
:�
�
appmesh*GatewayRouteSpecHttp2RouteMatchHeaderMatchaaws:appmesh/GatewayRouteSpecHttp2RouteMatchHeaderMatch:GatewayRouteSpecHttp2RouteMatchHeaderMatch�
�W
exactB" HHeader value sent by the client must match the specified value exactly.
Z
prefixB" JHeader value sent by the client must begin with the specified characters.
�
range�B�:�
�
appmesh/GatewayRouteSpecHttp2RouteMatchHeaderMatchRangekaws:appmesh/GatewayRouteSpecHttp2RouteMatchHeaderMatchRange:GatewayRouteSpecHttp2RouteMatchHeaderMatchRangeiObject that specifies the range of numbers that the header value sent by the client must be included in.
V
regexB" GHeader value sent by the client must include the specified characters.
X
suffixB" HHeader value sent by the client must end with the specified characters.
:�
�
appmesh/GatewayRouteSpecHttp2RouteMatchHeaderMatchRangekaws:appmesh/GatewayRouteSpecHttp2RouteMatchHeaderMatchRange:GatewayRouteSpecHttp2RouteMatchHeaderMatchRangeD
B
end End of the range.
!
start Start of the range.
:�
�
appmesh'GatewayRouteSpecHttp2RouteMatchHostname[aws:appmesh/GatewayRouteSpecHttp2RouteMatchHostname:GatewayRouteSpecHttp2RouteMatchHostname|
z,
exactB" Exact host name to match on.
J
suffixB" :Specified ending characters of the host name to match on.
:�
�
appmesh#GatewayRouteSpecHttp2RouteMatchPathSaws:appmesh/GatewayRouteSpecHttp2RouteMatchPath:GatewayRouteSpecHttp2RouteMatchPathb
`+
exactB" The exact path to match on.
1
regexB" "The regex used to match the path.
:�
�
appmesh-GatewayRouteSpecHttp2RouteMatchQueryParametergaws:appmesh/GatewayRouteSpecHttp2RouteMatchQueryParameter:GatewayRouteSpecHttp2RouteMatchQueryParameter�
��
match�B�:�
�
appmesh2GatewayRouteSpecHttp2RouteMatchQueryParameterMatchqaws:appmesh/GatewayRouteSpecHttp2RouteMatchQueryParameterMatch:GatewayRouteSpecHttp2RouteMatchQueryParameterMatch!The query parameter to match on.
B
name" 6Name for the query parameter that will be matched on.
:�
�
appmesh2GatewayRouteSpecHttp2RouteMatchQueryParameterMatchqaws:appmesh/GatewayRouteSpecHttp2RouteMatchQueryParameterMatch:GatewayRouteSpecHttp2RouteMatchQueryParameterMatch:
86
exactB" 'The exact query parameter to match on.
:�
e
appmeshGatewayRouteSpecHttpRoute?aws:appmesh/GatewayRouteSpecHttpRoute:GatewayRouteSpecHttpRoute�
��
action{:y
w
appmeshGatewayRouteSpecHttpRouteActionKaws:appmesh/GatewayRouteSpecHttpRouteAction:GatewayRouteSpecHttpRouteAction)Action to take if a match is determined.
�
matchx:v
t
appmeshGatewayRouteSpecHttpRouteMatchIaws:appmesh/GatewayRouteSpecHttpRouteMatch:GatewayRouteSpecHttpRouteMatch*Criteria for determining a request match.
:�
w
appmeshGatewayRouteSpecHttpRouteActionKaws:appmesh/GatewayRouteSpecHttpRouteAction:GatewayRouteSpecHttpRouteAction�
��
rewrite�B�:�
�
appmesh&GatewayRouteSpecHttpRouteActionRewriteYaws:appmesh/GatewayRouteSpecHttpRouteActionRewrite:GatewayRouteSpecHttpRouteActionRewrite!Gateway route action to rewrite.
�
target�:�
�
appmesh%GatewayRouteSpecHttpRouteActionTargetWaws:appmesh/GatewayRouteSpecHttpRouteActionTarget:GatewayRouteSpecHttpRouteActionTargetKTarget that traffic is routed to when a request matches the gateway route.
:�
�
appmesh&GatewayRouteSpecHttpRouteActionRewriteYaws:appmesh/GatewayRouteSpecHttpRouteActionRewrite:GatewayRouteSpecHttpRouteActionRewrite�
��
hostname�B�:�
�
appmesh.GatewayRouteSpecHttpRouteActionRewriteHostnameiaws:appmesh/GatewayRouteSpecHttpRouteActionRewriteHostname:GatewayRouteSpecHttpRouteActionRewriteHostnameHost name to rewrite.
�
path�B�:�
�
appmesh*GatewayRouteSpecHttpRouteActionRewritePathaaws:appmesh/GatewayRouteSpecHttpRouteActionRewritePath:GatewayRouteSpecHttpRouteActionRewritePathExact path to rewrite.
�
prefix�B�:�
�
appmesh,GatewayRouteSpecHttpRouteActionRewritePrefixeaws:appmesh/GatewayRouteSpecHttpRouteActionRewritePrefix:GatewayRouteSpecHttpRouteActionRewritePrefix+Specified beginning characters to rewrite.
:�
�
appmesh.GatewayRouteSpecHttpRouteActionRewriteHostnameiaws:appmesh/GatewayRouteSpecHttpRouteActionRewriteHostname:GatewayRouteSpecHttpRouteActionRewriteHostnamel
jh
defaultTargetHostname" KDefault target host name to write to. Valid values: `ENABLED`, `DISABLED`.
:�
�
appmesh*GatewayRouteSpecHttpRouteActionRewritePathaaws:appmesh/GatewayRouteSpecHttpRouteActionRewritePath:GatewayRouteSpecHttpRouteActionRewritePath-
+)
exact" The exact path to match on.
:�
�
appmesh,GatewayRouteSpecHttpRouteActionRewritePrefixeaws:appmesh/GatewayRouteSpecHttpRouteActionRewritePrefix:GatewayRouteSpecHttpRouteActionRewritePrefix�
��
defaultPrefixB" nDefault prefix used to replace the incoming route prefix when rewritten. Valid values: `ENABLED`, `DISABLED`.
O
valueB" @Value used to replace the incoming route prefix when rewritten.
:�
�
appmesh%GatewayRouteSpecHttpRouteActionTargetWaws:appmesh/GatewayRouteSpecHttpRouteActionTarget:GatewayRouteSpecHttpRouteActionTarget�
��
portB �The port number that corresponds to the target for Virtual Service provider port. This is required when the provider (router or node) of the Virtual Service has multiple listeners.
�
virtualService�:�
�
appmesh3GatewayRouteSpecHttpRouteActionTargetVirtualServicesaws:appmesh/GatewayRouteSpecHttpRouteActionTargetVirtualService:GatewayRouteSpecHttpRouteActionTargetVirtualService&Virtual service gateway route target.
:�
�
appmesh3GatewayRouteSpecHttpRouteActionTargetVirtualServicesaws:appmesh/GatewayRouteSpecHttpRouteActionTargetVirtualService:GatewayRouteSpecHttpRouteActionTargetVirtualService�
��
virtualServiceName" gName of the virtual service that traffic is routed to. Must be between 1 and 255 characters in length.
:�	
t
appmeshGatewayRouteSpecHttpRouteMatchIaws:appmesh/GatewayRouteSpecHttpRouteMatch:GatewayRouteSpecHttpRouteMatch�
��
headers�B�*�:�
�
appmesh$GatewayRouteSpecHttpRouteMatchHeaderUaws:appmesh/GatewayRouteSpecHttpRouteMatchHeader:GatewayRouteSpecHttpRouteMatchHeader$Client request headers to match on.
�
hostname�B�:�
�
appmesh&GatewayRouteSpecHttpRouteMatchHostnameYaws:appmesh/GatewayRouteSpecHttpRouteMatchHostname:GatewayRouteSpecHttpRouteMatchHostnameHost name to match on.
�
path�B�:�
�
appmesh"GatewayRouteSpecHttpRouteMatchPathQaws:appmesh/GatewayRouteSpecHttpRouteMatchPath:GatewayRouteSpecHttpRouteMatchPath!Client request path to match on.
9
portB +The port number to match from the request.
�
prefixB" �Path to match requests with. This parameter must always start with `/`, which by itself matches all requests to the virtual service name.
�
queryParameters�B�*�:�
�
appmesh,GatewayRouteSpecHttpRouteMatchQueryParametereaws:appmesh/GatewayRouteSpecHttpRouteMatchQueryParameter:GatewayRouteSpecHttpRouteMatchQueryParameter-Client request query parameters to match on.
:�
�
appmesh$GatewayRouteSpecHttpRouteMatchHeaderUaws:appmesh/GatewayRouteSpecHttpRouteMatchHeader:GatewayRouteSpecHttpRouteMatchHeader�
�m
invertB
 ]If `true`, the match is on the opposite of the `match` method and value. Default is `false`.
�
match�B�:�
�
appmesh)GatewayRouteSpecHttpRouteMatchHeaderMatch_aws:appmesh/GatewayRouteSpecHttpRouteMatchHeaderMatch:GatewayRouteSpecHttpRouteMatchHeaderMatchZMethod and value to match the header value sent with a request. Specify one match method.
T
name" HName for the HTTP header in the client request that will be matched on.
:�
�
appmesh)GatewayRouteSpecHttpRouteMatchHeaderMatch_aws:appmesh/GatewayRouteSpecHttpRouteMatchHeaderMatch:GatewayRouteSpecHttpRouteMatchHeaderMatch�
�W
exactB" HHeader value sent by the client must match the specified value exactly.
Z
prefixB" JHeader value sent by the client must begin with the specified characters.
�
range�B�:�
�
appmesh.GatewayRouteSpecHttpRouteMatchHeaderMatchRangeiaws:appmesh/GatewayRouteSpecHttpRouteMatchHeaderMatchRange:GatewayRouteSpecHttpRouteMatchHeaderMatchRangeiObject that specifies the range of numbers that the header value sent by the client must be included in.
V
regexB" GHeader value sent by the client must include the specified characters.
X
suffixB" HHeader value sent by the client must end with the specified characters.
:�
�
appmesh.GatewayRouteSpecHttpRouteMatchHeaderMatchRangeiaws:appmesh/GatewayRouteSpecHttpRouteMatchHeaderMatchRange:GatewayRouteSpecHttpRouteMatchHeaderMatchRangeD
B
end End of the range.
!
start Start of the range.
:�
�
appmesh&GatewayRouteSpecHttpRouteMatchHostnameYaws:appmesh/GatewayRouteSpecHttpRouteMatchHostname:GatewayRouteSpecHttpRouteMatchHostname|
z,
exactB" Exact host name to match on.
J
suffixB" :Specified ending characters of the host name to match on.
:�
�
appmesh"GatewayRouteSpecHttpRouteMatchPathQaws:appmesh/GatewayRouteSpecHttpRouteMatchPath:GatewayRouteSpecHttpRouteMatchPathb
`+
exactB" The exact path to match on.
1
regexB" "The regex used to match the path.
:�
�
appmesh,GatewayRouteSpecHttpRouteMatchQueryParametereaws:appmesh/GatewayRouteSpecHttpRouteMatchQueryParameter:GatewayRouteSpecHttpRouteMatchQueryParameter�
��
match�B�:�
�
appmesh1GatewayRouteSpecHttpRouteMatchQueryParameterMatchoaws:appmesh/GatewayRouteSpecHttpRouteMatchQueryParameterMatch:GatewayRouteSpecHttpRouteMatchQueryParameterMatch!The query parameter to match on.
B
name" 6Name for the query parameter that will be matched on.
:�
�
appmesh1GatewayRouteSpecHttpRouteMatchQueryParameterMatchoaws:appmesh/GatewayRouteSpecHttpRouteMatchQueryParameterMatch:GatewayRouteSpecHttpRouteMatchQueryParameterMatch:
86
exactB" 'The exact query parameter to match on.
:�
2
appmeshMeshSpecaws:appmesh/MeshSpec:MeshSpec�
��
egressFilter\BZ:X
V
appmeshMeshSpecEgressFilter5aws:appmesh/MeshSpecEgressFilter:MeshSpecEgressFilter*Egress filter rules for the service mesh.
�
serviceDiscoveryhBf:d
b
appmeshMeshSpecServiceDiscovery=aws:appmesh/MeshSpecServiceDiscovery:MeshSpecServiceDiscovery8The service discovery information for the service mesh.
:�
V
appmeshMeshSpecEgressFilter5aws:appmesh/MeshSpecEgressFilter:MeshSpecEgressFilterw
us
typeB" eEgress filter type. By default, the type is `DROP_ALL`. Valid values are `ALLOW_ALL` and `DROP_ALL`.
:�
b
appmeshMeshSpecServiceDiscovery=aws:appmesh/MeshSpecServiceDiscovery:MeshSpecServiceDiscovery�
��
ipPreferenceB" �The IP version to use to control traffic within the mesh. Valid values are `IPv6_PREFERRED`, `IPv4_PREFERRED`, `IPv4_ONLY`, and `IPv6_ONLY`.
:�
5
appmesh	RouteSpecaws:appmesh/RouteSpec:RouteSpec�
��
	grpcRouteVBT:R
P
appmeshRouteSpecGrpcRoute1aws:appmesh/RouteSpecGrpcRoute:RouteSpecGrpcRoute(GRPC routing information for the route.
�

http2RouteYBW:U
S
appmeshRouteSpecHttp2Route3aws:appmesh/RouteSpecHttp2Route:RouteSpecHttp2Route*HTTP/2 routing information for the route.
�
	httpRouteVBT:R
P
appmeshRouteSpecHttpRoute1aws:appmesh/RouteSpecHttpRoute:RouteSpecHttpRoute(HTTP routing information for the route.
�
priorityB �Priority for the route, between `0` and `1000`.
Routes are matched based on the specified value, where `0` is the highest priority.
�
tcpRouteSBQ:O
M
appmeshRouteSpecTcpRoute/aws:appmesh/RouteSpecTcpRoute:RouteSpecTcpRoute'TCP routing information for the route.
:�
P
appmeshRouteSpecGrpcRoute1aws:appmesh/RouteSpecGrpcRoute:RouteSpecGrpcRoute�
��
actionf:d
b
appmeshRouteSpecGrpcRouteAction=aws:appmesh/RouteSpecGrpcRouteAction:RouteSpecGrpcRouteAction)Action to take if a match is determined.
�
matcheBc:a
_
appmeshRouteSpecGrpcRouteMatch;aws:appmesh/RouteSpecGrpcRouteMatch:RouteSpecGrpcRouteMatch0Criteria for determining an gRPC request match.
�
retryPolicywBu:s
q
appmeshRouteSpecGrpcRouteRetryPolicyGaws:appmesh/RouteSpecGrpcRouteRetryPolicy:RouteSpecGrpcRouteRetryPolicyRetry policy.
�
timeoutkBi:g
e
appmeshRouteSpecGrpcRouteTimeout?aws:appmesh/RouteSpecGrpcRouteTimeout:RouteSpecGrpcRouteTimeoutTypes of timeouts.
:�
b
appmeshRouteSpecGrpcRouteAction=aws:appmesh/RouteSpecGrpcRouteAction:RouteSpecGrpcRouteAction�
��
weightedTargets�*�:�
�
appmesh&RouteSpecGrpcRouteActionWeightedTargetYaws:appmesh/RouteSpecGrpcRouteActionWeightedTarget:RouteSpecGrpcRouteActionWeightedTarget�Targets that traffic is routed to when a request matches the route.
You can specify one or more targets and their relative weights with which to distribute traffic.
:�
�
appmesh&RouteSpecGrpcRouteActionWeightedTargetYaws:appmesh/RouteSpecGrpcRouteActionWeightedTarget:RouteSpecGrpcRouteActionWeightedTarget�
�8
portB *The targeted port of the weighted object.
w
virtualNode" dVirtual node to associate with the weighted target. Must be between 1 and 255 characters in length.
T
weight FRelative weight of the weighted target. An integer between 0 and 100.
:�
_
appmeshRouteSpecGrpcRouteMatch;aws:appmesh/RouteSpecGrpcRouteMatch:RouteSpecGrpcRouteMatch�
��
	metadatasB}*{:y
w
appmeshRouteSpecGrpcRouteMatchMetadataKaws:appmesh/RouteSpecGrpcRouteMatchMetadata:RouteSpecGrpcRouteMatchMetadata%Data to match from the gRPC request.
z

methodNameB" fMethod name to match from the request. If you specify a name, you must also specify a `service_name`.
9
portB +The port number to match from the request.

prefixB" \
serviceNameB" GFully qualified domain name for the service to match from the request.
:�
w
appmeshRouteSpecGrpcRouteMatchMetadataKaws:appmesh/RouteSpecGrpcRouteMatchMetadata:RouteSpecGrpcRouteMatchMetadata�
�e
invertB
 UIf `true`, the match is on the opposite of the `match` criteria. Default is `false`.
�
match�B�:�
�
appmesh$RouteSpecGrpcRouteMatchMetadataMatchUaws:appmesh/RouteSpecGrpcRouteMatchMetadataMatch:RouteSpecGrpcRouteMatchMetadataMatch Data to match from the request.
N
name" BName of the route. Must be between 1 and 50 characters in length.
:�
�
appmesh$RouteSpecGrpcRouteMatchMetadataMatchUaws:appmesh/RouteSpecGrpcRouteMatchMetadataMatch:RouteSpecGrpcRouteMatchMetadataMatch�
��
exactB" qValue sent by the client must match the specified value exactly. Must be between 1 and 255 characters in length.
�
prefixB" sValue sent by the client must begin with the specified characters. Must be between 1 and 255 characters in length.
�
range�B�:�
�
appmesh)RouteSpecGrpcRouteMatchMetadataMatchRange_aws:appmesh/RouteSpecGrpcRouteMatchMetadataMatchRange:RouteSpecGrpcRouteMatchMetadataMatchRangebObject that specifies the range of numbers that the value sent by the client must be included in.

regexB" pValue sent by the client must include the specified characters. Must be between 1 and 255 characters in length.
�
suffixB" qValue sent by the client must end with the specified characters. Must be between 1 and 255 characters in length.
:�
�
appmesh)RouteSpecGrpcRouteMatchMetadataMatchRange_aws:appmesh/RouteSpecGrpcRouteMatchMetadataMatchRange:RouteSpecGrpcRouteMatchMetadataMatchRangeD
B
end End of the range.
!
start Start of the range.
:�
q
appmeshRouteSpecGrpcRouteRetryPolicyGaws:appmesh/RouteSpecGrpcRouteRetryPolicy:RouteSpecGrpcRouteRetryPolicy�
��
grpcRetryEventsB*" |List of gRPC retry events.
Valid values: `cancelled`, `deadline-exceeded`, `internal`, `resource-exhausted`, `unavailable`.
�
httpRetryEventsB*" �List of HTTP retry events.
Valid values: `client-error` (HTTP status code 409), `gateway-error` (HTTP status codes 502, 503, and 504), `server-error` (HTTP status codes 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, and 511), `stream-error` (retry on refused stream).
-

maxRetries Maximum number of retries.
�
perRetryTimeout�:�
�
appmesh,RouteSpecGrpcRouteRetryPolicyPerRetryTimeouteaws:appmesh/RouteSpecGrpcRouteRetryPolicyPerRetryTimeout:RouteSpecGrpcRouteRetryPolicyPerRetryTimeoutPer-retry timeout.
`
tcpRetryEventsB*" FList of TCP retry events. The only valid value is `connection-error`.
:�
�
appmesh,RouteSpecGrpcRouteRetryPolicyPerRetryTimeouteaws:appmesh/RouteSpecGrpcRouteRetryPolicyPerRetryTimeout:RouteSpecGrpcRouteRetryPolicyPerRetryTimeoutQ
O1
unit" %Retry unit. Valid values: `ms`, `s`.

value Retry value.
:�
e
appmeshRouteSpecGrpcRouteTimeout?aws:appmesh/RouteSpecGrpcRouteTimeout:RouteSpecGrpcRouteTimeout�
��
idlewBu:s
q
appmeshRouteSpecGrpcRouteTimeoutIdleGaws:appmesh/RouteSpecGrpcRouteTimeoutIdle:RouteSpecGrpcRouteTimeoutIdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
�

perRequest�B�:�
�
appmesh#RouteSpecGrpcRouteTimeoutPerRequestSaws:appmesh/RouteSpecGrpcRouteTimeoutPerRequest:RouteSpecGrpcRouteTimeoutPerRequestPer request timeout.
:�
q
appmeshRouteSpecGrpcRouteTimeoutIdleGaws:appmesh/RouteSpecGrpcRouteTimeoutIdle:RouteSpecGrpcRouteTimeoutIdler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh#RouteSpecGrpcRouteTimeoutPerRequestSaws:appmesh/RouteSpecGrpcRouteTimeoutPerRequest:RouteSpecGrpcRouteTimeoutPerRequestr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
S
appmeshRouteSpecHttp2Route3aws:appmesh/RouteSpecHttp2Route:RouteSpecHttp2Route�
��
actioni:g
e
appmeshRouteSpecHttp2RouteAction?aws:appmesh/RouteSpecHttp2RouteAction:RouteSpecHttp2RouteAction)Action to take if a match is determined.
�
matchf:d
b
appmeshRouteSpecHttp2RouteMatch=aws:appmesh/RouteSpecHttp2RouteMatch:RouteSpecHttp2RouteMatch0Criteria for determining an HTTP request match.
�
retryPolicyzBx:v
t
appmeshRouteSpecHttp2RouteRetryPolicyIaws:appmesh/RouteSpecHttp2RouteRetryPolicy:RouteSpecHttp2RouteRetryPolicyRetry policy.
�
timeoutnBl:j
h
appmeshRouteSpecHttp2RouteTimeoutAaws:appmesh/RouteSpecHttp2RouteTimeout:RouteSpecHttp2RouteTimeoutTypes of timeouts.
:�
e
appmeshRouteSpecHttp2RouteAction?aws:appmesh/RouteSpecHttp2RouteAction:RouteSpecHttp2RouteAction�
��
weightedTargets�*�:�
�
appmesh'RouteSpecHttp2RouteActionWeightedTarget[aws:appmesh/RouteSpecHttp2RouteActionWeightedTarget:RouteSpecHttp2RouteActionWeightedTarget�Targets that traffic is routed to when a request matches the route.
You can specify one or more targets and their relative weights with which to distribute traffic.
:�
�
appmesh'RouteSpecHttp2RouteActionWeightedTarget[aws:appmesh/RouteSpecHttp2RouteActionWeightedTarget:RouteSpecHttp2RouteActionWeightedTarget�
�8
portB *The targeted port of the weighted object.
w
virtualNode" dVirtual node to associate with the weighted target. Must be between 1 and 255 characters in length.
T
weight FRelative weight of the weighted target. An integer between 0 and 100.
:�
b
appmeshRouteSpecHttp2RouteMatch=aws:appmesh/RouteSpecHttp2RouteMatch:RouteSpecHttp2RouteMatch�
��
headers|Bz*x:v
t
appmeshRouteSpecHttp2RouteMatchHeaderIaws:appmesh/RouteSpecHttp2RouteMatchHeader:RouteSpecHttp2RouteMatchHeader$Client request headers to match on.
�
methodB" �Client request header method to match on. Valid values: `GET`, `HEAD`, `POST`, `PUT`, `DELETE`, `CONNECT`, `OPTIONS`, `TRACE`, `PATCH`.
�
pathtBr:p
n
appmeshRouteSpecHttp2RouteMatchPathEaws:appmesh/RouteSpecHttp2RouteMatchPath:RouteSpecHttp2RouteMatchPath!Client request path to match on.
9
portB +The port number to match from the request.
�
prefixB" �Path with which to match requests.
This parameter must always start with /, which by itself matches all requests to the virtual router service name.
�
queryParameters�B�*�:�
�
appmesh&RouteSpecHttp2RouteMatchQueryParameterYaws:appmesh/RouteSpecHttp2RouteMatchQueryParameter:RouteSpecHttp2RouteMatchQueryParameter-Client request query parameters to match on.
Y
schemeB" IClient request header scheme to match on. Valid values: `http`, `https`.
:�
t
appmeshRouteSpecHttp2RouteMatchHeaderIaws:appmesh/RouteSpecHttp2RouteMatchHeader:RouteSpecHttp2RouteMatchHeader�
�m
invertB
 ]If `true`, the match is on the opposite of the `match` method and value. Default is `false`.
�
match�B�:�
�
appmesh#RouteSpecHttp2RouteMatchHeaderMatchSaws:appmesh/RouteSpecHttp2RouteMatchHeaderMatch:RouteSpecHttp2RouteMatchHeaderMatchZMethod and value to match the header value sent with a request. Specify one match method.
T
name" HName for the HTTP header in the client request that will be matched on.
:�
�
appmesh#RouteSpecHttp2RouteMatchHeaderMatchSaws:appmesh/RouteSpecHttp2RouteMatchHeaderMatch:RouteSpecHttp2RouteMatchHeaderMatch�
�W
exactB" HHeader value sent by the client must match the specified value exactly.
Z
prefixB" JHeader value sent by the client must begin with the specified characters.
�
range�B�:�
�
appmesh(RouteSpecHttp2RouteMatchHeaderMatchRange]aws:appmesh/RouteSpecHttp2RouteMatchHeaderMatchRange:RouteSpecHttp2RouteMatchHeaderMatchRangeiObject that specifies the range of numbers that the header value sent by the client must be included in.
V
regexB" GHeader value sent by the client must include the specified characters.
X
suffixB" HHeader value sent by the client must end with the specified characters.
:�
�
appmesh(RouteSpecHttp2RouteMatchHeaderMatchRange]aws:appmesh/RouteSpecHttp2RouteMatchHeaderMatchRange:RouteSpecHttp2RouteMatchHeaderMatchRangeD
B
end End of the range.
!
start Start of the range.
:�
n
appmeshRouteSpecHttp2RouteMatchPathEaws:appmesh/RouteSpecHttp2RouteMatchPath:RouteSpecHttp2RouteMatchPathb
`+
exactB" The exact path to match on.
1
regexB" "The regex used to match the path.
:�
�
appmesh&RouteSpecHttp2RouteMatchQueryParameterYaws:appmesh/RouteSpecHttp2RouteMatchQueryParameter:RouteSpecHttp2RouteMatchQueryParameter�
��
match�B�:�
�
appmesh+RouteSpecHttp2RouteMatchQueryParameterMatchcaws:appmesh/RouteSpecHttp2RouteMatchQueryParameterMatch:RouteSpecHttp2RouteMatchQueryParameterMatch!The query parameter to match on.
B
name" 6Name for the query parameter that will be matched on.
:�
�
appmesh+RouteSpecHttp2RouteMatchQueryParameterMatchcaws:appmesh/RouteSpecHttp2RouteMatchQueryParameterMatch:RouteSpecHttp2RouteMatchQueryParameterMatch:
86
exactB" 'The exact query parameter to match on.
:�
t
appmeshRouteSpecHttp2RouteRetryPolicyIaws:appmesh/RouteSpecHttp2RouteRetryPolicy:RouteSpecHttp2RouteRetryPolicy�
��
httpRetryEventsB*" �List of HTTP retry events.
Valid values: `client-error` (HTTP status code 409), `gateway-error` (HTTP status codes 502, 503, and 504), `server-error` (HTTP status codes 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, and 511), `stream-error` (retry on refused stream).
-

maxRetries Maximum number of retries.
�
perRetryTimeout�:�
�
appmesh-RouteSpecHttp2RouteRetryPolicyPerRetryTimeoutgaws:appmesh/RouteSpecHttp2RouteRetryPolicyPerRetryTimeout:RouteSpecHttp2RouteRetryPolicyPerRetryTimeoutPer-retry timeout.
�
tcpRetryEventsB*" �List of TCP retry events. The only valid value is `connection-error`.

You must specify at least one value for `http_retry_events`, or at least one value for `tcp_retry_events`.
:�
�
appmesh-RouteSpecHttp2RouteRetryPolicyPerRetryTimeoutgaws:appmesh/RouteSpecHttp2RouteRetryPolicyPerRetryTimeout:RouteSpecHttp2RouteRetryPolicyPerRetryTimeoutQ
O1
unit" %Retry unit. Valid values: `ms`, `s`.

value Retry value.
:�
h
appmeshRouteSpecHttp2RouteTimeoutAaws:appmesh/RouteSpecHttp2RouteTimeout:RouteSpecHttp2RouteTimeout�
��
idlezBx:v
t
appmeshRouteSpecHttp2RouteTimeoutIdleIaws:appmesh/RouteSpecHttp2RouteTimeoutIdle:RouteSpecHttp2RouteTimeoutIdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
�

perRequest�B�:�
�
appmesh$RouteSpecHttp2RouteTimeoutPerRequestUaws:appmesh/RouteSpecHttp2RouteTimeoutPerRequest:RouteSpecHttp2RouteTimeoutPerRequestPer request timeout.
:�
t
appmeshRouteSpecHttp2RouteTimeoutIdleIaws:appmesh/RouteSpecHttp2RouteTimeoutIdle:RouteSpecHttp2RouteTimeoutIdler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh$RouteSpecHttp2RouteTimeoutPerRequestUaws:appmesh/RouteSpecHttp2RouteTimeoutPerRequest:RouteSpecHttp2RouteTimeoutPerRequestr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
P
appmeshRouteSpecHttpRoute1aws:appmesh/RouteSpecHttpRoute:RouteSpecHttpRoute�
��
actionf:d
b
appmeshRouteSpecHttpRouteAction=aws:appmesh/RouteSpecHttpRouteAction:RouteSpecHttpRouteAction)Action to take if a match is determined.
�
matchc:a
_
appmeshRouteSpecHttpRouteMatch;aws:appmesh/RouteSpecHttpRouteMatch:RouteSpecHttpRouteMatch0Criteria for determining an HTTP request match.
�
retryPolicywBu:s
q
appmeshRouteSpecHttpRouteRetryPolicyGaws:appmesh/RouteSpecHttpRouteRetryPolicy:RouteSpecHttpRouteRetryPolicyRetry policy.
�
timeoutkBi:g
e
appmeshRouteSpecHttpRouteTimeout?aws:appmesh/RouteSpecHttpRouteTimeout:RouteSpecHttpRouteTimeoutTypes of timeouts.
:�
b
appmeshRouteSpecHttpRouteAction=aws:appmesh/RouteSpecHttpRouteAction:RouteSpecHttpRouteAction�
��
weightedTargets�*�:�
�
appmesh&RouteSpecHttpRouteActionWeightedTargetYaws:appmesh/RouteSpecHttpRouteActionWeightedTarget:RouteSpecHttpRouteActionWeightedTarget�Targets that traffic is routed to when a request matches the route.
You can specify one or more targets and their relative weights with which to distribute traffic.
:�
�
appmesh&RouteSpecHttpRouteActionWeightedTargetYaws:appmesh/RouteSpecHttpRouteActionWeightedTarget:RouteSpecHttpRouteActionWeightedTarget�
�8
portB *The targeted port of the weighted object.
w
virtualNode" dVirtual node to associate with the weighted target. Must be between 1 and 255 characters in length.
T
weight FRelative weight of the weighted target. An integer between 0 and 100.
:�
_
appmeshRouteSpecHttpRouteMatch;aws:appmesh/RouteSpecHttpRouteMatch:RouteSpecHttpRouteMatch�
��
headersyBw*u:s
q
appmeshRouteSpecHttpRouteMatchHeaderGaws:appmesh/RouteSpecHttpRouteMatchHeader:RouteSpecHttpRouteMatchHeader$Client request headers to match on.
�
methodB" �Client request header method to match on. Valid values: `GET`, `HEAD`, `POST`, `PUT`, `DELETE`, `CONNECT`, `OPTIONS`, `TRACE`, `PATCH`.
�
pathqBo:m
k
appmeshRouteSpecHttpRouteMatchPathCaws:appmesh/RouteSpecHttpRouteMatchPath:RouteSpecHttpRouteMatchPath!Client request path to match on.
9
portB +The port number to match from the request.
�
prefixB" �Path with which to match requests.
This parameter must always start with /, which by itself matches all requests to the virtual router service name.
�
queryParameters�B�*�:�
�
appmesh%RouteSpecHttpRouteMatchQueryParameterWaws:appmesh/RouteSpecHttpRouteMatchQueryParameter:RouteSpecHttpRouteMatchQueryParameter-Client request query parameters to match on.
Y
schemeB" IClient request header scheme to match on. Valid values: `http`, `https`.
:�
q
appmeshRouteSpecHttpRouteMatchHeaderGaws:appmesh/RouteSpecHttpRouteMatchHeader:RouteSpecHttpRouteMatchHeader�
�m
invertB
 ]If `true`, the match is on the opposite of the `match` method and value. Default is `false`.
�
match�B�:�
�
appmesh"RouteSpecHttpRouteMatchHeaderMatchQaws:appmesh/RouteSpecHttpRouteMatchHeaderMatch:RouteSpecHttpRouteMatchHeaderMatchZMethod and value to match the header value sent with a request. Specify one match method.
T
name" HName for the HTTP header in the client request that will be matched on.
:�
�
appmesh"RouteSpecHttpRouteMatchHeaderMatchQaws:appmesh/RouteSpecHttpRouteMatchHeaderMatch:RouteSpecHttpRouteMatchHeaderMatch�
�W
exactB" HHeader value sent by the client must match the specified value exactly.
Z
prefixB" JHeader value sent by the client must begin with the specified characters.
�
range�B�:�
�
appmesh'RouteSpecHttpRouteMatchHeaderMatchRange[aws:appmesh/RouteSpecHttpRouteMatchHeaderMatchRange:RouteSpecHttpRouteMatchHeaderMatchRangeiObject that specifies the range of numbers that the header value sent by the client must be included in.
V
regexB" GHeader value sent by the client must include the specified characters.
X
suffixB" HHeader value sent by the client must end with the specified characters.
:�
�
appmesh'RouteSpecHttpRouteMatchHeaderMatchRange[aws:appmesh/RouteSpecHttpRouteMatchHeaderMatchRange:RouteSpecHttpRouteMatchHeaderMatchRangeD
B
end End of the range.
!
start Start of the range.
:�
k
appmeshRouteSpecHttpRouteMatchPathCaws:appmesh/RouteSpecHttpRouteMatchPath:RouteSpecHttpRouteMatchPathb
`+
exactB" The exact path to match on.
1
regexB" "The regex used to match the path.
:�
�
appmesh%RouteSpecHttpRouteMatchQueryParameterWaws:appmesh/RouteSpecHttpRouteMatchQueryParameter:RouteSpecHttpRouteMatchQueryParameter�
��
match�B�:�
�
appmesh*RouteSpecHttpRouteMatchQueryParameterMatchaaws:appmesh/RouteSpecHttpRouteMatchQueryParameterMatch:RouteSpecHttpRouteMatchQueryParameterMatch!The query parameter to match on.
B
name" 6Name for the query parameter that will be matched on.
:�
�
appmesh*RouteSpecHttpRouteMatchQueryParameterMatchaaws:appmesh/RouteSpecHttpRouteMatchQueryParameterMatch:RouteSpecHttpRouteMatchQueryParameterMatch:
86
exactB" 'The exact query parameter to match on.
:�
q
appmeshRouteSpecHttpRouteRetryPolicyGaws:appmesh/RouteSpecHttpRouteRetryPolicy:RouteSpecHttpRouteRetryPolicy�
��
httpRetryEventsB*" �List of HTTP retry events.
Valid values: `client-error` (HTTP status code 409), `gateway-error` (HTTP status codes 502, 503, and 504), `server-error` (HTTP status codes 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, and 511), `stream-error` (retry on refused stream).
-

maxRetries Maximum number of retries.
�
perRetryTimeout�:�
�
appmesh,RouteSpecHttpRouteRetryPolicyPerRetryTimeouteaws:appmesh/RouteSpecHttpRouteRetryPolicyPerRetryTimeout:RouteSpecHttpRouteRetryPolicyPerRetryTimeoutPer-retry timeout.
�
tcpRetryEventsB*" �List of TCP retry events. The only valid value is `connection-error`.

You must specify at least one value for `http_retry_events`, or at least one value for `tcp_retry_events`.
:�
�
appmesh,RouteSpecHttpRouteRetryPolicyPerRetryTimeouteaws:appmesh/RouteSpecHttpRouteRetryPolicyPerRetryTimeout:RouteSpecHttpRouteRetryPolicyPerRetryTimeoutQ
O1
unit" %Retry unit. Valid values: `ms`, `s`.

value Retry value.
:�
e
appmeshRouteSpecHttpRouteTimeout?aws:appmesh/RouteSpecHttpRouteTimeout:RouteSpecHttpRouteTimeout�
��
idlewBu:s
q
appmeshRouteSpecHttpRouteTimeoutIdleGaws:appmesh/RouteSpecHttpRouteTimeoutIdle:RouteSpecHttpRouteTimeoutIdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
�

perRequest�B�:�
�
appmesh#RouteSpecHttpRouteTimeoutPerRequestSaws:appmesh/RouteSpecHttpRouteTimeoutPerRequest:RouteSpecHttpRouteTimeoutPerRequestPer request timeout.
:�
q
appmeshRouteSpecHttpRouteTimeoutIdleGaws:appmesh/RouteSpecHttpRouteTimeoutIdle:RouteSpecHttpRouteTimeoutIdler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh#RouteSpecHttpRouteTimeoutPerRequestSaws:appmesh/RouteSpecHttpRouteTimeoutPerRequest:RouteSpecHttpRouteTimeoutPerRequestr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
M
appmeshRouteSpecTcpRoute/aws:appmesh/RouteSpecTcpRoute:RouteSpecTcpRoute�
��
actionc:a
_
appmeshRouteSpecTcpRouteAction;aws:appmesh/RouteSpecTcpRouteAction:RouteSpecTcpRouteAction)Action to take if a match is determined.
k
matchbB`:^
\
appmeshRouteSpecTcpRouteMatch9aws:appmesh/RouteSpecTcpRouteMatch:RouteSpecTcpRouteMatch�
timeouthBf:d
b
appmeshRouteSpecTcpRouteTimeout=aws:appmesh/RouteSpecTcpRouteTimeout:RouteSpecTcpRouteTimeoutTypes of timeouts.
:�
_
appmeshRouteSpecTcpRouteAction;aws:appmesh/RouteSpecTcpRouteAction:RouteSpecTcpRouteAction�
��
weightedTargets�*�:�
�
appmesh%RouteSpecTcpRouteActionWeightedTargetWaws:appmesh/RouteSpecTcpRouteActionWeightedTarget:RouteSpecTcpRouteActionWeightedTarget�Targets that traffic is routed to when a request matches the route.
You can specify one or more targets and their relative weights with which to distribute traffic.
:�
�
appmesh%RouteSpecTcpRouteActionWeightedTargetWaws:appmesh/RouteSpecTcpRouteActionWeightedTarget:RouteSpecTcpRouteActionWeightedTarget�
�8
portB *The targeted port of the weighted object.
w
virtualNode" dVirtual node to associate with the weighted target. Must be between 1 and 255 characters in length.
T
weight FRelative weight of the weighted target. An integer between 0 and 100.
:p
\
appmeshRouteSpecTcpRouteMatch9aws:appmesh/RouteSpecTcpRouteMatch:RouteSpecTcpRouteMatch

portB :�
b
appmeshRouteSpecTcpRouteTimeout=aws:appmesh/RouteSpecTcpRouteTimeout:RouteSpecTcpRouteTimeout�
��
idletBr:p
n
appmeshRouteSpecTcpRouteTimeoutIdleEaws:appmesh/RouteSpecTcpRouteTimeoutIdle:RouteSpecTcpRouteTimeoutIdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
:�
n
appmeshRouteSpecTcpRouteTimeoutIdleEaws:appmesh/RouteSpecTcpRouteTimeoutIdle:RouteSpecTcpRouteTimeoutIdler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
P
appmeshVirtualGatewaySpec1aws:appmesh/VirtualGatewaySpec:VirtualGatewaySpec�
��
backendDefaults�B�:
}
appmesh!VirtualGatewaySpecBackendDefaultsOaws:appmesh/VirtualGatewaySpecBackendDefaults:VirtualGatewaySpecBackendDefaultsDefaults for backends.
�
	listenersn*l:j
h
appmeshVirtualGatewaySpecListenerAaws:appmesh/VirtualGatewaySpecListener:VirtualGatewaySpecListenerlListeners that the mesh endpoint is expected to receive inbound traffic from. You can specify one listener.
�
loggingkBi:g
e
appmeshVirtualGatewaySpecLogging?aws:appmesh/VirtualGatewaySpecLogging:VirtualGatewaySpecLoggingIInbound and outbound access logging information for the virtual gateway.
:�
}
appmesh!VirtualGatewaySpecBackendDefaultsOaws:appmesh/VirtualGatewaySpecBackendDefaults:VirtualGatewaySpecBackendDefaults�
��
clientPolicy�B�:�
�
appmesh-VirtualGatewaySpecBackendDefaultsClientPolicygaws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicy:VirtualGatewaySpecBackendDefaultsClientPolicy4Default client policy for virtual gateway backends.
:�
�
appmesh-VirtualGatewaySpecBackendDefaultsClientPolicygaws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicy:VirtualGatewaySpecBackendDefaultsClientPolicy�
��
tls�B�:�
�
appmesh0VirtualGatewaySpecBackendDefaultsClientPolicyTlsmaws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTls:VirtualGatewaySpecBackendDefaultsClientPolicyTls.Transport Layer Security (TLS) client policy.
:�
�
appmesh0VirtualGatewaySpecBackendDefaultsClientPolicyTlsmaws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTls:VirtualGatewaySpecBackendDefaultsClientPolicyTls�
��
certificate�B�:�
�
appmesh;VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate:VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateListener's TLS certificate.
D
enforceB
 3Whether the policy is enforced. Default is `true`.
D
portsB* 3One or more ports that the policy is enforced for.
�

validation�:�
�
appmesh:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation>Listener's Transport Layer Security (TLS) validation context.
:�
�
appmesh;VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate:VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate�
��
file�B�:�
�
appmesh?VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFile�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFile:VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFileLocal file certificate.
�
sds�B�:�
�
appmesh>VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateSds�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateSds:VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateSds�A [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmesh?VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFile�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFile:VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
�

privateKey" �Private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmesh>VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateSds�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateSds:VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateSds�
��

secretName" }Name of the secret for a virtual gateway's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
�
appmesh:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation�
��
subjectAlternativeNames�B�:�
�
appmeshQVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames[SANs for a virtual gateway's listener's Transport Layer Security (TLS) validation context.
�
trust�:�
�
appmesh?VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustTLS validation context trust.
:�
�
appmeshQVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames�
��
match�:�
�
appmeshVVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch(Criteria for determining a SAN's match.
:�
�
appmeshVVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatchI
GE
exacts*" 5Values sent must match the specified values exactly.
:�

�
appmesh?VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust�
��
acm�B�:�
�
appmeshBVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcmOTLS validation context trust for an AWS Certificate Manager (ACM) certificate.
�
file�B�:�
�
appmeshCVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile;TLS validation context trust for a local file certificate.
�
sds�B�:�
�
appmeshBVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds�TLS validation context trust for a [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmeshBVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm<
:8
certificateAuthorityArns*" One or more ACM ARNs.
:�
�
appmeshCVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmeshBVirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds�aws:appmesh/VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds:VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds�
��

secretName" }Name of the secret for a virtual gateway's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
h
appmeshVirtualGatewaySpecListenerAaws:appmesh/VirtualGatewaySpecListener:VirtualGatewaySpecListener�
��
connectionPool�B�:�
�
appmesh(VirtualGatewaySpecListenerConnectionPool]aws:appmesh/VirtualGatewaySpecListenerConnectionPool:VirtualGatewaySpecListenerConnectionPool.Connection pool information for the listener.
�
healthCheck�B�:�
�
appmesh%VirtualGatewaySpecListenerHealthCheckWaws:appmesh/VirtualGatewaySpecListenerHealthCheck:VirtualGatewaySpecListenerHealthCheck+Health check information for the listener.
�
portMapping�:�
�
appmesh%VirtualGatewaySpecListenerPortMappingWaws:appmesh/VirtualGatewaySpecListenerPortMapping:VirtualGatewaySpecListenerPortMapping+Port mapping information for the listener.
�
tlswBu:s
q
appmeshVirtualGatewaySpecListenerTlsGaws:appmesh/VirtualGatewaySpecListenerTls:VirtualGatewaySpecListenerTls;Transport Layer Security (TLS) properties for the listener
:�
�
appmesh(VirtualGatewaySpecListenerConnectionPool]aws:appmesh/VirtualGatewaySpecListenerConnectionPool:VirtualGatewaySpecListenerConnectionPool�
��
grpc�B�:�
�
appmesh,VirtualGatewaySpecListenerConnectionPoolGrpceaws:appmesh/VirtualGatewaySpecListenerConnectionPoolGrpc:VirtualGatewaySpecListenerConnectionPoolGrpc0Connection pool information for gRPC listeners.
�
http�B�:�
�
appmesh,VirtualGatewaySpecListenerConnectionPoolHttpeaws:appmesh/VirtualGatewaySpecListenerConnectionPoolHttp:VirtualGatewaySpecListenerConnectionPoolHttp0Connection pool information for HTTP listeners.
�
http2�B�:�
�
appmesh-VirtualGatewaySpecListenerConnectionPoolHttp2gaws:appmesh/VirtualGatewaySpecListenerConnectionPoolHttp2:VirtualGatewaySpecListenerConnectionPoolHttp21Connection pool information for HTTP2 listeners.
:�
�
appmesh,VirtualGatewaySpecListenerConnectionPoolGrpceaws:appmesh/VirtualGatewaySpecListenerConnectionPoolGrpc:VirtualGatewaySpecListenerConnectionPoolGrpc�
��
maxRequests {Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster. Minimum value of `1`.
:�
�
appmesh,VirtualGatewaySpecListenerConnectionPoolHttpeaws:appmesh/VirtualGatewaySpecListenerConnectionPoolHttp:VirtualGatewaySpecListenerConnectionPoolHttp�
��
maxConnections �Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster. Minimum value of `1`.
�
maxPendingRequestsB sNumber of overflowing requests after `max_connections` Envoy will queue to upstream cluster. Minimum value of `1`.
:�
�
appmesh-VirtualGatewaySpecListenerConnectionPoolHttp2gaws:appmesh/VirtualGatewaySpecListenerConnectionPoolHttp2:VirtualGatewaySpecListenerConnectionPoolHttp2�
��
maxRequests {Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster. Minimum value of `1`.
:�
�
appmesh%VirtualGatewaySpecListenerHealthCheckWaws:appmesh/VirtualGatewaySpecListenerHealthCheck:VirtualGatewaySpecListenerHealthCheck�
�z
healthyThreshold bNumber of consecutive successful health checks that must occur before declaring listener healthy.
W
intervalMillis ATime period in milliseconds between each health check execution.
�
pathB" uDestination path for the health check request. This is only required if the specified protocol is `http` or `http2`.
�
portB }Destination port for the health check request. This port must match the port defined in the `port_mapping` for the listener.
e
protocol" UProtocol for the health check request. Valid values are `http`, `http2`, and `grpc`.
n
timeoutMillis YAmount of time to wait when receiving a response from the health check, in milliseconds.
�
unhealthyThreshold iNumber of consecutive failed health checks that must occur before declaring a virtual gateway unhealthy.
:�
�
appmesh%VirtualGatewaySpecListenerPortMappingWaws:appmesh/VirtualGatewaySpecListenerPortMapping:VirtualGatewaySpecListenerPortMapping�
�,
port  Port used for the port mapping.
h
protocol" XProtocol used for the port mapping. Valid values are `http`, `http2`, `tcp` and `grpc`.
:�
q
appmeshVirtualGatewaySpecListenerTlsGaws:appmesh/VirtualGatewaySpecListenerTls:VirtualGatewaySpecListenerTls�
��
certificate�:�
�
appmesh(VirtualGatewaySpecListenerTlsCertificate]aws:appmesh/VirtualGatewaySpecListenerTlsCertificate:VirtualGatewaySpecListenerTlsCertificateListener's TLS certificate.
S
mode" GListener's TLS mode. Valid values: `DISABLED`, `PERMISSIVE`, `STRICT`.
�

validation�B�:�
�
appmesh'VirtualGatewaySpecListenerTlsValidation[aws:appmesh/VirtualGatewaySpecListenerTlsValidation:VirtualGatewaySpecListenerTlsValidation>Listener's Transport Layer Security (TLS) validation context.
:�
�
appmesh(VirtualGatewaySpecListenerTlsCertificate]aws:appmesh/VirtualGatewaySpecListenerTlsCertificate:VirtualGatewaySpecListenerTlsCertificate�
��
acm�B�:�
�
appmesh+VirtualGatewaySpecListenerTlsCertificateAcmcaws:appmesh/VirtualGatewaySpecListenerTlsCertificateAcm:VirtualGatewaySpecListenerTlsCertificateAcm.An AWS Certificate Manager (ACM) certificate.
�
file�B�:�
�
appmesh,VirtualGatewaySpecListenerTlsCertificateFileeaws:appmesh/VirtualGatewaySpecListenerTlsCertificateFile:VirtualGatewaySpecListenerTlsCertificateFileLocal file certificate.
�
sds�B�:�
�
appmesh+VirtualGatewaySpecListenerTlsCertificateSdscaws:appmesh/VirtualGatewaySpecListenerTlsCertificateSds:VirtualGatewaySpecListenerTlsCertificateSds�A [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmesh+VirtualGatewaySpecListenerTlsCertificateAcmcaws:appmesh/VirtualGatewaySpecListenerTlsCertificateAcm:VirtualGatewaySpecListenerTlsCertificateAcm3
1/
certificateArn" ARN for the certificate.
:�
�
appmesh,VirtualGatewaySpecListenerTlsCertificateFileeaws:appmesh/VirtualGatewaySpecListenerTlsCertificateFile:VirtualGatewaySpecListenerTlsCertificateFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
�

privateKey" �Private key for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmesh+VirtualGatewaySpecListenerTlsCertificateSdscaws:appmesh/VirtualGatewaySpecListenerTlsCertificateSds:VirtualGatewaySpecListenerTlsCertificateSds�
��

secretName" }Name of the secret for a virtual gateway's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
�
appmesh'VirtualGatewaySpecListenerTlsValidation[aws:appmesh/VirtualGatewaySpecListenerTlsValidation:VirtualGatewaySpecListenerTlsValidation�
��
subjectAlternativeNames�B�:�
�
appmesh>VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNames:VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNames[SANs for a virtual gateway's listener's Transport Layer Security (TLS) validation context.
�
trust�:�
�
appmesh,VirtualGatewaySpecListenerTlsValidationTrusteaws:appmesh/VirtualGatewaySpecListenerTlsValidationTrust:VirtualGatewaySpecListenerTlsValidationTrustTLS validation context trust.
:�
�
appmesh>VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNames:VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNames�
��
match�:�
�
appmeshCVirtualGatewaySpecListenerTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNamesMatch:VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNamesMatch(Criteria for determining a SAN's match.
:�
�
appmeshCVirtualGatewaySpecListenerTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNamesMatch:VirtualGatewaySpecListenerTlsValidationSubjectAlternativeNamesMatchI
GE
exacts*" 5Values sent must match the specified values exactly.
:�
�
appmesh,VirtualGatewaySpecListenerTlsValidationTrusteaws:appmesh/VirtualGatewaySpecListenerTlsValidationTrust:VirtualGatewaySpecListenerTlsValidationTrust�
��
file�B�:�
�
appmesh0VirtualGatewaySpecListenerTlsValidationTrustFilemaws:appmesh/VirtualGatewaySpecListenerTlsValidationTrustFile:VirtualGatewaySpecListenerTlsValidationTrustFile;TLS validation context trust for a local file certificate.
�
sds�B�:�
�
appmesh/VirtualGatewaySpecListenerTlsValidationTrustSdskaws:appmesh/VirtualGatewaySpecListenerTlsValidationTrustSds:VirtualGatewaySpecListenerTlsValidationTrustSds�TLS validation context trust for a [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmesh0VirtualGatewaySpecListenerTlsValidationTrustFilemaws:appmesh/VirtualGatewaySpecListenerTlsValidationTrustFile:VirtualGatewaySpecListenerTlsValidationTrustFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmesh/VirtualGatewaySpecListenerTlsValidationTrustSdskaws:appmesh/VirtualGatewaySpecListenerTlsValidationTrustSds:VirtualGatewaySpecListenerTlsValidationTrustSds�
��

secretName" }Name of the secret for a virtual gateway's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
e
appmeshVirtualGatewaySpecLogging?aws:appmesh/VirtualGatewaySpecLogging:VirtualGatewaySpecLogging�
��
	accessLog�B�:�
�
appmesh"VirtualGatewaySpecLoggingAccessLogQaws:appmesh/VirtualGatewaySpecLoggingAccessLog:VirtualGatewaySpecLoggingAccessLog0Access log configuration for a virtual gateway.
:�
�
appmesh"VirtualGatewaySpecLoggingAccessLogQaws:appmesh/VirtualGatewaySpecLoggingAccessLog:VirtualGatewaySpecLoggingAccessLog�
��
file�B�:�
�
appmesh&VirtualGatewaySpecLoggingAccessLogFileYaws:appmesh/VirtualGatewaySpecLoggingAccessLogFile:VirtualGatewaySpecLoggingAccessLogFile4File object to send virtual gateway access logs to.
:�
�
appmesh&VirtualGatewaySpecLoggingAccessLogFileYaws:appmesh/VirtualGatewaySpecLoggingAccessLogFile:VirtualGatewaySpecLoggingAccessLogFile�
��
format�B�:�
�
appmesh,VirtualGatewaySpecLoggingAccessLogFileFormateaws:appmesh/VirtualGatewaySpecLoggingAccessLogFileFormat:VirtualGatewaySpecLoggingAccessLogFileFormat#The specified format for the logs.
�
path" �File path to write access logs to. You can use `/dev/stdout` to send access logs to standard out. Must be between 1 and 255 characters in length.
:�
�
appmesh,VirtualGatewaySpecLoggingAccessLogFileFormateaws:appmesh/VirtualGatewaySpecLoggingAccessLogFileFormat:VirtualGatewaySpecLoggingAccessLogFileFormat�
��
jsons�B�*�:�
�
appmesh0VirtualGatewaySpecLoggingAccessLogFileFormatJsonmaws:appmesh/VirtualGatewaySpecLoggingAccessLogFileFormatJson:VirtualGatewaySpecLoggingAccessLogFileFormatJsonThe logging format for JSON.
\
textB" NThe logging format for text. Must be between 1 and 1000 characters in length.
:�
�
appmesh0VirtualGatewaySpecLoggingAccessLogFileFormatJsonmaws:appmesh/VirtualGatewaySpecLoggingAccessLogFileFormatJson:VirtualGatewaySpecLoggingAccessLogFileFormatJson�
�[
key" PThe specified key for the JSON. Must be between 1 and 100 characters in length.
_
value" RThe specified value for the JSON. Must be between 1 and 100 characters in length.
:�
G
appmeshVirtualNodeSpec+aws:appmesh/VirtualNodeSpec:VirtualNodeSpec�
��
backendDefaultszBx:v
t
appmeshVirtualNodeSpecBackendDefaultsIaws:appmesh/VirtualNodeSpecBackendDefaults:VirtualNodeSpecBackendDefaultsDefaults for backends.
�
backendsdBb*`:^
\
appmeshVirtualNodeSpecBackend9aws:appmesh/VirtualNodeSpecBackend:VirtualNodeSpecBackendIBackends to which the virtual node is expected to send outbound traffic.
�
	listenersgBe*c:a
_
appmeshVirtualNodeSpecListener;aws:appmesh/VirtualNodeSpecListener:VirtualNodeSpecListenerNListeners from which the virtual node is expected to receive inbound traffic.
�
loggingbB`:^
\
appmeshVirtualNodeSpecLogging9aws:appmesh/VirtualNodeSpecLogging:VirtualNodeSpecLoggingFInbound and outbound access logging information for the virtual node.
�
serviceDiscovery}B{:y
w
appmeshVirtualNodeSpecServiceDiscoveryKaws:appmesh/VirtualNodeSpecServiceDiscovery:VirtualNodeSpecServiceDiscovery4Service discovery information for the virtual node.
:�
\
appmeshVirtualNodeSpecBackend9aws:appmesh/VirtualNodeSpecBackend:VirtualNodeSpecBackend�
��
virtualService�:�
�
appmesh$VirtualNodeSpecBackendVirtualServiceUaws:appmesh/VirtualNodeSpecBackendVirtualService:VirtualNodeSpecBackendVirtualService8Virtual service to use as a backend for a virtual node.
:�
t
appmeshVirtualNodeSpecBackendDefaultsIaws:appmesh/VirtualNodeSpecBackendDefaults:VirtualNodeSpecBackendDefaults�
��
clientPolicy�B�:�
�
appmesh*VirtualNodeSpecBackendDefaultsClientPolicyaaws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicy:VirtualNodeSpecBackendDefaultsClientPolicyKDefault client policy for virtual service backends. See above for details.
:�
�
appmesh*VirtualNodeSpecBackendDefaultsClientPolicyaaws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicy:VirtualNodeSpecBackendDefaultsClientPolicy�
��
tls�B�:�
�
appmesh-VirtualNodeSpecBackendDefaultsClientPolicyTlsgaws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTls:VirtualNodeSpecBackendDefaultsClientPolicyTls.Transport Layer Security (TLS) client policy.
:�
�
appmesh-VirtualNodeSpecBackendDefaultsClientPolicyTlsgaws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTls:VirtualNodeSpecBackendDefaultsClientPolicyTls�
��
certificate�B�:�
�
appmesh8VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificate}aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificate:VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateListener's TLS certificate.
D
enforceB
 3Whether the policy is enforced. Default is `true`.
D
portsB* 3One or more ports that the policy is enforced for.
�

validation�:�
�
appmesh7VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation{aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation>Listener's Transport Layer Security (TLS) validation context.
:�
�
appmesh8VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificate}aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificate:VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificate�
��
file�B�:�
�
appmesh<VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateFile�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateFile:VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateFileLocal file certificate.
�
sds�B�:�
�
appmesh;VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateSds�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateSds:VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateSds�A [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmesh<VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateFile�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateFile:VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
�

privateKey" �Private key for a certificate stored on the file system of the virtual node that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmesh;VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateSds�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateSds:VirtualNodeSpecBackendDefaultsClientPolicyTlsCertificateSds�
��

secretName" zName of the secret for a virtual node's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
�
appmesh7VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation{aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation�
��
subjectAlternativeNames�B�:�
�
appmeshNVirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames#SANs for a TLS validation context.
�
trust�:�
�
appmesh<VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrust�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrust:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustTLS validation context trust.
:�
�
appmeshNVirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames�
��
match�:�
�
appmeshSVirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch(Criteria for determining a SAN's match.
:�
�
appmeshSVirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatchI
GE
exacts*" 5Values sent must match the specified values exactly.
:�	
�
appmesh<VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrust�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrust:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrust�
��
acm�B�:�
�
appmesh?VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustAcm�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustAcm:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustAcmOTLS validation context trust for an AWS Certificate Manager (ACM) certificate.
�
file�B�:�
�
appmesh@VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustFile�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustFile:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustFile;TLS validation context trust for a local file certificate.
�
sds�B�:�
�
appmesh?VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustSds�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustSds:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustSds�TLS validation context trust for a [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmesh?VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustAcm�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustAcm:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustAcm<
:8
certificateAuthorityArns*" One or more ACM ARNs.
:�
�
appmesh@VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustFile�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustFile:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmesh?VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustSds�aws:appmesh/VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustSds:VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrustSds�
��

secretName" zName of the secret for a virtual node's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
�
appmesh$VirtualNodeSpecBackendVirtualServiceUaws:appmesh/VirtualNodeSpecBackendVirtualService:VirtualNodeSpecBackendVirtualService�
��
clientPolicy�B�:�
�
appmesh0VirtualNodeSpecBackendVirtualServiceClientPolicymaws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicy:VirtualNodeSpecBackendVirtualServiceClientPolicyClient policy for the backend.
�
virtualServiceName" vName of the virtual service that is acting as a virtual node backend. Must be between 1 and 255 characters in length.
:�
�
appmesh0VirtualNodeSpecBackendVirtualServiceClientPolicymaws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicy:VirtualNodeSpecBackendVirtualServiceClientPolicy�
��
tls�B�:�
�
appmesh3VirtualNodeSpecBackendVirtualServiceClientPolicyTlssaws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTls:VirtualNodeSpecBackendVirtualServiceClientPolicyTls.Transport Layer Security (TLS) client policy.
:�
�
appmesh3VirtualNodeSpecBackendVirtualServiceClientPolicyTlssaws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTls:VirtualNodeSpecBackendVirtualServiceClientPolicyTls�
��
certificate�B�:�
�
appmesh>VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificate�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificate:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateListener's TLS certificate.
D
enforceB
 3Whether the policy is enforced. Default is `true`.
D
portsB* 3One or more ports that the policy is enforced for.
�

validation�:�
�
appmesh=VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidation�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidation:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidation>Listener's Transport Layer Security (TLS) validation context.
:�
�
appmesh>VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificate�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificate:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificate�
��
file�B�:�
�
appmeshBVirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateFile�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateFile:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateFileLocal file certificate.
�
sds�B�:�
�
appmeshAVirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateSds�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateSds:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateSds�A [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmeshBVirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateFile�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateFile:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
�

privateKey" �Private key for a certificate stored on the file system of the virtual node that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmeshAVirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateSds�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateSds:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsCertificateSds�
��

secretName" zName of the secret for a virtual node's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
�
appmesh=VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidation�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidation:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidation�
��
subjectAlternativeNames�B�:�
�
appmeshTVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNames:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNames#SANs for a TLS validation context.
�
trust�:�
�
appmeshBVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrust�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrust:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustTLS validation context trust.
:�
�
appmeshTVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNames:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNames�
��
match�:�
�
appmeshYVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNamesMatch:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNamesMatch(Criteria for determining a SAN's match.
:�
�
appmeshYVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNamesMatch:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationSubjectAlternativeNamesMatchI
GE
exacts*" 5Values sent must match the specified values exactly.
:�

�
appmeshBVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrust�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrust:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrust�
��
acm�B�:�
�
appmeshEVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustAcm�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustAcm:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustAcmOTLS validation context trust for an AWS Certificate Manager (ACM) certificate.
�
file�B�:�
�
appmeshFVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustFile�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustFile:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustFile;TLS validation context trust for a local file certificate.
�
sds�B�:�
�
appmeshEVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustSds�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustSds:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustSds�TLS validation context trust for a [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmeshEVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustAcm�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustAcm:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustAcm<
:8
certificateAuthorityArns*" One or more ACM ARNs.
:�
�
appmeshFVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustFile�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustFile:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmeshEVirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustSds�aws:appmesh/VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustSds:VirtualNodeSpecBackendVirtualServiceClientPolicyTlsValidationTrustSds�
��

secretName" zName of the secret for a virtual node's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�

_
appmeshVirtualNodeSpecListener;aws:appmesh/VirtualNodeSpecListener:VirtualNodeSpecListener�	
�	�
connectionPool�B�:�
�
appmesh%VirtualNodeSpecListenerConnectionPoolWaws:appmesh/VirtualNodeSpecListenerConnectionPool:VirtualNodeSpecListenerConnectionPool.Connection pool information for the listener.
�
healthCheck�B�:�
�
appmesh"VirtualNodeSpecListenerHealthCheckQaws:appmesh/VirtualNodeSpecListenerHealthCheck:VirtualNodeSpecListenerHealthCheck+Health check information for the listener.
�
outlierDetection�B�:�
�
appmesh'VirtualNodeSpecListenerOutlierDetection[aws:appmesh/VirtualNodeSpecListenerOutlierDetection:VirtualNodeSpecListenerOutlierDetection0Outlier detection information for the listener.
�
portMapping�:�
�
appmesh"VirtualNodeSpecListenerPortMappingQaws:appmesh/VirtualNodeSpecListenerPortMapping:VirtualNodeSpecListenerPortMapping+Port mapping information for the listener.
�
timeoutzBx:v
t
appmeshVirtualNodeSpecListenerTimeoutIaws:appmesh/VirtualNodeSpecListenerTimeout:VirtualNodeSpecListenerTimeout"Timeouts for different protocols.
�
tlsnBl:j
h
appmeshVirtualNodeSpecListenerTlsAaws:appmesh/VirtualNodeSpecListenerTls:VirtualNodeSpecListenerTls;Transport Layer Security (TLS) properties for the listener
:�
�
appmesh%VirtualNodeSpecListenerConnectionPoolWaws:appmesh/VirtualNodeSpecListenerConnectionPool:VirtualNodeSpecListenerConnectionPool�
��
grpc�B�:�
�
appmesh)VirtualNodeSpecListenerConnectionPoolGrpc_aws:appmesh/VirtualNodeSpecListenerConnectionPoolGrpc:VirtualNodeSpecListenerConnectionPoolGrpc0Connection pool information for gRPC listeners.
�
http2s�B�*�:�
�
appmesh*VirtualNodeSpecListenerConnectionPoolHttp2aaws:appmesh/VirtualNodeSpecListenerConnectionPoolHttp2:VirtualNodeSpecListenerConnectionPoolHttp21Connection pool information for HTTP2 listeners.
�
https�B�*�:�
�
appmesh)VirtualNodeSpecListenerConnectionPoolHttp_aws:appmesh/VirtualNodeSpecListenerConnectionPoolHttp:VirtualNodeSpecListenerConnectionPoolHttp0Connection pool information for HTTP listeners.
�
tcps�B�*�:�
�
appmesh(VirtualNodeSpecListenerConnectionPoolTcp]aws:appmesh/VirtualNodeSpecListenerConnectionPoolTcp:VirtualNodeSpecListenerConnectionPoolTcp/Connection pool information for TCP listeners.
:�
�
appmesh)VirtualNodeSpecListenerConnectionPoolGrpc_aws:appmesh/VirtualNodeSpecListenerConnectionPoolGrpc:VirtualNodeSpecListenerConnectionPoolGrpc�
��
maxRequests {Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster. Minimum value of `1`.
:�
�
appmesh)VirtualNodeSpecListenerConnectionPoolHttp_aws:appmesh/VirtualNodeSpecListenerConnectionPoolHttp:VirtualNodeSpecListenerConnectionPoolHttp�
��
maxConnections �Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster. Minimum value of `1`.
�
maxPendingRequestsB sNumber of overflowing requests after `max_connections` Envoy will queue to upstream cluster. Minimum value of `1`.
:�
�
appmesh*VirtualNodeSpecListenerConnectionPoolHttp2aaws:appmesh/VirtualNodeSpecListenerConnectionPoolHttp2:VirtualNodeSpecListenerConnectionPoolHttp2�
��
maxRequests {Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster. Minimum value of `1`.
:�
�
appmesh(VirtualNodeSpecListenerConnectionPoolTcp]aws:appmesh/VirtualNodeSpecListenerConnectionPoolTcp:VirtualNodeSpecListenerConnectionPoolTcp�
��
maxConnections �Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster. Minimum value of `1`.
:�
�
appmesh"VirtualNodeSpecListenerHealthCheckQaws:appmesh/VirtualNodeSpecListenerHealthCheck:VirtualNodeSpecListenerHealthCheck�
�z
healthyThreshold bNumber of consecutive successful health checks that must occur before declaring listener healthy.
W
intervalMillis ATime period in milliseconds between each health check execution.
�
pathB" uDestination path for the health check request. This is only required if the specified protocol is `http` or `http2`.
�
portB }Destination port for the health check request. This port must match the port defined in the `port_mapping` for the listener.
k
protocol" [Protocol for the health check request. Valid values are `http`, `http2`, `tcp` and `grpc`.
n
timeoutMillis YAmount of time to wait when receiving a response from the health check, in milliseconds.
�
unhealthyThreshold fNumber of consecutive failed health checks that must occur before declaring a virtual node unhealthy.
:�
�
appmesh'VirtualNodeSpecListenerOutlierDetection[aws:appmesh/VirtualNodeSpecListenerOutlierDetection:VirtualNodeSpecListenerOutlierDetection�
��
baseEjectionDuration�:�
�
appmesh;VirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration�aws:appmesh/VirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration:VirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration1Base amount of time for which a host is ejected.
�
interval�:�
�
appmesh/VirtualNodeSpecListenerOutlierDetectionIntervalkaws:appmesh/VirtualNodeSpecListenerOutlierDetectionInterval:VirtualNodeSpecListenerOutlierDetectionInterval/Time interval between ejection sweep analysis.
�
maxEjectionPercent �Maximum percentage of hosts in load balancing pool for upstream service that can be ejected. Will eject at least one host regardless of the value.
Minimum value of `0`. Maximum value of `100`.
g
maxServerErrors PNumber of consecutive `5xx` errors required for ejection. Minimum value of `1`.
:�
�
appmesh;VirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration�aws:appmesh/VirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration:VirtualNodeSpecListenerOutlierDetectionBaseEjectionDurationr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh/VirtualNodeSpecListenerOutlierDetectionIntervalkaws:appmesh/VirtualNodeSpecListenerOutlierDetectionInterval:VirtualNodeSpecListenerOutlierDetectionIntervalr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh"VirtualNodeSpecListenerPortMappingQaws:appmesh/VirtualNodeSpecListenerPortMapping:VirtualNodeSpecListenerPortMapping�
�,
port  Port used for the port mapping.
h
protocol" XProtocol used for the port mapping. Valid values are `http`, `http2`, `tcp` and `grpc`.
:�
t
appmeshVirtualNodeSpecListenerTimeoutIaws:appmesh/VirtualNodeSpecListenerTimeout:VirtualNodeSpecListenerTimeout�
��
grpc�B�:�
�
appmesh"VirtualNodeSpecListenerTimeoutGrpcQaws:appmesh/VirtualNodeSpecListenerTimeoutGrpc:VirtualNodeSpecListenerTimeoutGrpcTimeouts for gRPC listeners.
�
http�B�:�
�
appmesh"VirtualNodeSpecListenerTimeoutHttpQaws:appmesh/VirtualNodeSpecListenerTimeoutHttp:VirtualNodeSpecListenerTimeoutHttpTimeouts for HTTP listeners.
�
http2�B�:�
�
appmesh#VirtualNodeSpecListenerTimeoutHttp2Saws:appmesh/VirtualNodeSpecListenerTimeoutHttp2:VirtualNodeSpecListenerTimeoutHttp2Timeouts for HTTP2 listeners.
�
tcp�B�:
}
appmesh!VirtualNodeSpecListenerTimeoutTcpOaws:appmesh/VirtualNodeSpecListenerTimeoutTcp:VirtualNodeSpecListenerTimeoutTcpTimeouts for TCP listeners.
:�
�
appmesh"VirtualNodeSpecListenerTimeoutGrpcQaws:appmesh/VirtualNodeSpecListenerTimeoutGrpc:VirtualNodeSpecListenerTimeoutGrpc�
��
idle�B�:�
�
appmesh&VirtualNodeSpecListenerTimeoutGrpcIdleYaws:appmesh/VirtualNodeSpecListenerTimeoutGrpcIdle:VirtualNodeSpecListenerTimeoutGrpcIdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
�

perRequest�B�:�
�
appmesh,VirtualNodeSpecListenerTimeoutGrpcPerRequesteaws:appmesh/VirtualNodeSpecListenerTimeoutGrpcPerRequest:VirtualNodeSpecListenerTimeoutGrpcPerRequestPer request timeout.
:�
�
appmesh&VirtualNodeSpecListenerTimeoutGrpcIdleYaws:appmesh/VirtualNodeSpecListenerTimeoutGrpcIdle:VirtualNodeSpecListenerTimeoutGrpcIdler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh,VirtualNodeSpecListenerTimeoutGrpcPerRequesteaws:appmesh/VirtualNodeSpecListenerTimeoutGrpcPerRequest:VirtualNodeSpecListenerTimeoutGrpcPerRequestr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh"VirtualNodeSpecListenerTimeoutHttpQaws:appmesh/VirtualNodeSpecListenerTimeoutHttp:VirtualNodeSpecListenerTimeoutHttp�
��
idle�B�:�
�
appmesh&VirtualNodeSpecListenerTimeoutHttpIdleYaws:appmesh/VirtualNodeSpecListenerTimeoutHttpIdle:VirtualNodeSpecListenerTimeoutHttpIdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
�

perRequest�B�:�
�
appmesh,VirtualNodeSpecListenerTimeoutHttpPerRequesteaws:appmesh/VirtualNodeSpecListenerTimeoutHttpPerRequest:VirtualNodeSpecListenerTimeoutHttpPerRequestPer request timeout.
:�
�
appmesh#VirtualNodeSpecListenerTimeoutHttp2Saws:appmesh/VirtualNodeSpecListenerTimeoutHttp2:VirtualNodeSpecListenerTimeoutHttp2�
��
idle�B�:�
�
appmesh'VirtualNodeSpecListenerTimeoutHttp2Idle[aws:appmesh/VirtualNodeSpecListenerTimeoutHttp2Idle:VirtualNodeSpecListenerTimeoutHttp2IdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
�

perRequest�B�:�
�
appmesh-VirtualNodeSpecListenerTimeoutHttp2PerRequestgaws:appmesh/VirtualNodeSpecListenerTimeoutHttp2PerRequest:VirtualNodeSpecListenerTimeoutHttp2PerRequestPer request timeout.
:�
�
appmesh'VirtualNodeSpecListenerTimeoutHttp2Idle[aws:appmesh/VirtualNodeSpecListenerTimeoutHttp2Idle:VirtualNodeSpecListenerTimeoutHttp2Idler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh-VirtualNodeSpecListenerTimeoutHttp2PerRequestgaws:appmesh/VirtualNodeSpecListenerTimeoutHttp2PerRequest:VirtualNodeSpecListenerTimeoutHttp2PerRequestr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh&VirtualNodeSpecListenerTimeoutHttpIdleYaws:appmesh/VirtualNodeSpecListenerTimeoutHttpIdle:VirtualNodeSpecListenerTimeoutHttpIdler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
�
appmesh,VirtualNodeSpecListenerTimeoutHttpPerRequesteaws:appmesh/VirtualNodeSpecListenerTimeoutHttpPerRequest:VirtualNodeSpecListenerTimeoutHttpPerRequestr
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
}
appmesh!VirtualNodeSpecListenerTimeoutTcpOaws:appmesh/VirtualNodeSpecListenerTimeoutTcp:VirtualNodeSpecListenerTimeoutTcp�
��
idle�B�:�
�
appmesh%VirtualNodeSpecListenerTimeoutTcpIdleWaws:appmesh/VirtualNodeSpecListenerTimeoutTcpIdle:VirtualNodeSpecListenerTimeoutTcpIdleWIdle timeout. An idle timeout bounds the amount of time that a connection may be idle.
:�
�
appmesh%VirtualNodeSpecListenerTimeoutTcpIdleWaws:appmesh/VirtualNodeSpecListenerTimeoutTcpIdle:VirtualNodeSpecListenerTimeoutTcpIdler
p3
unit" 'Unit of time. Valid values: `ms`, `s`.
9
value ,Number of time units. Minimum value of `0`.
:�
h
appmeshVirtualNodeSpecListenerTlsAaws:appmesh/VirtualNodeSpecListenerTls:VirtualNodeSpecListenerTls�
��
certificate�:�
�
appmesh%VirtualNodeSpecListenerTlsCertificateWaws:appmesh/VirtualNodeSpecListenerTlsCertificate:VirtualNodeSpecListenerTlsCertificateListener's TLS certificate.
S
mode" GListener's TLS mode. Valid values: `DISABLED`, `PERMISSIVE`, `STRICT`.
�

validation�B�:�
�
appmesh$VirtualNodeSpecListenerTlsValidationUaws:appmesh/VirtualNodeSpecListenerTlsValidation:VirtualNodeSpecListenerTlsValidation>Listener's Transport Layer Security (TLS) validation context.
:�
�
appmesh%VirtualNodeSpecListenerTlsCertificateWaws:appmesh/VirtualNodeSpecListenerTlsCertificate:VirtualNodeSpecListenerTlsCertificate�
��
acm�B�:�
�
appmesh(VirtualNodeSpecListenerTlsCertificateAcm]aws:appmesh/VirtualNodeSpecListenerTlsCertificateAcm:VirtualNodeSpecListenerTlsCertificateAcm.An AWS Certificate Manager (ACM) certificate.
�
file�B�:�
�
appmesh)VirtualNodeSpecListenerTlsCertificateFile_aws:appmesh/VirtualNodeSpecListenerTlsCertificateFile:VirtualNodeSpecListenerTlsCertificateFileLocal file certificate.
�
sds�B�:�
�
appmesh(VirtualNodeSpecListenerTlsCertificateSds]aws:appmesh/VirtualNodeSpecListenerTlsCertificateSds:VirtualNodeSpecListenerTlsCertificateSds�A [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmesh(VirtualNodeSpecListenerTlsCertificateAcm]aws:appmesh/VirtualNodeSpecListenerTlsCertificateAcm:VirtualNodeSpecListenerTlsCertificateAcm3
1/
certificateArn" ARN for the certificate.
:�
�
appmesh)VirtualNodeSpecListenerTlsCertificateFile_aws:appmesh/VirtualNodeSpecListenerTlsCertificateFile:VirtualNodeSpecListenerTlsCertificateFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
�

privateKey" �Private key for a certificate stored on the file system of the virtual node that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmesh(VirtualNodeSpecListenerTlsCertificateSds]aws:appmesh/VirtualNodeSpecListenerTlsCertificateSds:VirtualNodeSpecListenerTlsCertificateSds�
��

secretName" zName of the secret for a virtual node's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
�
appmesh$VirtualNodeSpecListenerTlsValidationUaws:appmesh/VirtualNodeSpecListenerTlsValidation:VirtualNodeSpecListenerTlsValidation�
��
subjectAlternativeNames�B�:�
�
appmesh;VirtualNodeSpecListenerTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualNodeSpecListenerTlsValidationSubjectAlternativeNames:VirtualNodeSpecListenerTlsValidationSubjectAlternativeNames#SANs for a TLS validation context.
�
trust�:�
�
appmesh)VirtualNodeSpecListenerTlsValidationTrust_aws:appmesh/VirtualNodeSpecListenerTlsValidationTrust:VirtualNodeSpecListenerTlsValidationTrustTLS validation context trust.
:�
�
appmesh;VirtualNodeSpecListenerTlsValidationSubjectAlternativeNames�aws:appmesh/VirtualNodeSpecListenerTlsValidationSubjectAlternativeNames:VirtualNodeSpecListenerTlsValidationSubjectAlternativeNames�
��
match�:�
�
appmesh@VirtualNodeSpecListenerTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualNodeSpecListenerTlsValidationSubjectAlternativeNamesMatch:VirtualNodeSpecListenerTlsValidationSubjectAlternativeNamesMatch(Criteria for determining a SAN's match.
:�
�
appmesh@VirtualNodeSpecListenerTlsValidationSubjectAlternativeNamesMatch�aws:appmesh/VirtualNodeSpecListenerTlsValidationSubjectAlternativeNamesMatch:VirtualNodeSpecListenerTlsValidationSubjectAlternativeNamesMatchI
GE
exacts*" 5Values sent must match the specified values exactly.
:�
�
appmesh)VirtualNodeSpecListenerTlsValidationTrust_aws:appmesh/VirtualNodeSpecListenerTlsValidationTrust:VirtualNodeSpecListenerTlsValidationTrust�
��
file�B�:�
�
appmesh-VirtualNodeSpecListenerTlsValidationTrustFilegaws:appmesh/VirtualNodeSpecListenerTlsValidationTrustFile:VirtualNodeSpecListenerTlsValidationTrustFile;TLS validation context trust for a local file certificate.
�
sds�B�:�
�
appmesh,VirtualNodeSpecListenerTlsValidationTrustSdseaws:appmesh/VirtualNodeSpecListenerTlsValidationTrustSds:VirtualNodeSpecListenerTlsValidationTrustSds�TLS validation context trust for a [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
:�
�
appmesh-VirtualNodeSpecListenerTlsValidationTrustFilegaws:appmesh/VirtualNodeSpecListenerTlsValidationTrustFile:VirtualNodeSpecListenerTlsValidationTrustFile�
��
certificateChain" �Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
:�
�
appmesh,VirtualNodeSpecListenerTlsValidationTrustSdseaws:appmesh/VirtualNodeSpecListenerTlsValidationTrustSds:VirtualNodeSpecListenerTlsValidationTrustSds�
��

secretName" zName of the secret for a virtual node's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
:�
\
appmeshVirtualNodeSpecLogging9aws:appmesh/VirtualNodeSpecLogging:VirtualNodeSpecLogging�
��
	accessLog}B{:y
w
appmeshVirtualNodeSpecLoggingAccessLogKaws:appmesh/VirtualNodeSpecLoggingAccessLog:VirtualNodeSpecLoggingAccessLog-Access log configuration for a virtual node.
:�
w
appmeshVirtualNodeSpecLoggingAccessLogKaws:appmesh/VirtualNodeSpecLoggingAccessLog:VirtualNodeSpecLoggingAccessLog�
��
file�B�:�
�
appmesh#VirtualNodeSpecLoggingAccessLogFileSaws:appmesh/VirtualNodeSpecLoggingAccessLogFile:VirtualNodeSpecLoggingAccessLogFile1File object to send virtual node access logs to.
:�
�
appmesh#VirtualNodeSpecLoggingAccessLogFileSaws:appmesh/VirtualNodeSpecLoggingAccessLogFile:VirtualNodeSpecLoggingAccessLogFile�
��
format�B�:�
�
appmesh)VirtualNodeSpecLoggingAccessLogFileFormat_aws:appmesh/VirtualNodeSpecLoggingAccessLogFileFormat:VirtualNodeSpecLoggingAccessLogFileFormat#The specified format for the logs.
�
path" �File path to write access logs to. You can use `/dev/stdout` to send access logs to standard out. Must be between 1 and 255 characters in length.
:�
�
appmesh)VirtualNodeSpecLoggingAccessLogFileFormat_aws:appmesh/VirtualNodeSpecLoggingAccessLogFileFormat:VirtualNodeSpecLoggingAccessLogFileFormat�
��
jsons�B�*�:�
�
appmesh-VirtualNodeSpecLoggingAccessLogFileFormatJsongaws:appmesh/VirtualNodeSpecLoggingAccessLogFileFormatJson:VirtualNodeSpecLoggingAccessLogFileFormatJsonThe logging format for JSON.
\
textB" NThe logging format for text. Must be between 1 and 1000 characters in length.
:�
�
appmesh-VirtualNodeSpecLoggingAccessLogFileFormatJsongaws:appmesh/VirtualNodeSpecLoggingAccessLogFileFormatJson:VirtualNodeSpecLoggingAccessLogFileFormatJson�
�[
key" PThe specified key for the JSON. Must be between 1 and 100 characters in length.
_
value" RThe specified value for the JSON. Must be between 1 and 100 characters in length.
:�
w
appmeshVirtualNodeSpecServiceDiscoveryKaws:appmesh/VirtualNodeSpecServiceDiscovery:VirtualNodeSpecServiceDiscovery�
��
awsCloudMap�B�:�
�
appmesh*VirtualNodeSpecServiceDiscoveryAwsCloudMapaaws:appmesh/VirtualNodeSpecServiceDiscoveryAwsCloudMap:VirtualNodeSpecServiceDiscoveryAwsCloudMap4Any AWS Cloud Map information for the virtual node.
�
dns�B�:�
�
appmesh"VirtualNodeSpecServiceDiscoveryDnsQaws:appmesh/VirtualNodeSpecServiceDiscoveryDns:VirtualNodeSpecServiceDiscoveryDns'DNS service name for the virtual node.
:�
�
appmesh*VirtualNodeSpecServiceDiscoveryAwsCloudMapaaws:appmesh/VirtualNodeSpecServiceDiscoveryAwsCloudMap:VirtualNodeSpecServiceDiscoveryAwsCloudMap�
��

attributesB2" �String map that contains attributes with values that you can use to filter instances by any custom attribute that you specified when you registered the instance. Only instances that match all of the specified key/value pairs will be returned.
�
namespaceName" �Name of the AWS Cloud Map namespace to use.
Use the `aws.servicediscovery.HttpNamespace` resource to configure a Cloud Map namespace. Must be between 1 and 1024 characters in length.
�
serviceName" �Name of the AWS Cloud Map service to use. Use the `aws.servicediscovery.Service` resource to configure a Cloud Map service. Must be between 1 and 1024 characters in length.
:�
�
appmesh"VirtualNodeSpecServiceDiscoveryDnsQaws:appmesh/VirtualNodeSpecServiceDiscoveryDns:VirtualNodeSpecServiceDiscoveryDns�
�5
hostname" %DNS host name for your virtual node.
�
ipPreferenceB" �The preferred IP version that this virtual node uses. Valid values: `IPv6_PREFERRED`, `IPv4_PREFERRED`, `IPv4_ONLY`, `IPv6_ONLY`.
m
responseTypeB" WThe DNS response type for the virtual node. Valid values: `LOADBALANCER`, `ENDPOINTS`.
:�
M
appmeshVirtualRouterSpec/aws:appmesh/VirtualRouterSpec:VirtualRouterSpec�
��
	listenersmBk*i:g
e
appmeshVirtualRouterSpecListener?aws:appmesh/VirtualRouterSpecListener:VirtualRouterSpecListener�Listeners that the virtual router is expected to receive inbound traffic from.
Currently only one listener is supported per virtual router.
:�
e
appmeshVirtualRouterSpecListener?aws:appmesh/VirtualRouterSpecListener:VirtualRouterSpecListener�
��
portMapping�:�
�
appmesh$VirtualRouterSpecListenerPortMappingUaws:appmesh/VirtualRouterSpecListenerPortMapping:VirtualRouterSpecListenerPortMapping+Port mapping information for the listener.
:�
�
appmesh$VirtualRouterSpecListenerPortMappingUaws:appmesh/VirtualRouterSpecListenerPortMapping:VirtualRouterSpecListenerPortMapping�
�,
port  Port used for the port mapping.
g
protocol" WProtocol used for the port mapping. Valid values are `http`,`http2`, `tcp` and `grpc`.
:�
P
appmeshVirtualServiceSpec1aws:appmesh/VirtualServiceSpec:VirtualServiceSpec�
��
providernBl:j
h
appmeshVirtualServiceSpecProviderAaws:appmesh/VirtualServiceSpecProvider:VirtualServiceSpecProviderApp Mesh object that is acting as the provider for a virtual service. You can specify a single virtual node or virtual router.
:�
h
appmeshVirtualServiceSpecProviderAaws:appmesh/VirtualServiceSpecProvider:VirtualServiceSpecProvider�
��
virtualNode�B�:�
�
appmesh%VirtualServiceSpecProviderVirtualNodeWaws:appmesh/VirtualServiceSpecProviderVirtualNode:VirtualServiceSpecProviderVirtualNode0Virtual node associated with a virtual service.
�
virtualRouter�B�:�
�
appmesh'VirtualServiceSpecProviderVirtualRouter[aws:appmesh/VirtualServiceSpecProviderVirtualRouter:VirtualServiceSpecProviderVirtualRouter2Virtual router associated with a virtual service.
:�
�
appmesh%VirtualServiceSpecProviderVirtualNodeWaws:appmesh/VirtualServiceSpecProviderVirtualNode:VirtualServiceSpecProviderVirtualNode�
��
virtualNodeName" oName of the virtual node that is acting as a service provider. Must be between 1 and 255 characters in length.
:�
�
appmesh'VirtualServiceSpecProviderVirtualRouter[aws:appmesh/VirtualServiceSpecProviderVirtualRouter:VirtualServiceSpecProviderVirtualRouter�
��
virtualRouterName" qName of the virtual router that is acting as a service provider. Must be between 1 and 255 characters in length.
:�
S
appmeshgetGatewayRouteSpec3aws:appmesh/getGatewayRouteSpec:getGatewayRouteSpec�
��

grpcRoutest*r:p
n
appmeshgetGatewayRouteSpecGrpcRouteEaws:appmesh/getGatewayRouteSpecGrpcRoute:getGatewayRouteSpecGrpcRoute�
http2Routesw*u:s
q
appmeshgetGatewayRouteSpecHttp2RouteGaws:appmesh/getGatewayRouteSpecHttp2Route:getGatewayRouteSpecHttp2Route�

httpRoutest*r:p
n
appmeshgetGatewayRouteSpecHttpRouteEaws:appmesh/getGatewayRouteSpecHttpRoute:getGatewayRouteSpecHttpRoute
priority :�
n
appmeshgetGatewayRouteSpecGrpcRouteEaws:appmesh/getGatewayRouteSpecGrpcRoute:getGatewayRouteSpecGrpcRoute�
��
actions�*�:�
�
appmesh"getGatewayRouteSpecGrpcRouteActionQaws:appmesh/getGatewayRouteSpecGrpcRouteAction:getGatewayRouteSpecGrpcRouteAction�
matches�*�:
}
appmesh!getGatewayRouteSpecGrpcRouteMatchOaws:appmesh/getGatewayRouteSpecGrpcRouteMatch:getGatewayRouteSpecGrpcRouteMatch:�
�
appmesh"getGatewayRouteSpecGrpcRouteActionQaws:appmesh/getGatewayRouteSpecGrpcRouteAction:getGatewayRouteSpecGrpcRouteAction�
��
targets�*�:�
�
appmesh(getGatewayRouteSpecGrpcRouteActionTarget]aws:appmesh/getGatewayRouteSpecGrpcRouteActionTarget:getGatewayRouteSpecGrpcRouteActionTarget:�
�
appmesh(getGatewayRouteSpecGrpcRouteActionTarget]aws:appmesh/getGatewayRouteSpecGrpcRouteActionTarget:getGatewayRouteSpecGrpcRouteActionTarget�
�

port �
virtualServices�*�:�
�
appmesh6getGatewayRouteSpecGrpcRouteActionTargetVirtualServiceyaws:appmesh/getGatewayRouteSpecGrpcRouteActionTargetVirtualService:getGatewayRouteSpecGrpcRouteActionTargetVirtualService:�
�
appmesh6getGatewayRouteSpecGrpcRouteActionTargetVirtualServiceyaws:appmesh/getGatewayRouteSpecGrpcRouteActionTargetVirtualService:getGatewayRouteSpecGrpcRouteActionTargetVirtualService

virtualServiceName" :�
}
appmesh!getGatewayRouteSpecGrpcRouteMatchOaws:appmesh/getGatewayRouteSpecGrpcRouteMatch:getGatewayRouteSpecGrpcRouteMatch!


port 
serviceName" :�
q
appmeshgetGatewayRouteSpecHttp2RouteGaws:appmesh/getGatewayRouteSpecHttp2Route:getGatewayRouteSpecHttp2Route�
��
actions�*�:�
�
appmesh#getGatewayRouteSpecHttp2RouteActionSaws:appmesh/getGatewayRouteSpecHttp2RouteAction:getGatewayRouteSpecHttp2RouteAction�
matches�*�:�
�
appmesh"getGatewayRouteSpecHttp2RouteMatchQaws:appmesh/getGatewayRouteSpecHttp2RouteMatch:getGatewayRouteSpecHttp2RouteMatch:�
�
appmesh#getGatewayRouteSpecHttp2RouteActionSaws:appmesh/getGatewayRouteSpecHttp2RouteAction:getGatewayRouteSpecHttp2RouteAction�
��
rewrites�*�:�
�
appmesh*getGatewayRouteSpecHttp2RouteActionRewriteaaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewrite:getGatewayRouteSpecHttp2RouteActionRewrite�
targets�*�:�
�
appmesh)getGatewayRouteSpecHttp2RouteActionTarget_aws:appmesh/getGatewayRouteSpecHttp2RouteActionTarget:getGatewayRouteSpecHttp2RouteActionTarget:�
�
appmesh*getGatewayRouteSpecHttp2RouteActionRewriteaaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewrite:getGatewayRouteSpecHttp2RouteActionRewrite�
��
	hostnames�*�:�
�
appmesh2getGatewayRouteSpecHttp2RouteActionRewriteHostnameqaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewriteHostname:getGatewayRouteSpecHttp2RouteActionRewriteHostname�
paths�*�:�
�
appmesh.getGatewayRouteSpecHttp2RouteActionRewritePathiaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewritePath:getGatewayRouteSpecHttp2RouteActionRewritePath�
prefixes�*�:�
�
appmesh0getGatewayRouteSpecHttp2RouteActionRewritePrefixmaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewritePrefix:getGatewayRouteSpecHttp2RouteActionRewritePrefix:�
�
appmesh2getGatewayRouteSpecHttp2RouteActionRewriteHostnameqaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewriteHostname:getGatewayRouteSpecHttp2RouteActionRewriteHostname

defaultTargetHostname" :�
�
appmesh.getGatewayRouteSpecHttp2RouteActionRewritePathiaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewritePath:getGatewayRouteSpecHttp2RouteActionRewritePath

exact" :�
�
appmesh0getGatewayRouteSpecHttp2RouteActionRewritePrefixmaws:appmesh/getGatewayRouteSpecHttp2RouteActionRewritePrefix:getGatewayRouteSpecHttp2RouteActionRewritePrefix$
"
defaultPrefix" 
value" :�
�
appmesh)getGatewayRouteSpecHttp2RouteActionTarget_aws:appmesh/getGatewayRouteSpecHttp2RouteActionTarget:getGatewayRouteSpecHttp2RouteActionTarget�
�

port �
virtualServices�*�:�
�
appmesh7getGatewayRouteSpecHttp2RouteActionTargetVirtualService{aws:appmesh/getGatewayRouteSpecHttp2RouteActionTargetVirtualService:getGatewayRouteSpecHttp2RouteActionTargetVirtualService:�
�
appmesh7getGatewayRouteSpecHttp2RouteActionTargetVirtualService{aws:appmesh/getGatewayRouteSpecHttp2RouteActionTargetVirtualService:getGatewayRouteSpecHttp2RouteActionTargetVirtualService

virtualServiceName" :�
�
appmesh"getGatewayRouteSpecHttp2RouteMatchQaws:appmesh/getGatewayRouteSpecHttp2RouteMatch:getGatewayRouteSpecHttp2RouteMatch�
��
headers�*�:�
�
appmesh(getGatewayRouteSpecHttp2RouteMatchHeader]aws:appmesh/getGatewayRouteSpecHttp2RouteMatchHeader:getGatewayRouteSpecHttp2RouteMatchHeader�
	hostnames�*�:�
�
appmesh*getGatewayRouteSpecHttp2RouteMatchHostnameaaws:appmesh/getGatewayRouteSpecHttp2RouteMatchHostname:getGatewayRouteSpecHttp2RouteMatchHostname�
paths�*�:�
�
appmesh&getGatewayRouteSpecHttp2RouteMatchPathYaws:appmesh/getGatewayRouteSpecHttp2RouteMatchPath:getGatewayRouteSpecHttp2RouteMatchPath

port 
prefix" �
queryParameters�*�:�
�
appmesh0getGatewayRouteSpecHttp2RouteMatchQueryParametermaws:appmesh/getGatewayRouteSpecHttp2RouteMatchQueryParameter:getGatewayRouteSpecHttp2RouteMatchQueryParameter:�
�
appmesh(getGatewayRouteSpecHttp2RouteMatchHeader]aws:appmesh/getGatewayRouteSpecHttp2RouteMatchHeader:getGatewayRouteSpecHttp2RouteMatchHeader�
�
invert
 �
matches�*�:�
�
appmesh-getGatewayRouteSpecHttp2RouteMatchHeaderMatchgaws:appmesh/getGatewayRouteSpecHttp2RouteMatchHeaderMatch:getGatewayRouteSpecHttp2RouteMatchHeaderMatch'
name" Name of the gateway route.
:�
�
appmesh-getGatewayRouteSpecHttp2RouteMatchHeaderMatchgaws:appmesh/getGatewayRouteSpecHttp2RouteMatchHeaderMatch:getGatewayRouteSpecHttp2RouteMatchHeaderMatch�
�
exact" 
prefix" �
ranges�*�:�
�
appmesh2getGatewayRouteSpecHttp2RouteMatchHeaderMatchRangeqaws:appmesh/getGatewayRouteSpecHttp2RouteMatchHeaderMatchRange:getGatewayRouteSpecHttp2RouteMatchHeaderMatchRange
regex" 
suffix" :�
�
appmesh2getGatewayRouteSpecHttp2RouteMatchHeaderMatchRangeqaws:appmesh/getGatewayRouteSpecHttp2RouteMatchHeaderMatchRange:getGatewayRouteSpecHttp2RouteMatchHeaderMatchRange
	
end 
start :�
�
appmesh*getGatewayRouteSpecHttp2RouteMatchHostnameaaws:appmesh/getGatewayRouteSpecHttp2RouteMatchHostname:getGatewayRouteSpecHttp2RouteMatchHostname

exact" 
suffix" :�
�
appmesh&getGatewayRouteSpecHttp2RouteMatchPathYaws:appmesh/getGatewayRouteSpecHttp2RouteMatchPath:getGatewayRouteSpecHttp2RouteMatchPath

exact" 
regex" :�
�
appmesh0getGatewayRouteSpecHttp2RouteMatchQueryParametermaws:appmesh/getGatewayRouteSpecHttp2RouteMatchQueryParameter:getGatewayRouteSpecHttp2RouteMatchQueryParameter�
��
matches�*�:�
�
appmesh5getGatewayRouteSpecHttp2RouteMatchQueryParameterMatchwaws:appmesh/getGatewayRouteSpecHttp2RouteMatchQueryParameterMatch:getGatewayRouteSpecHttp2RouteMatchQueryParameterMatch'
name" Name of the gateway route.
:�
�
appmesh5getGatewayRouteSpecHttp2RouteMatchQueryParameterMatchwaws:appmesh/getGatewayRouteSpecHttp2RouteMatchQueryParameterMatch:getGatewayRouteSpecHttp2RouteMatchQueryParameterMatch

exact" :�
n
appmeshgetGatewayRouteSpecHttpRouteEaws:appmesh/getGatewayRouteSpecHttpRoute:getGatewayRouteSpecHttpRoute�
��
actions�*�:�
�
appmesh"getGatewayRouteSpecHttpRouteActionQaws:appmesh/getGatewayRouteSpecHttpRouteAction:getGatewayRouteSpecHttpRouteAction�
matches�*�:
}
appmesh!getGatewayRouteSpecHttpRouteMatchOaws:appmesh/getGatewayRouteSpecHttpRouteMatch:getGatewayRouteSpecHttpRouteMatch:�
�
appmesh"getGatewayRouteSpecHttpRouteActionQaws:appmesh/getGatewayRouteSpecHttpRouteAction:getGatewayRouteSpecHttpRouteAction�
��
rewrites�*�:�
�
appmesh)getGatewayRouteSpecHttpRouteActionRewrite_aws:appmesh/getGatewayRouteSpecHttpRouteActionRewrite:getGatewayRouteSpecHttpRouteActionRewrite�
targets�*�:�
�
appmesh(getGatewayRouteSpecHttpRouteActionTarget]aws:appmesh/getGatewayRouteSpecHttpRouteActionTarget:getGatewayRouteSpecHttpRouteActionTarget:�
�
appmesh)getGatewayRouteSpecHttpRouteActionRewrite_aws:appmesh/getGatewayRouteSpecHttpRouteActionRewrite:getGatewayRouteSpecHttpRouteActionRewrite�
��
	hostnames�*�:�
�
appmesh1getGatewayRouteSpecHttpRouteActionRewriteHostnameoaws:appmesh/getGatewayRouteSpecHttpRouteActionRewriteHostname:getGatewayRouteSpecHttpRouteActionRewriteHostname�
paths�*�:�
�
appmesh-getGatewayRouteSpecHttpRouteActionRewritePathgaws:appmesh/getGatewayRouteSpecHttpRouteActionRewritePath:getGatewayRouteSpecHttpRouteActionRewritePath�
prefixes�*�:�
�
appmesh/getGatewayRouteSpecHttpRouteActionRewritePrefixkaws:appmesh/getGatewayRouteSpecHttpRouteActionRewritePrefix:getGatewayRouteSpecHttpRouteActionRewritePrefix:�
�
appmesh1getGatewayRouteSpecHttpRouteActionRewriteHostnameoaws:appmesh/getGatewayRouteSpecHttpRouteActionRewriteHostname:getGatewayRouteSpecHttpRouteActionRewriteHostname

defaultTargetHostname" :�
�
appmesh-getGatewayRouteSpecHttpRouteActionRewritePathgaws:appmesh/getGatewayRouteSpecHttpRouteActionRewritePath:getGatewayRouteSpecHttpRouteActionRewritePath

exact" :�
�
appmesh/getGatewayRouteSpecHttpRouteActionRewritePrefixkaws:appmesh/getGatewayRouteSpecHttpRouteActionRewritePrefix:getGatewayRouteSpecHttpRouteActionRewritePrefix$
"
defaultPrefix" 
value" :�
�
appmesh(getGatewayRouteSpecHttpRouteActionTarget]aws:appmesh/getGatewayRouteSpecHttpRouteActionTarget:getGatewayRouteSpecHttpRouteActionTarget�
�

port �
virtualServices�*�:�
�
appmesh6getGatewayRouteSpecHttpRouteActionTargetVirtualServiceyaws:appmesh/getGatewayRouteSpecHttpRouteActionTargetVirtualService:getGatewayRouteSpecHttpRouteActionTargetVirtualService:�
�
appmesh6getGatewayRouteSpecHttpRouteActionTargetVirtualServiceyaws:appmesh/getGatewayRouteSpecHttpRouteActionTargetVirtualService:getGatewayRouteSpecHttpRouteActionTargetVirtualService

virtualServiceName" :�
}
appmesh!getGatewayRouteSpecHttpRouteMatchOaws:appmesh/getGatewayRouteSpecHttpRouteMatch:getGatewayRouteSpecHttpRouteMatch�
��
headers�*�:�
�
appmesh'getGatewayRouteSpecHttpRouteMatchHeader[aws:appmesh/getGatewayRouteSpecHttpRouteMatchHeader:getGatewayRouteSpecHttpRouteMatchHeader�
	hostnames�*�:�
�
appmesh)getGatewayRouteSpecHttpRouteMatchHostname_aws:appmesh/getGatewayRouteSpecHttpRouteMatchHostname:getGatewayRouteSpecHttpRouteMatchHostname�
paths�*�:�
�
appmesh%getGatewayRouteSpecHttpRouteMatchPathWaws:appmesh/getGatewayRouteSpecHttpRouteMatchPath:getGatewayRouteSpecHttpRouteMatchPath

port 
prefix" �
queryParameters�*�:�
�
appmesh/getGatewayRouteSpecHttpRouteMatchQueryParameterkaws:appmesh/getGatewayRouteSpecHttpRouteMatchQueryParameter:getGatewayRouteSpecHttpRouteMatchQueryParameter:�
�
appmesh'getGatewayRouteSpecHttpRouteMatchHeader[aws:appmesh/getGatewayRouteSpecHttpRouteMatchHeader:getGatewayRouteSpecHttpRouteMatchHeader�
�
invert
 �
matches�*�:�
�
appmesh,getGatewayRouteSpecHttpRouteMatchHeaderMatcheaws:appmesh/getGatewayRouteSpecHttpRouteMatchHeaderMatch:getGatewayRouteSpecHttpRouteMatchHeaderMatch'
name" Name of the gateway route.
:�
�
appmesh,getGatewayRouteSpecHttpRouteMatchHeaderMatcheaws:appmesh/getGatewayRouteSpecHttpRouteMatchHeaderMatch:getGatewayRouteSpecHttpRouteMatchHeaderMatch�
�
exact" 
prefix" �
ranges�*�:�
�
appmesh1getGatewayRouteSpecHttpRouteMatchHeaderMatchRangeoaws:appmesh/getGatewayRouteSpecHttpRouteMatchHeaderMatchRange:getGatewayRouteSpecHttpRouteMatchHeaderMatchRange
regex" 
suffix" :�
�
appmesh1getGatewayRouteSpecHttpRouteMatchHeaderMatchRangeoaws:appmesh/getGatewayRouteSpecHttpRouteMatchHeaderMatchRange:getGatewayRouteSpecHttpRouteMatchHeaderMatchRange
	
end 
start :�
�
appmesh)getGatewayRouteSpecHttpRouteMatchHostname_aws:appmesh/getGatewayRouteSpecHttpRouteMatchHostname:getGatewayRouteSpecHttpRouteMatchHostname

exact" 
suffix" :�
�
appmesh%getGatewayRouteSpecHttpRouteMatchPathWaws:appmesh/getGatewayRouteSpecHttpRouteMatchPath:getGatewayRouteSpecHttpRouteMatchPath

exact" 
regex" :�
�
appmesh/getGatewayRouteSpecHttpRouteMatchQueryParameterkaws:appmesh/getGatewayRouteSpecHttpRouteMatchQueryParameter:getGatewayRouteSpecHttpRouteMatchQueryParameter�
��
matches�*�:�
�
appmesh4getGatewayRouteSpecHttpRouteMatchQueryParameterMatchuaws:appmesh/getGatewayRouteSpecHttpRouteMatchQueryParameterMatch:getGatewayRouteSpecHttpRouteMatchQueryParameterMatch'
name" Name of the gateway route.
:�
�
appmesh4getGatewayRouteSpecHttpRouteMatchQueryParameterMatchuaws:appmesh/getGatewayRouteSpecHttpRouteMatchQueryParameterMatch:getGatewayRouteSpecHttpRouteMatchQueryParameterMatch

exact" :�
;
appmeshgetMeshSpec#aws:appmesh/getMeshSpec:getMeshSpec�
�v
egressFilterse*c:a
_
appmeshgetMeshSpecEgressFilter;aws:appmesh/getMeshSpecEgressFilter:getMeshSpecEgressFilter�
serviceDiscoveriesq*o:m
k
appmeshgetMeshSpecServiceDiscoveryCaws:appmesh/getMeshSpecServiceDiscovery:getMeshSpecServiceDiscovery:q
_
appmeshgetMeshSpecEgressFilter;aws:appmesh/getMeshSpecEgressFilter:getMeshSpecEgressFilter


type" :�
k
appmeshgetMeshSpecServiceDiscoveryCaws:appmesh/getMeshSpecServiceDiscovery:getMeshSpecServiceDiscovery

ipPreference" :�
>
appmeshgetRouteSpec%aws:appmesh/getRouteSpec:getRouteSpec�
�m

grpcRoutes_*]:[
Y
appmeshgetRouteSpecGrpcRoute7aws:appmesh/getRouteSpecGrpcRoute:getRouteSpecGrpcRouteq
http2Routesb*`:^
\
appmeshgetRouteSpecHttp2Route9aws:appmesh/getRouteSpecHttp2Route:getRouteSpecHttp2Routem

httpRoutes_*]:[
Y
appmeshgetRouteSpecHttpRoute7aws:appmesh/getRouteSpecHttpRoute:getRouteSpecHttpRoute
priority i
	tcpRoutes\*Z:X
V
appmeshgetRouteSpecTcpRoute5aws:appmesh/getRouteSpecTcpRoute:getRouteSpecTcpRoute:�
Y
appmeshgetRouteSpecGrpcRoute7aws:appmesh/getRouteSpecGrpcRoute:getRouteSpecGrpcRoute�
�|
actionsq*o:m
k
appmeshgetRouteSpecGrpcRouteActionCaws:appmesh/getRouteSpecGrpcRouteAction:getRouteSpecGrpcRouteActiony
matchesn*l:j
h
appmeshgetRouteSpecGrpcRouteMatchAaws:appmesh/getRouteSpecGrpcRouteMatch:getRouteSpecGrpcRouteMatch�
retryPolicies�*~:|
z
appmesh getRouteSpecGrpcRouteRetryPolicyMaws:appmesh/getRouteSpecGrpcRouteRetryPolicy:getRouteSpecGrpcRouteRetryPolicy�
timeoutst*r:p
n
appmeshgetRouteSpecGrpcRouteTimeoutEaws:appmesh/getRouteSpecGrpcRouteTimeout:getRouteSpecGrpcRouteTimeout:�
k
appmeshgetRouteSpecGrpcRouteActionCaws:appmesh/getRouteSpecGrpcRouteAction:getRouteSpecGrpcRouteAction�
��
weightedTargets�*�:�
�
appmesh)getRouteSpecGrpcRouteActionWeightedTarget_aws:appmesh/getRouteSpecGrpcRouteActionWeightedTarget:getRouteSpecGrpcRouteActionWeightedTarget:�
�
appmesh)getRouteSpecGrpcRouteActionWeightedTarget_aws:appmesh/getRouteSpecGrpcRouteActionWeightedTarget:getRouteSpecGrpcRouteActionWeightedTarget/
-

port 
virtualNode" 
weight :�
h
appmeshgetRouteSpecGrpcRouteMatchAaws:appmesh/getRouteSpecGrpcRouteMatch:getRouteSpecGrpcRouteMatch�
��
	metadatas�*�:�
�
appmesh"getRouteSpecGrpcRouteMatchMetadataQaws:appmesh/getRouteSpecGrpcRouteMatchMetadata:getRouteSpecGrpcRouteMatchMetadata

methodName" 

port 
prefix" 
serviceName" :�
�
appmesh"getRouteSpecGrpcRouteMatchMetadataQaws:appmesh/getRouteSpecGrpcRouteMatchMetadata:getRouteSpecGrpcRouteMatchMetadata�
�
invert
 �
matches�*�:�
�
appmesh'getRouteSpecGrpcRouteMatchMetadataMatch[aws:appmesh/getRouteSpecGrpcRouteMatchMetadataMatch:getRouteSpecGrpcRouteMatchMetadataMatch
name" Name of the route.
:�
�
appmesh'getRouteSpecGrpcRouteMatchMetadataMatch[aws:appmesh/getRouteSpecGrpcRouteMatchMetadataMatch:getRouteSpecGrpcRouteMatchMetadataMatch�
�
exact" 
prefix" �
ranges�*�:�
�
appmesh,getRouteSpecGrpcRouteMatchMetadataMatchRangeeaws:appmesh/getRouteSpecGrpcRouteMatchMetadataMatchRange:getRouteSpecGrpcRouteMatchMetadataMatchRange
regex" 
suffix" :�
�
appmesh,getRouteSpecGrpcRouteMatchMetadataMatchRangeeaws:appmesh/getRouteSpecGrpcRouteMatchMetadataMatchRange:getRouteSpecGrpcRouteMatchMetadataMatchRange
	
end 
start :�
z
appmesh getRouteSpecGrpcRouteRetryPolicyMaws:appmesh/getRouteSpecGrpcRouteRetryPolicy:getRouteSpecGrpcRouteRetryPolicy�
�
grpcRetryEvents*" 
httpRetryEvents*" 

maxRetries �
perRetryTimeouts�*�:�
�
appmesh/getRouteSpecGrpcRouteRetryPolicyPerRetryTimeoutkaws:appmesh/getRouteSpecGrpcRouteRetryPolicyPerRetryTimeout:getRouteSpecGrpcRouteRetryPolicyPerRetryTimeout
tcpRetryEvents*" :�
�
appmesh/getRouteSpecGrpcRouteRetryPolicyPerRetryTimeoutkaws:appmesh/getRouteSpecGrpcRouteRetryPolicyPerRetryTimeout:getRouteSpecGrpcRouteRetryPolicyPerRetryTimeout


unit" 
value :�
n
appmeshgetRouteSpecGrpcRouteTimeoutEaws:appmesh/getRouteSpecGrpcRouteTimeout:getRouteSpecGrpcRouteTimeout�
��
idles�*~:|
z
appmesh getRouteSpecGrpcRouteTimeoutIdleMaws:appmesh/getRouteSpecGrpcRouteTimeoutIdle:getRouteSpecGrpcRouteTimeoutIdle�
perRequests�*�:�
�
appmesh&getRouteSpecGrpcRouteTimeoutPerRequestYaws:appmesh/getRouteSpecGrpcRouteTimeoutPerRequest:getRouteSpecGrpcRouteTimeoutPerRequest:�
z
appmesh getRouteSpecGrpcRouteTimeoutIdleMaws:appmesh/getRouteSpecGrpcRouteTimeoutIdle:getRouteSpecGrpcRouteTimeoutIdle


unit" 
value :�
�
appmesh&getRouteSpecGrpcRouteTimeoutPerRequestYaws:appmesh/getRouteSpecGrpcRouteTimeoutPerRequest:getRouteSpecGrpcRouteTimeoutPerRequest


unit" 
value :�
\
appmeshgetRouteSpecHttp2Route9aws:appmesh/getRouteSpecHttp2Route:getRouteSpecHttp2Route�
�
actionst*r:p
n
appmeshgetRouteSpecHttp2RouteActionEaws:appmesh/getRouteSpecHttp2RouteAction:getRouteSpecHttp2RouteAction|
matchesq*o:m
k
appmeshgetRouteSpecHttp2RouteMatchCaws:appmesh/getRouteSpecHttp2RouteMatch:getRouteSpecHttp2RouteMatch�
retryPolicies�*�:
}
appmesh!getRouteSpecHttp2RouteRetryPolicyOaws:appmesh/getRouteSpecHttp2RouteRetryPolicy:getRouteSpecHttp2RouteRetryPolicy�
timeoutsw*u:s
q
appmeshgetRouteSpecHttp2RouteTimeoutGaws:appmesh/getRouteSpecHttp2RouteTimeout:getRouteSpecHttp2RouteTimeout:�
n
appmeshgetRouteSpecHttp2RouteActionEaws:appmesh/getRouteSpecHttp2RouteAction:getRouteSpecHttp2RouteAction�
��
weightedTargets�*�:�
�
appmesh*getRouteSpecHttp2RouteActionWeightedTargetaaws:appmesh/getRouteSpecHttp2RouteActionWeightedTarget:getRouteSpecHttp2RouteActionWeightedTarget:�
�
appmesh*getRouteSpecHttp2RouteActionWeightedTargetaaws:appmesh/getRouteSpecHttp2RouteActionWeightedTarget:getRouteSpecHttp2RouteActionWeightedTarget/
-

port 
virtualNode" 
weight :�
k
appmeshgetRouteSpecHttp2RouteMatchCaws:appmesh/getRouteSpecHttp2RouteMatch:getRouteSpecHttp2RouteMatch�
��
headers�*�:
}
appmesh!getRouteSpecHttp2RouteMatchHeaderOaws:appmesh/getRouteSpecHttp2RouteMatchHeader:getRouteSpecHttp2RouteMatchHeader
method" �
paths}*{:y
w
appmeshgetRouteSpecHttp2RouteMatchPathKaws:appmesh/getRouteSpecHttp2RouteMatchPath:getRouteSpecHttp2RouteMatchPath

port 
prefix" �
queryParameters�*�:�
�
appmesh)getRouteSpecHttp2RouteMatchQueryParameter_aws:appmesh/getRouteSpecHttp2RouteMatchQueryParameter:getRouteSpecHttp2RouteMatchQueryParameter
scheme" :�
}
appmesh!getRouteSpecHttp2RouteMatchHeaderOaws:appmesh/getRouteSpecHttp2RouteMatchHeader:getRouteSpecHttp2RouteMatchHeader�
�
invert
 �
matches�*�:�
�
appmesh&getRouteSpecHttp2RouteMatchHeaderMatchYaws:appmesh/getRouteSpecHttp2RouteMatchHeaderMatch:getRouteSpecHttp2RouteMatchHeaderMatch
name" Name of the route.
:�
�
appmesh&getRouteSpecHttp2RouteMatchHeaderMatchYaws:appmesh/getRouteSpecHttp2RouteMatchHeaderMatch:getRouteSpecHttp2RouteMatchHeaderMatch�
�
exact" 
prefix" �
ranges�*�:�
�
appmesh+getRouteSpecHttp2RouteMatchHeaderMatchRangecaws:appmesh/getRouteSpecHttp2RouteMatchHeaderMatchRange:getRouteSpecHttp2RouteMatchHeaderMatchRange
regex" 
suffix" :�
�
appmesh+getRouteSpecHttp2RouteMatchHeaderMatchRangecaws:appmesh/getRouteSpecHttp2RouteMatchHeaderMatchRange:getRouteSpecHttp2RouteMatchHeaderMatchRange
	
end 
start :�
w
appmeshgetRouteSpecHttp2RouteMatchPathKaws:appmesh/getRouteSpecHttp2RouteMatchPath:getRouteSpecHttp2RouteMatchPath

exact" 
regex" :�
�
appmesh)getRouteSpecHttp2RouteMatchQueryParameter_aws:appmesh/getRouteSpecHttp2RouteMatchQueryParameter:getRouteSpecHttp2RouteMatchQueryParameter�
��
matches�*�:�
�
appmesh.getRouteSpecHttp2RouteMatchQueryParameterMatchiaws:appmesh/getRouteSpecHttp2RouteMatchQueryParameterMatch:getRouteSpecHttp2RouteMatchQueryParameterMatch
name" Name of the route.
:�
�
appmesh.getRouteSpecHttp2RouteMatchQueryParameterMatchiaws:appmesh/getRouteSpecHttp2RouteMatchQueryParameterMatch:getRouteSpecHttp2RouteMatchQueryParameterMatch

exact" :�
}
appmesh!getRouteSpecHttp2RouteRetryPolicyOaws:appmesh/getRouteSpecHttp2RouteRetryPolicy:getRouteSpecHttp2RouteRetryPolicy�
�
httpRetryEvents*" 

maxRetries �
perRetryTimeouts�*�:�
�
appmesh0getRouteSpecHttp2RouteRetryPolicyPerRetryTimeoutmaws:appmesh/getRouteSpecHttp2RouteRetryPolicyPerRetryTimeout:getRouteSpecHttp2RouteRetryPolicyPerRetryTimeout
tcpRetryEvents*" :�
�
appmesh0getRouteSpecHttp2RouteRetryPolicyPerRetryTimeoutmaws:appmesh/getRouteSpecHttp2RouteRetryPolicyPerRetryTimeout:getRouteSpecHttp2RouteRetryPolicyPerRetryTimeout


unit" 
value :�
q
appmeshgetRouteSpecHttp2RouteTimeoutGaws:appmesh/getRouteSpecHttp2RouteTimeout:getRouteSpecHttp2RouteTimeout�
��
idles�*�:
}
appmesh!getRouteSpecHttp2RouteTimeoutIdleOaws:appmesh/getRouteSpecHttp2RouteTimeoutIdle:getRouteSpecHttp2RouteTimeoutIdle�
perRequests�*�:�
�
appmesh'getRouteSpecHttp2RouteTimeoutPerRequest[aws:appmesh/getRouteSpecHttp2RouteTimeoutPerRequest:getRouteSpecHttp2RouteTimeoutPerRequest:�
}
appmesh!getRouteSpecHttp2RouteTimeoutIdleOaws:appmesh/getRouteSpecHttp2RouteTimeoutIdle:getRouteSpecHttp2RouteTimeoutIdle


unit" 
value :�
�
appmesh'getRouteSpecHttp2RouteTimeoutPerRequest[aws:appmesh/getRouteSpecHttp2RouteTimeoutPerRequest:getRouteSpecHttp2RouteTimeoutPerRequest


unit" 
value :�
Y
appmeshgetRouteSpecHttpRoute7aws:appmesh/getRouteSpecHttpRoute:getRouteSpecHttpRoute�
�|
actionsq*o:m
k
appmeshgetRouteSpecHttpRouteActionCaws:appmesh/getRouteSpecHttpRouteAction:getRouteSpecHttpRouteActiony
matchesn*l:j
h
appmeshgetRouteSpecHttpRouteMatchAaws:appmesh/getRouteSpecHttpRouteMatch:getRouteSpecHttpRouteMatch�
retryPolicies�*~:|
z
appmesh getRouteSpecHttpRouteRetryPolicyMaws:appmesh/getRouteSpecHttpRouteRetryPolicy:getRouteSpecHttpRouteRetryPolicy�
timeoutst*r:p
n
appmeshgetRouteSpecHttpRouteTimeoutEaws:appmesh/getRouteSpecHttpRouteTimeout:getRouteSpecHttpRouteTimeout:�
k
appmeshgetRouteSpecHttpRouteActionCaws:appmesh/getRouteSpecHttpRouteAction:getRouteSpecHttpRouteAction�
��
weightedTargets�*�:�
�
appmesh)getRouteSpecHttpRouteActionWeightedTarget_aws:appmesh/getRouteSpecHttpRouteActionWeightedTarget:getRouteSpecHttpRouteActionWeightedTarget:�
�
appmesh)getRouteSpecHttpRouteActionWeightedTarget_aws:appmesh/getRouteSpecHttpRouteActionWeightedTarget:getRouteSpecHttpRouteActionWeightedTarget/
-

port 
virtualNode" 
weight :�
h
appmeshgetRouteSpecHttpRouteMatchAaws:appmesh/getRouteSpecHttpRouteMatch:getRouteSpecHttpRouteMatch�
��
headers�*~:|
z
appmesh getRouteSpecHttpRouteMatchHeaderMaws:appmesh/getRouteSpecHttpRouteMatchHeader:getRouteSpecHttpRouteMatchHeader
method" �
pathsz*x:v
t
appmeshgetRouteSpecHttpRouteMatchPathIaws:appmesh/getRouteSpecHttpRouteMatchPath:getRouteSpecHttpRouteMatchPath

port 
prefix" �
queryParameters�*�:�
�
appmesh(getRouteSpecHttpRouteMatchQueryParameter]aws:appmesh/getRouteSpecHttpRouteMatchQueryParameter:getRouteSpecHttpRouteMatchQueryParameter
scheme" :�
z
appmesh getRouteSpecHttpRouteMatchHeaderMaws:appmesh/getRouteSpecHttpRouteMatchHeader:getRouteSpecHttpRouteMatchHeader�
�
invert
 �
matches�*�:�
�
appmesh%getRouteSpecHttpRouteMatchHeaderMatchWaws:appmesh/getRouteSpecHttpRouteMatchHeaderMatch:getRouteSpecHttpRouteMatchHeaderMatch
name" Name of the route.
:�
�
appmesh%getRouteSpecHttpRouteMatchHeaderMatchWaws:appmesh/getRouteSpecHttpRouteMatchHeaderMatch:getRouteSpecHttpRouteMatchHeaderMatch�
�
exact" 
prefix" �
ranges�*�:�
�
appmesh*getRouteSpecHttpRouteMatchHeaderMatchRangeaaws:appmesh/getRouteSpecHttpRouteMatchHeaderMatchRange:getRouteSpecHttpRouteMatchHeaderMatchRange
regex" 
suffix" :�
�
appmesh*getRouteSpecHttpRouteMatchHeaderMatchRangeaaws:appmesh/getRouteSpecHttpRouteMatchHeaderMatchRange:getRouteSpecHttpRouteMatchHeaderMatchRange
	
end 
start :�
t
appmeshgetRouteSpecHttpRouteMatchPathIaws:appmesh/getRouteSpecHttpRouteMatchPath:getRouteSpecHttpRouteMatchPath

exact" 
regex" :�
�
appmesh(getRouteSpecHttpRouteMatchQueryParameter]aws:appmesh/getRouteSpecHttpRouteMatchQueryParameter:getRouteSpecHttpRouteMatchQueryParameter�
��
matches�*�:�
�
appmesh-getRouteSpecHttpRouteMatchQueryParameterMatchgaws:appmesh/getRouteSpecHttpRouteMatchQueryParameterMatch:getRouteSpecHttpRouteMatchQueryParameterMatch
name" Name of the route.
:�
�
appmesh-getRouteSpecHttpRouteMatchQueryParameterMatchgaws:appmesh/getRouteSpecHttpRouteMatchQueryParameterMatch:getRouteSpecHttpRouteMatchQueryParameterMatch

exact" :�
z
appmesh getRouteSpecHttpRouteRetryPolicyMaws:appmesh/getRouteSpecHttpRouteRetryPolicy:getRouteSpecHttpRouteRetryPolicy�
�
httpRetryEvents*" 

maxRetries �
perRetryTimeouts�*�:�
�
appmesh/getRouteSpecHttpRouteRetryPolicyPerRetryTimeoutkaws:appmesh/getRouteSpecHttpRouteRetryPolicyPerRetryTimeout:getRouteSpecHttpRouteRetryPolicyPerRetryTimeout
tcpRetryEvents*" :�
�
appmesh/getRouteSpecHttpRouteRetryPolicyPerRetryTimeoutkaws:appmesh/getRouteSpecHttpRouteRetryPolicyPerRetryTimeout:getRouteSpecHttpRouteRetryPolicyPerRetryTimeout


unit" 
value :�
n
appmeshgetRouteSpecHttpRouteTimeoutEaws:appmesh/getRouteSpecHttpRouteTimeout:getRouteSpecHttpRouteTimeout�
��
idles�*~:|
z
appmesh getRouteSpecHttpRouteTimeoutIdleMaws:appmesh/getRouteSpecHttpRouteTimeoutIdle:getRouteSpecHttpRouteTimeoutIdle�
perRequests�*�:�
�
appmesh&getRouteSpecHttpRouteTimeoutPerRequestYaws:appmesh/getRouteSpecHttpRouteTimeoutPerRequest:getRouteSpecHttpRouteTimeoutPerRequest:�
z
appmesh getRouteSpecHttpRouteTimeoutIdleMaws:appmesh/getRouteSpecHttpRouteTimeoutIdle:getRouteSpecHttpRouteTimeoutIdle


unit" 
value :�
�
appmesh&getRouteSpecHttpRouteTimeoutPerRequestYaws:appmesh/getRouteSpecHttpRouteTimeoutPerRequest:getRouteSpecHttpRouteTimeoutPerRequest


unit" 
value :�
V
appmeshgetRouteSpecTcpRoute5aws:appmesh/getRouteSpecTcpRoute:getRouteSpecTcpRoute�
�y
actionsn*l:j
h
appmeshgetRouteSpecTcpRouteActionAaws:appmesh/getRouteSpecTcpRouteAction:getRouteSpecTcpRouteActionv
matchesk*i:g
e
appmeshgetRouteSpecTcpRouteMatch?aws:appmesh/getRouteSpecTcpRouteMatch:getRouteSpecTcpRouteMatch}
timeoutsq*o:m
k
appmeshgetRouteSpecTcpRouteTimeoutCaws:appmesh/getRouteSpecTcpRouteTimeout:getRouteSpecTcpRouteTimeout:�
h
appmeshgetRouteSpecTcpRouteActionAaws:appmesh/getRouteSpecTcpRouteAction:getRouteSpecTcpRouteAction�
��
weightedTargets�*�:�
�
appmesh(getRouteSpecTcpRouteActionWeightedTarget]aws:appmesh/getRouteSpecTcpRouteActionWeightedTarget:getRouteSpecTcpRouteActionWeightedTarget:�
�
appmesh(getRouteSpecTcpRouteActionWeightedTarget]aws:appmesh/getRouteSpecTcpRouteActionWeightedTarget:getRouteSpecTcpRouteActionWeightedTarget/
-

port 
virtualNode" 
weight :w
e
appmeshgetRouteSpecTcpRouteMatch?aws:appmesh/getRouteSpecTcpRouteMatch:getRouteSpecTcpRouteMatch


port :�
k
appmeshgetRouteSpecTcpRouteTimeoutCaws:appmesh/getRouteSpecTcpRouteTimeout:getRouteSpecTcpRouteTimeout�
��
idles}*{:y
w
appmeshgetRouteSpecTcpRouteTimeoutIdleKaws:appmesh/getRouteSpecTcpRouteTimeoutIdle:getRouteSpecTcpRouteTimeoutIdle:�
w
appmeshgetRouteSpecTcpRouteTimeoutIdleKaws:appmesh/getRouteSpecTcpRouteTimeoutIdle:getRouteSpecTcpRouteTimeoutIdle


unit" 
value :�
Y
appmeshgetVirtualGatewaySpec7aws:appmesh/getVirtualGatewaySpec:getVirtualGatewaySpec�
��
backendDefaults�*�:�
�
appmesh#getVirtualGatewaySpecBackendDefaultSaws:appmesh/getVirtualGatewaySpecBackendDefault:getVirtualGatewaySpecBackendDefault�
	listenersw*u:s
q
appmeshgetVirtualGatewaySpecListenerGaws:appmesh/getVirtualGatewaySpecListener:getVirtualGatewaySpecListener�
loggingst*r:p
n
appmeshgetVirtualGatewaySpecLoggingEaws:appmesh/getVirtualGatewaySpecLogging:getVirtualGatewaySpecLogging:�
�
appmesh#getVirtualGatewaySpecBackendDefaultSaws:appmesh/getVirtualGatewaySpecBackendDefault:getVirtualGatewaySpecBackendDefault�
��
clientPolicies�*�:�
�
appmesh/getVirtualGatewaySpecBackendDefaultClientPolicykaws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicy:getVirtualGatewaySpecBackendDefaultClientPolicy:�
�
appmesh/getVirtualGatewaySpecBackendDefaultClientPolicykaws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicy:getVirtualGatewaySpecBackendDefaultClientPolicy�
��
tls�*�:�
�
appmesh1getVirtualGatewaySpecBackendDefaultClientPolicyTloaws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTl:getVirtualGatewaySpecBackendDefaultClientPolicyTl:�
�
appmesh1getVirtualGatewaySpecBackendDefaultClientPolicyTloaws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTl:getVirtualGatewaySpecBackendDefaultClientPolicyTl�
��
certificates�*�:�
�
appmesh<getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificate�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificate:getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificate
enforce
 
ports* �
validations�*�:�
�
appmesh;getVirtualGatewaySpecBackendDefaultClientPolicyTlValidation�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidation:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidation:�
�
appmesh<getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificate�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificate:getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificate�
��
files�*�:�
�
appmesh@getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateFile�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateFile:getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateFile�
sds�*�:�
�
appmesh>getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateSd�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateSd:getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateSd:�
�
appmesh@getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateFile�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateFile:getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateFile,
*
certificateChain" 

privateKey" :�
�
appmesh>getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateSd�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateSd:getVirtualGatewaySpecBackendDefaultClientPolicyTlCertificateSd


secretName" :�
�
appmesh;getVirtualGatewaySpecBackendDefaultClientPolicyTlValidation�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidation:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidation�
��
subjectAlternativeNames�*�:�
�
appmeshQgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�
trusts�*�:�
�
appmesh@getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust:�
�
appmeshQgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�
��
matches�*�:�
�
appmeshVgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch:�
�
appmeshVgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch

exacts*" :�
�
appmesh@getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust�
��
acms�*�:�
�
appmeshCgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm�
files�*�:�
�
appmeshDgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile�
sds�*�:�
�
appmeshBgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd:�
�
appmeshCgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm$
" 
certificateAuthorityArns*" :�
�
appmeshDgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile

certificateChain" :�
�
appmeshBgetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd�aws:appmesh/getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd:getVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd


secretName" :�
q
appmeshgetVirtualGatewaySpecListenerGaws:appmesh/getVirtualGatewaySpecListener:getVirtualGatewaySpecListener�
��
connectionPools�*�:�
�
appmesh+getVirtualGatewaySpecListenerConnectionPoolcaws:appmesh/getVirtualGatewaySpecListenerConnectionPool:getVirtualGatewaySpecListenerConnectionPool�
healthChecks�*�:�
�
appmesh(getVirtualGatewaySpecListenerHealthCheck]aws:appmesh/getVirtualGatewaySpecListenerHealthCheck:getVirtualGatewaySpecListenerHealthCheck�
portMappings�*�:�
�
appmesh(getVirtualGatewaySpecListenerPortMapping]aws:appmesh/getVirtualGatewaySpecListenerPortMapping:getVirtualGatewaySpecListenerPortMapping�
tls}*{:y
w
appmeshgetVirtualGatewaySpecListenerTlKaws:appmesh/getVirtualGatewaySpecListenerTl:getVirtualGatewaySpecListenerTl:�
�
appmesh+getVirtualGatewaySpecListenerConnectionPoolcaws:appmesh/getVirtualGatewaySpecListenerConnectionPool:getVirtualGatewaySpecListenerConnectionPool�
��
grpcs�*�:�
�
appmesh/getVirtualGatewaySpecListenerConnectionPoolGrpckaws:appmesh/getVirtualGatewaySpecListenerConnectionPoolGrpc:getVirtualGatewaySpecListenerConnectionPoolGrpc�
http2s�*�:�
�
appmesh0getVirtualGatewaySpecListenerConnectionPoolHttp2maws:appmesh/getVirtualGatewaySpecListenerConnectionPoolHttp2:getVirtualGatewaySpecListenerConnectionPoolHttp2�
https�*�:�
�
appmesh/getVirtualGatewaySpecListenerConnectionPoolHttpkaws:appmesh/getVirtualGatewaySpecListenerConnectionPoolHttp:getVirtualGatewaySpecListenerConnectionPoolHttp:�
�
appmesh/getVirtualGatewaySpecListenerConnectionPoolGrpckaws:appmesh/getVirtualGatewaySpecListenerConnectionPoolGrpc:getVirtualGatewaySpecListenerConnectionPoolGrpc

maxRequests :�
�
appmesh/getVirtualGatewaySpecListenerConnectionPoolHttpkaws:appmesh/getVirtualGatewaySpecListenerConnectionPoolHttp:getVirtualGatewaySpecListenerConnectionPoolHttp2
0
maxConnections 
maxPendingRequests :�
�
appmesh0getVirtualGatewaySpecListenerConnectionPoolHttp2maws:appmesh/getVirtualGatewaySpecListenerConnectionPoolHttp2:getVirtualGatewaySpecListenerConnectionPoolHttp2

maxRequests :�
�
appmesh(getVirtualGatewaySpecListenerHealthCheck]aws:appmesh/getVirtualGatewaySpecListenerHealthCheck:getVirtualGatewaySpecListenerHealthCheck�
�
healthyThreshold 
intervalMillis 

path" 

port 
protocol" 
timeoutMillis 
unhealthyThreshold :�
�
appmesh(getVirtualGatewaySpecListenerPortMapping]aws:appmesh/getVirtualGatewaySpecListenerPortMapping:getVirtualGatewaySpecListenerPortMapping


port 
protocol" :�
w
appmeshgetVirtualGatewaySpecListenerTlKaws:appmesh/getVirtualGatewaySpecListenerTl:getVirtualGatewaySpecListenerTl�
��
certificates�*�:�
�
appmesh*getVirtualGatewaySpecListenerTlCertificateaaws:appmesh/getVirtualGatewaySpecListenerTlCertificate:getVirtualGatewaySpecListenerTlCertificate

mode" �
validations�*�:�
�
appmesh)getVirtualGatewaySpecListenerTlValidation_aws:appmesh/getVirtualGatewaySpecListenerTlValidation:getVirtualGatewaySpecListenerTlValidation:�
�
appmesh*getVirtualGatewaySpecListenerTlCertificateaaws:appmesh/getVirtualGatewaySpecListenerTlCertificate:getVirtualGatewaySpecListenerTlCertificate�
��
acms�*�:�
�
appmesh-getVirtualGatewaySpecListenerTlCertificateAcmgaws:appmesh/getVirtualGatewaySpecListenerTlCertificateAcm:getVirtualGatewaySpecListenerTlCertificateAcm�
files�*�:�
�
appmesh.getVirtualGatewaySpecListenerTlCertificateFileiaws:appmesh/getVirtualGatewaySpecListenerTlCertificateFile:getVirtualGatewaySpecListenerTlCertificateFile�
sds�*�:�
�
appmesh,getVirtualGatewaySpecListenerTlCertificateSdeaws:appmesh/getVirtualGatewaySpecListenerTlCertificateSd:getVirtualGatewaySpecListenerTlCertificateSd:�
�
appmesh-getVirtualGatewaySpecListenerTlCertificateAcmgaws:appmesh/getVirtualGatewaySpecListenerTlCertificateAcm:getVirtualGatewaySpecListenerTlCertificateAcm

certificateArn" :�
�
appmesh.getVirtualGatewaySpecListenerTlCertificateFileiaws:appmesh/getVirtualGatewaySpecListenerTlCertificateFile:getVirtualGatewaySpecListenerTlCertificateFile,
*
certificateChain" 

privateKey" :�
�
appmesh,getVirtualGatewaySpecListenerTlCertificateSdeaws:appmesh/getVirtualGatewaySpecListenerTlCertificateSd:getVirtualGatewaySpecListenerTlCertificateSd


secretName" :�
�
appmesh)getVirtualGatewaySpecListenerTlValidation_aws:appmesh/getVirtualGatewaySpecListenerTlValidation:getVirtualGatewaySpecListenerTlValidation�
��
subjectAlternativeNames�*�:�
�
appmesh?getVirtualGatewaySpecListenerTlValidationSubjectAlternativeName�aws:appmesh/getVirtualGatewaySpecListenerTlValidationSubjectAlternativeName:getVirtualGatewaySpecListenerTlValidationSubjectAlternativeName�
trusts�*�:�
�
appmesh.getVirtualGatewaySpecListenerTlValidationTrustiaws:appmesh/getVirtualGatewaySpecListenerTlValidationTrust:getVirtualGatewaySpecListenerTlValidationTrust:�
�
appmesh?getVirtualGatewaySpecListenerTlValidationSubjectAlternativeName�aws:appmesh/getVirtualGatewaySpecListenerTlValidationSubjectAlternativeName:getVirtualGatewaySpecListenerTlValidationSubjectAlternativeName�
��
matches�*�:�
�
appmeshDgetVirtualGatewaySpecListenerTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualGatewaySpecListenerTlValidationSubjectAlternativeNameMatch:getVirtualGatewaySpecListenerTlValidationSubjectAlternativeNameMatch:�
�
appmeshDgetVirtualGatewaySpecListenerTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualGatewaySpecListenerTlValidationSubjectAlternativeNameMatch:getVirtualGatewaySpecListenerTlValidationSubjectAlternativeNameMatch

exacts*" :�
�
appmesh.getVirtualGatewaySpecListenerTlValidationTrustiaws:appmesh/getVirtualGatewaySpecListenerTlValidationTrust:getVirtualGatewaySpecListenerTlValidationTrust�
��
files�*�:�
�
appmesh2getVirtualGatewaySpecListenerTlValidationTrustFileqaws:appmesh/getVirtualGatewaySpecListenerTlValidationTrustFile:getVirtualGatewaySpecListenerTlValidationTrustFile�
sds�*�:�
�
appmesh0getVirtualGatewaySpecListenerTlValidationTrustSdmaws:appmesh/getVirtualGatewaySpecListenerTlValidationTrustSd:getVirtualGatewaySpecListenerTlValidationTrustSd:�
�
appmesh2getVirtualGatewaySpecListenerTlValidationTrustFileqaws:appmesh/getVirtualGatewaySpecListenerTlValidationTrustFile:getVirtualGatewaySpecListenerTlValidationTrustFile

certificateChain" :�
�
appmesh0getVirtualGatewaySpecListenerTlValidationTrustSdmaws:appmesh/getVirtualGatewaySpecListenerTlValidationTrustSd:getVirtualGatewaySpecListenerTlValidationTrustSd


secretName" :�
n
appmeshgetVirtualGatewaySpecLoggingEaws:appmesh/getVirtualGatewaySpecLogging:getVirtualGatewaySpecLogging�
��

accessLogs�*�:�
�
appmesh%getVirtualGatewaySpecLoggingAccessLogWaws:appmesh/getVirtualGatewaySpecLoggingAccessLog:getVirtualGatewaySpecLoggingAccessLog:�
�
appmesh%getVirtualGatewaySpecLoggingAccessLogWaws:appmesh/getVirtualGatewaySpecLoggingAccessLog:getVirtualGatewaySpecLoggingAccessLog�
��
files�*�:�
�
appmesh)getVirtualGatewaySpecLoggingAccessLogFile_aws:appmesh/getVirtualGatewaySpecLoggingAccessLogFile:getVirtualGatewaySpecLoggingAccessLogFile:�
�
appmesh)getVirtualGatewaySpecLoggingAccessLogFile_aws:appmesh/getVirtualGatewaySpecLoggingAccessLogFile:getVirtualGatewaySpecLoggingAccessLogFile�
��
formats�*�:�
�
appmesh/getVirtualGatewaySpecLoggingAccessLogFileFormatkaws:appmesh/getVirtualGatewaySpecLoggingAccessLogFileFormat:getVirtualGatewaySpecLoggingAccessLogFileFormat

path" :�
�
appmesh/getVirtualGatewaySpecLoggingAccessLogFileFormatkaws:appmesh/getVirtualGatewaySpecLoggingAccessLogFileFormat:getVirtualGatewaySpecLoggingAccessLogFileFormat�
��
jsons�*�:�
�
appmesh3getVirtualGatewaySpecLoggingAccessLogFileFormatJsonsaws:appmesh/getVirtualGatewaySpecLoggingAccessLogFileFormatJson:getVirtualGatewaySpecLoggingAccessLogFileFormatJson

text" :�
�
appmesh3getVirtualGatewaySpecLoggingAccessLogFileFormatJsonsaws:appmesh/getVirtualGatewaySpecLoggingAccessLogFileFormatJson:getVirtualGatewaySpecLoggingAccessLogFileFormatJson
	
key" 
value" :�
P
appmeshgetVirtualNodeSpec1aws:appmesh/getVirtualNodeSpec:getVirtualNodeSpec�
��
backendDefaults�*~:|
z
appmesh getVirtualNodeSpecBackendDefaultMaws:appmesh/getVirtualNodeSpecBackendDefault:getVirtualNodeSpecBackendDefaultw
backendsk*i:g
e
appmeshgetVirtualNodeSpecBackend?aws:appmesh/getVirtualNodeSpecBackend:getVirtualNodeSpecBackend{
	listenersn*l:j
h
appmeshgetVirtualNodeSpecListenerAaws:appmesh/getVirtualNodeSpecListener:getVirtualNodeSpecListenerw
loggingsk*i:g
e
appmeshgetVirtualNodeSpecLogging?aws:appmesh/getVirtualNodeSpecLogging:getVirtualNodeSpecLogging�
serviceDiscoveries�*�:�
�
appmesh"getVirtualNodeSpecServiceDiscoveryQaws:appmesh/getVirtualNodeSpecServiceDiscovery:getVirtualNodeSpecServiceDiscovery:�
e
appmeshgetVirtualNodeSpecBackend?aws:appmesh/getVirtualNodeSpecBackend:getVirtualNodeSpecBackend�
��
virtualServices�*�:�
�
appmesh'getVirtualNodeSpecBackendVirtualService[aws:appmesh/getVirtualNodeSpecBackendVirtualService:getVirtualNodeSpecBackendVirtualService:�
z
appmesh getVirtualNodeSpecBackendDefaultMaws:appmesh/getVirtualNodeSpecBackendDefault:getVirtualNodeSpecBackendDefault�
��
clientPolicies�*�:�
�
appmesh,getVirtualNodeSpecBackendDefaultClientPolicyeaws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicy:getVirtualNodeSpecBackendDefaultClientPolicy:�
�
appmesh,getVirtualNodeSpecBackendDefaultClientPolicyeaws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicy:getVirtualNodeSpecBackendDefaultClientPolicy�
��
tls�*�:�
�
appmesh.getVirtualNodeSpecBackendDefaultClientPolicyTliaws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTl:getVirtualNodeSpecBackendDefaultClientPolicyTl:�
�
appmesh.getVirtualNodeSpecBackendDefaultClientPolicyTliaws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTl:getVirtualNodeSpecBackendDefaultClientPolicyTl�
��
certificates�*�:�
�
appmesh9getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateaws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlCertificate:getVirtualNodeSpecBackendDefaultClientPolicyTlCertificate
enforce
 
ports* �
validations�*�:�
�
appmesh8getVirtualNodeSpecBackendDefaultClientPolicyTlValidation}aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidation:getVirtualNodeSpecBackendDefaultClientPolicyTlValidation:�
�
appmesh9getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateaws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlCertificate:getVirtualNodeSpecBackendDefaultClientPolicyTlCertificate�
��
files�*�:�
�
appmesh=getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateFile�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateFile:getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateFile�
sds�*�:�
�
appmesh;getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateSd�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateSd:getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateSd:�
�
appmesh=getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateFile�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateFile:getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateFile,
*
certificateChain" 

privateKey" :�
�
appmesh;getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateSd�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateSd:getVirtualNodeSpecBackendDefaultClientPolicyTlCertificateSd


secretName" :�
�
appmesh8getVirtualNodeSpecBackendDefaultClientPolicyTlValidation}aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidation:getVirtualNodeSpecBackendDefaultClientPolicyTlValidation�
��
subjectAlternativeNames�*�:�
�
appmeshNgetVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�
trusts�*�:�
�
appmesh=getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrust�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrust:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrust:�
�
appmeshNgetVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeName�
��
matches�*�:�
�
appmeshSgetVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch:�
�
appmeshSgetVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationSubjectAlternativeNameMatch

exacts*" :�
�
appmesh=getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrust�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrust:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrust�
��
acms�*�:�
�
appmesh@getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustAcm�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustAcm:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustAcm�
files�*�:�
�
appmeshAgetVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustFile�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustFile:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustFile�
sds�*�:�
�
appmesh?getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustSd�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustSd:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustSd:�
�
appmesh@getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustAcm�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustAcm:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustAcm$
" 
certificateAuthorityArns*" :�
�
appmeshAgetVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustFile�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustFile:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustFile

certificateChain" :�
�
appmesh?getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustSd�aws:appmesh/getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustSd:getVirtualNodeSpecBackendDefaultClientPolicyTlValidationTrustSd


secretName" :�
�
appmesh'getVirtualNodeSpecBackendVirtualService[aws:appmesh/getVirtualNodeSpecBackendVirtualService:getVirtualNodeSpecBackendVirtualService�
��
clientPolicies�*�:�
�
appmesh3getVirtualNodeSpecBackendVirtualServiceClientPolicysaws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicy:getVirtualNodeSpecBackendVirtualServiceClientPolicy
virtualServiceName" :�
�
appmesh3getVirtualNodeSpecBackendVirtualServiceClientPolicysaws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicy:getVirtualNodeSpecBackendVirtualServiceClientPolicy�
��
tls�*�:�
�
appmesh5getVirtualNodeSpecBackendVirtualServiceClientPolicyTlwaws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTl:getVirtualNodeSpecBackendVirtualServiceClientPolicyTl:�
�
appmesh5getVirtualNodeSpecBackendVirtualServiceClientPolicyTlwaws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTl:getVirtualNodeSpecBackendVirtualServiceClientPolicyTl�
��
certificates�*�:�
�
appmesh@getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate
enforce
 
ports* �
validations�*�:�
�
appmesh?getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation:�
�
appmesh@getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate�
��
files�*�:�
�
appmeshDgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateFile�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateFile:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateFile�
sds�*�:�
�
appmeshBgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateSd�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateSd:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateSd:�
�
appmeshDgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateFile�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateFile:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateFile,
*
certificateChain" 

privateKey" :�
�
appmeshBgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateSd�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateSd:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateSd


secretName" :�
�
appmesh?getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation�
��
subjectAlternativeNames�*�:�
�
appmeshUgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeName�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeName:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeName�
trusts�*�:�
�
appmeshDgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust:�
�
appmeshUgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeName�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeName:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeName�
��
matches�*�:�
�
appmeshZgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeNameMatch:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeNameMatch:�
�
appmeshZgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeNameMatch:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationSubjectAlternativeNameMatch

exacts*" :�
�
appmeshDgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust�
��
acms�*�:�
�
appmeshGgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustAcm�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustAcm:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustAcm�
files�*�:�
�
appmeshHgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustFile�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustFile:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustFile�
sds�*�:�
�
appmeshFgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustSd�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustSd:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustSd:�
�
appmeshGgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustAcm�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustAcm:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustAcm$
" 
certificateAuthorityArns*" :�
�
appmeshHgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustFile�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustFile:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustFile

certificateChain" :�
�
appmeshFgetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustSd�aws:appmesh/getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustSd:getVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustSd


secretName" :�
h
appmeshgetVirtualNodeSpecListenerAaws:appmesh/getVirtualNodeSpecListener:getVirtualNodeSpecListener�
��
connectionPools�*�:�
�
appmesh(getVirtualNodeSpecListenerConnectionPool]aws:appmesh/getVirtualNodeSpecListenerConnectionPool:getVirtualNodeSpecListenerConnectionPool�
healthChecks�*�:�
�
appmesh%getVirtualNodeSpecListenerHealthCheckWaws:appmesh/getVirtualNodeSpecListenerHealthCheck:getVirtualNodeSpecListenerHealthCheck�
outlierDetections�*�:�
�
appmesh*getVirtualNodeSpecListenerOutlierDetectionaaws:appmesh/getVirtualNodeSpecListenerOutlierDetection:getVirtualNodeSpecListenerOutlierDetection�
portMappings�*�:�
�
appmesh%getVirtualNodeSpecListenerPortMappingWaws:appmesh/getVirtualNodeSpecListenerPortMapping:getVirtualNodeSpecListenerPortMapping�
timeouts�*�:
}
appmesh!getVirtualNodeSpecListenerTimeoutOaws:appmesh/getVirtualNodeSpecListenerTimeout:getVirtualNodeSpecListenerTimeout{
tlst*r:p
n
appmeshgetVirtualNodeSpecListenerTlEaws:appmesh/getVirtualNodeSpecListenerTl:getVirtualNodeSpecListenerTl:�
�
appmesh(getVirtualNodeSpecListenerConnectionPool]aws:appmesh/getVirtualNodeSpecListenerConnectionPool:getVirtualNodeSpecListenerConnectionPool�
��
grpcs�*�:�
�
appmesh,getVirtualNodeSpecListenerConnectionPoolGrpceaws:appmesh/getVirtualNodeSpecListenerConnectionPoolGrpc:getVirtualNodeSpecListenerConnectionPoolGrpc�
http2s�*�:�
�
appmesh-getVirtualNodeSpecListenerConnectionPoolHttp2gaws:appmesh/getVirtualNodeSpecListenerConnectionPoolHttp2:getVirtualNodeSpecListenerConnectionPoolHttp2�
https�*�:�
�
appmesh,getVirtualNodeSpecListenerConnectionPoolHttpeaws:appmesh/getVirtualNodeSpecListenerConnectionPoolHttp:getVirtualNodeSpecListenerConnectionPoolHttp�
tcps�*�:�
�
appmesh+getVirtualNodeSpecListenerConnectionPoolTcpcaws:appmesh/getVirtualNodeSpecListenerConnectionPoolTcp:getVirtualNodeSpecListenerConnectionPoolTcp:�
�
appmesh,getVirtualNodeSpecListenerConnectionPoolGrpceaws:appmesh/getVirtualNodeSpecListenerConnectionPoolGrpc:getVirtualNodeSpecListenerConnectionPoolGrpc

maxRequests :�
�
appmesh,getVirtualNodeSpecListenerConnectionPoolHttpeaws:appmesh/getVirtualNodeSpecListenerConnectionPoolHttp:getVirtualNodeSpecListenerConnectionPoolHttp2
0
maxConnections 
maxPendingRequests :�
�
appmesh-getVirtualNodeSpecListenerConnectionPoolHttp2gaws:appmesh/getVirtualNodeSpecListenerConnectionPoolHttp2:getVirtualNodeSpecListenerConnectionPoolHttp2

maxRequests :�
�
appmesh+getVirtualNodeSpecListenerConnectionPoolTcpcaws:appmesh/getVirtualNodeSpecListenerConnectionPoolTcp:getVirtualNodeSpecListenerConnectionPoolTcp

maxConnections :�
�
appmesh%getVirtualNodeSpecListenerHealthCheckWaws:appmesh/getVirtualNodeSpecListenerHealthCheck:getVirtualNodeSpecListenerHealthCheck�
�
healthyThreshold 
intervalMillis 

path" 

port 
protocol" 
timeoutMillis 
unhealthyThreshold :�
�
appmesh*getVirtualNodeSpecListenerOutlierDetectionaaws:appmesh/getVirtualNodeSpecListenerOutlierDetection:getVirtualNodeSpecListenerOutlierDetection�
��
baseEjectionDurations�*�:�
�
appmesh>getVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration�aws:appmesh/getVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration:getVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration�
	intervals�*�:�
�
appmesh2getVirtualNodeSpecListenerOutlierDetectionIntervalqaws:appmesh/getVirtualNodeSpecListenerOutlierDetectionInterval:getVirtualNodeSpecListenerOutlierDetectionInterval
maxEjectionPercent 
maxServerErrors :�
�
appmesh>getVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration�aws:appmesh/getVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration:getVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration


unit" 
value :�
�
appmesh2getVirtualNodeSpecListenerOutlierDetectionIntervalqaws:appmesh/getVirtualNodeSpecListenerOutlierDetectionInterval:getVirtualNodeSpecListenerOutlierDetectionInterval


unit" 
value :�
�
appmesh%getVirtualNodeSpecListenerPortMappingWaws:appmesh/getVirtualNodeSpecListenerPortMapping:getVirtualNodeSpecListenerPortMapping


port 
protocol" :�
}
appmesh!getVirtualNodeSpecListenerTimeoutOaws:appmesh/getVirtualNodeSpecListenerTimeout:getVirtualNodeSpecListenerTimeout�
��
grpcs�*�:�
�
appmesh%getVirtualNodeSpecListenerTimeoutGrpcWaws:appmesh/getVirtualNodeSpecListenerTimeoutGrpc:getVirtualNodeSpecListenerTimeoutGrpc�
http2s�*�:�
�
appmesh&getVirtualNodeSpecListenerTimeoutHttp2Yaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp2:getVirtualNodeSpecListenerTimeoutHttp2�
https�*�:�
�
appmesh%getVirtualNodeSpecListenerTimeoutHttpWaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp:getVirtualNodeSpecListenerTimeoutHttp�
tcps�*�:�
�
appmesh$getVirtualNodeSpecListenerTimeoutTcpUaws:appmesh/getVirtualNodeSpecListenerTimeoutTcp:getVirtualNodeSpecListenerTimeoutTcp:�
�
appmesh%getVirtualNodeSpecListenerTimeoutGrpcWaws:appmesh/getVirtualNodeSpecListenerTimeoutGrpc:getVirtualNodeSpecListenerTimeoutGrpc�
��
idles�*�:�
�
appmesh)getVirtualNodeSpecListenerTimeoutGrpcIdle_aws:appmesh/getVirtualNodeSpecListenerTimeoutGrpcIdle:getVirtualNodeSpecListenerTimeoutGrpcIdle�
perRequests�*�:�
�
appmesh/getVirtualNodeSpecListenerTimeoutGrpcPerRequestkaws:appmesh/getVirtualNodeSpecListenerTimeoutGrpcPerRequest:getVirtualNodeSpecListenerTimeoutGrpcPerRequest:�
�
appmesh)getVirtualNodeSpecListenerTimeoutGrpcIdle_aws:appmesh/getVirtualNodeSpecListenerTimeoutGrpcIdle:getVirtualNodeSpecListenerTimeoutGrpcIdle


unit" 
value :�
�
appmesh/getVirtualNodeSpecListenerTimeoutGrpcPerRequestkaws:appmesh/getVirtualNodeSpecListenerTimeoutGrpcPerRequest:getVirtualNodeSpecListenerTimeoutGrpcPerRequest


unit" 
value :�
�
appmesh%getVirtualNodeSpecListenerTimeoutHttpWaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp:getVirtualNodeSpecListenerTimeoutHttp�
��
idles�*�:�
�
appmesh)getVirtualNodeSpecListenerTimeoutHttpIdle_aws:appmesh/getVirtualNodeSpecListenerTimeoutHttpIdle:getVirtualNodeSpecListenerTimeoutHttpIdle�
perRequests�*�:�
�
appmesh/getVirtualNodeSpecListenerTimeoutHttpPerRequestkaws:appmesh/getVirtualNodeSpecListenerTimeoutHttpPerRequest:getVirtualNodeSpecListenerTimeoutHttpPerRequest:�
�
appmesh&getVirtualNodeSpecListenerTimeoutHttp2Yaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp2:getVirtualNodeSpecListenerTimeoutHttp2�
��
idles�*�:�
�
appmesh*getVirtualNodeSpecListenerTimeoutHttp2Idleaaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp2Idle:getVirtualNodeSpecListenerTimeoutHttp2Idle�
perRequests�*�:�
�
appmesh0getVirtualNodeSpecListenerTimeoutHttp2PerRequestmaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp2PerRequest:getVirtualNodeSpecListenerTimeoutHttp2PerRequest:�
�
appmesh*getVirtualNodeSpecListenerTimeoutHttp2Idleaaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp2Idle:getVirtualNodeSpecListenerTimeoutHttp2Idle


unit" 
value :�
�
appmesh0getVirtualNodeSpecListenerTimeoutHttp2PerRequestmaws:appmesh/getVirtualNodeSpecListenerTimeoutHttp2PerRequest:getVirtualNodeSpecListenerTimeoutHttp2PerRequest


unit" 
value :�
�
appmesh)getVirtualNodeSpecListenerTimeoutHttpIdle_aws:appmesh/getVirtualNodeSpecListenerTimeoutHttpIdle:getVirtualNodeSpecListenerTimeoutHttpIdle


unit" 
value :�
�
appmesh/getVirtualNodeSpecListenerTimeoutHttpPerRequestkaws:appmesh/getVirtualNodeSpecListenerTimeoutHttpPerRequest:getVirtualNodeSpecListenerTimeoutHttpPerRequest


unit" 
value :�
�
appmesh$getVirtualNodeSpecListenerTimeoutTcpUaws:appmesh/getVirtualNodeSpecListenerTimeoutTcp:getVirtualNodeSpecListenerTimeoutTcp�
��
idles�*�:�
�
appmesh(getVirtualNodeSpecListenerTimeoutTcpIdle]aws:appmesh/getVirtualNodeSpecListenerTimeoutTcpIdle:getVirtualNodeSpecListenerTimeoutTcpIdle:�
�
appmesh(getVirtualNodeSpecListenerTimeoutTcpIdle]aws:appmesh/getVirtualNodeSpecListenerTimeoutTcpIdle:getVirtualNodeSpecListenerTimeoutTcpIdle


unit" 
value :�
n
appmeshgetVirtualNodeSpecListenerTlEaws:appmesh/getVirtualNodeSpecListenerTl:getVirtualNodeSpecListenerTl�
��
certificates�*�:�
�
appmesh'getVirtualNodeSpecListenerTlCertificate[aws:appmesh/getVirtualNodeSpecListenerTlCertificate:getVirtualNodeSpecListenerTlCertificate

mode" �
validations�*�:�
�
appmesh&getVirtualNodeSpecListenerTlValidationYaws:appmesh/getVirtualNodeSpecListenerTlValidation:getVirtualNodeSpecListenerTlValidation:�
�
appmesh'getVirtualNodeSpecListenerTlCertificate[aws:appmesh/getVirtualNodeSpecListenerTlCertificate:getVirtualNodeSpecListenerTlCertificate�
��
acms�*�:�
�
appmesh*getVirtualNodeSpecListenerTlCertificateAcmaaws:appmesh/getVirtualNodeSpecListenerTlCertificateAcm:getVirtualNodeSpecListenerTlCertificateAcm�
files�*�:�
�
appmesh+getVirtualNodeSpecListenerTlCertificateFilecaws:appmesh/getVirtualNodeSpecListenerTlCertificateFile:getVirtualNodeSpecListenerTlCertificateFile�
sds�*�:�
�
appmesh)getVirtualNodeSpecListenerTlCertificateSd_aws:appmesh/getVirtualNodeSpecListenerTlCertificateSd:getVirtualNodeSpecListenerTlCertificateSd:�
�
appmesh*getVirtualNodeSpecListenerTlCertificateAcmaaws:appmesh/getVirtualNodeSpecListenerTlCertificateAcm:getVirtualNodeSpecListenerTlCertificateAcm

certificateArn" :�
�
appmesh+getVirtualNodeSpecListenerTlCertificateFilecaws:appmesh/getVirtualNodeSpecListenerTlCertificateFile:getVirtualNodeSpecListenerTlCertificateFile,
*
certificateChain" 

privateKey" :�
�
appmesh)getVirtualNodeSpecListenerTlCertificateSd_aws:appmesh/getVirtualNodeSpecListenerTlCertificateSd:getVirtualNodeSpecListenerTlCertificateSd


secretName" :�
�
appmesh&getVirtualNodeSpecListenerTlValidationYaws:appmesh/getVirtualNodeSpecListenerTlValidation:getVirtualNodeSpecListenerTlValidation�
��
subjectAlternativeNames�*�:�
�
appmesh<getVirtualNodeSpecListenerTlValidationSubjectAlternativeName�aws:appmesh/getVirtualNodeSpecListenerTlValidationSubjectAlternativeName:getVirtualNodeSpecListenerTlValidationSubjectAlternativeName�
trusts�*�:�
�
appmesh+getVirtualNodeSpecListenerTlValidationTrustcaws:appmesh/getVirtualNodeSpecListenerTlValidationTrust:getVirtualNodeSpecListenerTlValidationTrust:�
�
appmesh<getVirtualNodeSpecListenerTlValidationSubjectAlternativeName�aws:appmesh/getVirtualNodeSpecListenerTlValidationSubjectAlternativeName:getVirtualNodeSpecListenerTlValidationSubjectAlternativeName�
��
matches�*�:�
�
appmeshAgetVirtualNodeSpecListenerTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualNodeSpecListenerTlValidationSubjectAlternativeNameMatch:getVirtualNodeSpecListenerTlValidationSubjectAlternativeNameMatch:�
�
appmeshAgetVirtualNodeSpecListenerTlValidationSubjectAlternativeNameMatch�aws:appmesh/getVirtualNodeSpecListenerTlValidationSubjectAlternativeNameMatch:getVirtualNodeSpecListenerTlValidationSubjectAlternativeNameMatch

exacts*" :�
�
appmesh+getVirtualNodeSpecListenerTlValidationTrustcaws:appmesh/getVirtualNodeSpecListenerTlValidationTrust:getVirtualNodeSpecListenerTlValidationTrust�
��
files�*�:�
�
appmesh/getVirtualNodeSpecListenerTlValidationTrustFilekaws:appmesh/getVirtualNodeSpecListenerTlValidationTrustFile:getVirtualNodeSpecListenerTlValidationTrustFile�
sds�*�:�
�
appmesh-getVirtualNodeSpecListenerTlValidationTrustSdgaws:appmesh/getVirtualNodeSpecListenerTlValidationTrustSd:getVirtualNodeSpecListenerTlValidationTrustSd:�
�
appmesh/getVirtualNodeSpecListenerTlValidationTrustFilekaws:appmesh/getVirtualNodeSpecListenerTlValidationTrustFile:getVirtualNodeSpecListenerTlValidationTrustFile

certificateChain" :�
�
appmesh-getVirtualNodeSpecListenerTlValidationTrustSdgaws:appmesh/getVirtualNodeSpecListenerTlValidationTrustSd:getVirtualNodeSpecListenerTlValidationTrustSd


secretName" :�
e
appmeshgetVirtualNodeSpecLogging?aws:appmesh/getVirtualNodeSpecLogging:getVirtualNodeSpecLogging�
��

accessLogs�*�:�
�
appmesh"getVirtualNodeSpecLoggingAccessLogQaws:appmesh/getVirtualNodeSpecLoggingAccessLog:getVirtualNodeSpecLoggingAccessLog:�
�
appmesh"getVirtualNodeSpecLoggingAccessLogQaws:appmesh/getVirtualNodeSpecLoggingAccessLog:getVirtualNodeSpecLoggingAccessLog�
��
files�*�:�
�
appmesh&getVirtualNodeSpecLoggingAccessLogFileYaws:appmesh/getVirtualNodeSpecLoggingAccessLogFile:getVirtualNodeSpecLoggingAccessLogFile:�
�
appmesh&getVirtualNodeSpecLoggingAccessLogFileYaws:appmesh/getVirtualNodeSpecLoggingAccessLogFile:getVirtualNodeSpecLoggingAccessLogFile�
��
formats�*�:�
�
appmesh,getVirtualNodeSpecLoggingAccessLogFileFormateaws:appmesh/getVirtualNodeSpecLoggingAccessLogFileFormat:getVirtualNodeSpecLoggingAccessLogFileFormat

path" :�
�
appmesh,getVirtualNodeSpecLoggingAccessLogFileFormateaws:appmesh/getVirtualNodeSpecLoggingAccessLogFileFormat:getVirtualNodeSpecLoggingAccessLogFileFormat�
��
jsons�*�:�
�
appmesh0getVirtualNodeSpecLoggingAccessLogFileFormatJsonmaws:appmesh/getVirtualNodeSpecLoggingAccessLogFileFormatJson:getVirtualNodeSpecLoggingAccessLogFileFormatJson

text" :�
�
appmesh0getVirtualNodeSpecLoggingAccessLogFileFormatJsonmaws:appmesh/getVirtualNodeSpecLoggingAccessLogFileFormatJson:getVirtualNodeSpecLoggingAccessLogFileFormatJson
	
key" 
value" :�
�
appmesh"getVirtualNodeSpecServiceDiscoveryQaws:appmesh/getVirtualNodeSpecServiceDiscovery:getVirtualNodeSpecServiceDiscovery�
��
awsCloudMaps�*�:�
�
appmesh-getVirtualNodeSpecServiceDiscoveryAwsCloudMapgaws:appmesh/getVirtualNodeSpecServiceDiscoveryAwsCloudMap:getVirtualNodeSpecServiceDiscoveryAwsCloudMap�
dns�*�:�
�
appmesh$getVirtualNodeSpecServiceDiscoveryDnUaws:appmesh/getVirtualNodeSpecServiceDiscoveryDn:getVirtualNodeSpecServiceDiscoveryDn:�
�
appmesh-getVirtualNodeSpecServiceDiscoveryAwsCloudMapgaws:appmesh/getVirtualNodeSpecServiceDiscoveryAwsCloudMap:getVirtualNodeSpecServiceDiscoveryAwsCloudMap>
<

attributes2" 
namespaceName" 
serviceName" :�
�
appmesh$getVirtualNodeSpecServiceDiscoveryDnUaws:appmesh/getVirtualNodeSpecServiceDiscoveryDn:getVirtualNodeSpecServiceDiscoveryDn:
8
hostname" 
ipPreference" 
responseType" :�
V
appmeshgetVirtualRouterSpec5aws:appmesh/getVirtualRouterSpec:getVirtualRouterSpec�
��
	listenerst*r:p
n
appmeshgetVirtualRouterSpecListenerEaws:appmesh/getVirtualRouterSpecListener:getVirtualRouterSpecListener:�
n
appmeshgetVirtualRouterSpecListenerEaws:appmesh/getVirtualRouterSpecListener:getVirtualRouterSpecListener�
��
portMappings�*�:�
�
appmesh'getVirtualRouterSpecListenerPortMapping[aws:appmesh/getVirtualRouterSpecListenerPortMapping:getVirtualRouterSpecListenerPortMapping:�
�
appmesh'getVirtualRouterSpecListenerPortMapping[aws:appmesh/getVirtualRouterSpecListenerPortMapping:getVirtualRouterSpecListenerPortMapping


port 
protocol" :�
Y
appmeshgetVirtualServiceSpec7aws:appmesh/getVirtualServiceSpec:getVirtualServiceSpec�
��
	providersw*u:s
q
appmeshgetVirtualServiceSpecProviderGaws:appmesh/getVirtualServiceSpecProvider:getVirtualServiceSpecProvider:�
q
appmeshgetVirtualServiceSpecProviderGaws:appmesh/getVirtualServiceSpecProvider:getVirtualServiceSpecProvider�
��
virtualNodes�*�:�
�
appmesh(getVirtualServiceSpecProviderVirtualNode]aws:appmesh/getVirtualServiceSpecProviderVirtualNode:getVirtualServiceSpecProviderVirtualNode�
virtualRouters�*�:�
�
appmesh*getVirtualServiceSpecProviderVirtualRouteraaws:appmesh/getVirtualServiceSpecProviderVirtualRouter:getVirtualServiceSpecProviderVirtualRouter:�
�
appmesh(getVirtualServiceSpecProviderVirtualNode]aws:appmesh/getVirtualServiceSpecProviderVirtualNode:getVirtualServiceSpecProviderVirtualNode

virtualNodeName" :�
�
appmesh*getVirtualServiceSpecProviderVirtualRouteraaws:appmesh/getVirtualServiceSpecProviderVirtualRouter:getVirtualServiceSpecProviderVirtualRouter

virtualRouterName" :�
�
	apprunner2CustomDomainAssociationCertificateValidationRecordsaws:apprunner/CustomDomainAssociationCertificateValidationRecord:CustomDomainAssociationCertificateValidationRecord�
�-
nameB" Certificate CNAME record name.
�
statusB" �Current state of the certificate CNAME record validation. It should change to `SUCCESS` after App Runner completes validation with your DNS.
+
typeB" Record type, always `CNAME`.
/
valueB"  Certificate CNAME record value.
:�
T
	apprunnerDeploymentTimeouts3aws:apprunner/DeploymentTimeouts:DeploymentTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
�
	apprunner,ObservabilityConfigurationTraceConfigurationgaws:apprunner/ObservabilityConfigurationTraceConfiguration:ObservabilityConfigurationTraceConfigurationm
ki
vendorB" YImplementation provider chosen for tracing App Runner services. Valid values: `AWSXRAY`.
:�
x
	apprunnerServiceEncryptionConfigurationKaws:apprunner/ServiceEncryptionConfiguration:ServiceEncryptionConfiguration:
86
kmsKey" (ARN of the KMS key used for encryption.
:�	
{
	apprunnerServiceHealthCheckConfigurationMaws:apprunner/ServiceHealthCheckConfiguration:ServiceHealthCheckConfiguration�
��
healthyThresholdB �Number of consecutive checks that must succeed before App Runner decides that the service is healthy. Defaults to 1. Minimum value of 1. Maximum value of 20.
|
intervalB jTime interval, in seconds, between health checks. Defaults to 5. Minimum value of 1. Maximum value of 20.
x
pathB" jURL to send requests to for health checks. Defaults to `/`. Minimum length of 0. Maximum length of 51200.
�
protocolB" �IP protocol that App Runner uses to perform health checks for your service. Valid values: `TCP`, `HTTP`. Defaults to `TCP`. If you set protocol to `HTTP`, App Runner sends health check requests to the HTTP path specified by `path`.
�
timeoutB �Time, in seconds, to wait for a health check response before deciding it failed. Defaults to 2. Minimum value of  1. Maximum value of 20.
�
unhealthyThresholdB �Number of consecutive checks that must fail before App Runner decides that the service is unhealthy. Defaults to 5. Minimum value of  1. Maximum value of 20.
:�
r
	apprunnerServiceInstanceConfigurationGaws:apprunner/ServiceInstanceConfiguration:ServiceInstanceConfiguration�
��
cpuB" �Number of CPU units reserved for each instance of your App Runner service represented as a String. Defaults to `1024`. Valid values: `256|512|1024|2048|4096|(0.25|0.5|1|2|4) vCPU`.
�
instanceRoleArnB" �ARN of an IAM role that provides permissions to your App Runner service. These are permissions that your code needs when it calls any AWS APIs.
�
memoryB" �Amount of memory, in MB or GB, reserved for each instance of your App Runner service. Defaults to `2048`. Valid values: `512|1024|2048|3072|4096|6144|8192|10240|12288|(0.5|1|2|3|4|6|8|10|12) GB`.
:�
o
	apprunnerServiceNetworkConfigurationEaws:apprunner/ServiceNetworkConfiguration:ServiceNetworkConfiguration�
��
egressConfiguration�B�:�
�
	apprunner.ServiceNetworkConfigurationEgressConfigurationkaws:apprunner/ServiceNetworkConfigurationEgressConfiguration:ServiceNetworkConfigurationEgressConfigurationnNetwork configuration settings for outbound message traffic. See Egress Configuration below for more details.
�
ingressConfiguration�B�:�
�
	apprunner/ServiceNetworkConfigurationIngressConfigurationmaws:apprunner/ServiceNetworkConfigurationIngressConfiguration:ServiceNetworkConfigurationIngressConfigurationnNetwork configuration settings for inbound network traffic. See Ingress Configuration below for more details.
�
ipAddressTypeB" �App Runner provides you with the option to choose between Internet Protocol version 4 (IPv4) and dual stack (IPv4 and IPv6) for your incoming public network configuration. Valid values: `IPV4`, `DUAL_STACK`. Default: `IPV4`.
:�
�
	apprunner.ServiceNetworkConfigurationEgressConfigurationkaws:apprunner/ServiceNetworkConfigurationEgressConfiguration:ServiceNetworkConfigurationEgressConfiguration�
�]

egressTypeB" IThe type of egress configuration. Valid values are: `DEFAULT` and `VPC`.
�
vpcConnectorArnB" �The Amazon Resource Name (ARN) of the App Runner VPC connector that you want to associate with your App Runner service. Only valid when `EgressType = VPC`.
:�
�
	apprunner/ServiceNetworkConfigurationIngressConfigurationmaws:apprunner/ServiceNetworkConfigurationIngressConfiguration:ServiceNetworkConfigurationIngressConfiguration�
��
isPubliclyAccessibleB
 �Specifies whether your App Runner service is publicly accessible. To make the service publicly accessible set it to True. To make the service privately accessible, from only within an Amazon VPC set it to False.
:�
�
	apprunner!ServiceObservabilityConfigurationQaws:apprunner/ServiceObservabilityConfiguration:ServiceObservabilityConfiguration�
��
observabilityConfigurationArnB" �ARN of the observability configuration that is associated with the service. Specified only when `observability_enabled` is `true`.
q
observabilityEnabled
 UWhen `true`, an observability configuration resource is associated with the service.
:�

l
	apprunnerServiceSourceConfigurationCaws:apprunner/ServiceSourceConfiguration:ServiceSourceConfiguration�	
�	�
authenticationConfiguration�B�:�
�
	apprunner5ServiceSourceConfigurationAuthenticationConfigurationyaws:apprunner/ServiceSourceConfigurationAuthenticationConfiguration:ServiceSourceConfigurationAuthenticationConfiguration�Describes resources needed to authenticate access to some source repositories. See Authentication Configuration below for more details.
�
autoDeploymentsEnabledB
 �Whether continuous integration from the source repository is enabled for the App Runner service. If set to `true`, each repository change (source code commit or new image version) starts a deployment. Defaults to `true`.
�
codeRepository�B�:�
�
	apprunner(ServiceSourceConfigurationCodeRepository_aws:apprunner/ServiceSourceConfigurationCodeRepository:ServiceSourceConfigurationCodeRepositoryUDescription of a source code repository. See Code Repository below for more details.
�
imageRepository�B�:�
�
	apprunner)ServiceSourceConfigurationImageRepositoryaaws:apprunner/ServiceSourceConfigurationImageRepository:ServiceSourceConfigurationImageRepositoryWDescription of a source image repository. See Image Repository below for more details.
:�
�
	apprunner5ServiceSourceConfigurationAuthenticationConfigurationyaws:apprunner/ServiceSourceConfigurationAuthenticationConfiguration:ServiceSourceConfigurationAuthenticationConfiguration�
��
accessRoleArnB" �ARN of the IAM role that grants the App Runner service access to a source repository. Required for ECR image repositories (but not for ECR Public)
�
connectionArnB" �ARN of the App Runner connection that enables the App Runner service to connect to a source repository. Required for GitHub code repositories.
:�	
�
	apprunner(ServiceSourceConfigurationCodeRepository_aws:apprunner/ServiceSourceConfigurationCodeRepository:ServiceSourceConfigurationCodeRepository�
��
codeConfiguration�B�:�
�
	apprunner9ServiceSourceConfigurationCodeRepositoryCodeConfiguration�aws:apprunner/ServiceSourceConfigurationCodeRepositoryCodeConfiguration:ServiceSourceConfigurationCodeRepositoryCodeConfiguration�Configuration for building and running the service from a source code repository. See Code Configuration below for more details.
O
repositoryUrl" :Location of the repository that contains the source code.
�
sourceCodeVersion�:�
�
	apprunner9ServiceSourceConfigurationCodeRepositorySourceCodeVersion�aws:apprunner/ServiceSourceConfigurationCodeRepositorySourceCodeVersion:ServiceSourceConfigurationCodeRepositorySourceCodeVersionoVersion that should be used within the source code repository. See Source Code Version below for more details.
�
sourceDirectoryB" �The path of the directory that stores source code and configuration files. The build and start commands also execute from here. The path is absolute from root and, if not specified, defaults to the repository root.
:�

�
	apprunner9ServiceSourceConfigurationCodeRepositoryCodeConfiguration�aws:apprunner/ServiceSourceConfigurationCodeRepositoryCodeConfiguration:ServiceSourceConfigurationCodeRepositoryCodeConfiguration�
��
codeConfigurationValues�B�:�
�
	apprunnerPServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues�aws:apprunner/ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues:ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues�Basic configuration for building and running the App Runner service. Use this parameter to quickly launch an App Runner service without providing an apprunner.yaml file in the source code repository (or ignoring the file if it exists). See Code Configuration Values below for more details.
�
configurationSource" �Source of the App Runner configuration. Valid values: `REPOSITORY`, `API`. Values are interpreted as follows:
* `REPOSITORY` - App Runner reads configuration values from the apprunner.yaml file in the
source code repository and ignores the CodeConfigurationValues parameter.
* `API` - App Runner uses configuration values provided in the CodeConfigurationValues
parameter and ignores the apprunner.yaml file in the source code repository.
:�
�
	apprunnerPServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues�aws:apprunner/ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues:ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues�	
�	I
buildCommandB" 3Command App Runner runs to build your application.
\
portB" NPort that your application listens to in the container. Defaults to `"8080"`.
�
runtime" �Runtime environment type for building and running an App Runner service. Represents a programming language runtime. Valid values: `PYTHON_3`, `NODEJS_12`, `NODEJS_14`, `NODEJS_16`, `CORRETTO_8`, `CORRETTO_11`, `GO_1`, `DOTNET_6`, `PHP_81`, `RUBY_31`.
�
runtimeEnvironmentSecretsB2" �Secrets and parameters available to your service as environment variables. A map of key/value pairs, where the key is the desired name of the Secret in the environment (i.e. it does not have to match the name of the secret in Secrets Manager or SSM Parameter Store), and the value is the ARN of the secret from AWS Secrets Manager or the ARN of the parameter in AWS SSM Parameter Store.
�
runtimeEnvironmentVariablesB2" �Environment variables available to your running App Runner service. A map of key/value pairs. Keys with a prefix of `AWSAPPRUNNER` are reserved for system use and aren't valid.
I
startCommandB" 3Command App Runner runs to start your application.
:�
�
	apprunner9ServiceSourceConfigurationCodeRepositorySourceCodeVersion�aws:apprunner/ServiceSourceConfigurationCodeRepositorySourceCodeVersion:ServiceSourceConfigurationCodeRepositorySourceCodeVersion�
�y
type" mType of version identifier. For a git-based repository, branches represent versions. Valid values: `BRANCH`.
�
value" �Source code version. For a git-based repository, a branch name maps to a specific version. App Runner uses the most recent commit to the branch.
:�
�
	apprunner)ServiceSourceConfigurationImageRepositoryaaws:apprunner/ServiceSourceConfigurationImageRepository:ServiceSourceConfigurationImageRepository�
��
imageConfiguration�B�:�
�
	apprunner;ServiceSourceConfigurationImageRepositoryImageConfiguration�aws:apprunner/ServiceSourceConfigurationImageRepositoryImageConfiguration:ServiceSourceConfigurationImageRepositoryImageConfiguration`Configuration for running the identified image. See Image Configuration below for more details.
�
imageIdentifier" �Identifier of an image. For an image in Amazon Elastic Container Registry (Amazon ECR), this is an image name. For the
image name format, see Pulling an image in the Amazon ECR User Guide.
�
imageRepositoryType" �Type of the image repository. This reflects the repository provider and whether the repository is private or public. Valid values: `ECR` , `ECR_PUBLIC`.
:�
�
	apprunner;ServiceSourceConfigurationImageRepositoryImageConfiguration�aws:apprunner/ServiceSourceConfigurationImageRepositoryImageConfiguration:ServiceSourceConfigurationImageRepositoryImageConfiguration�
�\
portB" NPort that your application listens to in the container. Defaults to `"8080"`.
�
runtimeEnvironmentSecretsB2" �Secrets and parameters available to your service as environment variables. A map of key/value pairs, where the key is the desired name of the Secret in the environment (i.e. it does not have to match the name of the secret in Secrets Manager or SSM Parameter Store), and the value is the ARN of the secret from AWS Secrets Manager or the ARN of the parameter in AWS SSM Parameter Store.
�
runtimeEnvironmentVariablesB2" �Environment variables available to your running App Runner service. A map of key/value pairs. Keys with a prefix of `AWSAPPRUNNER` are reserved for system use and aren't valid.
�
startCommandB" �Command App Runner runs to start the application in the source image. If specified, this command overrides the Docker image’s default start command.
:�
�
	apprunner+VpcIngressConnectionIngressVpcConfigurationeaws:apprunner/VpcIngressConnectionIngressVpcConfiguration:VpcIngressConnectionIngressVpcConfiguration�
�\
vpcEndpointIdB" EThe ID of the VPC endpoint that your App Runner service connects to.
D
vpcIdB" 5The ID of the VPC that is used for the VPC endpoint.
:�
�
	appstream(DirectoryConfigServiceAccountCredentials_aws:appstream/DirectoryConfigServiceAccountCredentials:DirectoryConfigServiceAccountCredentials�
��
accountName" �User name of the account. This account must have the following privileges: create computer objects, join computers to the domain, and change/reset the password on descendant computer objects for the organizational units specified.
1
accountPassword" Password for the account.
:�
Z
	appstreamFleetComputeCapacity7aws:appstream/FleetComputeCapacity:FleetComputeCapacity�
�`
	availableB MNumber of currently available instances that can be used to stream sessions.
A
desiredInstancesB 'Desired number of streaming instances.
�
desiredSessionsB jDesired number of user sessions for a multi-session fleet. This is not allowed for single-session fleets.
9
inUseB *Number of instances in use for streaming.
T
runningB CTotal number of simultaneous streaming instances that are running.
:�
W
	appstreamFleetDomainJoinInfo5aws:appstream/FleetDomainJoinInfo:FleetDomainJoinInfo�
�^
directoryNameB" GFully qualified name of the directory (for example, corp.example.com).
r
#organizationalUnitDistinguishedNameB" EDistinguished name of the organizational unit for computer accounts.
:�
H
	appstreamFleetVpcConfig+aws:appstream/FleetVpcConfig:FleetVpcConfig�
�_
securityGroupIdsB*" CIdentifiers of the security groups for the fleet or image builder.
�
	subnetIdsB*" wIdentifiers of the subnets to which a network interface is attached from the fleet instance or image builder instance.
:�
l
	appstreamImageBuilderAccessEndpointCaws:appstream/ImageBuilderAccessEndpoint:ImageBuilderAccessEndpoint�
��
endpointType" �Type of interface endpoint. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/appstream2/latest/APIReference/API_AccessEndpoint.html).
?
vpceIdB" /Identifier (ID) of the interface VPC endpoint.
:�
l
	appstreamImageBuilderDomainJoinInfoCaws:appstream/ImageBuilderDomainJoinInfo:ImageBuilderDomainJoinInfo�
�^
directoryNameB" GFully qualified name of the directory (for example, corp.example.com).
r
#organizationalUnitDistinguishedNameB" EDistinguished name of the organizational unit for computer accounts.
:�
]
	appstreamImageBuilderVpcConfig9aws:appstream/ImageBuilderVpcConfig:ImageBuilderVpcConfig�
�g
securityGroupIdsB*" KIdentifiers of the security groups for the image builder or image builder.
x
	subnetIdsB*" cIdentifier of the subnet to which a network interface is attached from the image builder instance.
:�
W
	appstreamStackAccessEndpoint5aws:appstream/StackAccessEndpoint:StackAccessEndpoint�
��
endpointType" �Type of the interface endpoint.
See the [`AccessEndpoint` AWS API documentation](https://docs.aws.amazon.com/appstream2/latest/APIReference/API_AccessEndpoint.html) for valid values.
G
vpceIdB" 7ID of the VPC in which the interface endpoint is used.
:�
f
	appstreamStackApplicationSettings?aws:appstream/StackApplicationSettings:StackApplicationSettings�
�A
enabled
 2Whether application settings should be persisted.
s
settingsGroupB" \Name of the settings group.
Required when `enabled` is `true`.
Can be up to 100 characters.
:�
]
	appstreamStackStorageConnector9aws:appstream/StackStorageConnector:StackStorageConnector�
�p
connectorType" [Type of storage connector.
Valid values are `HOMEFOLDERS`, `GOOGLE_DRIVE`, or `ONE_DRIVE`.
9
domainsB*" &Names of the domains for the account.
:
resourceIdentifierB" ARN of the storage connector.
:�
~
	appstream StackStreamingExperienceSettingsOaws:appstream/StackStreamingExperienceSettings:StackStreamingExperienceSettings�
��
preferredProtocolB" pThe preferred protocol that you want to use while streaming your application.
Valid values are `TCP` and `UDP`.
:�
N
	appstreamStackUserSetting/aws:appstream/StackUserSetting:StackUserSetting�
��
action" �Action that is enabled or disabled.
Valid values are `AUTO_TIME_ZONE_REDIRECTION`, `CLIPBOARD_COPY_FROM_LOCAL_DEVICE`, `CLIPBOARD_COPY_TO_LOCAL_DEVICE`, `DOMAIN_PASSWORD_SIGNIN`, `DOMAIN_SMART_CARD_SIGNIN`, `FILE_UPLOAD`, `FILE_DOWNLOAD`, or `PRINTING_TO_LOCAL_DEVICE`.
g

permission" UWhether the action is enabled or disabled.
Valid values are `ENABLED` or `DISABLED`.
:�
W
	appstreamgetImageApplication5aws:appstream/getImageApplication:getImageApplication�
�9
appBlockArn" &The app block ARN of the application.
X
arn" MArn of the image being searched for. Cannot be used with name_regex or name.
9
createdTime" &Time at which this image was created.
)
description" Description of image.
*
displayName" Image name to display.
<
enabled
 -Bool based on if the application is enabled.
�
iconS3Locations�*�:�
�
	appstream!getImageApplicationIconS3LocationQaws:appstream/getImageApplicationIconS3Location:getImageApplicationIconS3Location;A list named icon_s3_location that contains the following:
J
iconUrl" ;URL of the application icon. This URL may be time-limited.
L
instanceFamilies*" 2List of the instance families of the application.
U
launchParameters" =Arguments that are passed to the application at it's launch.
I

launchPath" 7Path to the application's excecutable in the instance.
�
metadata2" String to string map that contains additional attributes used to describe the application.
* `Name` - Name of the application.
Y
name" MName of the image being searched for. Cannot be used with name_regex or arn.
�
	platforms*" �Array of strings describing the platforms on which the application can run.
Values will be from: WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019 | WINDOWS_SERVER_2022 | AMAZON_LINUX2
?
workingDirectory" 'Working directory for the application.
:�
�
	appstream!getImageApplicationIconS3LocationQaws:appstream/getImageApplicationIconS3Location:getImageApplicationIconS3LocationX
V,
s3Bucket" S3 bucket of the S3 object.
&
s3Key" S3 key of the S3 object.
:�
c
	appstreamgetImageImagePermission=aws:appstream/getImageImagePermission:getImageImagePermission�
�K

allowFleet
 9Boolean indicating if the image can be used for a fleet.
W
allowImageBuilder
 >indicated whether the image can be used for an image builder.
:�
i
	appstreamgetImageStateChangeReasonAaws:appstream/getImageStateChangeReason:getImageStateChangeReason


code" 
message" :�
b
appsyncDataSourceDynamodbConfig=aws:appsync/DataSourceDynamodbConfig:DataSourceDynamodbConfig�
��
deltaSyncConfig�B�:�
�
appsync'DataSourceDynamodbConfigDeltaSyncConfig[aws:appsync/DataSourceDynamodbConfigDeltaSyncConfig:DataSourceDynamodbConfigDeltaSyncConfig\The DeltaSyncConfig for a versioned data source. See `delta_sync_config` Block for details.
N
regionB" >AWS region of the DynamoDB table. Defaults to current region.
-
	tableName" Name of the DynamoDB table.
e
useCallerCredentialsB
 GSet to `true` to use Amazon Cognito credentials with this data source.
T
	versionedB
 ADetects Conflict Detection and Resolution with this data source.
:�
�
appsync'DataSourceDynamodbConfigDeltaSyncConfig[aws:appsync/DataSourceDynamodbConfigDeltaSyncConfig:DataSourceDynamodbConfigDeltaSyncConfig�
�W
baseTableTtlB AThe number of minutes that an Item is stored in the data source.
*
deltaSyncTableName" The table name.
p
deltaSyncTableTtlB UThe number of minutes that a Delta Sync log entry is stored in the Delta Sync table.
:�
q
appsyncDataSourceElasticsearchConfigGaws:appsync/DataSourceElasticsearchConfig:DataSourceElasticsearchConfig�
�;
endpoint" +HTTP endpoint of the Elasticsearch domain.
P
regionB" @AWS region of Elasticsearch domain. Defaults to current region.
:�
k
appsyncDataSourceEventBridgeConfigCaws:appsync/DataSourceEventBridgeConfig:DataSourceEventBridgeConfig4
20
eventBusArn" ARN for the EventBridge bus.
:�
V
appsyncDataSourceHttpConfig5aws:appsync/DataSourceHttpConfig:DataSourceHttpConfig�
��
authorizationConfig�B�:�
�
appsync'DataSourceHttpConfigAuthorizationConfig[aws:appsync/DataSourceHttpConfigAuthorizationConfig:DataSourceHttpConfigAuthorizationConfig|Authorization configuration in case the HTTP endpoint requires authorization. See `authorization_config` Block for details.

endpoint" 
HTTP URL.
:�
�
appsync'DataSourceHttpConfigAuthorizationConfig[aws:appsync/DataSourceHttpConfigAuthorizationConfig:DataSourceHttpConfigAuthorizationConfig�
�l
authorizationTypeB" QAuthorization type that the HTTP endpoint requires. Default values is `AWS_IAM`.
�
awsIamConfig�B�:�
�
appsync3DataSourceHttpConfigAuthorizationConfigAwsIamConfigsaws:appsync/DataSourceHttpConfigAuthorizationConfigAwsIamConfig:DataSourceHttpConfigAuthorizationConfigAwsIamConfigWIdentity and Access Management (IAM) settings. See `aws_iam_config` Block for details.
:�
�
appsync3DataSourceHttpConfigAuthorizationConfigAwsIamConfigsaws:appsync/DataSourceHttpConfigAuthorizationConfigAwsIamConfig:DataSourceHttpConfigAuthorizationConfigAwsIamConfig�
�Q
signingRegionB" :Signing Amazon Web Services Region for IAM authorization.
H
signingServiceNameB" ,Signing service name for IAM authorization.
:�
\
appsyncDataSourceLambdaConfig9aws:appsync/DataSourceLambdaConfig:DataSourceLambdaConfig4
20
functionArn" ARN for the Lambda function.
:�
}
appsync!DataSourceOpensearchserviceConfigOaws:appsync/DataSourceOpensearchserviceConfig:DataSourceOpensearchserviceConfig�
�8
endpoint" (HTTP endpoint of the OpenSearch domain.
Q
regionB" AAWS region of the OpenSearch domain. Defaults to current region.
:�
�
appsync"DataSourceRelationalDatabaseConfigQaws:appsync/DataSourceRelationalDatabaseConfig:DataSourceRelationalDatabaseConfig�
��
httpEndpointConfig�B�:�
�
appsync4DataSourceRelationalDatabaseConfigHttpEndpointConfiguaws:appsync/DataSourceRelationalDatabaseConfigHttpEndpointConfig:DataSourceRelationalDatabaseConfigHttpEndpointConfigVAmazon RDS HTTP endpoint configuration. See `http_endpoint_config` Block for details.
`

sourceTypeB" LSource type for the relational database. Valid values: `RDS_HTTP_ENDPOINT`.
:�
�
appsync4DataSourceRelationalDatabaseConfigHttpEndpointConfiguaws:appsync/DataSourceRelationalDatabaseConfigHttpEndpointConfig:DataSourceRelationalDatabaseConfigHttpEndpointConfig�
�H
awsSecretStoreArn" /AWS secret store ARN for database credentials.
-
databaseNameB" Logical database name.
:
dbClusterIdentifier" Amazon RDS cluster identifier.
N
regionB" >AWS Region for RDS HTTP endpoint. Defaults to current region.
%
schemaB" Logical schema name.
:�
G
appsyncFunctionRuntime+aws:appsync/FunctionRuntime:FunctionRuntime�
�_
name" SThe name of the runtime to use. Currently, the only allowed value is `APPSYNC_JS`.
i
runtimeVersion" SThe version of the runtime to use. Currently, the only allowed version is `1.0.0`.
:�
P
appsyncFunctionSyncConfig1aws:appsync/FunctionSyncConfig:FunctionSyncConfig�
�f
conflictDetectionB" KConflict Detection strategy to use. Valid values are `NONE` and `VERSION`.
�
conflictHandlerB" �Conflict Resolution strategy to perform in the event of a conflict. Valid values are `NONE`, `OPTIMISTIC_CONCURRENCY`, `AUTOMERGE`, and `LAMBDA`.
�
lambdaConflictHandlerConfig�B�:�
�
appsync-FunctionSyncConfigLambdaConflictHandlerConfiggaws:appsync/FunctionSyncConfigLambdaConflictHandlerConfig:FunctionSyncConfigLambdaConflictHandlerConfig�Lambda Conflict Handler Config when configuring `LAMBDA` as the Conflict Handler. See `lambda_conflict_handler_config` Block for details.
:�
�
appsync-FunctionSyncConfigLambdaConflictHandlerConfiggaws:appsync/FunctionSyncConfigLambdaConflictHandlerConfig:FunctionSyncConfigLambdaConflictHandlerConfigb
`^
lambdaConflictHandlerArnB" <ARN for the Lambda function to use as the Conflict Handler.
:�

�
appsync*GraphQLApiAdditionalAuthenticationProvideraaws:appsync/GraphQLApiAdditionalAuthenticationProvider:GraphQLApiAdditionalAuthenticationProvider�	
�	�
authenticationType" uAuthentication type. Valid values: `API_KEY`, `AWS_IAM`, `AMAZON_COGNITO_USER_POOLS`, `OPENID_CONNECT`, `AWS_LAMBDA`
�
lambdaAuthorizerConfig�B�:�
�
appsync@GraphQLApiAdditionalAuthenticationProviderLambdaAuthorizerConfig�aws:appsync/GraphQLApiAdditionalAuthenticationProviderLambdaAuthorizerConfig:GraphQLApiAdditionalAuthenticationProviderLambdaAuthorizerConfignNested argument containing Lambda authorizer configuration. See `lambda_authorizer_config` Block for details.
�
openidConnectConfig�B�:�
�
appsync=GraphQLApiAdditionalAuthenticationProviderOpenidConnectConfig�aws:appsync/GraphQLApiAdditionalAuthenticationProviderOpenidConnectConfig:GraphQLApiAdditionalAuthenticationProviderOpenidConnectConfighNested argument containing OpenID Connect configuration. See `openid_connect_config` Block for details.
�
userPoolConfig�B�:�
�
appsync8GraphQLApiAdditionalAuthenticationProviderUserPoolConfig}aws:appsync/GraphQLApiAdditionalAuthenticationProviderUserPoolConfig:GraphQLApiAdditionalAuthenticationProviderUserPoolConfigRAmazon Cognito User Pool configuration. See `user_pool_config` Block for details.
:�
�
appsync@GraphQLApiAdditionalAuthenticationProviderLambdaAuthorizerConfig�aws:appsync/GraphQLApiAdditionalAuthenticationProviderLambdaAuthorizerConfig:GraphQLApiAdditionalAuthenticationProviderLambdaAuthorizerConfig�
��
authorizerResultTtlInSecondsB �Number of seconds a response should be cached for. The default is 5 minutes (300 seconds). The Lambda function can override this by returning a `ttlOverride` key in its response. A value of 0 disables caching of responses. Minimum value of 0. Maximum value of 3600.
�
authorizerUri" �ARN of the Lambda function to be called for authorization. Note: This Lambda function must have a resource-based policy assigned to it, to allow `lambda:InvokeFunction` from service principal `appsync.amazonaws.com`.
x
identityValidationExpressionB" RRegular expression for validation of tokens before the Lambda function is called.
:�
�
appsync=GraphQLApiAdditionalAuthenticationProviderOpenidConnectConfig�aws:appsync/GraphQLApiAdditionalAuthenticationProviderOpenidConnectConfig:GraphQLApiAdditionalAuthenticationProviderOpenidConnectConfig�
�T
authTtlB CNumber of milliseconds a token is valid after being authenticated.
�
clientIdB" �Client identifier of the Relying party at the OpenID identity provider. This identifier is typically obtained when the Relying party is registered with the OpenID identity provider. You can specify a regular expression so the AWS AppSync can validate against multiple client identifiers at a time.
V
iatTtlB FNumber of milliseconds a token is valid after being issued to a user.
�
issuer" �Issuer for the OpenID Connect configuration. The issuer returned by discovery MUST exactly match the value of iss in the ID Token.
:�
�
appsync8GraphQLApiAdditionalAuthenticationProviderUserPoolConfig}aws:appsync/GraphQLApiAdditionalAuthenticationProviderUserPoolConfig:GraphQLApiAdditionalAuthenticationProviderUserPoolConfig�
�q
appIdClientRegexB" WRegular expression for validating the incoming Amazon Cognito User Pool app client ID.
B
	awsRegionB" /AWS region in which the user pool was created.
 

userPoolId" User pool ID.
:�
w
appsyncGraphQLApiEnhancedMetricsConfigKaws:appsync/GraphQLApiEnhancedMetricsConfig:GraphQLApiEnhancedMetricsConfig�
��
dataSourceLevelMetricsBehavior" �How data source metrics will be emitted to CloudWatch. Valid values: `FULL_REQUEST_DATA_SOURCE_METRICS`, `PER_DATA_SOURCE_METRICS`
|
operationLevelMetricsConfig" YHow operation metrics will be emitted to CloudWatch. Valid values: `ENABLED`, `DISABLED`
�
resolverLevelMetricsBehavior" zHow resolver metrics will be emitted to CloudWatch. Valid values: `FULL_REQUEST_RESOLVER_METRICS`, `PER_RESOLVER_METRICS`
:�
z
appsync GraphQLApiLambdaAuthorizerConfigMaws:appsync/GraphQLApiLambdaAuthorizerConfig:GraphQLApiLambdaAuthorizerConfig�
��
authorizerResultTtlInSecondsB �Number of seconds a response should be cached for. The default is 5 minutes (300 seconds). The Lambda function can override this by returning a `ttlOverride` key in its response. A value of 0 disables caching of responses. Minimum value of 0. Maximum value of 3600.
�
authorizerUri" �ARN of the Lambda function to be called for authorization. Note: This Lambda function must have a resource-based policy assigned to it, to allow `lambda:InvokeFunction` from service principal `appsync.amazonaws.com`.
x
identityValidationExpressionB" RRegular expression for validation of tokens before the Lambda function is called.
:�
S
appsyncGraphQLApiLogConfig3aws:appsync/GraphQLApiLogConfig:GraphQLApiLogConfig�
��
cloudwatchLogsRoleArn" |Amazon Resource Name of the service role that AWS AppSync will assume to publish to Amazon CloudWatch logs in your account.
�
excludeVerboseContentB
 �Set to TRUE to exclude sections that contain information such as headers, context, and evaluated mapping templates, regardless of logging  level. Valid values: `true`, `false`. Default value: `false`
P
fieldLogLevel" ;Field logging level. Valid values: `ALL`, `ERROR`, `NONE`.
:�
q
appsyncGraphQLApiOpenidConnectConfigGaws:appsync/GraphQLApiOpenidConnectConfig:GraphQLApiOpenidConnectConfig�
�T
authTtlB CNumber of milliseconds a token is valid after being authenticated.
�
clientIdB" �Client identifier of the Relying party at the OpenID identity provider. This identifier is typically obtained when the Relying party is registered with the OpenID identity provider. You can specify a regular expression so the AWS AppSync can validate against multiple client identifiers at a time.
V
iatTtlB FNumber of milliseconds a token is valid after being issued to a user.
�
issuer" �Issuer for the OpenID Connect configuration. The issuer returned by discovery MUST exactly match the value of iss in the ID Token.
:�
b
appsyncGraphQLApiUserPoolConfig=aws:appsync/GraphQLApiUserPoolConfig:GraphQLApiUserPoolConfig�
�q
appIdClientRegexB" WRegular expression for validating the incoming Amazon Cognito User Pool app client ID.
B
	awsRegionB" /AWS region in which the user pool was created.
�
defaultAction" �Action that you want your GraphQL API to take when a request that uses Amazon Cognito User Pool authentication doesn't match the Amazon Cognito User Pool configuration. Valid: `ALLOW` and `DENY`
 

userPoolId" User pool ID.
:�
Y
appsyncResolverCachingConfig7aws:appsync/ResolverCachingConfig:ResolverCachingConfig�
��
cachingKeysB*" �The caching keys for a resolver that has caching activated. Valid values are entries from the $context.arguments, $context.source, and $context.identity maps.
|
ttlB oThe TTL in seconds for a resolver that has caching activated. Valid values are between `1` and `3600` seconds.
:�
\
appsyncResolverPipelineConfig9aws:appsync/ResolverPipelineConfig:ResolverPipelineConfig5
31
	functionsB*" A list of Function objects.
:�
G
appsyncResolverRuntime+aws:appsync/ResolverRuntime:ResolverRuntime�
�_
name" SThe name of the runtime to use. Currently, the only allowed value is `APPSYNC_JS`.
i
runtimeVersion" SThe version of the runtime to use. Currently, the only allowed version is `1.0.0`.
:�
P
appsyncResolverSyncConfig1aws:appsync/ResolverSyncConfig:ResolverSyncConfig�
�f
conflictDetectionB" KConflict Detection strategy to use. Valid values are `NONE` and `VERSION`.
�
conflictHandlerB" �Conflict Resolution strategy to perform in the event of a conflict. Valid values are `NONE`, `OPTIMISTIC_CONCURRENCY`, `AUTOMERGE`, and `LAMBDA`.
�
lambdaConflictHandlerConfig�B�:�
�
appsync-ResolverSyncConfigLambdaConflictHandlerConfiggaws:appsync/ResolverSyncConfigLambdaConflictHandlerConfig:ResolverSyncConfigLambdaConflictHandlerConfigvLambda Conflict Handler Config when configuring `LAMBDA` as the Conflict Handler. See Lambda Conflict Handler Config.
:�
�
appsync-ResolverSyncConfigLambdaConflictHandlerConfiggaws:appsync/ResolverSyncConfigLambdaConflictHandlerConfig:ResolverSyncConfigLambdaConflictHandlerConfigb
`^
lambdaConflictHandlerArnB" <ARN for the Lambda function to use as the Conflict Handler.
:�
�
appsync.SourceApiAssociationSourceApiAssociationConfigiaws:appsync/SourceApiAssociationSourceApiAssociationConfig:SourceApiAssociationSourceApiAssociationConfigL
JH
	mergeType" 7Merge type. Valid values: `MANUAL_MERGE`, `AUTO_MERGE`
:�
n
appsyncSourceApiAssociationTimeoutsEaws:appsync/SourceApiAssociationTimeouts:SourceApiAssociationTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
