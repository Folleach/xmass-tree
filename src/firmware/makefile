DEVICE       = atmega328p
CLOCK        = 16000000
AVRDUDE      = avrdude -carduino -P/dev/ttyUSB0 -b115200 -D -p $(DEVICE)
OBJECTS      = bin/main.o
HARDWARE     = -DF_CPU=$(CLOCK) -mmcu=$(DEVICE)
COMPILE_OPTS = -Wall -Os
INCLUDE      = -I/usr/lib/avr/include/

OPTS = $(HARDWARE) $(INCLUDE) $(COMPILE_OPTS)

COMPILE = avr-gcc $(OPTS)
BIN_DIR = bin

all: $(BIN_DIR) $(BIN_DIR)/main.hex

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/%.o: %.c
	$(COMPILE) -c $< -o $@

$(BIN_DIR)/%.o: %.S
	$(COMPILE) -x assembler-with-cpp -c $< -o $@

$(BIN_DIR)/%.s: %.c
	$(COMPILE) -S $< -o $@

flash: all
	$(AVRDUDE) -U flash:w:$(BIN_DIR)/main.hex:i

clean:
	rm -f $(BIN_DIR)/main.hex $(BIN_DIR)/main.elf $(OBJECTS)

$(BIN_DIR)/main.elf: $(OBJECTS)
	$(COMPILE) -o $(BIN_DIR)/main.elf $(OBJECTS)

$(BIN_DIR)/main.hex: $(BIN_DIR)/main.elf
	rm -f $(BIN_DIR)/main.hex
	avr-objcopy -j .text -j .data -O ihex $(BIN_DIR)/main.elf $(BIN_DIR)/main.hex
	avr-size --format=avr --mcu=$(DEVICE) $(BIN_DIR)/main.elf
