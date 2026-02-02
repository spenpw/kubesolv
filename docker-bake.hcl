variable "IMAGE_PREFIX" {
    description = "Prefix for Docker images"
    type        = string
    default     = "ghcr.io/spenpw/"
}

variable "IMAGE_TAG" {
    description = "Tag for Docker images"
    type        = string
    default     = "latest"
}

group "default" {
    targets = ["operator"]
}

target "operator" {
    context = "operator"
    dockerfile = "Dockerfile"
    tags = [
        "${IMAGE_PREFIX}operator:${IMAGE_TAG}"
    ]
}
