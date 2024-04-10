#!/bin/bash

docker run \
    -v ./data:/enso-darknet/data \
    -v ./media:/enso-darknet/media \
    dogen/enso-darknet ./sd-cli --prompt "Some beautiful girls runnig on the beach" \
    --clip-weights ./data/clip-2.1.ot \
    --vae-weights ./data/vae-2.1.ot \
    --unet-weights ./data/unet-2.1.ot \
    --vocab-file ./data/bpe_simple_vocab_16e6.txt \
    --n-steps 5 \
    --final-image ./media/output.jpg