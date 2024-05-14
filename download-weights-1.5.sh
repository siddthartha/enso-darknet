#!/bin/bash

# download vocabulary
wget https://huggingface.co/lmz/rust-stable-diffusion-v1-5/raw/main/weights/bpe_simple_vocab_16e6.txt -O ./data/vocab_16e6.txt

# SD 2.1
wget https://huggingface.co/lmz/rust-stable-diffusion-v1-5/resolve/main/weights/clip_v1.5.ot -O ./data/clip-1.5.ot
wget https://huggingface.co/lmz/rust-stable-diffusion-v1-5/resolve/main/weights/unet_v1.5.ot -O ./data/unet-1.5.ot
wget https://huggingface.co/lmz/rust-stable-diffusion-v1-5/resolve/main/weights/vae_v1.5.ot -O ./data/vae-1.5.ot
