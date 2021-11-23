// Automaticly update Github Last Update Date :P

fetch("https://api.github.com/users/basicprogrammer10")
  .then((r) => r.json())
  .then((r) => {
    let lastUpdate = r["updated_at"].split("-");
    let data = `${lastUpdate[1]}/${lastUpdate[2].split("T")[0]}/${
      lastUpdate[0]
    }`;
    document.querySelector(
      "html body div.projects a button#github.item span.date"
    ).innerHTML = data;
  });

VanillaTilt.init(document.querySelectorAll(".item"), {
  speed: 400,
  scale: 1.07,
  glare: true,
  "max-glare": 0.1
});
