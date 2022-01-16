terraform {
  required_version = "=1.0.11"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~>3.64.0"
    }
  }

#   backend "s3" {
#     bucket = "terraform-state-bucket"
#     key    = "state/terraform_state.tfstate"
#     region = "us-east-1"
#   }
}

provider "aws" {
  region     = var.aws_region
}