const TIME_UNITS = [
  ["s", 60],
  ["m", 60],
  ["h", 24],
];

let isLiked = false;
let doc = "";

function loadPath(path) {
  let out = `<a href="/writing">writing</a> `;
  let point = "/writing";

  let pathParts = path.split("/");
  pathParts.forEach((item) => {
    point += `/${item}`;
    out += ` <i class='fa fa-angle-right'></i> <a href="${point}">${item}</a>`;
  });

  return out;
}

function loadTime(time) {
  for (let i = 0; i < TIME_UNITS.length; i++) {
    let unit = TIME_UNITS[i];
    if (time < unit[1]) return `${Math.round(time)}${unit[0]}`;

    time /= unit[1];
  }
  return `${Math.round(time)}h`;
}

function setLikes(likes, ogLiked) {
  isLiked = ogLiked;
  let likeButton = document.querySelector("[like-button]");
  if (likes == null) return;

  updateLikesUI(likes, ogLiked);
  likeButton.addEventListener("click", () => {
    isLiked = !isLiked;
    updateLikesUI(likes, ogLiked);
    updateLikesApi(isLiked);
  });
}

function updateLikesUI(likes, ogLiked) {
  let likeContent = document.querySelector("[like-content]");

  if (isLiked && !ogLiked) likes = likes + 1;
  if (!isLiked && ogLiked) likes = likes - 1;

  if (isLiked)
    likeContent.innerHTML = `<i class="icon fa fa-heart"></i> <p>${likes}</p>`;
  else
    likeContent.innerHTML = `<i class="icon fa fa-heart-o"></i> <p>${likes}</p>`;
}

function updateLikesApi(isLiked) {
  let doc = window.location.pathname.split("/writing/");
  doc.shift();
  doc = doc.join("/writing/");

  fetch("/api/writing/like", {
    method: "POST",
    body: JSON.stringify({ doc, like: isLiked }),
    headers: {
      "Content-Type": "application/json",
    },
  });
}

const TAGS = ["H1", "H2", "H3", "H4", "H5"];
function initContents() {
  let contentsButton = [
    document.querySelector("[contents-button]"),
    document.querySelector("[contents-button-icon]"),
  ];
  let contentsDropdown = document.querySelector("[contents-dropdown]");
  let elements = Array.from(document.querySelector("article").children);

  elements.forEach((e) =>
    TAGS.forEach((eName, offset) => {
      if (e.tagName !== eName) return;

      let ele = document.createElement("div");
      ele.innerText = e.innerText;
      ele.style.paddingLeft = `${2 * offset}em`;

      ele.addEventListener("click", () => {
        window.location = e.children[0].href;
      });

      contentsDropdown.appendChild(ele);
    })
  );

  document.addEventListener("click", (e) => {
    if (contentsButton.includes(e.target)) {
      contentsDropdown.classList.toggle("hidden");
      return;
    }

    contentsDropdown.classList.add("hidden");
  });
}
