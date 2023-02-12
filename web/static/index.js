// Automaticly update Github Last Update Date :P

let load = false;
let data = null;

fetch("https://api.github.com/users/basicprogrammer10")
  .then((r) => r.json())
  .then((r) => {
    let lastUpdate = r["updated_at"].split("-");
    data = `${lastUpdate[1]}-${lastUpdate[2].split("T")[0]}-${lastUpdate[0]}`;

    if (load)
      document.querySelector(
        "html body div.container div.projects a button#github.item div.info div.date p"
      ).innerHTML = data;
  });

window.addEventListener("load", () => {
  load = true;

  VanillaTilt.init(document.querySelectorAll(".item"), {
    max: 5,
    scale: 1.02,
    glare: true,
    "max-glare": 0.1,
  });

  if (data)
    document.querySelector(
      "html body div.container div.projects a button#github.item div.info div.date p"
    ).innerHTML = data;
});
