#!/bin/bash

docker run \
    -v ./data:/enso-darknet/data \
    -v ./media:/enso-darknet/media \
    enso-darknet enso-darknet \
    --clip-weights ./data/clip_v2.1.ot \
    --vae-weights ./data/vae_v2.1.ot \
    --unet-weights ./data/unet_v2.1.ot \
    --vocab-file ./data/vocab_v2.1.txt