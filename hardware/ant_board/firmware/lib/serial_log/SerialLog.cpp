#include "SerialLog.h"

SerialLog::SerialLog(const SerialLogLevel level, const unsigned long baud)
    : baudRate(baud), logLevel(level)
{
    Serial.begin(baudRate);
}
