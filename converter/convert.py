import numpy as np
import torch
model = torch.load("./data/clip-2.1.bin")
np.savez("./data/clip-2.1.npz", **{k: v.numpy() for k, v in model.items() if "text_model" in k})

model = torch.load("./data/unet-2.1.bin")
np.savez("./data/unet-2.1.npz", **{k: v.numpy() for k, v in model.items() if "text_model" in k})

model = torch.load("./data/vae-2.1.bin")
np.savez("./data/vae-2.1.npz", **{k: v.numpy() for k, v in model.items() if "text_model" in k})
