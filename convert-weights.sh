#!/bin/bash

#tensor-tool cp ./data/clip-1.5.bin clip-1.5.safetensors
#tensor-tool cp ./data/vae-1.5.bin vae-1.5.safetensors
#tensor-tool cp ./data/unet-1.5.bin unet-1.5.safetensors

tensor-tool cp ./data/clip-2.1.npz ./data/clip-2.1.ot
tensor-tool cp ./data/vae-2.1.npz ./data/vae-2.1.ot
tensor-tool cp ./data/unet-2.1.npz ./data/unet-2.1.ot
