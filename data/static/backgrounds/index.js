const images = [
  "Background1.png",
  "Background13.png",
  "Background17.png",
  "Background20.png",
  "Background6.png",
  "Background10.png",
  "Background14.png",
  "Background18.png",
  "Background3.png",
  "Background7.png",
  "Background11.png",
  "Background15.png",
  "Background19.png",
  "Background4.png",
  "Background8.png",
  "Background12.png",
  "Background16.png",
  "Background2.png",
  "Background5.png",
  "Background9.png",
];

let working = "";

images.forEach((i) => {
  working += `<img class="img" src="/backgrounds/Images/${i}"></img>`;
});

document.querySelector("#images").innerHTML = working;

// var cache = new Array(images.length);
//
// let nextimage = -1;
// slideshow();
//
// function slideshow() {
//   nextimage++;
//   if (nextimage >= images.length) nextimage = 0;
//
//   if (cache[nextimage] === undefined) {
//     fetch(`/backgrounds/Images/${images[nextimage]}`).then((r) => {
//       r.blob()
//         .then((r) => r.arrayBuffer())
//         .then((r) => {
//           let base64Image = arrayBufferToBase64(r);
//           cache[nextimage] = `data:image/png;base64,${base64Image}`;
//
//           document.querySelector(
//             "body"
//           ).style.backgroundImage = `url(${cache[nextimage]})`;
//           setTimeout(slideshow, 4000);
//         });
//     });
//   }
//
// }
//
// function arrayBufferToBase64(buffer) {
//   let binary = "";
//   let bytes = new Uint8Array(buffer);
//   for (var i = 0; i < bytes.byteLength; i++)
//     binary += String.fromCharCode(bytes[i]);
//   return window.btoa(binary);
// }
