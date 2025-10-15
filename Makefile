update:
	cmake -S . -B build/$(board) -DPICO_BOARD=pico2_w && cmake --build build/$(board)
	sudo /home/ricky/Desktop/embedded/picotool/build/picotool load build/$(board)/pico_lights.uf2
	sudo /home/ricky/Desktop/embedded/picotool/build/picotool reboot
debug:
	cmake -S . -B build/$(board) -DPICO_BOARD=pico2_w -DCMAKE_BUILD_TYPE=Debug && cmake --build build/$(board)
	sudo /home/ricky/Desktop/embedded/picotool/build/picotool load build/$(board)/pico_lights.elf
	sudo /home/ricky/Desktop/embedded/picotool/build/picotool reboot
