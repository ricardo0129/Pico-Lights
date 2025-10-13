build:
	cmake -S . -B build/pico2_w -DPICO_BOARD=pico2_w && cmake --build build/pico2_w
	cmake -S . -B build/pico_w -DPICO_BOARD=pico_w && cmake --build build/pico_w

upload:
	cmake -S . -B build/pico2_w -DPICO_BOARD=pico2_w && cmake --build build/pico2_w
	

