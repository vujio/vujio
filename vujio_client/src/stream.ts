let ctx = new AudioContext();
let bufferSource = ctx.createBufferSource();

let connected = false;

// Use an object to make it harder for
// memory leaks to exist by accident.
let socket = [];

const createSocket = () => {
  let loc = window.location;
  const websocket = (socket[0] = new WebSocket(
    `ws${loc.protocol === "https:" ? "s" : ""}://${loc.host}${loc.pathname}ws`
  ));

  websocket.onopen = function () {
    console.log("connected");
    connected = true;
  };

  websocket.onclose = function () {
    console.log("disconnected");
    connected = false;
    setTimeout(createSocket, 1000);
  };

  websocket.onmessage = (message) => {
    message.data.arrayBuffer().then((buffer) => {
      ctx.decodeAudioData(buffer).then((audioData) => {
        console.log("decoded audio");
      });
    });
  };
};

navigator.mediaDevices.getUserMedia({ audio: true }).then((stream) => {
  const mediaRecorder = new MediaRecorder(stream, {
    audioBitsPerSecond: 48000,
    mimeType: "audio/webm;codecs=opus",
  });

  setInterval(() => {
    mediaRecorder.requestData();
  }, 250);

  mediaRecorder.ondataavailable = handleDataAvailable;
  mediaRecorder.start();

  function handleDataAvailable(event) {
    if (event.data.size !== 0) {
      if (connected) {
        socket[0].send(event.data);
      }
    }
  }
});

createSocket();
