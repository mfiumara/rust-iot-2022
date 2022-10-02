# Readme

This example code does the following:

1. Set up a WiFi connection on the ESP32-C3
2. Spawn a thread using std::thread in which we listen for incoming MQTT events.
3. In the newly spawned thread, handle MQTT events and look specifically for a Received event using match, then print the contents as a string in the console.
4. Back in our main thread, subscribe to a topic and create an infinite loop in which we continuously publish to an MQTT topic, with pauses of 1 second in between.


[Accompanying article](https://mfiumara.medium.com/rust-for-iot-is-it-time-67b14ab34b8)

