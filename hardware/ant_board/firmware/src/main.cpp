#include <Arduino.h>
#include <SerialLog.h>
#include <HX711.h>
#include <MQTT.h>

#ifdef ARDUINO_ARCH_ESP8266
#include <ESP8266WiFi.h>
#include <ESP8266mDNS.h>
#elif defined(ARDUINO_ARCH_ESP32)
#include <WiFi.h>
#include <ESPmDNS.h>
#endif

//// SERVICE INSTANTIATION ////
SerialLog logger(SerialLog::DEBUG);
HX711 loadcell;

//// PIN DEFINITIONS (keep boot pins free) ////
#ifdef ARDUINO_ARCH_ESP32
const int LOADCELL_DOUT_PIN = 5; // D1
const int LOADCELL_SCK_PIN = 4;  // D2
#elif defined(ARDUINO_ARCH_ESP8266)
const int LOADCELL_DOUT_PIN = D2; // safe for ESP8266
const int LOADCELL_SCK_PIN = D1;  // safe for ESP8266
#endif

//// CONSTANTS ////
const long LOADCELL_OFFSET = 50682624;
const long LOADCELL_DIVIDER = 10;

//// WiFi ////
const char *WIFI_SSID = "SSID";
const char *WIFI_PASSWORD = "password";
WiFiClient wifiClient;

//// MQTT ////
const char *HOST_URL = "broker.local";
const char *MQTT_TOPIC_STATUS = "sensor/status";
const char *MQTT_TOPIC_MEASUREMENT = "sensor/measurement";
const int MQTT_PORT = 1883;
MQTTClient mqtt;

void setup()
{
  delay(2000);

  // initialize loadcell
  logger.log("\nInitialize HX711 library", SerialLog::DEBUG);
  loadcell.begin(LOADCELL_DOUT_PIN, LOADCELL_SCK_PIN);
  loadcell.set_offset(loadcell.read_average(10)); // tare empty scale
  loadcell.set_scale(100);                        // ca grams

  // connect to WiFi
  logger.log("\nConnecting to WiFi...", SerialLog::DEBUG);
  WiFi.mode(WIFI_STA);
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  while (WiFi.status() != WL_CONNECTED)
  {
    delay(1000);
    logger.log(".", SerialLog::DEBUG);
  }
  logger.log("\nWiFi connected!", SerialLog::DEBUG);
  logger.log("\nIP address: ", SerialLog::DEBUG);
  logger.log(WiFi.localIP().toString(), SerialLog::DEBUG);

  // mDNS setup
#ifdef ARDUINO_ARCH_ESP32
  if (MDNS.begin("esp32"))
  {
#elif defined(ARDUINO_ARCH_ESP8266)
  if (MDNS.begin("esp8266"))
  {
#endif
    logger.log("mDNS responder started", SerialLog::DEBUG);
  }
  else
  {
    logger.log("Error setting up mDNS responder!", SerialLog::DEBUG);
  }

  // resolve broker hostname
  IPAddress brokerIP;
  if (WiFi.hostByName(HOST_URL, brokerIP))
  {
    logger.log("Broker IP: " + brokerIP.toString(), SerialLog::DEBUG);
  }
  else
  {
    logger.log("Failed to resolve broker.local", SerialLog::DEBUG);
  }

  // initialize mqtt
  logger.log("\nConnecting to MQTT...", SerialLog::DEBUG);
  mqtt.begin(brokerIP, MQTT_PORT, wifiClient);
  mqtt.connect("arduino", "public", "public");

  // Wait for MQTT connection
  int retry_count = 0;
  while (!mqtt.connected() && retry_count < 10)
  {
    mqtt.connect("arduino", "public", "public");
    retry_count++;
    logger.log(".", SerialLog::DEBUG);
  }

  if (mqtt.connected())
  {
    logger.log("\nMQTT connected!", SerialLog::DEBUG);
  }
  else
  {
    logger.log("\nMQTT connection failed!", SerialLog::DEBUG);
  }

  logger.log("\nSetup finished\n");
}

void loop()
{
  logger.log("\nin main loop");
  mqtt.loop();

  // Check connection status
  if (!mqtt.connected())
  {
    logger.log("\nMQTT disconnected! Reconnecting...", SerialLog::DEBUG);
    mqtt.connect("arduino", "public", "public");
    delay(1000);
    return;
  }

  bool published = mqtt.publish(MQTT_TOPIC_STATUS, "idle");
  if (published)
  {
    logger.log("\nMQTT status sent successfully", SerialLog::DEBUG);
  }
  else
  {
    logger.log("\nMQTT status send FAILED", SerialLog::DEBUG);
  }

  if (loadcell.wait_ready_timeout(1000))
  {
    int measurement_no = 0;
    mqtt.publish(MQTT_TOPIC_STATUS, "measuring");
    while (measurement_no <= 1000)
    {
      measurement_no += 1;
      long reading = loadcell.get_units(3);
      mqtt.publish(MQTT_TOPIC_MEASUREMENT, String(reading).c_str());
      if (measurement_no % 10 == 0)
      {
        logger.log("\nWeight: " + String(reading), SerialLog::DEBUG);
      }
      delay(1);
    }
  }
  else
  {
    Serial.println("HX711 not found.");
  }

  mqtt.publish(MQTT_TOPIC_STATUS, "idle");
  delay(3000);
}
