terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.8.0"
}

provider "aws" {
  region = "us-east-1"
}

## Config

locals {
  s3_origin_id = "WebS3Origin"
  bucket_name  = "www.spencer.imbleau.com"
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
