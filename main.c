#include <avr/io.h>
#include <util/delay.h>
#include "common.c"

#define LED_PIN PD5
#define NUM_LEDS 10

#define INIT_STEP_DELTA 15;
#define STEP_DELTA 1;

rgb leds[NUM_LEDS];
hsv hsv_leds[NUM_LEDS];

int main()
{
	DDRB |= 1 << LED_PIN;
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
	return 0;
}
