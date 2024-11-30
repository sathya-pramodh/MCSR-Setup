run:
	rm ~/.config/mcsr-setup/config.json
	VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.x86_64.json VK_LAYER_PATH=/usr/share/vulkan/explicit_layer.d cargo run
