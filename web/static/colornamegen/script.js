// By Connor Slade!!
// looking back at this code like a year later...
// wow... this um is really somthing

const color = [
  "Red",
  "Orange",
  "Yellow",
  "Green",
  "Blue",
  "Purple",
  "Pink",
  "Brown",
  "Black",
  "Grey",
  "White",
];

function getRandomInt(min, max) {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

function Hcolor(color) {
  if (color === "Red") return getRandomInt(0, 10);
  if (color === "Orange") return getRandomInt(10, 30);
  if (color === "Yellow") return getRandomInt(30, 70);
  if (color === "Green") return getRandomInt(70, 160);
  if (color === "Blue") return getRandomInt(160, 250);
  if (color === "Purple") return getRandomInt(250, 280);
  if (color === "Pink") return getRandomInt(280, 340);
  if (color === "White") return getRandomInt(0, 359);
  if (color === "Brown") return getRandomInt(0, 359);
  if (color === "Black") return getRandomInt(0, 359);
  if (color === "Grey") return getRandomInt(0, 359);
}

function Scolor(color) {
  if (color === "White") return getRandomInt(0, 10);
  if (color === "Grey") return getRandomInt(0, 50);
  return getRandomInt(30, 100);
}

function Lcolor(color) {
  if (color === "Brown") return getRandomInt(90, 100);
  if (color === "Black") return getRandomInt(90, 100);
  if (color === "Grey") return getRandomInt(90, 80);
  if (color === "White") return getRandomInt(0, 30);
  return getRandomInt(40, 50);
}

/// Load Nouns here
let noun = [];

function colorgen() {
  let newcolor = color[Math.floor(Math.random() * color.length)];
  let newnoun = noun[Math.floor(Math.random() * noun.length)];

  document.getElementById("p1").innerHTML = newcolor;
  document.getElementById("p2").innerHTML = newnoun;

  let print = `hsl(${Hcolor(newcolor).toString()}, ${Scolor(
    newcolor
  ).toString()}%, ${(100 - Lcolor(newcolor)).toString()}%)`;
  document.getElementById("p1").style.color = print;
  document.getElementById("p2").style.color = print;

  if (newnoun.substring(0, 1) === "G") {
    document
      .getElementById("a1")
      .setAttribute("href", "https://classicreload.com/macex-glider-pro.html");
  }
}

function main() {
  setTimeout(() => document.querySelector(".button").classList.remove("animated", "bounceIn")
  , 1000);

  fetch("/colornamegen/words.txt")
    .then((r) => {
      r.text().then((r) => {
        noun = r.split("\n");
        colorgen();
      });
    })
    .catch((e) => {
      console.error(e);
      document.getElementById("p1").innerHTML = "Error Loading";
      document.getElementById("p2").innerHTML = "";
    });
}

const node = document.querySelectorAll('.button')[0]
node.addEventListener('click', () => {
  colorgen();
  node.classList.add("animate__animated", "animate__jackInTheBox", "animate__faster");
  node.addEventListener("animationend", () => {
    node.classList.remove("animate__animated", "animate__jackInTheBox", "animate__faster");
    node.removeEventListener("animationend", () => {});
  });
})


document.querySelector('#ddg').addEventListener('click', () => {
  window.open(
    "https://duckduckgo.com/?q=" +
      document.getElementById("p2").textContent.toString() +
      "&ia=definition",
    "_blank"
  );
});

main();
