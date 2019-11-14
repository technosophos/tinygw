const { events } = require("brigadier");

events.on("interval", (e, p) => {
    console.log("Triggered by 'interval' event.")
});