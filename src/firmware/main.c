#include <avr/io.h>
#include <util/delay.h>
#include "common.c"
#include "serial.c"

#define LED_PIN PD5
#define NUM_LEDS 120

#define INIT_STEP_DELTA 3;
#define STEP_DELTA 2;

rgb leds[NUM_LEDS];
hsv hsv_leds[NUM_LEDS];

void read_rgb_array(uint8_t* buffer, uint16_t buf_size, rgb* colors, uint16_t rgb_size)
{
	uint16_t rgb_i = 0;
	for (uint16_t i = 0; (i + 2) < buf_size && rgb_i < rgb_size; i += 3)
	{
		colors[rgb_i].r = buffer[i];
		colors[rgb_i].g = buffer[i + 1];
		colors[rgb_i].b = buffer[i + 2];
		rgb_i++;
	}
}

void serial_impl()
{
	init_uart();
	for (;;)
	{
		int32_t value = read_serial_int32_t();
		if (value > NUM_LEDS * 3)
		{
			write_serial_buffer("err: too large\n", 15);
			continue;
		}
		uint8_t buffer[value];
		read_serial_buffer(&buffer, value);
		read_rgb_array(&buffer, value, &leds, NUM_LEDS);
		send_rgb_array(&leds, NUM_LEDS, LED_PIN);
		write_serial_buffer("ok\n", 3);
	}
}

void stand_alone_impl()
{
	hsv base = { .h = 0, .s = 1, .v = 0.02 };

	for (uint8_t i = 0; i < NUM_LEDS; i++)
	{
		base.h += INIT_STEP_DELTA;
		base.h %= 360;
		hsv_leds[i] = base;
	}

	while (1)
	{
		for (uint8_t i = 0; i < NUM_LEDS; i++)
		{
			hsv* color = &hsv_leds[i];
			color->h += STEP_DELTA;
			color->h %= 360;
			leds[i] = hsv_to_rgb(*color);
		}
		send_rgb_array(leds, NUM_LEDS, LED_PIN);

		// another example with stand alone leds
		// base.h += 1;
		// base.h %= 360;
		// rgb color = hsv_to_rgb(hsv);
		// send_rgb(color, LED_PIN);
		// send_rgb(color, LED_PIN);
		// send_rgb(color, LED_PIN);

		_delay_ms(10);
	}
}

void constant_impl()
{
	rgb rgb = { .r = 10, .g = 10, .b = 10 };
	for (int i = 0; i < NUM_LEDS; i++) {
		leds[i] = rgb;
	}
	while (1)
	{
		send_rgb_array(leds, NUM_LEDS, LED_PIN);
		_delay_ms(1000);
	}
	
}

int main()
{
	DDRB |= 1 << LED_PIN;

	stand_alone_impl();

	return 0;
}
