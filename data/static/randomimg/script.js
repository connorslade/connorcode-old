document.getElementById("button").addEventListener("click", newImage);

function newImage() {
  document.getElementById("rand").style.opacity = 0;

  fetch("/randomimg/image.png")
    .then((r) => {
      if (r.status !== 200) {
        newImage();
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
        });
    })
    .catch(() => newImage());
}

function arrayBufferToBase64(buffer) {
  let binary = "";
  let bytes = new Uint8Array(buffer);
  for (var i = 0; i < bytes.byteLength; i++)
    binary += String.fromCharCode(bytes[i]);
  return window.btoa(binary);
}
