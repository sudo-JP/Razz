PROJECT_NAME = RazzTFTRender
SKETCH := ../$(PROJECT_NAME)
BOARD = arduino:avr:uno
PORT = /dev/ttyUSB0
BUILD_DIR = build


all: clean build upload work_name

build: build_name
	arduino-cli compile \
		--fqbn $(BOARD) \
		--build-path $(BUILD_DIR) \
		$(SKETCH)

build_name: 
	mv ./$(PROJECT_NAME).cpp ./$(PROJECT_NAME).ino

work_name: 
	mv ./$(PROJECT_NAME).ino ./$(PROJECT_NAME).cpp

upload:
	arduino-cli upload \
		--fqbn $(BOARD) \
		-p $(PORT) \
		--input-dir $(BUILD_DIR)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all build upload clean
