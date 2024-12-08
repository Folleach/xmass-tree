#include <avr/io.h>
#include <util/delay.h>

#define FASTLED_SLOW_CLOCK_ADJUST asm __volatile__ ("mov r0,r0\n\t");

typedef struct rgb
{
	uint8_t r;
	uint8_t g;
	uint8_t b;
} rgb;

typedef struct hsv
{
	uint16_t h;
	float s;
	float v;
} hsv;

// the function was taken from https://github.com/Inseckto/HSV-to-RGB
rgb hsv_to_rgb(hsv hsv)
{
	float r, g, b;

	float h = (float)hsv.h / 360;
	float s = (float)hsv.s;
	float v = (float)hsv.v;

	int i = floor(h * 6);
	float f = h * 6 - i;
	float p = v * (1 - s);
	float q = v * (1 - f * s);
	float t = v * (1 - (1 - f) * s);

	switch (i % 6) {
		case 0: r = v, g = t, b = p; break;
		case 1: r = q, g = v, b = p; break;
		case 2: r = p, g = v, b = t; break;
		case 3: r = p, g = q, b = v; break;
		case 4: r = t, g = p, b = v; break;
		case 5: r = v, g = p, b = q; break;
	}

	rgb color;
	color.r = r * 255;
	color.g = g * 255;
	color.b = b * 255;

	return color;
}

// idk what i'm doing...
void send_raw(uint32_t value, uint8_t pin)
{
	for (uint8_t i = 0; i < 24; i++)
	{
		if ((value >> (23 - i)) & 0x01)
		{
			PORTB |= (1 << pin);
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			PORTB &= ~(1 << pin);
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
		}
		else
		{
			PORTB |= (1 << pin);
			FASTLED_SLOW_CLOCK_ADJUST
			PORTB &= ~(1 << pin);
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
			FASTLED_SLOW_CLOCK_ADJUST
		}
	}
}

void send_rgb(rgb color, uint8_t pin)
{
	send_raw((uint64_t)color.g << 16 | (uint64_t)color.r << 8 | (uint64_t)color.b, pin);
}

void send_rgb_array(const rgb* array, uint8_t size, uint8_t pin)
{
	for (uint8_t i = 0; i < size; i++)
		send_rgb(array[i], pin);
}
