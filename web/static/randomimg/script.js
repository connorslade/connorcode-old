document.getElementById("button").addEventListener("click", newImage);

let count = 0;

function newImage() {
  document.getElementById("rand").style.opacity = 0;

  fetch("/randomimg/image.png")
    .then((r) => {
      if (r.status !== 200) {
        reTry();
        return;
      }
      r.blob()
        .then((r) => r.arrayBuffer())
        .then((r) => {
          let base64Image = arrayBufferToBase64(r);
          document.getElementById(
            "rand"
          ).innerHTML = `<img src="data:image/png;base64,${base64Image}"></img>`;
          document.getElementById("rand").style.opacity = 1;
          count = 0;
        });
    })
    .catch(() => reTry());
}

function basic() {
  let x = stringGen(6).toString();
  window.open(`https://prnt.sc/${x}`, '_blank');
}

function reTry() {
  {
    if (count >= 1) {
      basic();
      count--;
      return;
    }

    count++;
    newImage();
  }
}

function stringGen(len) {
  var text = "";
  var charset = "abcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < len; i++)
    text += charset.charAt(Math.floor(Math.random() * charset.length));
  return text;
}

function arrayBufferToBase64(buffer) {
  let binary = "";
  let bytes = new Uint8Array(buffer);
  for (var i = 0; i < bytes.byteLength; i++)
    binary += String.fromCharCode(bytes[i]);
  return window.btoa(binary);
}
