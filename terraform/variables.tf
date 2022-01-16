variable "app_name" {
  default = "solar-blaster"
}

variable "aws_region" {
  default = "eu-central-1"
}

variable "aws_cloudwatch_retention_in_days" {
  type        = number
  description = "AWS CloudWatch Logs Retention in Days"
  default     = 7
}

variable "docker_image_url" {
  description = "URL of the docker image to be used including tag"
  default = "whatname/solar-blaster:latest"
}

variable "ecs_min_capacity" {
  default = 1
}