#ifndef SERIAL_LOG_H
#define SERIAL_LOG_H

#include <Arduino.h>

class SerialLog
{
public:
    enum SerialLogLevel
    {
        DEFAULT_LEVEL,
        DEBUG
    };

    SerialLog(const SerialLogLevel level = DEFAULT_LEVEL, const unsigned long baud = 9600);

    template <typename T>
    void log(const T &message, const SerialLogLevel level = DEFAULT_LEVEL)
    {
        if (this->logLevel == DEFAULT_LEVEL)
        {
            if (level == DEFAULT_LEVEL)
            {
                Serial.print(message);
            }
        }
        else if (this->logLevel == DEBUG)
        {
            Serial.print(message);
        }
    }

private:
    unsigned long baudRate;
    SerialLogLevel logLevel;
};

#endif
