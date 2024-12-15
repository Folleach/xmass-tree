#include <avr/io.h>

void init_uart()
{
	UBRR0L = 103;
	UCSR0B = (1 << TXEN0) | (1 << RXEN0);
	UCSR0C = (1 << UCSZ01) | (3 << UCSZ00);
}

void write_serial_buffer(uint8_t* buffer, uint16_t size)
{
	for(int i = 0; i < size; i++)
	{
		while(!(UCSR0A & (1 << UDRE0)))
		{
		}
		UDR0 = buffer[i];
	}
}

int32_t read_serial_int32_t()
{
	uint8_t buf[4];
	read_serial_buffer(&buf, 4);
	return (int32_t)buf[0] << 24
			| (int32_t)buf[1] << 16
			| (int32_t)buf[2] << 8
			| (int32_t)buf[3] << 0;
}

void read_serial_buffer(uint8_t* buffer, uint16_t size)
{
	for (uint16_t i = 0; i < size; i++)
	{
		while (!(UCSR0A & (1 << RXC0)))
		{
		}
		buffer[i] = UDR0;
	}
}
