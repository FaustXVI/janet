var mqtt = require('mqtt');
let request = require('request');
var hostname = "mqtt://janet";
var client  = mqtt.connect(hostname);
client.on('connect', function () {
    console.log("[Snips Log] Connected to MQTT broker " + hostname);
    client.subscribe('hermes/#');
});
client.on('message', function (topic, message) {
    if (topic === "hermes/asr/startListening") {
        onListeningStateChanged(true);
    } else if (topic === "hermes/asr/stopListening") {
        onListeningStateChanged(false);
    } else if (topic.match(/hermes\/hotword\/.+\/detected/g) !== null) {
        onHotwordDetected()
    } else if (topic.match(/hermes\/intent\/.+/g) !== null) {
        onIntentDetected(JSON.parse(message));
    }
});
function onIntentDetected(intent) {
    let action = intent.intent.intentName;
    if( action === "xadet:turnlightson" ){
        console.log("switch on");
        request.post("http://janet/api/light", {form: {status:"On"}});
    }
    else{
        console.log("switch off");
        request.post("http://janet/api/light", {form: {status:"Off"}});
    }
}
function onHotwordDetected() {
    console.log("[Snips Log] Hotword detected");
}
function onListeningStateChanged(listening) {
    console.log("[Snips Log] " + (listening ? "Start" : "Stop") + " listening");
}
