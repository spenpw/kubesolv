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
    targets = ["operator", "default-solvers"]
}

group "default-solvers" {
    targets = [
        "nissy-optimal",
        "nissy-light",
        "cfop-exploration"
    ]
}

target "operator" {
    context = "operator"
    dockerfile = "Dockerfile"
    tags = [
        "${IMAGE_PREFIX}operator:${IMAGE_TAG}"
    ]
}

target "nissy-optimal" {
    context = "./solvers/nissy"
    target = "nissy-optimal"
    tags = [
        "${IMAGE_PREFIX}nissy-optimal:${IMAGE_TAG}"
    ]
    contexts = {
        solvers-rs = "./solvers/solvers-rs"
    }
}

target "nissy-light" {
    context = "./solvers/nissy"
    target = "nissy-light"
    tags = [
        "${IMAGE_PREFIX}nissy-light:${IMAGE_TAG}"
    ]
    contexts = {
        solvers-rs = "./solvers/solvers-rs"
    }
}

target "cfop-exploration" {
    context = "./solvers/cfop"
    target = "cfop-exploration"
    tags = [
        "${IMAGE_PREFIX}cfop-exploration:${IMAGE_TAG}"
    ]
    contexts = {
        solvers-rs = "./solvers/solvers-rs"
    }
}
