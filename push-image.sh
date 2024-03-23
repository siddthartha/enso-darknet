#!/bin/bash

docker push dogen/enso-darknet:latest
docker pull dogen/enso-darknet:latest

docker push dogen/enso-darknet-converter:latest
docker pull dogen/enso-darknet-converter:latest