#!/bin/bash

set -x

sudo docker compose down
sudo docker compose up --build -d

