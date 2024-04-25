variable "cloudflare_api_token" {
  type        = string
  description = "The Cloudflare API token with access for the imbleau.com zone"
}

variable "aws_region" {
  type        = string
  description = "The AWS region"
}

variable "aws_access_key_id" {
  type        = string
  description = "The AWS Access Key ID"
}

variable "aws_secret_access_key" {
  type        = string
  description = "The AWS Secret Access Key"
}
