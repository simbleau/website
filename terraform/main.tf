terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
  required_version = ">= 1.8.0"

  backend "s3" {
    bucket = "imbleau-terraform-state"
    key    = "terraform.tfstate"
    region = "us-east-1"
  }
}

provider "aws" {
  region     = var.aws_region
  access_key = var.aws_access_key_id
  secret_key = var.aws_secret_access_key
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

## Config

locals {
  s3_origin_id = "WebS3Origin"
  bucket_name  = "www.spencer.imbleau.com"
  zone         = "9aad55f2e0a8d9373badd4361227cabe"
}

## S3

resource "aws_s3_bucket" "web_bucket" {
  bucket = local.bucket_name
}

## SSL Cert

resource "aws_acm_certificate" "tls_cert" {
  domain_name       = "www.spencer.imbleau.com"
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }
}

## Cloudfront

resource "aws_cloudfront_origin_access_control" "web_oac" {
  name                              = "WebsiteAccessControl"
  description                       = "Web OAC Policy"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

resource "aws_cloudfront_distribution" "web_distribution" {

  origin {
    domain_name              = aws_s3_bucket.web_bucket.bucket_regional_domain_name
    origin_access_control_id = aws_cloudfront_origin_access_control.web_oac.id
    origin_id                = local.s3_origin_id
  }

  enabled             = true
  comment             = "PROD - Website Distribution"
  aliases             = ["www.spencer.imbleau.com"]
  default_root_object = "index.html"

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = local.s3_origin_id
    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  custom_error_response {
    error_code         = 404
    response_code      = 200
    response_page_path = "/index.html"
  }

  price_class = "PriceClass_All"

  restrictions {
    geo_restriction {
      restriction_type = "none"
      locations        = []
    }
  }

  viewer_certificate {
    acm_certificate_arn            = "arn:aws:acm:us-east-1:804184581672:certificate/924d54f4-5e73-4e87-a560-8e285ff119c4"
    ssl_support_method             = "sni-only"
    minimum_protocol_version       = "TLSv1"
    cloudfront_default_certificate = false
  }
}

## Bucket policy

resource "aws_s3_bucket_policy" "web_access_policy" {
  bucket = local.bucket_name
  policy = data.aws_iam_policy_document.view_objects_policy.json
}

data "aws_iam_policy_document" "view_objects_policy" {
  statement {
    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }
    actions = [
      "s3:GetObject",
      "s3:ListBucket",
    ]
    resources = [
      aws_s3_bucket.web_bucket.arn,
      "${aws_s3_bucket.web_bucket.arn}/*",
    ]
    condition {
      test     = "ForAnyValue:StringEquals"
      variable = "AWS:SourceArn"
      values   = [aws_cloudfront_distribution.web_distribution.arn]
    }

  }
}

## Cloudfront

resource "cloudflare_record" "tls_dns_validation" {
  zone_id = local.zone
  comment = "ACM Verification for ${aws_acm_certificate.tls_cert.domain_name}"
  name    = tolist(aws_acm_certificate.tls_cert.domain_validation_options)[0].resource_record_name
  value   = tolist(aws_acm_certificate.tls_cert.domain_validation_options)[0].resource_record_value
  type    = tolist(aws_acm_certificate.tls_cert.domain_validation_options)[0].resource_record_type
  proxied = "false"
}

resource "cloudflare_record" "web_distribution_naked" {
  zone_id = local.zone
  name    = "@"
  value   = "192.0.2.1"
  type    = "A"
  proxied = "true"
}

resource "cloudflare_record" "web_distribution_www" {
  zone_id = local.zone
  name    = "www"
  value   = "192.0.2.1"
  type    = "A"
  proxied = "true"
}

resource "cloudflare_record" "web_distribution_naked_spencer" {
  zone_id = local.zone
  name    = "spencer"
  value   = "192.0.2.1"
  type    = "A"
  proxied = "true"
}

resource "cloudflare_record" "web_distribution_cn" {
  zone_id = local.zone
  name    = "www.spencer"
  value   = aws_cloudfront_distribution.web_distribution.domain_name
  type    = "CNAME"
  proxied = "false"
}

resource "cloudflare_record" "web_distribution_naked_aws" {
  zone_id = local.zone
  name    = "aws"
  value   = "192.0.2.1"
  type    = "A"
  proxied = "true"
}

resource "cloudflare_record" "web_distribution_aws" {
  zone_id = local.zone
  name    = "www.aws"
  value   = "192.0.2.1"
  type    = "A"
  proxied = "true"
}

resource "cloudflare_ruleset" "redirect_rules" {
  zone_id = local.zone
  kind    = "zone"
  phase   = "http_request_dynamic_redirect"
  name    = "Redirect rules"

  rules {
    description = "Redirect Non-CNAME"
    action      = "redirect"
    expression  = "(http.host eq \"imbleau.com\") or (http.host eq \"spencer.imbleau.com\") or (http.host eq \"www.imbleau.com\")"
    action_parameters {
      from_value {
        status_code = 301
        target_url {
          expression = "concat(\"https://www.spencer.imbleau.com\", http.request.uri.path)"
        }
        preserve_query_string = true
      }
    }
    enabled = true
  }

  rules {
    description = "Redirect AWS"
    action      = "redirect"
    expression  = "(http.host eq \"aws.imbleau.com\") or (http.host eq \"www.aws.imbleau.com\")"
    action_parameters {
      from_value {
        status_code = 301
        target_url {
          value = "https://804184581672.signin.aws.amazon.com/console"
        }
        preserve_query_string = false
      }
    }
    enabled = true
  }
}

