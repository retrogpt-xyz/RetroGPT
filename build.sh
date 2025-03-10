#!/usr/bin/env bash

# Default values
profile=""
detached_flag=""

# A simple usage function
usage() {
  echo "Usage: $0 [ -p <dev|prod> ] [ -d ]"
  echo "Options:"
  echo "  -p, --profile     Specify 'dev' or 'prod' profile."
  echo "  -d, --detached    Run containers in detached mode."
}

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    -p|--profile)
      profile="$2"
      shift 2
      ;;
    -d|--detached)
      detached_flag="-d"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      usage
      exit 1
      ;;
  esac
done

# Make sure 'profile' was specified
if [[ -z "$profile" ]]; then
  profile="dev"  # Set default profile to 'dev'
fi

# Validate 'profile' value
if [[ "$profile" != "dev" && "$profile" != "prod" ]]; then
  echo "Error: Profile must be 'dev' or 'prod'."
  usage
  exit 1
fi

# Execute the docker compose command
# echo "Running: docker compose --profile $profile up --build --remove-orphans $detached_flag"
set -xe
COMPOSE_BAKE=true docker compose --profile "$profile" up --build --remove-orphans $detached_flag

